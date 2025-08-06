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

            Expression::Add(
                lhs, 
                rhs
            ) => {
                let l = lhs.codegen(ctx).into_int_value();
                let r = rhs.codegen(ctx).into_int_value();
                ctx.builder.build_int_add(l, r, "addtmp").unwrap().into()
            },

            Expression::Substract(
                lhs, 
                rhs
            ) => {
                todo!()
            },

            Expression::Multiply(
                lhs, 
                rhs
            ) => {
                todo!()
            },

            Expression::Divide(
                lhs, 
                rhs
            ) => {
                todo!()
            },

            Expression::Var { 
                name, 
                rhs, 
                then 
            } => {
                let value = rhs.codegen(ctx);
                let ptr = ctx.builder.build_alloca(ctx.context.f32_type(), name).unwrap();
                let _ = ctx.builder.build_store(ptr, value);
                ctx.variables.insert(name.to_string(), ptr);
                then.codegen(ctx)
            },

            Expression::Function { 
                name, 
                args, 
                body, 
                then 
            } => {
                todo!()
            },

            Expression::Unit => {
                todo!()
            },
        }
    }
}