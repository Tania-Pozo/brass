use crate::e;
use crate::s;

s!(Method {
    self_typ: SelfTyp,
    function: super::fndef::FnDef
});

e!(SelfTyp {
    Owned,
    Ref,
    MutRef,
    NotPresent
});
