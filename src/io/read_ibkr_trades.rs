use std::io::Read;

use chrono::NaiveDate;
use csv::Reader;
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
pub struct IbkrInputLine {
    #[serde(rename = "DataDiscriminator")]
    data_discriminator: String,
    #[serde(rename = "Currency")]
    currency: String,
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "Date/Time")]
    #[serde(deserialize_with = "parse_ibkr_date_time")]
    date: NaiveDate,
    #[serde(rename = "Quantity")]
    quantity: f64,
    #[serde(rename = "T. Price")]
    t_price: f64,
    #[serde(rename = "Proceeds")]
    proceeds: Option<f64>,
    #[serde(rename = "Basis")]
    basis: f64,
}

pub fn read_ibkr_trades<R>(reader: &mut Reader<R>) -> Result<Vec<IbkrInputLine>, csv::Error>
where
    R: Read,
{
    reader.deserialize().collect()
}

fn parse_ibkr_date_time<'de, D>(date_time: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_time = String::deserialize(date_time)?;
    let date = date_time
        .split(",")
        .next()
        .ok_or_else(|| Error::custom("Missing date"))?;
    NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(Error::custom)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_read_ibkr_trades() {
        let mut reader = Reader::from_path("test_files/test_ibkr.csv").unwrap();
        let trades = read_ibkr_trades(&mut reader).unwrap();
        assert_eq!(
            trades,
            vec![
                IbkrInputLine {
                    data_discriminator: "Order".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    proceeds: Some(-20.0),
                    basis: 20.0
                },
                IbkrInputLine {
                    data_discriminator: "Trade".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    proceeds: Some(-20.0),
                    basis: 20.0
                },
                IbkrInputLine {
                    data_discriminator: "Order".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 7, 10).unwrap(),
                    quantity: -3.0,
                    t_price: 12.0,
                    proceeds: Some(36.0),
                    basis: 31.0
                },
                IbkrInputLine {
                    data_discriminator: "Trade".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 7, 10).unwrap(),
                    quantity: -3.0,
                    t_price: 12.0,
                    proceeds: Some(36.0),
                    basis: 31.0
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 4, 2).unwrap(),
                    quantity: 1.0,
                    t_price: 11.0,
                    proceeds: None,
                    basis: 11.0
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 2, 4).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    proceeds: None,
                    basis: 22.0
                },
            ]
        )
    }
}
