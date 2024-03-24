use csv::Reader;
use serde::Deserialize;

use crate::conversions::{currency::CurrencyType, daily_exchange_rates::DailyExchangeRates};

pub fn read_exchange_rates<N, D>(file_path: &str) -> Result<DailyExchangeRates<N, D>, csv::Error>
where
    N: CurrencyType + for<'de> Deserialize<'de>,
    D: CurrencyType + for<'de> Deserialize<'de>,
{
    let reader = Reader::from_path(file_path)?;
    DailyExchangeRates::read_from_csv(reader)
}
