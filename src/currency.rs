use log;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Currency {
    USD,
    EUR,
    Other(String),
}

impl FromStr for Currency {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => {
                log::warn!("Unrecognized currency: {}", s);
                Ok(Currency::Other(s.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("USD".parse(), Ok(Currency::USD));
        assert_eq!("EUR".parse(), Ok(Currency::EUR));
        assert_eq!("JPY".parse(), Ok(Currency::Other("JPY".to_string())));
    }
}
