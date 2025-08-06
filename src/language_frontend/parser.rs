use chumsky::{
    IterParser, Parser,
    combinator::Or,
    prelude::{choice, just, recursive},
    recursive, select,
    text::{self, ascii::ident},
};

use crate::{language_frontend::abstract_syntax_tree::ast::Expression, language_frontend::lexer::tokens::Token};

// goal of parsing is to construct an abstract syntax tree

#[allow(clippy::let_and_return)]
pub fn parser<'src>() -> impl Parser<'src, &'src [Token<'src>], Expression<'src>> {
    let ident = select! {
        Token::Ident(ident) => ident
    };

    let keyword = |kw: &'static str| {
        select! {
            Token::Keyword(k) if k == kw => ()
        }
    };

    let eq = just(Token::Equals);

    let expr = recursive(|expr| {
        let atom = {
            let parenthesized = expr
                .clone()
                .delimited_by(just(Token::ParenBegin), just(Token::ParenEnd));

            let integer = select! {
                Token::Integer(n) => Expression::Integer(n),
            };

            parenthesized.or(integer)
        };

        let unary = just(Token::Substract)
            .repeated()
            .foldr(atom, |_op, rhs| Expression::Negatation(Box::new(rhs)));

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
    });

    let decl = recursive(|decl| {
        let r#var = keyword("var")
            .ignore_then(ident.clone())
            .then_ignore(eq.clone())
            .then(decl.clone())
            .then(decl.clone())
            .map(|((name, rhs), then)| Expression::Var {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then),
            });

        let r#fun = keyword("fun")
            .ignore_then(ident.clone())
            .then(ident.clone().repeated().collect())
            .then_ignore(eq.clone())
            .then(decl.clone())
            .then(decl.clone())
            .map(|(((name, args), body), then)| Expression::Function {
                name,
                args,
                body: Box::new(body),
                then: Box::new(then),
            });

        var.or(r#fun).or(expr)
    });

    decl
}
