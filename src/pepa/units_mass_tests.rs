use rust_decimal_macros::dec;

use crate::utils::test_units;
use crate::UnitPrefix::Kilo;
use crate::{MassUnit, Unit};

#[test]
fn test_length() {
    test_units(
        "(1 kg + 1000 grams +1 t) in kg",
        &[(dec!(1002), Some(Unit::Mass(MassUnit::Gram(Some(Kilo)))))],
    );

    test_units(
        "(1 Mt) in kg",
        &[(
            dec!(1000000000),
            Some(Unit::Mass(MassUnit::Gram(Some(Kilo)))),
        )],
    );

    test_units(
        "(1 dag + 1 dkg) in g",
        &[(dec!(20), Some(Unit::Mass(MassUnit::Gram(None))))],
    );

    test_units(
        "1 long ton in kilogrammes",
        &[(dec!(1016.05), Some(Unit::Mass(MassUnit::Gram(Some(Kilo)))))],
    );

    test_units(
        "1 short ton in kilogrammes",
        &[(dec!(907.18), Some(Unit::Mass(MassUnit::Gram(Some(Kilo)))))],
    );

    test_units(
        "1 lb in grams",
        &[(dec!(453.59), Some(Unit::Mass(MassUnit::Gram(None))))],
    );

    test_units(
        "1 oz in grams",
        &[(dec!(28.35), Some(Unit::Mass(MassUnit::Gram(None))))],
    );

    test_units(
        "1 troy ounce in grams",
        &[(dec!(31.10), Some(Unit::Mass(MassUnit::Gram(None))))],
    );

    test_units(
        "1 troy pound in grams",
        &[(dec!(373.24), Some(Unit::Mass(MassUnit::Gram(None))))],
    );

    test_units(
        "1 slug in grams",
        &[(dec!(14593.90), Some(Unit::Mass(MassUnit::Gram(None))))],
    );

    test_units(
        "1000 grains in grams",
        &[(dec!(64.80), Some(Unit::Mass(MassUnit::Gram(None))))],
    );
}
