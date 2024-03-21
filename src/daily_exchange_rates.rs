use std::collections::HashMap;

use chrono::NaiveDate;

use crate::currency::CurrencyType;
use crate::exchange_rate::ExchangeRate;

pub struct DailyExchangeRates<T, F>
where
    T: CurrencyType,
    F: CurrencyType,
{
    rates: HashMap<NaiveDate, ExchangeRate<T, F>>,
}

impl<T, F> DailyExchangeRates<T, F>
where
    T: CurrencyType,
    F: CurrencyType,
{
    pub fn day_rate(&self, date: &NaiveDate) -> Option<&ExchangeRate<T, F>> {
        self.rates.get(&date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::{EUR, USD};

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
}
