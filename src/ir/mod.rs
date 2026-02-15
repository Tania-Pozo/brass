pub mod product;
pub mod sum;
pub mod block;
pub mod fndef;

pub struct Code {
    pub defs: Vec<Definition>
}

pub enum Definition {
    TypeDef((TypeDef)),
    FnDef(fndef::FnDef),
    Inclusion(),
    External()
}


pub enum TypeDef {
    Prod(product::Product),
    Sum(sum::Sum)
}

pub enum Typ {
    BigSelf,
    SmallSelf,
    Tuple(Vec<Typ>),
    AnonStruct(Vec<(String, Typ)>),
    Expansion(Vec<String>),
    Ref(Box<Typ>),
    MutRef(Box<Typ>)
}

pub enum Value {
    Tuple(Vec<Value>),
    AnonStruct(Vec<(String, Value)>),
    Literal(String),
    TypLit(Typ, Box<Value>)
}
