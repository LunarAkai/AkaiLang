use crate::ast::Expression;


impl Expression {
    pub fn eval(&self) -> isize {
        match self {
            Expression::Integer(n) => *n,

            
            Expression::Negate(rhs) => -rhs.eval(),

            Expression::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            Expression::Substract(lhs, rhs) => lhs.eval() - rhs.eval(),
            Expression::Multiply(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expression::Divide(lhs, rhs) => lhs.eval() / rhs.eval(),
        }
    }
}