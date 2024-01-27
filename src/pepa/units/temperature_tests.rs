use rust_decimal_macros::dec;

use crate::units::unit::test_units;
use crate::units::{Temperature, Unit};

#[test]
fn test_celsius() {
    let unit = Some(Unit::Temperature(Temperature::DegreesCelsius));

    test_units("(33 + 3) celsius + 15", &[(dec!(51.0), unit)]);
    test_units("celsius((33 + 3) kelvins)", &[(dec!(-237.15), unit)]);
    test_units("celsius(33 + 3 kelvins)", &[(dec!(-237.15), unit)]);
    test_units("celsius(55)", &[(dec!(55.0), unit)]);
    test_units("pow(55 celsius, 2)", &[(dec!(3025.0), unit)]);
    test_units("55 celsius", &[(dec!(55.0), unit)]);
    test_units("celsius 55", &[(dec!(55.0), unit)]);
    test_units("55°C", &[(dec!(55.0), unit)]);
    test_units("°c55", &[(dec!(55.0), unit)]);
    test_units("degrees celsius 55", &[(dec!(55.0), unit)]);
    test_units("55 degrees celsius", &[(dec!(55.0), unit)]);
    test_units("10 * 5 celsius", &[(dec!(50.0), unit)]);
    test_units("10 * celsius(5 + 5)", &[(dec!(100.0), unit)]);
    test_units("15 fahrenheits + 3 celsius", &[(dec!(-6.44), unit)]);
    test_units(
        "15 fahrenheits + 3 kelvins + 2 * 20 celsius",
        &[(dec!(33.56), unit)],
    );
}

#[test]
fn test_kelvin() {
    let unit = Some(Unit::Temperature(Temperature::Kelvin));

    test_units("kelvin(pow(55 celsius, 2))", &[(dec!(3298.15), unit)]);
}

#[test]
fn test_fahrenheit() {
    let unit = Some(Unit::Temperature(Temperature::DegreesFahrenheit));

    test_units("10 celsius + pow(5,2) Fahrenheit", &[(dec!(75), unit)]);
    test_units("10 celsius + (pow(5,2) Fahrenheit)", &[(dec!(75), unit)]);
    test_units("10 celsius + Fahrenheit(pow(5,2)) ", &[(dec!(75), unit)]);
}

#[test]
fn test_in_operator() {
    let unit = Some(Unit::Temperature(Temperature::DegreesFahrenheit));

    test_units("pow(5,2) Celsius to Fahrenheit", &[(dec!(77), unit)]);
    test_units(
        "10 kelvins to celsius to kelvins to Fahrenheit",
        &[(dec!(-441.67), unit)],
    );
}

#[test]
fn test_list_fce() {
    let unit = Some(Unit::Temperature(Temperature::DegreesCelsius));

    test_units(
        "min(55 Celsius, 180 Kelvin) into celsius",
        &[(dec!(-93.15), unit)],
    );
    test_units(
        "sum(5 Celsius, 180 Kelvin, 10 Fahrenheit, 10 Celsius)",
        &[(dec!(-90.37), unit)],
    );
}
