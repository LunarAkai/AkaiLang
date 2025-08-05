pub mod evaluator;

#[derive(Debug)]
pub enum Expression {
    Integer(isize),
    Negate(Box<Expression>),
    // Binary operators,
    Add(Box<Expression>, Box<Expression>),
    Substract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}
