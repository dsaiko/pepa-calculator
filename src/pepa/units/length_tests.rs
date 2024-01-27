use rust_decimal_macros::dec;

use crate::units::{Length, Prefix, Unit};
use crate::utils::test_units;

#[test]
fn test_length() {
    test_units(
        "3 kilom + 500 METRES",
        &[(dec!(3500), Some(Unit::Length(Length::Meter(None))))],
    );

    test_units(
        "3 kilometres + 500 METRES",
        &[(dec!(3500), Some(Unit::Length(Length::Meter(None))))],
    );

    test_units(
        "(1 Mm + 1000mm) in metres",
        &[(dec!(1000001), Some(Unit::Length(Length::Meter(None))))],
    );

    test_units(
        "3 kilom + 500 m",
        &[(dec!(3500), Some(Unit::Length(Length::Meter(None))))],
    );

    test_units(
        "3 km in nm",
        &[(
            dec!(3000000000000),
            Some(Unit::Length(Length::Meter(Some(Prefix::Nano)))),
        )],
    );

    test_units(
        "1 pc in km",
        &[(
            dec!(30856775814913.67),
            Some(Unit::Length(Length::Meter(Some(Prefix::Kilo)))),
        )],
    );

    test_units(
        "1 au in km",
        &[(
            dec!(149597870.70),
            Some(Unit::Length(Length::Meter(Some(Prefix::Kilo)))),
        )],
    );

    test_units(
        "1 ly in km",
        &[(
            dec!(9460730472580.80),
            Some(Unit::Length(Length::Meter(Some(Prefix::Kilo)))),
        )],
    );

    test_units(
        "1 inch in cm",
        &[(
            dec!(2.54),
            Some(Unit::Length(Length::Meter(Some(Prefix::Centi)))),
        )],
    );

    test_units(
        "10 th in inches",
        &[(dec!(0.01), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 barleycorn in inches",
        &[(dec!(0.33), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "10 foots in inches",
        &[(dec!(120), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "10 yards in inches",
        &[(dec!(360), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 mile in inches",
        &[(dec!(63360), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 pole in inches",
        &[(dec!(198), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 rod in inches",
        &[(dec!(198), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 Fathom in inches",
        &[(dec!(72), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 NM in metres",
        &[(dec!(1852), Some(Unit::Length(Length::Meter(None))))],
    );

    test_units(
        "1 League in inches",
        &[(dec!(190080), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 NL in miles",
        &[(dec!(3.45), Some(Unit::Length(Length::Mile)))],
    );

    test_units(
        "1 Furlong in inches",
        &[(dec!(7920), Some(Unit::Length(Length::Inch)))],
    );

    test_units(
        "1 Chain in inches",
        &[(dec!(792), Some(Unit::Length(Length::Inch)))],
    );
}
