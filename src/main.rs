use clap::Parser;
use clap::*;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use rand::rngs::OsRng;
mod commands;
mod utils;
use commands::Commands;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    cli.command.do_action();
}
