use crate::{e, s};
use std::collections::HashMap;

s!(Product {
    name: String,
    fields: ProductFields,
});

e! (ProductFields {
    Tuple(Vec<super::Typ>),
    Named(HashMap<String, super::Typ>)
});
