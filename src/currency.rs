use std::{fmt::Display, marker::PhantomData};

pub trait CurrencyType {
    fn formatter(raw_amount: i64) -> String;

    /// The factor to store the currency amount in the struct.
    fn store_factor() -> f64;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    fn store_factor() -> f64 {
        100.0
    }
}

impl CurrencyType for USD {
    fn formatter(raw_amount: i64) -> String {
        format!("$ {:.2}", raw_amount as f64 / 100.0)
    }

    fn store_factor() -> f64 {
        100.0
    }
}

impl<T> Currency<T>
where
    T: CurrencyType,
{
    pub fn from_raw_amount(raw_amount: i64) -> Self {
        Currency(raw_amount, PhantomData)
    }

    pub fn raw_amount(&self) -> i64 {
        self.0
    }
}

impl<T> From<f64> for Currency<T>
where
    T: CurrencyType,
{
    fn from(value: f64) -> Self {
        Currency::from_raw_amount((value * T::store_factor()) as i64)
    }
}

impl<T> From<i64> for Currency<T>
where
    T: CurrencyType,
{
    fn from(value: i64) -> Self {
        Currency::from_raw_amount(value * T::store_factor() as i64)
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

    // #[test]
    // fn test_currency_convert() {
    //     let amount = 100.00;
    //     let amount = Currency::<EUR>::from(amount);

    //     let rate = ExchangeRate::<USD, EUR>::new(1.2);
    //     let us_amount = amount.convert(rate);

    //     assert_eq!(format!("{}", us_amount), "$ 120.00");
    // }
}
