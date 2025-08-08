use chumsky::input::{Input, Stream};
use chumsky::prelude::end;
use chumsky::Parser;
use logos::Logos;

use crate::language_frontend::abstract_syntax_tree::parser::parse;

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
    //println!("Token Stream:");
    //for (token, span) in token_iter.clone() {
    //    println!("{:?} at {:?}", token, span);
    //}

    println!("{:?}", sourcecode);

    /*
    let lexer =  Token::lexer(&sourcecode)
        .spanned();
        //.collect::<Vec<_>>();
    
    for token in lexer {
        println!("{:?}", token);
    } 
    */ 

    match parse(&sourcecode) {
        Ok(res) => println!("{:?}", res),
        Err(e) => {
            panic!("{:#?}", e)
        }
    }; 
}
