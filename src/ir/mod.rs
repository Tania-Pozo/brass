
pub mod block;
pub mod fndef;
pub mod method;
pub mod product;
pub mod sum;

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
                map.entry(stringify!($name).into()).or_insert_with(||{
                    should_recurse = true;
                    let mut buf = "{\n".into();
                        $(buf += format!("\t\"{}\" : {}\n", stringify!($field), crate::ir::tn2::<$typ>()).as_str();)*
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
                map.entry(stringify!($name).into()).or_insert_with(||{
                    should_recurse = true;
                    let mut buf = "\n".into();
                    $(
                        buf += format!("\t|\"{}\"", stringify!($variant)).as_str();
                        $(buf += ": ["; $(buf += crate::ir::tn2::<$typs>().as_str(); buf += ", ";)* buf += "]";)?
                        $(buf += ": {\n"; $(buf += format!("\t\t\"{}\" : {}\n", stringify!($fields), crate::ir::tn2::<$ftyps>()).as_str();)* buf += "\t}";)?
                        buf += "\n";
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
    fn document(map: &mut std::collections::HashMap<String, String>);
}

macro_rules! easy {
    ($typ: ty) => {
        impl Documented for $typ {
            fn document(map: &mut HashMap<String, String>) {
                map.entry(stringify!($typ).into())
                    .or_insert_with(|| format!("<{}>", tn2::<$typ>().to_uppercase()));
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
                let mut buf = "(".into();
                buf += crate::ir::tn2::<$last>().split("::").last().unwrap();
                $(
                    buf += ", ";
                    buf += crate::ir::tn2::<$others>().split("::").last().unwrap();
                )*
                buf += ")";
               map.entry(buf)
                .or_insert_with(|| {
                    let mut buf = "[<".into();
                    buf += crate::ir::tn2::<$last>().as_str();
                    buf += ">";
                    $(
                        buf += ", <";
                        buf += crate::ir::tn2::<$others>().as_str();
                        buf += ">";
                    )*
                    buf += "]";
                    buf
                });
            }
        }
        tuples!($($others)*);
    };
}

use std::collections::HashMap;

tuples!(A B C);

easy!(String);
easy!(bool);

pub fn tn2<T: std::any::Any>() -> String {
    let full = std::any::type_name::<T>();
    let mut result = String::new();
    let mut chars = full.chars().peekable();
    let mut segment = String::new();

    while let Some(c) = chars.next() {
        match c {
            ':' if chars.peek() == Some(&':') => {
                chars.next();
                segment.clear();
            }
            '<' | '>' | '(' | ')' | ',' | ' ' => {
                result.push_str(&segment);
                segment.clear();
                result.push(c);
            }
            _ => segment.push(c),
        }
    }
    result.push_str(&segment);
    result
}

impl<T: Documented> Documented for Vec<T> {
    fn document(map: &mut HashMap<String, String>) {
        map.entry(tn2::<Self>())
            .or_insert_with(|| format!("[{} ...]", tn2::<T>()));
        T::document(map);
    }
}

impl<T: Documented> Documented for Box<T> {
    fn document(map: &mut HashMap<String, String>) {
        T::document(map)
    }
}

impl<T: Documented> Documented for Option<T> {
    fn document(map: &mut HashMap<String, String>) {
        map.entry(tn2::<Self>())
            .or_insert_with(|| format!("null | {}", tn2::<T>()));
        T::document(map);
    }
}

impl<A: Documented, B: Documented> Documented for HashMap<A, B> {
    fn document(map: &mut HashMap<String, String>) {
        map.entry(tn2::<Self>())
            .or_insert_with(|| format!("{{{} -> {}}}", tn2::<A>(), tn2::<B>()));
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
