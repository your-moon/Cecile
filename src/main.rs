use cecile::cli::Cli;
use clap::Parser;

fn main() {
    Cli::parse().run();
}
