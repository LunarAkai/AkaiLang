use chumsky::{
    Boxed, ConfigIterParser, IterParser, ParseResult, Parser,
    combinator::Or,
    error::{Rich, Simple},
    extra,
    input::{Input, Stream, ValueInput},
    prelude::{choice, end, just, nested_delimiters, recursive, skip_then_retry_until, via_parser},
    primitive::select,
    recursive, select, select_ref,
    span::{self, SimpleSpan},
    text::{
        self,
        ascii::{ident, keyword},
        newline, whitespace,
    },
};
use logos::{Logos, source};

use crate::language_frontend::{
    abstract_syntax_tree::{ast::Expr, definitions::*},
    lexer::tokens::Token,
};

// goal of parsing is to construct an abstract syntax tree

pub fn parse(source: &str) -> Result<Vec<Expr>, Vec<Rich<'_, Token>>> {
    let token_iter = Token::lexer(source)
        .spanned()
        .map(|(token, span)| (token.unwrap_or(Token::Error), span.into()));
    let end_of_input: SimpleSpan = (0..source.len()).into();
    let token_stream = Stream::from_iter(token_iter).map(
        (0..end_of_input.into_iter().len()).into(),
        |(t, s): (_, _)| (t, s),
    );

    parser().parse(token_stream).into_result()
}

fn parser<'src, I>() -> impl Parser<'src, I, Vec<Expr>, extra::Err<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>,
{
    let ident = select! { Token::Identifier(s) => s, }.labelled("identifier");

    let expr = recursive(|expr| {
        let atom = select! {
            Token::FloatLiteral(x) => Expr::FloatLiteral(x),
            Token::IntLiteral(x) => Expr::IntLiteral(x),
            Token::BoolLiteral(x) => Expr::BoolLiteral(x),
            Token::StringLiteral(s) => Expr::StringLiteral(s),
        }
        .labelled("value")
        .or(expr
            .clone()
            .delimited_by(just(Token::LParen), just(Token::RParen)));

        // Product
        let mul_div = atom.clone().foldl(
            choice((
                just(Token::Multiply).to(BinaryOp::Multiply),
                just(Token::Divide).to(BinaryOp::Divide),
            ))
            .then(atom)
            .then_ignore(just(Token::NewLine).or_not())
            .repeated(),
            |lhs, (op, rhs)| {
                Expr::BinaryExpr(Binary {
                    lhs: Box::new(lhs),
                    operator: op,
                    rhs: Box::new(rhs),
                })
            },
        );

        // Sum
        let add_sub = mul_div.clone().foldl(
            choice((
                just(Token::Add).to(BinaryOp::Add),
                just(Token::Substract).to(BinaryOp::Substract),
            ))
            .then(mul_div)
            .then_ignore(just(Token::NewLine).or_not())
            .repeated(),
            |lhs, (op, rhs)| {
                Expr::BinaryExpr(Binary {
                    lhs: Box::new(lhs),
                    operator: op,
                    rhs: Box::new(rhs),
                })
            },
        );

        let assign_expr = ident
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|(name, value)| {
                Expr::AssignmentExpr(Assignment {
                    target: name,
                    value: Box::new(value),
                })
            });

        assign_expr.or(add_sub)
    });

    let decl = recursive(|decl| {
        let var = just(Token::Var)
            .ignore_then(ident)
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .then_ignore(just(Token::NewLine).or_not())
            .map(|(name, rhs)| {
                Expr::VarExpr(Var {
                    ty: None,
                    ident: Identifier(String::from(name)),
                    value: Box::new(rhs),
                })
            });

        let type_parser = choice((
            just(Token::IntType).to(Type::Integer),
            just(Token::FloatType).to(Type::Float),
            just(Token::BoolType).to(Type::Bool),
            just(Token::StringType).to(Type::String),
        ));

        let return_type_parser = just(Token::Colon).ignore_then(type_parser.clone()).or_not();

        //---------------------------------------------------------------------------------------
        // Function Parser
        //---------------------------------------------------------------------------------------
        let fun = just(Token::Fun)
            .ignore_then(ident) // function name
            .then(
                ident
                    .then_ignore(just(Token::Colon))
                    .then(type_parser.clone())
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::LParen), just(Token::RParen))
                    .or_not(),
            )
            .then(return_type_parser.clone())
            .then_ignore(just(Token::LBrace))
            .then_ignore(just(Token::NewLine).repeated())
            .then(
                expr.clone()
                    .then_ignore(just(Token::NewLine))
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(Token::RBrace))
            .map(|(((name, params), return_ty), stmts)| {
                Expr::FunctionExpr(Function {
                    name: name,
                    params,
                    return_type: return_ty,
                    body: Some(stmts),
                    body_expr: None,
                })
            });

        var.or(fun).or(expr)
    });

    decl.clone()
        .then_ignore(just(Token::NewLine).repeated())
        .repeated()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_expr() {
        let negate_two = parse("!2");
        assert!(negate_two.is_ok());
        assert_eq!(
            negate_two.clone().unwrap(),
            vec![Expr::UnaryExpr(Unary {
                operator: UnaryOp::Negate,
                operand: Box::new(Expr::IntLiteral(2)),
            })]
        )
    }

    #[test]
    fn test_bool() {
        let var_bool = parse("var isUwU = true");
        assert!(var_bool.is_ok());
        assert_eq!(
            var_bool.clone().unwrap(),
            vec![Expr::VarExpr(Var {
                ty: None,
                ident: Identifier(String::from("isUwU")),
                value: Box::new(Expr::BoolLiteral(true))
            })]
        )
    }

    #[test]
    fn test_binary_expr() {
        let sum = parse("1 + 2");
        assert!(sum.is_ok());
        assert_eq!(
            sum.clone().unwrap(),
            vec![Expr::BinaryExpr(Binary {
                lhs: Box::new(Expr::IntLiteral(1)),
                operator: BinaryOp::Add,
                rhs: Box::new(Expr::IntLiteral(2))
            })]
        )
    }

    #[test]
    fn test_variable_decl() {
        let var_without_expl_type = parse("var x = 12");
        assert!(var_without_expl_type.is_ok());
        assert_eq!(
            var_without_expl_type.clone().unwrap(),
            vec![Expr::VarExpr(Var {
                ty: None,
                ident: Identifier(String::from("x")),
                value: Box::new(Expr::IntLiteral(12))
            })]
        )
    }

    #[test]
    fn test_assignment() {
        let assign = parse("x = 12");
        assert!(assign.is_ok());
        assert_eq!(
            assign.clone().unwrap(),
            vec![Expr::AssignmentExpr(Assignment {
                target: String::from("x"),
                value: Box::new(Expr::IntLiteral(12))
            })]
        )
    }

    #[test]
    fn test_function_decl() {
        // test without a body or args
        let empty_fun = parse("fun helloWorld() { }");
        assert!(empty_fun.is_ok());
        assert_eq!(
            empty_fun.clone().unwrap(),
            vec![Expr::FunctionExpr(Function {
                name: String::from("helloWorld"),
                params: Some([].to_vec()),
                return_type: None,
                body: Some([].to_vec()),
                body_expr: None,
            })]
        );

        // tests if empty new lines within a function works
        let empty_fun_with_new_lines = parse(
            r"fun emptyMulLines() {

            }
        ",
        );
        assert_eq!(
            empty_fun_with_new_lines.clone().unwrap(),
            vec![Expr::FunctionExpr(Function {
                name: String::from("emptyMulLines"),
                params: Some([].to_vec()),
                return_type: None,
                body: Some([].to_vec()),
                body_expr: None,
            })]
        );

        // tests for return expr in functions
        let fun_that_returns_int = parse(
            r"fun returnsInt(): int {
                -> 12
            }
        ",
        );
        assert_eq!(
            fun_that_returns_int.clone().unwrap(),
            vec![Expr::FunctionExpr(Function {
                name: String::from("returnsInt"),
                params: None,
                return_type: Some(Type::Integer),
                body: Some(vec![Expr::IntLiteral(12)]),
                body_expr: Some(Box::new(Expr::ReturnExpr)),
            })]
        )
    }
}

/*
var x = 10\nvar y = 5\n{\n    var z = 7\n}\n10 + 10\n10 - 5\n5 * 5\n10 / 2
*/
