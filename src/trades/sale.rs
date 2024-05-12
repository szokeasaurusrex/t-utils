use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    slice::Iter,
};

use chrono::NaiveDate;

use crate::{
    currency::Currency,
    io::read_ibkr_trades::{IbkrInput, IbkrInputLine},
};

#[derive(Debug, PartialEq)]
pub struct IbkrSales {
    sales: Vec<IbkrSale>,
}

#[derive(Debug, PartialEq)]
pub struct IbkrSale {
    currency: Currency,
    symbol: String,
    date: NaiveDate,
    t_price: f64,
    proceeds: f64,
    closed_lots: Vec<ClosedLot>,
}

#[derive(Debug, PartialEq)]
struct ClosedLot {
    date: NaiveDate,
    quantity: f64,
    t_price: f64,
    basis: f64,
}

#[derive(Debug)]
pub enum IbkrSaleError {
    NoSales,
    UnmatchedClosedLot,
    LotClosedAfterTrade,
    LotSumMismatch,
    TradeMissingProceeds,
}

impl Display for IbkrSaleError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoSales => write!(f, "No sales found"),
            Self::UnmatchedClosedLot => write!(f, "Unmatched closed lot"),
            Self::LotClosedAfterTrade => write!(f, "Lot closed after trade"),
            Self::TradeMissingProceeds => write!(f, "Trade missing proceeds"),
            Self::LotSumMismatch => write!(f, "Sum of closed lots does not match trade basis"),
        }
    }
}
impl Error for IbkrSaleError {}

impl ClosedLot {
    fn from_closed_lot_line(
        line: &IbkrInputLine,
        trade: &IbkrInputLine,
    ) -> Result<Self, IbkrSaleError> {
        let date = line.date;
        let quantity = line.quantity;
        let t_price = line.t_price;
        let basis = line.basis;

        if trade.symbol != line.symbol || trade.currency != line.currency {
            return Err(IbkrSaleError::UnmatchedClosedLot);
        } else if line.date > trade.date {
            return Err(IbkrSaleError::LotClosedAfterTrade);
        }

        Ok(Self {
            date,
            quantity,
            t_price,
            basis,
        })
    }
}

impl IbkrSale {
    fn from_sales_slice(sale_info: &[IbkrInputLine]) -> Result<Self, IbkrSaleError> {
        let trade = &sale_info[0];
        let closed_lots = sale_info[1..]
            .iter()
            .map(|line| ClosedLot::from_closed_lot_line(line, &trade))
            .collect::<Result<Vec<_>, _>>()?;

        if closed_lots.iter().map(|lot| lot.basis).sum::<f64>() != trade.basis
            || closed_lots.iter().map(|lot| lot.quantity).sum::<f64>() != trade.quantity
        {
            return Err(IbkrSaleError::LotSumMismatch);
        }

        Ok(Self {
            currency: trade.currency.parse().expect("Infallible"),
            symbol: trade.symbol.clone(),
            date: trade.date,
            t_price: trade.t_price,
            proceeds: trade.proceeds.ok_or(IbkrSaleError::TradeMissingProceeds)?,
            closed_lots,
        })
    }
}

impl TryFrom<IbkrInput> for IbkrSales {
    type Error = IbkrSaleError;

    fn try_from(input: IbkrInput) -> Result<Self, Self::Error> {
        let trades: Vec<_> = input
            .lines
            .into_iter()
            .filter(|line| {
                line.data_discriminator == "Trade" || line.data_discriminator == "ClosedLot"
            })
            .filter(|line| line.proceeds.map_or(true, |proceeds| proceeds >= 0.0))
            .collect();

        let sales_slices = sales_slices(&trades)?;

        let sales = sales_slices
            .iter()
            .map(|slice| IbkrSale::from_sales_slice(slice))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { sales })
    }
}

fn sales_slices(sales_trades: &[IbkrInputLine]) -> Result<Vec<&[IbkrInputLine]>, IbkrSaleError> {
    let trades_indices: Vec<_> = sales_trades
        .iter()
        .enumerate()
        .filter_map(|(i, trade)| {
            if trade.data_discriminator == "Trade" {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    if trades_indices.len() == 0 {
        return Err(IbkrSaleError::NoSales);
    } else if trades_indices[0] != 0 {
        return Err(IbkrSaleError::UnmatchedClosedLot);
    }

    Ok(trades_indices
        .iter()
        .zip(trades_indices[1..].iter().chain(&[sales_trades.len()]))
        .map(|(start, end)| &sales_trades[*start..*end])
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ibkr_sales() {
        let input = IbkrInput {
            lines: vec![
                IbkrInputLine {
                    data_discriminator: "Trade".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                    quantity: 2.0,
                    t_price: 11.0,
                    proceeds: Some(22.0),
                    basis: 20.0,
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    proceeds: None,
                    basis: 20.0,
                },
            ],
        };

        let sales = IbkrSales::try_from(input).unwrap();

        assert!(sales.sales.len() == 1);
        assert_eq!(
            sales.sales[0],
            IbkrSale {
                currency: Currency::USD,
                symbol: "TST".into(),
                date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                t_price: 11.0,
                proceeds: 22.0,
                closed_lots: vec![ClosedLot {
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    basis: 20.0,
                }],
            }
        );
    }

    #[test]
    fn test_ibkr_sales_multiple_lots() {
        let input = IbkrInput {
            lines: vec![
                IbkrInputLine {
                    data_discriminator: "Trade".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                    quantity: 3.0,
                    t_price: 11.0,
                    proceeds: Some(33.0),
                    basis: 30.0,
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 1.0,
                    t_price: 10.0,
                    proceeds: None,
                    basis: 10.0,
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    proceeds: None,
                    basis: 20.0,
                },
            ],
        };

        let sales = IbkrSales::try_from(input).unwrap();

        assert!(sales.sales.len() == 1);
        assert_eq!(
            sales.sales[0],
            IbkrSale {
                currency: Currency::USD,
                symbol: "TST".into(),
                date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                t_price: 11.0,
                proceeds: 33.0,
                closed_lots: vec![
                    ClosedLot {
                        date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                        quantity: 1.0,
                        t_price: 10.0,
                        basis: 10.0,
                    },
                    ClosedLot {
                        date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                        quantity: 2.0,
                        t_price: 10.0,
                        basis: 20.0,
                    },
                ],
            }
        );
    }

    #[test]
    fn test_ibkr_sales_concatenated_inputs() {
        let input = IbkrInput {
            lines: vec![
                IbkrInputLine {
                    data_discriminator: "Trade".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                    quantity: 2.0,
                    t_price: 11.0,
                    proceeds: Some(22.0),
                    basis: 20.0,
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    proceeds: None,
                    basis: 20.0,
                },
                IbkrInputLine {
                    data_discriminator: "Trade".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                    quantity: 3.0,
                    t_price: 11.0,
                    proceeds: Some(33.0),
                    basis: 30.0,
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 1.0,
                    t_price: 10.0,
                    proceeds: None,
                    basis: 10.0,
                },
                IbkrInputLine {
                    data_discriminator: "ClosedLot".into(),
                    currency: "USD".into(),
                    symbol: "TST".into(),
                    date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    proceeds: None,
                    basis: 20.0,
                },
            ],
        };

        let sales = IbkrSales::try_from(input).unwrap();

        assert!(sales.sales.len() == 2);
        assert_eq!(
            sales.sales[0],
            IbkrSale {
                currency: Currency::USD,
                symbol: "TST".into(),
                date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                t_price: 11.0,
                proceeds: 22.0,
                closed_lots: vec![ClosedLot {
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    quantity: 2.0,
                    t_price: 10.0,
                    basis: 20.0,
                },],
            }
        );
        assert_eq!(
            sales.sales[1],
            IbkrSale {
                currency: Currency::USD,
                symbol: "TST".into(),
                date: NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(),
                t_price: 11.0,
                proceeds: 33.0,
                closed_lots: vec![
                    ClosedLot {
                        date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                        quantity: 1.0,
                        t_price: 10.0,
                        basis: 10.0,
                    },
                    ClosedLot {
                        date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                        quantity: 2.0,
                        t_price: 10.0,
                        basis: 20.0,
                    },
                ],
            }
        );
    }
}
