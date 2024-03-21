use std::{fmt::Display, marker::PhantomData};

pub trait CurrencyType {}

pub trait CentCurrency: CurrencyType {}

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

impl PrependSymbolCurrency for EUR {
    fn symbol() -> &'static str {
        "€"
    }
}

impl PrependSymbolCurrency for USD {
    fn symbol() -> &'static str {
        "$"
    }
}

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

impl<T> Display for Currency<T>
where
    T: PrependSymbolCurrency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", T::symbol(), self.0 as f64 / 100.0)
    }
}
