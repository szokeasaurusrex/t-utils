use std::fmt::Display;

use crate::currency::{Currency, CurrencyType, EUR};
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Transaction<T>
where
    T: CurrencyType,
{
    date: NaiveDate,
    amount: Currency<T>,
}

impl<T> Transaction<T>
where
    T: CurrencyType,
{
    pub fn new(date: NaiveDate, amount: Currency<T>) -> Self {
        Transaction { date, amount }
    }
}

impl<T> Display for Transaction<T>
where
    T: CurrencyType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.date.format("%Y-%m-%d"), self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let amount: Currency<EUR> = 123.45.into();
        let transaction = Transaction::new(date, amount);
        assert_eq!(format!("{}", transaction), "2021-01-01: € 123.45");
    }
}
