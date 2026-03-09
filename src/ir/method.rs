use crate::s;
use crate::e;


s! (Method {
    self_typ: SelfTyp,
    function: super::fndef::FnDef
});

e! (SelfTyp {
    Owned,
    Ref,
    MutRef,
    NotPresent
});
