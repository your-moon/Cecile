use lalrpop_util::lalrpop_mod;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::cc_lexer::Lexer;
use crate::vm::chunk::Chunk;
use crate::{cc_lexer::Token, cc_parser::grammar};
use logos::{Logos, SpannedIter};
mod cc_lexer;
mod cc_parser;
mod vm;
fn main() {
    let input = r#"1+1; 1-1; 2*3;"#;
    let mut lexer = Lexer::new(input).map(|token| match token {
        Ok((l, token, r)) => {
            println!("{:?}", token);
            return (l, token, r);
        }
        Err(_) => todo!("Error handling"),
    });
    let parser = grammar::ProgramParser::new();
    let mut program = parser.parse(lexer).unwrap();
    for (statement, _range) in &program.statements {
        println!("{:?}", statement);
    }

    let mut chunk = Chunk::new();
    chunk.compile(&mut program);
    println!("{:?}", chunk);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    writeln!(&mut stdout, "magenta text!");
}
