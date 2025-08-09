use std::{ops::Range, rc::Rc};

use crate::language_frontend::abstract_syntax_tree::ast::Expr;

/// Abstract Syntax Tree
pub type BlockStatement = Vec<Statement>;

pub type Span = Range<usize>;


#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Var(Ident, Option<Type>)
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
pub struct Ident(pub Rc<str>);

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


/// Example: `x = 5` 
#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
}

/// Example: `var String: foo = "Test"`
#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub ty: Option<Type>,
    pub ident: String,
    pub value: Box<Expr>
}

/// Example: `x++`
#[derive(Clone, Debug, PartialEq)]
pub struct Unary {
    pub operator: UnaryOp,
    pub operand: Box<Expr>,
}

/// Example: `x++`
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
    pub name: Rc<str>,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
    pub body_expr: Option<Type>
}


#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
}
