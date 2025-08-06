/// Abstract Syntax Tree

#[derive(Debug)]
pub enum Expression<'src> {
    // Identifier
    Ident(&'src str),

    // Types
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),

    // Operations
    Negatation(Box<Expression<'src>>),
    Add(Box<Expression<'src>>, Box<Expression<'src>>),
    Substract(Box<Expression<'src>>, Box<Expression<'src>>),
    Multiply(Box<Expression<'src>>, Box<Expression<'src>>),
    Divide(Box<Expression<'src>>, Box<Expression<'src>>),

    // Keywords
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
        Expression::Ident(name) => {
            if let Some((_, val)) = vars.iter().rev().find(|(var, _)| var == name) {
                Ok(*val)
            } else {
                Err(format!("Cannot find variable `{name}` in scope"))
            }
        },

        // Types
        Expression::Integer(x) => Ok((*x) as f64), // todo

        Expression::Float(x) => Ok(*x),

        Expression::String(_) => todo!(),

        Expression::Bool(_) => todo!(),

        // Operations
        Expression::Negatation(lhs) => todo!(),

        Expression::Add(lhs, rhs) => Ok(eval(lhs, vars, funcs)? + eval(rhs, vars, funcs)?),

        Expression::Substract(lhs, rhs) => Ok(eval(lhs, vars, funcs)? - eval(rhs, vars, funcs)?),

        Expression::Multiply(lhs, rhs) => Ok(eval(lhs, vars, funcs)? * eval(rhs, vars, funcs)?),

        Expression::Divide(lhs, rhs) => Ok(eval(lhs, vars, funcs)? / eval(rhs, vars, funcs)?),

        // Keywords
        Expression::Var { name, rhs, then } => {
            let rhs = eval(rhs, vars, funcs)?;
            vars.push((*name, rhs));
            let output = eval(then, vars, funcs);
            vars.pop();
            output
        },

        Expression::Function {
            name,
            args,
            body,
            then,
        } => {
            funcs.push((name, args, body));
            let output = eval(then, vars, funcs);
            funcs.pop();
            output
        },

        Expression::Unit => todo!(),
    }
}
