use std::collections::HashMap;


pub struct Product {
    pub name: String,
    pub fields: ProductFields,
}


pub enum ProductFields {
    Tuple(Vec<super::Typ>),
    Named(HashMap<String, super::Typ>)
}
