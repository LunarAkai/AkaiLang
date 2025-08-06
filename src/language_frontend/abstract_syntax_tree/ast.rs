/// Abstract Syntax Tree

#[derive(Debug)]
pub enum Expression<'src> {
    Ident(&'src str),

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

    Unit,
}

pub fn eval<'src>(
    expr: &'src Expression<'src>,
    vars: &mut Vec<(&'src str, f64)>,
    funcs: &mut Vec<(&'src str, &'src [&'src str], &'src Expression<'src>)>,
) -> Result<f64, String> {
    match expr {
        Expression::Ident(_) => todo!(),

        Expression::Integer(x) => Ok((*x) as f64),

        Expression::Float(_) => todo!(),

        Expression::String(_) => todo!(),

        Expression::Bool(_) => todo!(),

        Expression::Negatation(lhs) => todo!(),

        Expression::Add(lhs, rhs) => Ok(eval(lhs, vars, funcs)? + eval(rhs, vars, funcs)?),

        Expression::Substract(lhs, rhs) => Ok(eval(lhs, vars, funcs)? - eval(rhs, vars, funcs)?),

        Expression::Multiply(lhs, rhs) => Ok(eval(lhs, vars, funcs)? * eval(rhs, vars, funcs)?),

        Expression::Divide(lhs, rhs) => Ok(eval(lhs, vars, funcs)? / eval(rhs, vars, funcs)?),

        Expression::Var { name, rhs, then } => todo!(),

        Expression::Function {
            name,
            args,
            body,
            then,
        } => todo!(),

        Expression::Unit => todo!(),
    }
}
