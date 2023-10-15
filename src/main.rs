use crate::allocator::allocation::CeAllocation;
use std::io::Write;
use termcolor::WriteColor;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream};
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;
fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let input = r#"
    let a: Int = 1;
    let b: Int = 2;
    fn add() -> Int {
        print "Hello" + " World";
        return a + b;
    }
    let c = add();
    print(c);

    "#;
    let mut allocator = CeAllocation::new();
    let mut vm = vm::VM::new(&mut allocator);
    if let Err(e) = vm.run(input, &mut stdout) {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        writeln!(&mut stdout, "Error: {:?}", e).expect("Failed to write to stdout");
    }

    println!("allocations: {:?}", allocator);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    writeln!(&mut stdout, "Program exited successfully");
}
