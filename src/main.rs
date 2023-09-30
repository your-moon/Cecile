use crate::allocator::allocation::CeAllocation;
use crate::cc_lexer::Lexer;
use crate::cc_parser::grammar;
use crate::vm::chunk::Chunk;
use crate::vm::compiler::Compiler;
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
    for(let i = 0; i < 10; i = i + 1) {
        print i;
    }
    fn main() {
        print "Hello World!";
    }
    fn add() {
        print a + b;
    }
    "#;
    let lexer = Lexer::new(input).map(|token| match token {
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

    let mut allocator = CeAllocation::new();
    let mut compiler = Compiler::new(&mut allocator);
    compiler.compile(&mut program, &mut allocator);
    println!("{:?}", unsafe {
        (*compiler.current_compiler.function)
            .chunk
            .disassemble("script")
    });
    println!("--------------");

    // println!("{:?}", chunk);
    // let mut vm = vm::VM::new(compiler.chunk.clone(), &mut allocator);
    // vm.run();

    // println!("{:?} VM", vm);
    println!("{:?}", allocator);

    // let mut stdout = StandardStream::stdout(ColorChoice::Always);
    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    // writeln!(&mut stdout, "magenta text!");
}
