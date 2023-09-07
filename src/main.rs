use lalrpop_util::lalrpop_mod;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use logos::{Logos, SpannedIter};

use crate::lexer::Token;
mod ast;
mod lexer;
fn main() {
    lalrpop_mod!(pub grammar);
    let input = r#"{ a; }"#;
    let mut lexer = lexer::Lexer::new(input).map(|token| match token {
        Ok((l, token, r)) => (l, token, r),
        Err(_) => todo!("Error handling"),
    });
    let parser = grammar::ProgramParser::new();
    let program = parser.parse(lexer).unwrap();
    for statement in &program.statements {
        println!("{:?}", statement);
    }

    // let program = parser.parse(lexer).unwrap();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    writeln!(&mut stdout, "magenta text!");
}
