use clap::Parser;

use self::command::Commands;
pub mod command;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn run(self) -> Self {
        match self.command {
            Some(ref command) => command.run().unwrap(),
            None => {
                println!("No command specified");
                std::process::exit(1);
            }
        }
        self
    }
}
