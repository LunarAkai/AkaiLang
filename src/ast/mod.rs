use std::ops::Range;

use crate::ast::op::Op;


pub mod evaluator;
pub mod op;


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
    }
}
