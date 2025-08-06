use chumsky::Parser;
use logos::Logos;

use crate::{
    language_frontend::lexer::tokens::Token, language_frontend::abstract_syntax_tree::parser::parser};

use crate::language_frontend::abstract_syntax_tree::ast::{eval, Expression};   

mod language_frontend;

/*
Simple Compiler -> 4 Stages:
- lex
- parse
- type-check
- translate to machine instructions
*/

fn main() {
    let sourcecode = std::fs::read_to_string("sample.akai").unwrap();
    let lexer = Token::lexer(&sourcecode);

    let mut tokens = vec![];
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => tokens.push(token),
            Err(e) => {
                println!("lexer error at {:?}: {:?}", span, e);
                return;
            }
        }
    }

    match parser().parse(&tokens).into_result() {
        Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
            Ok(output) => println!("{output}"),
            Err(eval_err) => println!("Evaluation error: {eval_err}"),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|err| println!("Parse error: {err}")),
    }; 

    //println!("\n[result]\n{}", abstract_syntax_tree::ast::eval(absyntr, vars, funcs));
}
