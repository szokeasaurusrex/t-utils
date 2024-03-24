use std::error::Error;

use clap::{Parser, Subcommand};

mod commands;
mod conversions;
mod io;

/// Simple CLI tool to help with currency conversions.
#[derive(Debug, Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ConvertTransactions(commands::convert_transactions::ConvertTransactions),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertTransactions(args) => {
            commands::convert_transactions::convert_transactions(&args)
        }
    }
}
