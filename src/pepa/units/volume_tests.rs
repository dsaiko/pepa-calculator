use rust_decimal_macros::dec;

use crate::units::angle::Angle;
use crate::units::unit::test_units;
use crate::units::{Length, Unit};

#[test]
fn test_angle() {
    test_units(
        "180 degrees in radians",
        &[(dec!(3.1415), Some(Unit::Angle(Angle::Radian)))],
    );

    test_units(
        "180 degrees in gradians",
        &[(dec!(200), Some(Unit::Angle(Angle::Gradian)))],
    );

    test_units(
        "180 degrees in turns",
        &[(dec!(0.5), Some(Unit::Angle(Angle::Turn)))],
    );
}
