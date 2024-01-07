use rust_decimal_macros::dec;

use crate::unit_prefixes::UnitPrefix;

#[test]
fn prefix() {
    assert_eq!(UnitPrefix::Centi.multiplier() * dec!(100), dec!(1));
    assert_eq!(UnitPrefix::Milli.multiplier() * dec!(1000), dec!(1));
    assert_eq!(UnitPrefix::Mega.multiplier(), dec!(1_000_000));
}
