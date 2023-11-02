use std::{fmt, num::ParseFloatError};

use crate::vm::error::{Error, ErrorS, SyntaxError};
use logos::{Logos, SpannedIter};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Зай болон мөр алгасах тэмдэгийг алгасанa.
#[logos(skip r"//.*")]
pub enum Token {
    // Төрөл
    #[token("String")]
    TypeString,
    #[token("Int")]
    TypeInt,
    // Түлхүүр үгс
    #[token("let")]
    Let,
    #[token("and")]
    And,

    #[token("struct")]
    Struct,

    #[token("bool")]
    TypeBool,

    #[token("print")]
    Print,

    #[token("println")]
    PrintLn,

    #[token("if")]
    If,
    #[token("else")]
    Else,

    #[token("fn")]
    Fn,

    #[token("return")]
    Return,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("nil")]
    Nil,

    #[token("or")]
    Or,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("break")]
    Break,

    // Үг
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", cecile_identifier)]
    Identifier(String),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, cecile_string)]
    String(String),

    #[regex(r#"[0-9]+(\.[0-9]+)?"#, cecile_number)]
    Number(f64),

    // Нэг үсэгт токэн.
    #[token("%")]
    Percent,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,

    // Нэг болон хоёр үсэгт токэн.
    #[token("->")]
    Arrow,
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    Error,
}

fn cecile_identifier(lexer: &mut logos::Lexer<Token>) -> String {
    lexer.slice().to_string()
}

fn cecile_string(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice[1..slice.len() - 1].to_string()
}
fn cecile_number(lexer: &mut logos::Lexer<Token>) -> Option<f64> {
    lexer.slice().parse::<f64>().ok()
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), ErrorS>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| match token {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(_) => Err((
                Error::SyntaxError(SyntaxError::UnexpectedInput {
                    token: self.token_stream.source()[span.start..span.end].to_string(),
                }),
                span,
            )),
        })
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::cc_parser::ast;

    use super::*;

    #[test]
    fn test_comment() {
        let input = r#"
        // this is comment 
        let 
        "#;
        let expected_tokens = [];
        let lexer = Token::lexer(input);

        for (token, expected_token) in lexer.zip(expected_tokens.iter()) {
            assert_eq!(token.unwrap(), *expected_token);
        }
    }

    #[test]
    fn test_lexer() {
        let input = r#"let variable_1: String = "Hello"; "#;
        let expected_tokens = [
            Token::Let,
            Token::Identifier("variable_1".to_string()),
            Token::Colon,
            Token::TypeString,
            Token::Equal,
            Token::String("Hello".to_string()),
            Token::Semicolon,
        ];
        let lexer = Token::lexer(input);

        for (token, expected_token) in lexer.zip(expected_tokens.iter()) {
            assert_eq!(token.unwrap(), *expected_token);
        }

        let input = r#"
            and
            struct
            identifier123
            "string"
            42
            ( ) { }
            , . - + ;
            != == > >= < <=
        "#;

        let expected_tokens = [
            Token::And,
            Token::Struct,
            Token::Identifier("identifier123".to_string()),
            Token::String("string".to_string()),
            Token::Number(42.0),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Dot,
            Token::Minus,
            Token::Plus,
            Token::Semicolon,
            Token::BangEqual,
            Token::EqualEqual,
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ];

        let lexer = Token::lexer(input);

        for (token, expected_token) in lexer.zip(expected_tokens.iter()) {
            assert_eq!(token.unwrap(), *expected_token);
        }
    }
}
