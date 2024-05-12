use serde::{Deserialize, Deserializer, Serialize};

use crate::conversions::currency::{Currency, CurrencyType};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExchangeRate<N, D>(
    f64,
    std::marker::PhantomData<N>,
    std::marker::PhantomData<D>,
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

    pub fn rate(&self) -> f64 {
        self.0 * D::store_factor() / N::store_factor()
    }

    pub fn invert(&self) -> ExchangeRate<D, N> {
        ExchangeRate(1.0 / self.0, PhantomData, PhantomData)
    }

    pub fn convert(&self, from_amount: Currency<D>) -> Currency<N> {
        Currency::from_raw_amount((from_amount.raw_amount() as f64 * self.0).round() as i64)
    }
}

impl<'de, N, D> Deserialize<'de> for ExchangeRate<N, D>
where
    N: CurrencyType,
    D: CurrencyType,
{
    fn deserialize<De>(deserializer: De) -> Result<ExchangeRate<N, D>, De::Error>
    where
        De: Deserializer<'de>,
    {
        let rate = f64::deserialize(deserializer)?;
        Ok(ExchangeRate::new(rate))
    }
}

impl<N, D> Serialize for ExchangeRate<N, D>
where
    N: CurrencyType,
    D: CurrencyType,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.rate().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversions::currency::{EUR, USD};
    use csv::Reader;

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

    #[test]
    fn test_rate_deserialize() {
        let csv = "rate\n0.8\n";
        let rate: ExchangeRate<EUR, USD> = Reader::from_reader(csv.as_bytes())
            .deserialize()
            .next()
            .unwrap()
            .unwrap();

        assert_eq!(rate, ExchangeRate(0.8, PhantomData, PhantomData));
    }

    #[test]
    fn test_rate_deserialize_usc() {
        let csv = "rate\n100\n";
        let rate: ExchangeRate<USC, USD> = Reader::from_reader(csv.as_bytes())
            .deserialize()
            .next()
            .unwrap()
            .unwrap();

        assert_eq!(rate, ExchangeRate(1.0, PhantomData, PhantomData));
    }
}
