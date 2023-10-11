use anyhow::bail;
use cc_parser::ast::Program;
use lalrpop_util::ParseError;

use crate::allocator::allocation::CeAllocation;
use crate::cc_lexer::{Lexer, Token};
use crate::cc_parser::grammar;
use crate::vm::chunk::Chunk;
use crate::vm::compiler::Compiler;
use core::result::Result::Ok;
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;
fn main() {
    // let input = r#"
    // {
    // let a = 1 + 1;
    // {
    // let b = 2 + 2;
    // let x = 3*3;
    // b = 5;
    // print x;
    // print b;
    // }
    // }
    // let c = 3 + 3;
    // let b = c + 3;
    // print c;
    // print b;
    // "#;
    let input = r#"
    return 1;
    "#;
    let mut allocator = CeAllocation::new();
    let mut vm = vm::VM::new(&mut allocator);
    if let Err(e) = vm.run(input) {
        println!("error: {:?}", e);
    }

    // let mut stdout = StandardStream::stdout(ColorChoice::Always);
    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    // writeln!(&mut stdout, "magenta text!");
}
