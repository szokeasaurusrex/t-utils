use std::error::Error;

use clap::{Parser, Subcommand};

mod commands;
mod conversions;
mod currency;
mod io;
mod trades;

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

pub fn run_cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertTransactions(args) => {
            commands::convert_transactions::convert_transactions(&args)
        }
    }
}
