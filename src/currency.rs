use crate::exchange_rate::ExchangeRate;
use std::{fmt::Display, marker::PhantomData};

pub trait CurrencyType {
    fn formatter(raw_amount: i64) -> String;
}

pub trait CentCurrency: CurrencyType {}

#[derive(Clone, Copy, Debug)]
pub struct Currency<T>(i64, PhantomData<T>)
where
    T: CurrencyType;

#[derive(Debug, Eq, PartialEq)]
pub struct EUR;

#[derive(Debug, Eq, PartialEq)]
pub struct USD;

impl CurrencyType for EUR {
    fn formatter(raw_amount: i64) -> String {
        format!("€ {:.2}", raw_amount as f64 / 100.0)
    }
}

impl CurrencyType for USD {
    fn formatter(raw_amount: i64) -> String {
        format!("$ {:.2}", raw_amount as f64 / 100.0)
    }
}

impl CentCurrency for EUR {}

impl CentCurrency for USD {}

impl<T> Currency<T>
where
    T: CurrencyType,
{
    fn new(value: i64) -> Self {
        Currency(value, PhantomData)
    }
}

impl<F> Currency<F>
where
    F: CentCurrency,
{
    pub fn convert<T>(&self, rate: ExchangeRate<T, F>) -> Currency<T>
    where
        T: CentCurrency,
    {
        Currency::new((self.0 as f64 * rate.0).round() as i64)
    }
}

impl<T> From<f64> for Currency<T>
where
    T: CentCurrency,
{
    fn from(value: f64) -> Self {
        Currency::new((value * 100.0) as i64)
    }
}

impl<T> From<i64> for Currency<T>
where
    T: CentCurrency,
{
    fn from(value: i64) -> Self {
        Currency::new(value * 100)
    }
}

impl<T> Display for Currency<T>
where
    T: CurrencyType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", T::formatter(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency() {
        let amount = 123.45;
        let amount = Currency::<EUR>::from(amount);

        assert_eq!(format!("{}", amount), "€ 123.45");
    }

    #[test]
    fn test_currency_usd() {
        let amount = 100.00;
        let amount = Currency::<USD>::from(amount);

        assert_eq!(format!("{}", amount), "$ 100.00");
    }

    #[test]
    fn test_currency_int() {
        let amount = 100;
        let amount = Currency::<USD>::from(amount);

        assert_eq!(format!("{}", amount), "$ 100.00");
    }

    #[test]
    fn test_currency_convert() {
        let amount = 100.00;
        let amount = Currency::<EUR>::from(amount);

        let rate = ExchangeRate::<USD, EUR>::new(1.2);
        let us_amount = amount.convert(rate);

        assert_eq!(format!("{}", us_amount), "$ 120.00");
    }
}
