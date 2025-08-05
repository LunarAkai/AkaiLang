use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")] // Skips whitespace
pub enum Token<'source> {
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

    #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Integer(isize),
    
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*")]
    Ident(&'source str),

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
    String(String),

    #[token("class")]
    #[token("fun")]
    #[token("var")]
    #[token("if")]
    #[token("else")]
    Keyword(&'source str),
}


