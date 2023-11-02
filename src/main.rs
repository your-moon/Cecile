use crate::allocator::allocation::CeAllocation;
use std::fs;
use std::io::Write;
use termcolor::WriteColor;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream};
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;
fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // let input = r#"
    // fn is_even(n) -> bool {
    //     if (n == 0) {
    //         return true;
    //     } else {
    //         return is_odd(n - 1);
    //     }
    // }
    //
    // is_even(12);
    //
    // "#;

    let path = std::env::args().nth(1).expect("No file path provided");

    let source = fs::read_to_string(path).expect("Failed to read file");

    let mut allocator = CeAllocation::new();

    let mut vm = vm::VM::new(&mut allocator);

    if let Err(e) = vm.run(source.as_str(), &mut stdout) {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "Error: {:?}", e).expect("Failed to write to stdout");
    }

    println!("Allocations: {:?}", allocator);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "Program exited successfully")?;
    return Ok(());
}
