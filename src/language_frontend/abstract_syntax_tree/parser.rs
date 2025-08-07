use chumsky::{
    combinator::Or, error::Rich, extra, input::ValueInput, prelude::{choice, end, just, nested_delimiters, recursive, via_parser}, primitive::select, recursive, select, select_ref, span::{self, SimpleSpan}, text::{self, ascii::{ident, keyword}, whitespace}, Boxed, ConfigIterParser, IterParser, Parser
};

use crate::language_frontend::{abstract_syntax_tree::ast::{BinaryOp, Expr}, lexer::tokens::Token};

// goal of parsing is to construct an abstract syntax tree

#[allow(clippy::let_and_return)]
pub fn parser<'tokens, 'src: 'tokens, I>() 
    -> impl Parser<'tokens, I, Expr, extra::Err<Rich<'tokens, Token>>> 
where 
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let ident = select! {
        Token::Identifier(s) => s,
    };
    /* 
    let block = recursive(|block| {
        let indent = just(Token::NewLine)
            .ignore_then(just(Token::Indent))
            .ignore_then(block.clone().separated_by(just(Token::NewLine)).at_least(1))
            .then_ignore(just(Token::Dedent));

        block.with_ctx(0)
    });

    */

    let expr = recursive(|expr| {
        let atom = select! {
            Token::FloatLiteral(x) => Expr::FloatLiteral(x),
            Token::IntLiteral(x) => Expr::IntLiteral(x),
        }
        .or(expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)));

        let mul_div = atom.clone().foldl(
            choice((
                just(Token::Multiply).to(BinaryOp::Multiply),
                just(Token::Divide).to(BinaryOp::Divide),
            ))
            .then(atom)
            .repeated(),
            |lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                operator: op,
                rhs: Box::new(rhs),
            },
        );

        let add_sub = mul_div.clone().foldl(
            choice((
                just(Token::Add).to(BinaryOp::Add),
                just(Token::Substract).to(BinaryOp::Substract),
            ))
            .then(mul_div)
            .repeated(),
            |lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                operator: op,
                rhs: Box::new(rhs),
            },
        );

        add_sub
    });

    let var = just(Token::Var)
        .ignore_then(ident)
        .then_ignore(just(Token::Assign))
        .then(expr.clone())
        .then_ignore(just(Token::NewLine).or_not())
        .map(|(name, rhs)| Expr::Assignment {
            target: Box::new(Expr::Ident(name)), 
            value: Box::new(rhs), 
        });
    

    var.or(expr)
  

}
