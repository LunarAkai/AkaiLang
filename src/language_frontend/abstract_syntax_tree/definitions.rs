use std::{ops::Range, rc::Rc};

use crate::language_frontend::abstract_syntax_tree::ast::Expr;

// Abstract Syntax Tree

pub type BlockExpression = Vec<Expr>;

pub type Span = Range<usize>;

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Multiply,
    Divide,
    Add,
    Substract,

    // Comparision
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    // Logical
    And,
    Or,
    // todo: bitwise
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Not,
    Negate,
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
pub enum Type {
    Integer,
    Float,
    Bool,
    Char,
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
}

impl Value {
    pub fn is_type(&self, ty: &Type) -> bool {
        match (ty, self) {
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Char, Value::Char(_)) => true,
            (Type::Integer, Value::Integer(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::String, Value::String(_)) => true,
            _ => false,
        }
    }
}

//---------------------------------------
//      Structs
//---------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: BlockExpression,
}

/// Example: `if (a > 12) { ... }`
#[derive(Clone, Debug, PartialEq)]
pub struct Condition {
    pub condition: Box<Expr>,
    pub if_body: BlockExpression,
    pub else_body: Option<BlockExpression>,
}

/// Example: `x = y`
#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub target: String,
    pub value: Box<Expr>,
}

/// Example: `var String: foo = "Test"`
#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub ty: Option<Type>,
    pub ident: Identifier,
    pub value: Box<Expr>,
}

/// Example: `x++`
#[derive(Clone, Debug, PartialEq)]
pub struct Unary {
    pub operator: UnaryOp,
    pub operand: Box<Expr>,
}

/// Example: `1 + 1`
#[derive(Clone, Debug, PartialEq)]
pub struct Binary {
    pub lhs: Box<Expr>,
    pub operator: BinaryOp,
    pub rhs: Box<Expr>,
}

/// Represents the Structure of a `Function` in AkaiLang
///
/// Examples:
///```AkaiLang
///fun helloWorld() {
///    print("Hello World")
///}
///```
/// <br>
///
///```AkaiLang
///fun returnsIntPlusOne(i: int): int {
///    -> i + 1
///}
///```
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Option<Vec<(Identifier, Type)>>,
    pub return_type: Option<Type>,
    pub body: Option<BlockExpression>,
    pub body_expr: Option<Box<Expr>>, // ' -> (return)'
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    /// name of the function being called
    pub name: Box<Identifier>,

    /// arguments supplied
    pub arguments: Vec<Expr>,
}
