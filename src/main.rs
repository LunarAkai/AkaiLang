use chumsky::input::{Input, Stream};
use chumsky::Parser;
use logos::Logos;

mod language_frontend;

use crate::{
    language_frontend::lexer::tokens::Token, language_frontend::abstract_syntax_tree::parser::parse};

use crate::language_frontend::abstract_syntax_tree::ast::{Expr};  

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

    
    let lexer =  Token::lexer(&sourcecode)
        .spanned()
        .collect::<Vec<_>>();
    
    for token in lexer {
        println!("{:?}", token);
    } 
    
    let token_iter = Token::lexer(&sourcecode).spanned().map(|(tok, span)| tok.map(|t| (t, span))).filter_map(Result::ok);

    let token_stream = Stream::from_iter(token_iter)
        // Tell chumsky to split the (Token, SimpleSpan) stream into its parts so that it can handle the spans for us
        // This involves giving chumsky an 'end of input' span: we just use a zero-width span at the end of the string
        .map((0..sourcecode.len()).into(), |(t, s): (_, _)| (t, s));


    match parse(&sourcecode)  {
        Ok(res) => println!("{:?}", res),
        Err(e) => {
            panic!("{:#?}", e)
        }
    }; 
}
