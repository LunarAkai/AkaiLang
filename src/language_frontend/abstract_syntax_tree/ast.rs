use std::rc::Rc;

use chumsky::span::Span;



/// Abstract Syntax Tree
pub type BlockStatement = Vec<Statement>;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(String),

    IntLiteral(i64),

    FloatLiteral(f64),

    StringLiteral(String),

    BoolLiteral(bool),

    CharLiteral(char),

    Null,


    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },

    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },

    Binary {
        lhs: Box<Expr>,
        operator: BinaryOp,
        rhs: Box<Expr>,
    },

    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub name: Rc<str>,
    pub parameters: Vec<Expr>,
    pub(crate) args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Multiply,
    Divide,
    Add,
    Substract,

    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Not,
    Minus,
    Plus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    UnsignedInteger(u64),
    Bool(bool),
    Char(char),
    String(Rc<str>),
    Int(i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident(pub Rc<str>);

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Var(Ident, Option<Type>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct While {
    pub condition: Expr,
    pub body: BlockStatement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Condition {
    pub condition: Expr,
    pub if_body: BlockStatement,
    pub else_body: Option<BlockStatement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    UnsignedInteger,
    SignedInteger,
    Bool,
    Char,
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    UnsignedInteger(u32),
    SignedInteger(i32),
    Bool(bool),
    Char(char),
    String(String),
}

impl Value {
    pub fn is_type(&self, ty: &Type) -> bool {
        match (ty, self) {
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Char, Value::Char(_)) => true,
            (Type::SignedInteger, Value::SignedInteger(_)) => true,
            (Type::UnsignedInteger, Value::UnsignedInteger(_)) => true,
            (Type::String, Value::String(_)) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Rc<str>,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}