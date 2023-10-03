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
        fn add(a: Int, b: Int) -> Int {
            return a + b;
        }
        let a: Int = 1;
        let b: Int = 2;
        let c: Int = add(a, b);
        print c;


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
    for (statement, _range) in &program.statements {
        println!("{:?}", statement);
    }

    let mut allocator = CeAllocation::new();
    let mut vm = vm::VM::new(&mut allocator);
    vm.run(&mut program);

    // let mut stdout = StandardStream::stdout(ColorChoice::Always);
    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    // writeln!(&mut stdout, "magenta text!");
}
