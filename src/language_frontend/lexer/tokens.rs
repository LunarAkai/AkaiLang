use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[regex(r"[\t\f]+", logos::skip)]
pub enum Token {
    // Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    // Keywords
    #[token("fun")]
    Fun,

    #[token("class")]
    Class,

    #[token("var")]
    Var,

    #[token("interface")]
    Interface,

    #[token("derive")]
    Derive,

    #[token("impl")]
    Impl,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("->")]
    Return,

    #[token("enum")]
    Enum,

    // Types
    #[token("int")]
    IntType,

    #[token("float")]
    FloatType,

    #[token("bool")]
    BoolType,

    #[token("String")]
    StringType,

    // Literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_owned())]
    StringLiteral(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntLiteral(i64),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    FloatLiteral(f64),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    BoolLiteral(bool),

    // Operators
    #[token("=")]
    Assign,

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEquals,

    #[token("<", priority = 2)]
    Less,

    #[token("<=")]
    LessEquals,

    #[token(">", priority = 2)]
    Greater,

    #[token(">=")]
    GreaterEquals,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("+")]
    Add,

    #[token("-")]
    Substract,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    // Punctiuation
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    // Special
    #[regex(r"\n")]
    NewLine,

    #[regex(r"//[^\r]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Comment,

    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,

    Eof,

    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(ident) => write!(f, "{ident}"),
            Token::Fun => write!(f, "fun"),
            Token::Class => write!(f, "class"),
            Token::Var => write!(f, "var"),
            Token::Interface => write!(f, "interface"),
            Token::Derive => write!(f, "derive"),
            Token::Impl => write!(f, "impl"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "->"),
            Token::Enum => write!(f, "enum"),
            Token::IntType => write!(f, "int"),
            Token::FloatType => write!(f, "float"),
            Token::BoolType => write!(f, "bool"),
            Token::StringType => write!(f, "String"),
            Token::StringLiteral(s) => write!(f, "{s}"),
            Token::IntLiteral(i) => write!(f, "{i}"),
            Token::FloatLiteral(fl) => write!(f, "{fl}"),
            Token::BoolLiteral(b) => write!(f, "{b}"),
            Token::Assign => write!(f, "="),
            Token::Equals => write!(f, "=="),
            Token::NotEquals => write!(f, "!="),
            Token::Less => write!(f, "<"),
            Token::LessEquals => write!(f, "<="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEquals => write!(f, ">="),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Add => write!(f, "+"),
            Token::Substract => write!(f, "-"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Comment => write!(f, ""),
            Token::NewLine => write!(f, "\n"),
            Token::Whitespace => write!(f, ""),
            Token::Eof => write!(f, ""),
            Token::Error => write!(f, "<error>"),
        }
    }
}
