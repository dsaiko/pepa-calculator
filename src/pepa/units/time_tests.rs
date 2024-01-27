use rust_decimal_macros::dec;

use crate::units::{Time, Unit};
use crate::utils::test_units;

#[test]
fn test_time() {
    let unit = Some(Unit::Time(Time::Hour));

    test_units(
        "(5 days + 1 hour + (60 * 30) seconds + 15 min) in hours",
        &[(dec!(121.75), unit)],
    );
    test_units(
        "(5 d + 1 h + (60 * 30) s + 15 min) in hours",
        &[(dec!(121.75), unit)],
    );
}
