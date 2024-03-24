use chrono::NaiveDate;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::conversions::{
    currency::{Currency, CurrencyType},
    daily_exchange_rates::{ConversionError, DailyExchangeRates},
    exchange_rate::ExchangeRate,
    transaction::Transaction,
};

#[derive(Debug, Serialize)]
struct OutputLine<'a, N, D>
where
    N: CurrencyType + Serialize,
    D: CurrencyType + Serialize,
{
    date: NaiveDate,
    from_amount: Currency<D>,
    exchange_rate: Option<&'a ExchangeRate<N, D>>,
    to_amount: Result<Currency<N>, ConversionError>,
}

impl<'a, N, D> OutputLine<'a, N, D>
where
    N: CurrencyType + Serialize,
    D: CurrencyType + Serialize,
{
    fn from_transactions(
        from_transaction: Transaction<D>,
        rate: Option<&'a ExchangeRate<N, D>>,
        to_transaction: Result<Transaction<N>, ConversionError>,
    ) -> Self {
        if let Ok(to_transaction) = &to_transaction {
            assert_eq!(from_transaction.date(), to_transaction.date());
        }

        OutputLine {
            date: from_transaction.date().clone(),
            from_amount: from_transaction.amount(),
            exchange_rate: rate.clone(),
            to_amount: to_transaction.map(|t| t.amount()),
        }
    }
}

pub fn convert<N, D>(
    input_path: &str,
    output_path: &str,
    rates: DailyExchangeRates<N, D>,
) -> Result<(), csv::Error>
where
    N: CurrencyType + for<'de> Deserialize<'de>,
    D: CurrencyType + for<'de> Deserialize<'de>,
{
    let mut writer = csv::Writer::from_path(output_path)?;

    Reader::from_path(input_path)?
        .deserialize()
        .map(|result| {
            let transaction = result?;
            let to_transaction = rates.convert(&transaction);
            let transaction_date = transaction.date().clone();
            Ok(OutputLine::from_transactions(
                transaction,
                rates.day_rate(&transaction_date),
                to_transaction,
            ))
        })
        .collect::<Result<Vec<_>, csv::Error>>()?
        .iter()
        .map(|line| {
            writer.serialize(line)?;
            Ok(())
        })
        .collect::<Result<Vec<_>, csv::Error>>()?;

    Ok(())
}
