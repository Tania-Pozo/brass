
pub struct Token<'a> {
    pub source: &'a str,
    pub kind: TokenKind<'a>
}

pub enum TokenKind<'a> {
    NumberLiteral,
    Ident,
    Keyword,
    SChar,
    StringLit(Option<&'a str>)
}


