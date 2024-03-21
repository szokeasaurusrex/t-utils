use std::fmt::Display;

use crate::currency::{Currency, CurrencyType};
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
