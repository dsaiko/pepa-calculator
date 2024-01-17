use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::path::Prefix;

use pepino::{Calc, LengthUnit, NumericExpression, TemperatureUnit, TimeUnit, Unit, UnitPrefix};

fn test(test: &str, res: &[(Decimal, Option<Unit>)]) {
    let mut computer = Calc::default();
    let statement = computer.compute(test).unwrap();

    if let Err(e) = statement.expression {
        panic!("Error in expression: '{:?}': {:?}", test, e);
    }

    match statement.result {
        Err(e) => {
            panic!("Error in computation: '{:?}': {:?}", test, e);
        }
        Ok(n) => {
            let t = NumericExpression::with_multiple_units(res.to_vec());
            if n != t {
                panic!("{:?}: {:?} != {:?}", test, n, t);
            }
        }
    }
}

#[test]
fn none() {
    test("55 + 55", &[(dec!(110.0), None)]);
}

#[test]
fn temperature_celsius() {
    let unit = Some(Unit::Temperature(TemperatureUnit::DegreesCelsius));

    test("(33 + 3) celsius + 15", &[(dec!(51.0), unit)]);
    test("celsius((33 + 3) kelvins)", &[(dec!(-237.15), unit)]);
    test("celsius(33 + 3 kelvins)", &[(dec!(-237.15), unit)]);
    test("celsius(55)", &[(dec!(55.0), unit)]);
    test("pow(55 celsius, 2)", &[(dec!(3025.0), unit)]);
    test("55 celsius", &[(dec!(55.0), unit)]);
    test("celsius 55", &[(dec!(55.0), unit)]);
    test("55°C", &[(dec!(55.0), unit)]);
    test("°c55", &[(dec!(55.0), unit)]);
    test("degrees celsius 55", &[(dec!(55.0), unit)]);
    test("55 degrees celsius", &[(dec!(55.0), unit)]);
    test("10 * 5 celsius", &[(dec!(50.0), unit)]);
    test("10 * celsius(5 + 5)", &[(dec!(100.0), unit)]);
    test("15 fahrenheits + 3 celsius", &[(dec!(-6.44), unit)]);
    test(
        "15 fahrenheits + 3 kelvins + 2 * 20 celsius",
        &[(dec!(33.56), unit)],
    );
}

#[test]
fn temperature_kelvin() {
    let unit = Some(Unit::Temperature(TemperatureUnit::Kelvin));

    test("kelvin(pow(55 celsius, 2))", &[(dec!(3298.15), unit)]);
}

#[test]
fn temperature_fahrenheit() {
    let unit = Some(Unit::Temperature(TemperatureUnit::DegreesFahrenheit));

    test("10 celsius + pow(5,2) Fahrenheit", &[(dec!(75), unit)]);
    test("10 celsius + (pow(5,2) Fahrenheit)", &[(dec!(75), unit)]);
    test("10 celsius + Fahrenheit(pow(5,2)) ", &[(dec!(75), unit)]);
}

#[test]
fn in_operator() {
    let unit = Some(Unit::Temperature(TemperatureUnit::DegreesFahrenheit));

    test("pow(5,2) Celsius to Fahrenheit", &[(dec!(77), unit)]);
    test(
        "10 kelvins to celsius to kelvins to Fahrenheit",
        &[(dec!(-441.67), unit)],
    );
}

#[test]
fn list_fce() {
    let unit = Some(Unit::Temperature(TemperatureUnit::DegreesCelsius));

    test(
        "min(55 Celsius, 180 Kelvin) into celsius",
        &[(dec!(-93.15), unit)],
    );
    test(
        "sum(5 Celsius, 180 Kelvin, 10 Fahrenheit, 10 Celsius)",
        &[(dec!(-90.37), unit)],
    );
}

#[test]
fn time() {
    let unit = Some(Unit::Time(TimeUnit::Hour));

    test(
        "(5 days + 1 hour + (60 * 30) seconds + 15 min) in hours",
        &[(dec!(121.75), unit)],
    );
    test(
        "(5 d + 1 h + (60 * 30) s + 15 min) in hours",
        &[(dec!(121.75), unit)],
    );
}

#[test]
fn pow() {
    let unit = Some(Unit::Time(TimeUnit::Hour));

    test("pow(5 hours, 2 hours)", &[(dec!(25), unit)]);
}

#[test]
fn multiunits() {
    test(
        "30 m + 1 h",
        &[(dec!(1.5), Some(Unit::Time(TimeUnit::Hour)))],
    );

    test(
        "5m to km to cm",
        &[(
            dec!(500),
            Some(Unit::Length(LengthUnit::Meter(Some(UnitPrefix::Centi)))),
        )],
    );

    test(
        "6m to sec to hours to m",
        &[(dec!(6), Some(Unit::Time(TimeUnit::Minute)))],
    );

    test(
        "5d + 5m + 5h + 5s",
        &[(dec!(450305), Some(Unit::Time(TimeUnit::Second(None))))],
    );

    test(
        "1s + 1m",
        &[(dec!(1.02), Some(Unit::Time(TimeUnit::Minute)))],
    );

    test(
        "30000ms + 5m",
        &[(dec!(5.5), Some(Unit::Time(TimeUnit::Minute)))],
    );

    test(
        "5 + 5m",
        &[
            (dec!(10), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(10), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );

    // min(5, 10, 5h)
    // min (5,10,5m)

    // test(
    //     &[("5m + 5m", dec!(10))],
    //     Some(Unit::Length(LengthUnit::Meter(Some(UnitPrefix::Kilo)))),
    // );

    // let tests = vec![("500 m + 10 km", dec!(10.5))];
    // test(
    //     tests,
    //     Some(Unit::Length(LengthUnit::Meter(Some(UnitPrefix::Kilo)))),
    // );
}
