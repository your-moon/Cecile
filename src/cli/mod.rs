use anyhow::Result;
use clap::Parser;

use self::command::Commands;
pub mod command;

pub struct Cli;

impl Cli {
    pub fn run() -> Result<()> {
        let commands = Commands::parse();
        commands.run()
    }
}
