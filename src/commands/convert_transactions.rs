use std::error::Error;

use clap::Args;

use crate::{
    conversions::currency::{EUR, USD},
    io::{convert_transactions, utils::read_exchange_rates},
};

/// Convert transactions to a different currency.
#[derive(Args, Debug)]
pub struct ConvertTransactions {
    /// The input CSV file.
    #[clap(short, long)]
    input: Box<str>,

    /// The output CSV file.
    #[clap(short, long)]
    output: Box<str>,

    /// The exchange rates file.
    #[clap(short, long)]
    exchange_rates: Box<str>,
}

pub fn convert_transactions(args: &ConvertTransactions) -> Result<(), Box<dyn Error>> {
    let rates = read_exchange_rates::<USD, EUR>(&args.exchange_rates)?;

    convert_transactions::convert(&args.input, &args.output, rates)?;

    Ok(())
}
