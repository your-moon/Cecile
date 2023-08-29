use std::num::ParseFloatError;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Зай болон мөр алгасах тэмдэгийг алгасанa.
pub enum Token {
    // Түлхүүр үгс
    #[token("and")]
    And,

    #[token("struct")]
    Struct,

    // Үг
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", cecile_identifier)]
    Identifier(String),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, cecile_string)]
    String(String),

    #[regex(r#"[0-9]+(\.[0-9]+)?"#, cecile_number)]
    Number(f64),

    // Нэг үсэгт токэн.
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
    #[token("/")]
    Slash,
    #[token("*")]
    Star,

    // Нэг болон хоёр үсэгт токэн.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
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
