use crate::currency::CurrencyType;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct ExchangeRate<T, F>(
    pub f64,
    std::marker::PhantomData<T>,
    std::marker::PhantomData<F>,
)
where
    T: CurrencyType,
    F: CurrencyType;

impl<T, F> ExchangeRate<T, F>
where
    T: CurrencyType,
    F: CurrencyType,
{
    pub fn new(rate: f64) -> Self {
        ExchangeRate(rate, PhantomData, PhantomData)
    }
}
