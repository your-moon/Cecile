use lalrpop_util::lalrpop_mod;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::allocator::allocation::CeAllocation;
use crate::allocator::Allocator;
use crate::cc_lexer::Lexer;
use crate::vm::chunk::Chunk;
use crate::vm::compiler::Compiler;
use crate::vm::object::Object;
use crate::{cc_lexer::Token, cc_parser::grammar};
use logos::{Logos, SpannedIter};
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;
fn main() {
    let input = r#" 
    {
    let a = 1 + 1;
    {
    let b = 2 + 2;
    let x = 3*3;
    b = 5;
    print x;
    print b;
    }
    }
    let c = 3 + 3;
    let b = c + 3;
    print c;
    print b;
    "#;
    let mut lexer = Lexer::new(input).map(|token| match token {
        Ok((l, token, r)) => {
            // println!("{:?}", token);
            return (l, token, r);
        }
        Err(_) => todo!("Error handling"),
    });
    let parser = grammar::ProgramParser::new();
    let mut program = parser.parse(lexer).unwrap();
    // for (statement, _range) in &program.statements {
    //     println!("{:?}", statement);
    // }

    let mut chunk = Chunk::new();
    let mut allocator = CeAllocation::new();
    let mut compiler = Compiler::new(&mut chunk);

    compiler.compile(&mut program, &mut allocator);
    println!("{:?}", compiler.globals);

    // println!("{:?}", chunk);
    let mut vm = vm::VM::new(chunk, &mut allocator);
    vm.run();

    println!("{:?} VM", vm);

    // let mut stdout = StandardStream::stdout(ColorChoice::Always);
    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    // writeln!(&mut stdout, "magenta text!");
}
