use aeris_compiler_cli::subcommands::Subcommands;
use clap::Parser;
use std::env;

#[derive(Parser)]
#[command(name = "aerisc", version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Subcommands,
}

fn main() {
    let cli = CLI::parse();
}
