use std::{any::type_name, collections::HashMap};

pub mod product;
pub mod sum;
pub mod block;
pub mod fndef;
pub mod method;

#[macro_export]
macro_rules! s {
    ($name: ident {$(
        $field: ident : $typ: ty
    $(,)?),*}) => {
        #[derive(Clone, PartialEq, serde::Serialize, serde:: Deserialize)]
        pub struct $name {
            $(pub $field : $typ),*    
        }

        impl crate::ir::Documented for $name {
            fn document(map: &mut std::collections::HashMap<String, String>){
                let mut should_recurse = false;
                map.entry(std::any::type_name::<Self>().into()).or_insert_with(||{
                    should_recurse = true;
                    let mut buf = "{\n".into();
                        $(buf += format!("\t\"{}\" : {}\n", stringify!($field), std::any::type_name::<$typ>()).as_str();)*
                    buf += "}\n";
                    buf
                });
                if !should_recurse {return}
                $(<$typ as crate::ir::Documented>::document(map);)*
            }
        }
    };
}

#[macro_export]
macro_rules! e {
    ($name: ident {$(
        $variant: ident $(($($typs: ty),*))? $({$($fields: ident : $ftyps: ty),*$(,)?})?
    $(,)?),*}) => {
        #[derive(Clone, PartialEq, serde::Serialize, serde:: Deserialize)]
        pub enum $name {
            $($variant $(($($typs),*))? $({$($fields : $ftyps),*})?),*    
        }
        
        impl crate::ir::Documented for $name {
        fn document(map: &mut std::collections::HashMap<String, String>) { 
                let mut should_recurse = false;
                map.entry(std::any::type_name::<Self>().into()).or_insert_with(||{
                    should_recurse = true;
                    let mut buf = "\n".into();
                    $(
                        buf += format!("\t|\"{}\"", stringify!($variant)).as_str(); 
                        $(buf += ": ["; $(buf += std::any::type_name::<$typs>();)* buf += "]\n";)?    
                        $(buf += ": {\n"; $(buf += format!("\t\t\"{}\" : {}\n", stringify!($fields), std::any::type_name::<$ftyps>()).as_str();)* buf += "\t}\n";)?    
                    )*
                    buf
                });
                if !should_recurse {return}
                $($($(<$typs as crate::ir::Documented>::document(map);)*)?)*
                $($($(<$ftyps as crate::ir::Documented>::document(map);)*)?)*
            }
        }
    };
}



pub trait Documented: std::any::Any {
    fn document(map: &mut HashMap<String, String>);
}

macro_rules! easy {
    ($typ: ty) => {
        impl Documented for $typ {
           fn document(map: &mut HashMap<String, String>) {
                map.entry(type_name::<Self>().into()).or_insert_with(|| {format!("<{}>", type_name::<$typ>().to_uppercase())});
            } 
        }
    };
}

macro_rules! tuples {
    ($one: ident) => {

    };
    ($last: ident $($others: ident)*) => {
        impl<$last : Documented $(, $others : Documented)*> Documented for ($last $(,$others)*) {
            fn document(map: &mut HashMap<String, String>) {
               map.entry(std::any::type_name::<Self>().into())
                .or_insert_with(|| {
                    let mut buf = "<".into();
                    buf += std::any::type_name::<$last>();
                    buf += ">";
                    $(
                        buf += ", <";
                        buf += std::any::type_name::<$others>();
                        buf += ">";
                    )*
                    buf
                });
            }
        }
        tuples!($($others)*);
    };
}

tuples!(A B C);

easy!(String);
easy!(bool);

impl<T:Documented> Documented for Vec<T> {
    fn document(map: &mut HashMap<String, String>){ 
        map.entry(type_name::<Self>().into()).or_insert_with(|| {format!("[{} ...]", type_name::<T>())});
        T::document(map);
    }
}

impl<T:Documented> Documented for Box<T> {
    fn document(map: &mut HashMap<String, String>) {
        T::document(map)
    }
}

impl<T:Documented> Documented for Option<T> {
    fn document(map: &mut HashMap<String, String>) {
        map.entry(type_name::<Self>().into()).or_insert_with(|| {format!("nil | {}", type_name::<T>())});
        T::document(map);
    }
}

impl <A: Documented, B: Documented> Documented for HashMap<A, B> {
    fn document(map: &mut HashMap<String, String>) {
        map.entry(type_name::<Self>().into()).or_insert_with(|| {format!("{{{} -> {}}}", type_name::<A>(), type_name::<B>())});
        A::document(map);
        B::document(map);
    }
}

s! (
    Code {
        defs: Vec<Definition>
    }
);

e! (Definition {
    TypeDef(TypeDef, Vec<method::Method>),
    FnDef(fndef::FnDef),
    ExternFn,
});

e! (TypeDef {
    Prod(product::Product),
    Sum(sum::Sum)
});

e! (Typ {
    BigSelf,
    SmallSelf,
    Tuple(Vec<Typ>),
    AnonStruct(Vec<(String, Typ)>),
    Expansion(Vec<String>),
    Ref(Box<Typ>),
    MutRef(Box<Typ>)
});

e! (Value {
    Tuple(Vec<Value>),
    AnonStruct(Vec<(String, Value)>),
    Literal(String),
    TypLit(Typ, Box<Value>)
});
