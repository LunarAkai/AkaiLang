use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")] // Skips whitespace
enum Token<'source> {
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
    
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*")]
    Ident(&'source str),

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
    String(String),
}

fn float<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<f64, ()> {
    lex.slice().parse().map_err(|_| ())
}


