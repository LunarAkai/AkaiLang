use std::rc::Rc;



/// Abstract Syntax Tree
pub type BlockStatement = Vec<Statement>;

#[derive(Clone, Debug)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    Call(FunctionCall),
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub parameters: Vec<Expr>,
    pub name: Rc<str>,
}

#[derive(Debug, Clone)]
pub enum ExprResult {
    Bool(bool),
    UnsignedInteger(usize),
    SignedInteger(isize),
    Char(char),
    Return(Box<ExprResult>),
    Void,
}

impl std::fmt::Display for ExprResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprResult::Bool(b) => write!(f, "{b}"),
            ExprResult::UnsignedInteger(i) => write!(f, "{i}"),
            ExprResult::SignedInteger(i) => write!(f, "{i}"),
            ExprResult::Char(c) => write!(f, "{c}"),
            ExprResult::Return(v) => write!(f, "{}", *v),
            ExprResult::Void => write!(f, ""),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Not,
    Minus,
    Plus,
}

#[derive(Clone, Debug)]
pub enum Literal {
    UnsignedInteger(usize),
    Bool(bool),
    Char(char),
    String(Rc<str>),
}

#[derive(Clone, Debug)]
pub struct Ident(pub Rc<str>);

#[derive(Clone, Debug)]
pub enum Statement {
    Var(Ident, Option<Type>)
}

#[derive(Clone, Debug)]
pub struct While {
    pub condition: Expr,
    pub body: BlockStatement,
}

#[derive(Clone, Debug)]
pub struct Condition {
    pub condition: Expr,
    pub if_body: BlockStatement,
    pub else_body: Option<BlockStatement>,
}

#[derive(Clone, Debug)]
pub enum Type {
    UnsignedInteger,
    SignedInteger,
    Bool,
    Char,
    String,
}

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

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Rc<str>,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}