mod currency;
mod exchange_rate;
mod transaction;

use currency::{Currency, EUR, USD};

fn main() {
    let amount = 123.45;
    let amount: Currency<EUR> = amount.into();

    let us_amount = 1.0;
    let us_amount: Currency<USD> = Currency::<USD>::from(us_amount);

    println!("US amount: {us_amount}")
}
