use crate::currency::CurrencyType;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn invert(&self) -> ExchangeRate<F, T> {
        ExchangeRate(1.0 / self.0, PhantomData, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::{EUR, USD};

    #[test]
    fn test_exchange_rate() {
        let rate = ExchangeRate::<EUR, USD>::new(0.8);
        assert_eq!(rate, ExchangeRate(0.8, PhantomData, PhantomData));
        assert_eq!(rate.invert(), ExchangeRate(1.25, PhantomData, PhantomData));
    }
}
