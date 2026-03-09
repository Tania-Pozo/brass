
use crate::s;

s!( Sum {
    name: String,
    variants: Vec<Variant>
});

s! (Variant {
    name: String,
    raw_val: Option<super::Value>,
});
