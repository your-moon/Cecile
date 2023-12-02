use crate::allocator::allocation::CeAllocation;
use std::fs;
use std::io::Write;
use termcolor::WriteColor;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream};
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;

use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    file_path: String,
}

fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let args = Args::parse();
    let debug = args.debug;
    let path = args.file_path;

    let source = fs::read_to_string(path).expect("Failed to read file");

    let mut allocator = CeAllocation::new();

    let mut vm = vm::VM::new(&mut allocator);

    if let Err(e) = vm.run(source.as_str(), &mut stdout, debug) {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "Error: {:?}", e).expect("Failed to write to stdout");
    }

    // println!("Allocations: {:?}", allocator);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "Program exited successfully")?;
    return Ok(());
}
