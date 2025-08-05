use std::collections::HashMap;

use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

use crate::ast::ast::Expression;

pub mod compiler;

// LLVM Codegen

struct CodegenContext<'ctx> {
    builder: inkwell::builder::Builder<'ctx>,
    module: inkwell::module::Module<'ctx>,
    context: &'ctx inkwell::context::Context,
    variables: HashMap<String, PointerValue<'ctx>>,
    functions: HashMap<String, FunctionValue<'ctx>>,
}


impl<'ctx, 'src> Expression<'src> {
    fn codegen(&self, ctx: &mut CodegenContext<'ctx>) -> BasicValueEnum<'ctx> {
        match self {
            Expression::VariableName(name) => {
                todo!()
            },

            Expression::Integer(_) => {
                todo!()
            },

            Expression::Float(_) => {
                todo!()
            },

            Expression::String(_) => {
                todo!()
            },

            Expression::Bool(_) => {
                todo!()
            },

            Expression::Negatation(expression) => {
                todo!()
            },

            Expression::Add(expression, expression1) => {
                todo!()
            },

            Expression::Substract(expression, expression1) => {
                todo!()
            },

            Expression::Multiply(expression, expression1) => {
                todo!()
            },

            Expression::Divide(expression, expression1) => {
                todo!()
            },

            Expression::Var { name, rhs, then } => {
                todo!()
            },

            Expression::Function { name, args, body, then } => {
                todo!()
            },

            Expression::Unit => {
                todo!()
            },
        }
    }
}