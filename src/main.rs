use chumsky::Parser;
use logos::Logos;

use crate::{language_frontend::parser::parser, language_frontend::tokens::Token};

mod language_frontend;

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

    println!("\n[result]\n{}", ast.evaluate());
}
