use std::{fmt::Display, marker::PhantomData};

pub trait CurrencyType {}

trait CentCurrency: CurrencyType {}

pub trait PrependSymbolCurrency: CurrencyType {
    fn symbol() -> &'static str;
}

#[derive(Clone, Copy, Debug)]
pub struct Currency<T>(i64, PhantomData<T>)
where
    T: CurrencyType;

pub struct EUR;
pub struct USD;

impl CurrencyType for EUR {}

impl CurrencyType for USD {}

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
    T: PrependSymbolCurrency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:.2}", T::symbol(), self.0 as f64 / 100.0)
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
}
