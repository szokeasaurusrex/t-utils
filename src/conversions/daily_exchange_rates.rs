use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use chrono::NaiveDate;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::conversions::currency::CurrencyType;
use crate::conversions::exchange_rate::ExchangeRate;
use crate::conversions::transaction::Transaction;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ConversionError {
    MissingExchangeRate,
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::MissingExchangeRate => write!(f, "Missing exchange rate"),
        }
    }
}

impl Error for ConversionError {}

#[derive(Debug, PartialEq)]
pub struct DailyExchangeRates<N, D>
where
    N: CurrencyType,
    D: CurrencyType,
{
    rates: HashMap<NaiveDate, ExchangeRate<N, D>>,
}

#[derive(Debug, Deserialize)]
struct DayRate<N, D>
where
    N: CurrencyType,
    D: CurrencyType,
{
    date: NaiveDate,
    rate: ExchangeRate<N, D>,
}

impl<N, D> DailyExchangeRates<N, D>
where
    N: CurrencyType + for<'de> Deserialize<'de>,
    D: CurrencyType + for<'de> Deserialize<'de>,
{
    pub fn day_rate(&self, date: &NaiveDate) -> Option<&ExchangeRate<N, D>> {
        self.rates.get(&date)
    }

    pub fn convert(&self, transaction: &Transaction<D>) -> Result<Transaction<N>, ConversionError> {
        let rate = self
            .day_rate(transaction.date())
            .ok_or(ConversionError::MissingExchangeRate)?;

        Ok(Transaction::new(
            *transaction.date(),
            rate.convert(transaction.amount()),
        ))
    }

    pub fn read_from_csv<R>(mut reader: Reader<R>) -> Result<Self, csv::Error>
    where
        R: std::io::Read,
    {
        Ok(DailyExchangeRates {
            rates: reader
                .deserialize()
                .map(|result| result.map(|day_rate: DayRate<N, D>| (day_rate.date, day_rate.rate)))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversions::currency::{Currency, EUR, USD};

    #[test]
    fn test_daily_exchange_rates() {
        let mut rates: HashMap<NaiveDate, ExchangeRate<EUR, USD>> = HashMap::new();
        rates.insert(
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            ExchangeRate::new(0.8),
        );
        rates.insert(
            NaiveDate::from_ymd_opt(2021, 1, 2).unwrap(),
            ExchangeRate::new(0.9),
        );
        let daily_rates = DailyExchangeRates { rates };

        assert_eq!(
            daily_rates.day_rate(&NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()),
            Some(&ExchangeRate::new(0.8))
        );
        assert_eq!(
            daily_rates.day_rate(&NaiveDate::from_ymd_opt(2021, 1, 2).unwrap()),
            Some(&ExchangeRate::new(0.9))
        );
        assert_eq!(
            daily_rates.day_rate(&NaiveDate::from_ymd_opt(2021, 1, 3).unwrap()),
            None
        );
    }

    #[test]
    fn test_daily_rates_convert() {
        let mut rates: HashMap<NaiveDate, ExchangeRate<EUR, USD>> = HashMap::new();
        rates.insert(
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            ExchangeRate::new(0.8),
        );
        rates.insert(
            NaiveDate::from_ymd_opt(2021, 1, 3).unwrap(),
            ExchangeRate::new(0.9),
        );
        let daily_rates = DailyExchangeRates { rates };

        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let amount = Currency::<USD>::from(100);
        let transaction = Transaction::new(date, amount);

        assert_eq!(
            daily_rates.convert(&transaction),
            Ok(Transaction::new(date, Currency::<EUR>::from(80)))
        );

        let date = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
        let amount = Currency::<USD>::from(100);
        let transaction = Transaction::new(date, amount);

        assert_eq!(
            daily_rates.convert(&transaction),
            Err(ConversionError::MissingExchangeRate)
        );

        let date = NaiveDate::from_ymd_opt(2021, 1, 3).unwrap();
        let amount = Currency::<USD>::from(100);
        let transaction = Transaction::new(date, amount);

        assert_eq!(
            daily_rates.convert(&transaction),
            Ok(Transaction::new(date, Currency::<EUR>::from(90)))
        );
    }

    #[test]
    fn test_read_from_csv() {
        let csv = "date,rate
                        2021-01-01,0.8
                        2021-01-02,0.9";
        let reader = Reader::from_reader(csv.as_bytes());
        let daily_rates: DailyExchangeRates<EUR, USD> =
            DailyExchangeRates::read_from_csv(reader).unwrap();

        assert_eq!(
            daily_rates,
            DailyExchangeRates {
                rates: vec![
                    (
                        NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        ExchangeRate::new(0.8)
                    ),
                    (
                        NaiveDate::from_ymd_opt(2021, 1, 2).unwrap(),
                        ExchangeRate::new(0.9)
                    ),
                ]
                .into_iter()
                .collect(),
            }
        );
    }
}
