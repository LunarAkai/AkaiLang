use chumsky::{
    combinator::Or, error::Rich, extra, input::ValueInput, prelude::{choice, just, recursive}, primitive::select, recursive, select, select_ref, span::SimpleSpan, text::{self, ascii::{ident, keyword}, whitespace}, Boxed, IterParser, Parser
};

use crate::{language_frontend::abstract_syntax_tree::ast::Expression, language_frontend::lexer::tokens::Token};

// goal of parsing is to construct an abstract syntax tree

#[allow(clippy::let_and_return)]
pub fn parser<'tokens, 'src: 'tokens, I>() 
    -> impl Parser<'tokens, I, Expression<'src>, extra::Err<Rich<'tokens, Token<'src>>>> 
where 
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let ident = select! {
        Token::Ident(s) => s,
    };


    let keyword = |kw: &'static str| {
        select! {
            Token::Keyword(k) if k == kw => (),
        }
    };
    
    let eq = just(Token::Equals).labelled("=");

    let expr = recursive(|expr| {


        let atom = select! {
            Token::Float(x) => Expression::Float(x),
            Token::Integer(x) => Expression::Integer(x),
        }.or(expr.clone().delimited_by(just(Token::ParenBegin), just(Token::ParenEnd)));


        let unary = just(Token::Substract)
            .repeated()
            .foldr(atom.clone(), |_op, rhs| Expression::Negatation(Box::new(rhs)));

        let mul_div = unary.clone().foldl(
            just(Token::Multiply).or(just(Token::Divide)).then(unary).repeated(),
            |lhs, (op, rhs)| match op {
                Token::Multiply => Expression::Multiply(Box::new(lhs), Box::new(rhs)),
                Token::Divide => Expression::Divide(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let add_sub = mul_div.clone().foldl(
            just(Token::Add).or(just(Token::Substract)).then(mul_div).repeated(),
            |lhs, (op, rhs)| match op {
                Token::Add => Expression::Add(Box::new(lhs), Box::new(rhs)),
                Token::Substract => Expression::Substract(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        add_sub
    });

    let decl = recursive(|decl| {
        let var = keyword("var")
            .ignore_then(ident)
            .then_ignore(eq.clone())
            .then(expr.clone())
            .then(decl.clone())
            .map(|((name, rhs), then)| Expression::Var {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then),
            });

        let fun = keyword("fun")
            .ignore_then(ident.clone())
            .then(ident.repeated().collect::<Vec<_>>())
            .then_ignore(eq.clone())
            .then(expr.clone())
            .then(decl)
            .map(|(((name, args), body), then)| Expression::Function {
                name,
                args,
                body: Box::new(body),
                then: Box::new(then),
            });

        var.or(fun).or(expr)
    });

    decl
}
