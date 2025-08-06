use std::fmt;

use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")] // Skips whitespace
pub enum Token<'src> {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("+")]
    Add,

    #[token("-")]
    Substract,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("=")]
    Equals,

    #[token(":")]
    Colon,

    #[token("(")]
    ParenBegin,

    #[token(")")]
    ParenEnd,

    #[token("{")]
    BraceBegin,

    #[token("}")]
    BraceEnd,

    #[regex(r"[+-]?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap(), priority = 3)]
    Integer(i64),

    #[regex(r"[+-]?([0-9]*[.])?[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*")]
    Ident(&'src str),

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
    String(String),

    #[token("class")]
    #[token("fun")]
    #[token("var")]
    #[token("if")]
    #[token("else")]
    Keyword(&'src str),
}


impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Float(s) => write!(f, "{s}"),
            Token::Add => write!(f, "+"),
            Token::Bool(_) => write!(f, "+"),
            Token::Substract => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Equals => write!(f, "="),
            Token::Colon => write!(f, ":"),
            Token::ParenBegin => write!(f, "("),
            Token::ParenEnd => write!(f, ")"),
            Token::BraceBegin => write!(f, "{{"),
            Token::BraceEnd => write!(f, "}}"),
            Token::Integer(s) => write!(f, "{s}"),
            Token::Ident(s) => write!(f, "{s}"),
            Token::String(s) => write!(f, "{s}"),
            Token::Keyword(s) => write!(f, "{s}"),
        }
    }
}
