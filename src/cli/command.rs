use crate::vm::error::report_errors;
use clap::Subcommand;
use std::io::{self, Write};

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[arg(value_hint = clap::ValueHint::FilePath, value_name = "INPUT FILE PATH")]
        file: String,
        #[arg(short, long)]
        trace: bool,
        #[arg(short, long)]
        debug: bool,
        #[arg(short, long)]
        ast_debug: bool,
    },
    Repl {
        #[arg(short, long)]
        trace: bool,
        #[arg(short, long)]
        debug: bool,
    },
}

impl Commands {
    pub fn run(&self) -> std::io::Result<()> {
        match self {
            Commands::Run {
                file,
                trace,
                debug,
                ast_debug,
            } => {
                let source = std::fs::read_to_string(file)?;

                let mut color = termcolor::StandardStream::stderr(termcolor::ColorChoice::Always);

                let stdout = &mut io::stdout().lock();
                let stderr = &mut io::stderr().lock();

                let mut vm = crate::vm::VM::new(*trace);
                if let Err(e) = vm.run(source.as_str(), &mut color, *debug, *ast_debug) {
                    report_errors(stderr, &source, &e);
                }
                Ok(())
            }
            Commands::Repl { trace, debug } => {
                // let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
                // let mut allocator = crate::allocator::allocation::CeAllocation::new();
                // let mut vm = crate::vm::VM::new(&mut allocator, *trace);
                // crate::repl::repl(&mut vm, &mut stdout, *debug);
                Ok(())
            }
        }
    }
}
