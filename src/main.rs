use chumsky::Parser;
use logos::Logos;

use crate::{parser::parser, tokens::Token};

mod tokens;
mod ast;
mod parser;
mod code_generation;

fn main() {
    let lexer = Token::lexer("(1 + 1) * 3");

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

    let ast = match parser().parse(&tokens).into_result() {
        Ok(expr) => {
            println!("[AST]\n{:#?}", expr);
            expr
        }
        Err(e) => {
            println!("parse error: {:#?}", e);
            return;
        }
    };

    println!("\n[result]\n{}", ast.eval());
}
