use clap::Parser;

use self::command::Commands;
pub mod command;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
