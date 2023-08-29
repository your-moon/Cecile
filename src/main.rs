use crate::lexer::Token;
use logos::Logos;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod lexer;
fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    writeln!(&mut stdout, "green text!");
}
