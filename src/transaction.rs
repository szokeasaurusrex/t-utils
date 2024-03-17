use chrono::NaiveDate;
use crate::currency::CurrencyType;

pub struct Transaction<T>
    where T: CurrencyType {
    date: NaiveDate,
    amount: T,
}
