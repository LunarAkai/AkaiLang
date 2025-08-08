use chumsky::{
    combinator::Or, error::Rich, extra, input::{Input, Stream, ValueInput}, prelude::{choice, end, just, nested_delimiters, recursive, skip_then_retry_until, via_parser}, primitive::select, recursive, select, select_ref, span::{self, SimpleSpan}, text::{self, ascii::{ident, keyword}, newline, whitespace}, Boxed, ConfigIterParser, IterParser, ParseResult, Parser
};
use logos::{source, Logos};

use crate::language_frontend::{abstract_syntax_tree::{ast::Expr, definitions::*}, lexer::tokens::Token};

// goal of parsing is to construct an abstract syntax tree

pub fn parse(source: &str) ->Result<Vec<Expr>, Vec<Rich<'_, Token>>> {
    let token_iter = Token::lexer(source).spanned().map(|(token, span)| (token.unwrap_or(Token::Error), span.into()));

    let end_of_input: SimpleSpan = (0..source.len()).into();
    let token_stream = Stream::from_iter(token_iter)
        // Tell chumsky to split the (Token, SimpleSpan) stream into its parts so that it can handle the spans for us
        // This involves giving chumsky an 'end of input' span: we just use a zero-width span at the end of the string
        .map((0..end_of_input.into_iter().len()).into(), |(t, s): (_, _)| (t, s));

    parser().parse(token_stream).into_result()
}


fn parser<'src, I>() 
    -> impl Parser<'src, I, Vec<Expr>, extra::Err<Rich<'src, Token>>>
where 
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>,
{
    let ident = select! { Token::Identifier(s) => s, };
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
    let decl = recursive(|decl| {
        let var = just(Token::Var)
            .ignore_then(ident)
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .then_ignore(just(Token::NewLine))
            .map(|(name, rhs)| Expr::Assignment {
                target: Box::new(Expr::Ident(name)), 
                value: Box::new(rhs), 
            });
        var.or(expr)
    });

    decl.repeated().collect()

}

#[cfg(test)]
mod tests {
    use super::*;

  
}