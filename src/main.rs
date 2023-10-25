use crate::allocator::allocation::CeAllocation;
use std::io::Write;
use termcolor::WriteColor;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream};
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;
fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let input = r#"
    let a = 1;
    let b = 2;
    println a + b;
        "#;

    let mut allocator = CeAllocation::new();

    let mut vm = vm::VM::new(&mut allocator);

    if let Err(e) = vm.run(input, &mut stdout) {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "Error: {:?}", e).expect("Failed to write to stdout");
    }

    println!("Allocated: {:?}", allocator);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "Program exited successfully")?;
    return Ok(());
}
