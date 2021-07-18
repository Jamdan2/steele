
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Space,
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice().parse())]
    String(String),
    #[regex(r"[+-]?[0-9]*[.]?[0-9]+(?:[eE][+-]?[0-9]+)?", |lex| lex.slice().parse(), priority = 2)]
    Number(f32),
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r"[-a-zA-Z_][a-zA-Z0-9_-]*", |lex| lex.slice().parse())]
    Ident(String),
    #[token(":")]
    Colon,
    #[token("(")]
    ParaOpen,
    #[token(")")]
    ParaClose,
    #[token("[")]
    SquareOpen,
    #[token("]")]
    SquareClose,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
}

pub fn lex(source: &str) -> Vec<Token> {
    Token::lexer(source).collect()
}
