use crate::e;
use crate::ir::Typ;
use crate::s;

s! (ArgsDef {args: Vec<(String, super::Typ)>});
s! (ArgsApp{args: Vec<Expr>});

s! (Block{block: Vec<Statement>});

e! ( Statement {
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

});

s! (IfStmt{if_expr: Expr, block: Block, else_stmt: Option<Box<ElseStmt>>});

e! (ElseStmt {
    Unconditional(Block),
    Conditional(IfStmt)
});

e! (Expr {
    Variable(String),
    Operation(Box<(Expr, String, Expr)>),
    FnApp(Box<Expr>, ArgsApp),
    Reference{expr: Box<Expr>, mutable: bool},
    Dereference{expr: Box<Expr>, mutable: bool},
    UntypedLiteral(Literal),
    TypedLiteral(Typ, TypedLit),
    FieldAccess(Box<Expr>, String)
});

e! ( Literal {
    Num(String),
    StringLit(String),
    Tuple(Vec<Expr>),
});

e! (TypedLit {
    Literal(Literal),
    StructLit(Vec<(String, Expr)>)
});
