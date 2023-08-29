use crate::lexer::Token;
use lalrpop_util::lalrpop_mod;
use logos::Logos;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
mod ast;
mod lexer;
fn main() {
    lalrpop_mod!(pub grammar);
    let expr = grammar::TermParser::new();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    writeln!(&mut stdout, "green text!");
}
