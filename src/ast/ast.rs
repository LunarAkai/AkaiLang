/// Abstract Syntax Tree
#[derive(Debug)]
pub enum Expression<'src> {
    VariableName(&'src str),
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),

    Negatation(Box<Expression<'src>>),
    Add(Box<Expression<'src>>, Box<Expression<'src>>),
    Substract(Box<Expression<'src>>, Box<Expression<'src>>),
    Multiply(Box<Expression<'src>>, Box<Expression<'src>>),
    Divide(Box<Expression<'src>>, Box<Expression<'src>>),
    
    Var {
        name: &'src str,
        rhs: Box<Expression<'src>>,
        then: Box<Expression<'src>>,
    },

    Function {
        name: &'src str,
        args: Vec<&'src str>,
        body: Box<Expression<'src>>,
        then: Box<Expression<'src>>,
    },

    Unit
}

impl<'src> Expression<'src> {
    pub fn evaluate(&self) -> String {
        match self {
            Expression::VariableName(_) => todo!(),

            Expression::Integer(_) => todo!(),

            Expression::Float(_) => todo!(),

            Expression::String(_) => todo!(),

            Expression::Bool(_) => todo!(),

            Expression::Negatation(expression) => todo!(),

            Expression::Add(expression, expression1) => todo!(),

            Expression::Substract(expression, expression1) => todo!(),

            Expression::Multiply(expression, expression1) => todo!(),

            Expression::Divide(expression, expression1) => todo!(),

            Expression::Var { name, rhs, then } => todo!(),

            Expression::Function { name, args, body, then } => todo!(),

            Expression::Unit => todo!(),
        }
    }
}
