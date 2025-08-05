use crate::ast::Expression;



impl<'src> Expression<'src> {
    pub fn eval(&self) -> String {
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
        }
    }
}