use crate::language_frontend::abstract_syntax_tree::definitions::*;

// Option and Result -> define in Std
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // var/fun_name
    Ident(Identifier),

    // 4
    IntLiteral(i64),

    // 12.5
    FloatLiteral(f64),

    // "eepy"
    StringLiteral(String),

    // true
    BoolLiteral(bool),

    // 'c'
    CharLiteral(char),

    // var x = intPlusOne(12)
    CallExpr(Call),

    // !x
    UnaryExpr(Unary),

    // 1 + 11
    BinaryExpr(Binary),

    // x = 64
    AssignmentExpr(Assignment),

    // var foo = "bar"
    VarExpr(Var),

    // if(...) { ... } else { ... }
    Condition(Condition),

    // fun helloWorld() { ... }
    FunctionExpr(Function),

    // ->
    ReturnExpr,

    Error,
}
