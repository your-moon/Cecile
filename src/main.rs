use crate::allocator::allocation::CeAllocation;
use std::io::Write;
use std::{fs, io};
use termcolor::WriteColor;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream};
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;

use clap::Parser;
use vm::error::{report_error, ErrorS};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    trace: bool,
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    file_path: String,
}

fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let args = Args::parse();
    let trace = args.trace;
    let debug = args.debug;
    let path = args.file_path;

    let source = fs::read_to_string(path).expect("Failed to read file");

    let mut allocator = CeAllocation::new();

    let mut vm = vm::VM::new(&mut allocator, trace);

    if let Err(e) = vm.run(source.as_str(), &mut stdout, debug) {
        report_err(&source, e);
    } else {
        if debug {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut stdout, "Program exited successfully")?;
        }
    }
    Ok(())
}
fn report_err(source: &str, errors: Vec<ErrorS>) {
    let mut buffer = termcolor::Buffer::ansi();
    for err in errors {
        report_error(&mut buffer, source, &err);
    }
    io::stderr()
        .write_all(buffer.as_slice())
        .expect("failed to write to stderr");
}
