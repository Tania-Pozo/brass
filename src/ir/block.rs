pub struct ArgsDef(Vec<(String, super::Typ)>);
pub struct ArgsApp(Vec<Expr>);

pub struct Block(Vec<Statement>);

pub enum Statement {
    Return(Expr),
    FnApp(String, ArgsApp),
    LetBinding {
        mutable: bool,
        name: String,
        expr: Expr
    },
    Asignment(Expr, Expr),
    // This one's weird
    ForLoop {
        expr: Expr,
        block: Block
    },
    IfBlock(IfStmt),
    WhileLoop(Expr, Block),
    Loop(Block),

    Break,
    Continue,

}

pub struct IfStmt(Expr, Block, Option<Box<ElseStmt>>);

enum ElseStmt {
    Unconditional(Block),
    Conditional(IfStmt)
}

pub enum Expr {
    Variable(String),
    Operation(Box<(Expr, String, Expr)>),
    FnApp(Box<Expr>, ArgsApp),
    Reference{expr: Box<Expr>, mutable: bool},
    Dereference{expr: Box<Expr>, mutable: bool}

}
