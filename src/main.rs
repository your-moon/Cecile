use crate::allocator::allocation::CeAllocation;
mod allocator;
mod cc_lexer;
mod cc_parser;
mod vm;
fn main() {
    let input = r#"
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
