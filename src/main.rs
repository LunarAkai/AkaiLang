use chumsky::Parser;
use chumsky::input::{Input, Stream};
use logos::Logos;

mod language_frontend;
mod llvm_backend;

use crate::{
    language_frontend::abstract_syntax_tree::parser::parse, language_frontend::lexer::tokens::Token,
};

use crate::language_frontend::abstract_syntax_tree::ast::Expr;

/*
Simple Compiler -> 4 Stages:
- lex
- parse
- type-check
- translate to machine instructions
*/

fn main() {
    let sourcecode = std::fs::read_to_string("sample.akai").unwrap();
    //println!("Token Stream:");
    //for (token, span) in token_iter.clone() {
    //    println!("{:?} at {:?}", token, span);
    //}

    println!("{:?}", sourcecode);

    match parse(&sourcecode) {
        Ok(res) => println!("{:#?}", res),
        Err(e) => {
            panic!("{:#?}", e)
        }
    };
}
