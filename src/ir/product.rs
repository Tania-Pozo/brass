use std::collections::HashMap;
use crate::{s, e};

s!(Product {
    name: String,
    fields: ProductFields,
});


e! (ProductFields {
    Tuple(Vec<super::Typ>),
    Named(HashMap<String, super::Typ>)
});
