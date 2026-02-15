
pub struct Sum {
    name: String,
    variants: Vec<Variant>
}

pub struct Variant {
    name: String,
    raw_val: Option<super::Value>,
}
