mod currency;
mod transaction;

use currency::{ Currency, EUR, USD };

fn main() {
    let amount = 123.45;
    let amount: Currency<EUR> = amount.into();

    let us_amount = 234.56;
    let us_amount: Currency<USD> = us_amount.into();

    println!("{amount}, {us_amount}")
}
