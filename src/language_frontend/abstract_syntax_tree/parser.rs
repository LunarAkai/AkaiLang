use chumsky::{
    combinator::Or, error::Rich, extra, input::ValueInput, prelude::{choice, just, recursive}, recursive, select, select_ref, span::SimpleSpan, text::{self, ascii::ident, whitespace}, IterParser, Parser
};

use crate::{language_frontend::abstract_syntax_tree::ast::Expression, language_frontend::lexer::tokens::Token};

// goal of parsing is to construct an abstract syntax tree

#[allow(clippy::let_and_return)]
pub fn parser<'tokens, 'src: 'tokens, I>() -> impl Parser<'tokens, I, Expression<'src>, extra::Err<Rich<'tokens, Token<'src>>>> 
where 
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    
    let expr = recursive(|expr| {
        
        let atom = select! {
            Token::Float(x) => Expression::Float(x),
            Token::Integer(x) => Expression::Integer(x),
        };

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
    expr
}
