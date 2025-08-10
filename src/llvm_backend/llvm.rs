use chumsky::prelude::todo;
use inkwell::{
    OptimizationLevel,
    builder::{self, Builder},
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FunctionValue},
};

use crate::language_frontend::abstract_syntax_tree::{
    ast::Expr,
    definitions::{Function, Type},
};

pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) exec_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str, opt_level: OptimizationLevel) -> Self {
        let module = context.create_module(module_name);
        let exec_engine = module.create_jit_execution_engine(opt_level).unwrap();
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
            exec_engine,
        }
    }
}
