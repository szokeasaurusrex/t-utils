use serde::{Deserialize, Serialize};

use crate::conversions::currency::{Currency, CurrencyType};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExchangeRate<N, D>(
    f64,
    #[serde(skip)] std::marker::PhantomData<N>,
    #[serde(skip)] std::marker::PhantomData<D>,
)
where
    N: CurrencyType,
    D: CurrencyType;

impl<N, D> ExchangeRate<N, D>
where
    N: CurrencyType,
    D: CurrencyType,
{
    pub fn new(rate: f64) -> Self {
        ExchangeRate(
            rate * N::store_factor() / D::store_factor(),
            PhantomData,
            PhantomData,
        )
    }

    pub fn invert(&self) -> ExchangeRate<D, N> {
        ExchangeRate(1.0 / self.0, PhantomData, PhantomData)
    }

    pub fn convert(&self, from_amount: Currency<D>) -> Currency<N> {
        Currency::from_raw_amount((from_amount.raw_amount() as f64 * self.0).round() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversions::currency::{EUR, USD};

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
    struct USC;

    impl CurrencyType for USC {
        fn formatter(_: i64) -> String {
            panic!("Not implemented")
        }

        fn store_factor() -> f64 {
            1.0
        }
    }

    #[test]
    fn test_exchange_rate() {
        let rate = ExchangeRate::<EUR, USD>::new(0.8);
        assert_eq!(rate, ExchangeRate(0.8, PhantomData, PhantomData));
        assert_eq!(rate.invert(), ExchangeRate(1.25, PhantomData, PhantomData));
    }

    #[test]
    fn test_convert_different_store_factor() {
        let rate = ExchangeRate::<USC, USD>::new(100.0);
        let amount = Currency::<USD>::from(100);

        assert_eq!(rate.convert(amount), Currency::<USC>::from(10000));
    }
}
