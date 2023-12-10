use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Run {
        file: String,
        trace: bool,
        debug: bool,
    },
    Repl {
        trace: bool,
        debug: bool,
    },
}
