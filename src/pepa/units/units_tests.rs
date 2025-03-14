use rust_decimal_macros::dec;

use crate::units::unit::test_units;
use crate::units::{Length, Prefix, Time, Unit};

#[test]
fn test_none() {
    test_units("55 + 55", &[(dec!(110.0), None)]);
}

#[test]
fn test_functions() {
    test_units("(5 h) ^ 2", &[(dec!(25), Some(Unit::Time(Time::Hour)))]);
    test_units(
        "(5 m) ^ 2",
        &[
            (dec!(25), Some(Unit::Time(Time::Minute))),
            (dec!(25), Some(Unit::Length(Length::Meter(None)))),
        ],
    );
    test_units(
        "(5 m) ^ 2 + 1m",
        &[
            (dec!(26), Some(Unit::Time(Time::Minute))),
            (dec!(26), Some(Unit::Length(Length::Meter(None)))),
        ],
    );
    test_units(
        "((5 m) ^ 2 + 1km) to meters",
        &[(dec!(1025), Some(Unit::Length(Length::Meter(None))))],
    );

    test_units(
        "min(5 m, 4m, 1)",
        &[
            (dec!(1), Some(Unit::Time(Time::Minute))),
            (dec!(1), Some(Unit::Length(Length::Meter(None)))),
        ],
    );

    test_units(
        "min(5 m, 4m)",
        &[
            (dec!(4), Some(Unit::Time(Time::Minute))),
            (dec!(4), Some(Unit::Length(Length::Meter(None)))),
        ],
    );

    test_units(
        "max(5 m, 4m, 1km)",
        &[(
            dec!(1),
            Some(Unit::Length(Length::Meter(Some(Prefix::Kilo)))),
        )],
    );

    test_units(
        "max(5 m, 4m, 6m)",
        &[
            (dec!(6), Some(Unit::Time(Time::Minute))),
            (dec!(6), Some(Unit::Length(Length::Meter(None)))),
        ],
    );

    test_units(
        "max(500 m, 4m, 6m) in km",
        &[(
            dec!(0.5),
            Some(Unit::Length(Length::Meter(Some(Prefix::Kilo)))),
        )],
    );

    test_units(
        "pow(60 m, 1 hour)",
        &[(dec!(1), Some(Unit::Time(Time::Hour)))],
    );
}

#[test]
fn test_multi_units() {
    test_units("30 m + 1 h", &[(dec!(1.5), Some(Unit::Time(Time::Hour)))]);

    test_units(
        "5m to km to cm",
        &[(
            dec!(500),
            Some(Unit::Length(Length::Meter(Some(Prefix::Centi)))),
        )],
    );

    test_units(
        "6m to sec to hours to m",
        &[(dec!(6), Some(Unit::Time(Time::Minute)))],
    );

    test_units(
        "5d + 5m + 5h + 5s",
        &[(dec!(450305), Some(Unit::Time(Time::Second(None))))],
    );

    test_units("1s + 1m", &[(dec!(1.02), Some(Unit::Time(Time::Minute)))]);

    test_units(
        "30000ms + 5m",
        &[(dec!(5.5), Some(Unit::Time(Time::Minute)))],
    );

    test_units(
        "5 + 5m",
        &[
            (dec!(10), Some(Unit::Time(Time::Minute))),
            (dec!(10), Some(Unit::Length(Length::Meter(None)))),
        ],
    );

    test_units(
        "5m + 5m",
        &[
            (dec!(10), Some(Unit::Time(Time::Minute))),
            (dec!(10), Some(Unit::Length(Length::Meter(None)))),
        ],
    );

    test_units(
        "500m + 5km",
        &[(
            dec!(5.5),
            Some(Unit::Length(Length::Meter(Some(Prefix::Kilo)))),
        )],
    );

    test_units(
        "(33 + 3) m + 15",
        &[
            (dec!(51), Some(Unit::Time(Time::Minute))),
            (dec!(51), Some(Unit::Length(Length::Meter(None)))),
        ],
    );

    test_units(
        "(33m + 3m) m + 15m",
        &[
            (dec!(51), Some(Unit::Time(Time::Minute))),
            (dec!(51), Some(Unit::Length(Length::Meter(None)))),
        ],
    );

    test_units(
        "(33m + 3m) m + 3 + 15m + 1",
        &[
            (dec!(55), Some(Unit::Time(Time::Minute))),
            (dec!(55), Some(Unit::Length(Length::Meter(None)))),
        ],
    );
}
