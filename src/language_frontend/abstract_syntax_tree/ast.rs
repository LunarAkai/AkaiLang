use crate::language_frontend::abstract_syntax_tree::definitions::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(String),

    IntLiteral(i64),

    FloatLiteral(f64),

    StringLiteral(String),

    BoolLiteral(bool),

    CharLiteral(char),

    Result, // todo

    Option, // todo

    Ok,     // todo

    Err,    // todo

    Call(Call),

    Unary(Unary),

    Binary(Binary),

    Assignment(Assignment),

    Var(Var),

    Function(Function),

    Error,
}