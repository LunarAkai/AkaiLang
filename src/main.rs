use chumsky::input::{Input, Stream};
use chumsky::prelude::end;
use chumsky::Parser;
use logos::Logos;

use crate::{
    language_frontend::lexer::tokens::Token, language_frontend::abstract_syntax_tree::parser::parser};

use crate::language_frontend::abstract_syntax_tree::ast::{Expr};   

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

    // Create a logos lexer over the source code
    let token_iter = Token::lexer(&sourcecode)
        .spanned()
        // Convert logos errors into tokens. We want parsing to be recoverable and not fail at the lexing stage, so
        // we have a dedicated `Token::Error` variant that represents a token error that was previously encountered
        .map(|(tok, span)| match tok {
            // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
            // to work with
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

    //println!("Token Stream:");
    //for (token, span) in token_iter.clone() {
    //    println!("{:?} at {:?}", token, span);
    //}

    // Turn the token iterator into a stream that chumsky can use for things like backtracking
    let token_stream = Stream::from_iter(token_iter)
        // Tell chumsky to split the (Token, SimpleSpan) stream into its parts so that it can handle the spans for us
        // This involves giving chumsky an 'end of input' span: we just use a zero-width span at the end of the string
        .map((0..sourcecode.len()).into(), |(t, s): (_, _)| (t, s));

    println!("{:?}", sourcecode);

    /*
    let lexer =  Token::lexer(&sourcecode)
        .spanned();
        //.collect::<Vec<_>>();
    
    for token in lexer {
        println!("{:?}", token);
    } 
    */ 

    match parser().then_ignore(end()).parse(token_stream).into_result() {
        Ok(res) => println!("{:?}", res),
        Err(e) => {
            panic!("{:#?}", e)
        }
    }; 
}
