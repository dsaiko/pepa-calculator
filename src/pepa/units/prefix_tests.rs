use rust_decimal_macros::dec;

use crate::units::Prefix;

#[test]
fn test_prefix() {
    assert_eq!(Prefix::Centi.multiplier() * dec!(100), dec!(1));
    assert_eq!(Prefix::Milli.multiplier() * dec!(1000), dec!(1));
    assert_eq!(Prefix::Mega.multiplier(), dec!(1_000_000));
}
