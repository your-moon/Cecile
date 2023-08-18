use crate::lexer::Token;
use logos::Logos;

mod lexer;
fn main() {
    let mut lex = Token::lexer("You and i are in struct");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier("You".to_string()))));
    assert_eq!(lex.next(), Some(Ok(Token::And)));
    assert_eq!(lex.slice(), "and");
    println!("{:?}", lex.span());
    println!("{:?}", lex);
    let mut lex = Token::lexer("! { } 32");
    assert_eq!(lex.next(), Some(Ok(Token::Bang)));
    assert_eq!(lex.next(), Some(Ok(Token::LeftBrace)));
    assert_eq!(lex.next(), Some(Ok(Token::RightBrace)));
    assert_eq!(lex.next(), Some(Ok(Token::Number(32.0))));
    println!("{:?}", lex);
}
