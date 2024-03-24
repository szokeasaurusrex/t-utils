use std::{fmt::Display, marker::PhantomData};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait CurrencyType: core::fmt::Debug + Serialize + Copy {
    fn formatter(raw_amount: i64) -> String;

    /// The factor to store the currency amount in the struct.
    fn store_factor() -> f64;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Currency<T>(i64, PhantomData<T>)
where
    T: CurrencyType;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EUR;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

impl<'de, T> Deserialize<'de> for Currency<T>
where
    T: CurrencyType,
{
    fn deserialize<D>(deserializer: D) -> Result<Currency<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let amount = f64::deserialize(deserializer)?;
        Ok(Currency::from(amount))
    }
}

impl<T> Serialize for Currency<T>
where
    T: CurrencyType,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.0 as f64 / T::store_factor())
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

    #[test]
    fn test_currency_deserialize() {
        let csv = "amount\n123.45";
        let amount: Currency<EUR> = csv::Reader::from_reader(csv.as_bytes())
            .deserialize()
            .next()
            .unwrap()
            .unwrap();
        assert_eq!(format!("{}", amount), "€ 123.45");
    }

    #[test]
    fn test_currency_deserialize_int() {
        let csv = "amount\n100";
        let amount: Currency<USD> = csv::Reader::from_reader(csv.as_bytes())
            .deserialize()
            .next()
            .unwrap()
            .unwrap();
        assert_eq!(format!("{}", amount), "$ 100.00");
    }

    #[test]
    fn test_currency_serialize() {
        let amount = Currency::<EUR>::from(123.45);
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(amount).unwrap();
        assert_eq!(
            String::from_utf8(wtr.into_inner().unwrap()).unwrap(),
            "123.45\n"
        );
    }
}
