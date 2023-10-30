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

    let input = r#"
    fn fizzbuzz(n: Int) {
        if (n % 15 == 0) {
            print("FizzBuzz");
        } else if (n % 3 == 0) {
            print("Fizz");
        } else if (n % 5 == 0) {
            print("Buzz");
        } else {
            print(n);
        }
    }
    for (let i = 0; i < 10; i = i + 1) {
        print(i);
    }
    "#;

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
