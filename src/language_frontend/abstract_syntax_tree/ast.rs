use crate::language_frontend::abstract_syntax_tree::definitions::*;

// Option and Result -> define in Std
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(String),

    IntLiteral(i64),

    FloatLiteral(f64),

    StringLiteral(String),

    BoolLiteral(bool),

    CharLiteral(char),

    CalExpr(Call),

    UnaryExpr(Unary),

    BinaryExpr(Binary),

    AssignmentExpr(Assignment),

    VarExpr(Var),

    FunctionExpr(Function),

    Error,
}
