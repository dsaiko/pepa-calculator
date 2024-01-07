use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", dec!(648000) / Decimal::PI * dec!(149_597_870_700));

    Ok(())
}
