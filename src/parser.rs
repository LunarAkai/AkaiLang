use chumsky::{prelude::{just, recursive}, recursive, select, IterParser, Parser};

use crate::{ast::Expression, tokens::Token};


#[allow(clippy::let_and_return)]
/* ANCHOR: parser */
pub fn parser<'src>(
) -> impl Parser<'src, &'src [Token<'src>], Expression, chumsky::extra::Err<chumsky::error::Simple<'src, Token<'src>>>>
{
    recursive(
        |p| 
        {
            let atom = {
            let parenthesized = p
                .clone()
                .delimited_by(just(Token::ParenBegin), just(Token::ParenEnd));

            let integer = select! {
                Token::Integer(n) => Expression::Integer(n),
            };

            parenthesized.or(integer)
        };

        let unary = just(Token::Substract)
            .repeated()
            .foldr(atom, |_op, rhs| Expression::Negate(Box::new(rhs)));

        let binary_1 = unary.clone().foldl(
            just(Token::Multiply)
                .or(just(Token::Divide))
                .then(unary)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Multiply => Expression::Multiply(Box::new(lhs), Box::new(rhs)),
                Token::Divide => Expression::Divide(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let binary_2 = binary_1.clone().foldl(
            just(Token::Add)
                .or(just(Token::Substract))
                .then(binary_1)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Add => Expression::Add(Box::new(lhs), Box::new(rhs)),
                Token::Substract => Expression::Substract(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        binary_2
    })
}