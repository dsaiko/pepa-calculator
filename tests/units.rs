use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use pepino::{Calc, TemperatureUnit, TimeUnit, Unit};

fn test(tests: Vec<(&str, Decimal)>, unit: Option<Unit>) {
    for test in tests {
        let mut computer = Calc::default();
        let statement = computer.compute(test.0).unwrap();

        if let Err(e) = statement.expression {
            panic!("Error in expression: '{:?}': {:?}", test.0, e);
        }

        match statement.result {
            Err(e) => {
                panic!("Error in computation: '{:?}': {:?}", test.0, e);
            }
            Ok(n) => {
                // round v
                let v = (n.value * dec!(100.0)).round() / dec!(100.0);
                if v != test.1 || n.unit != unit {
                    panic!("{:?}: {:?} != {:?} {:?}", test.0, v, test.1, unit);
                }
            }
        }
    }
}

#[test]
fn none() {
    let tests = vec![("55 + 55", dec!(110.0))];

    test(tests, None);
}

#[test]
fn temperature_celsius() {
    let tests = vec![
        ("(33 + 3) celsius + 15", dec!(51.0)),
        ("celsius((33 + 3) kelvins)", dec!(-237.15)),
        ("celsius(33 + 3 kelvins)", dec!(-237.15)),
        ("celsius(55)", dec!(55.0)),
        ("pow(55 celsius, 2)", dec!(3025.0)),
        ("55 celsius", dec!(55.0)),
        ("celsius 55", dec!(55.0)),
        ("55°C", dec!(55.0)),
        ("°c55", dec!(55.0)),
        ("degrees celsius 55", dec!(55.0)),
        ("55 degrees celsius", dec!(55.0)),
        ("10 * 5 celsius", dec!(50.0)),
        ("10 * celsius(5 + 5)", dec!(100.0)),
        ("15 fahrenheits + 3 celsius", dec!(-6.44)),
        ("15 fahrenheits + 3 kelvins + 2 * 20 celsius", dec!(33.56)),
    ];

    test(
        tests,
        Some(Unit::Temperature(TemperatureUnit::DegreesCelsius)),
    );
}

#[test]
fn temperature_kelvin() {
    let tests = vec![("kelvin(pow(55 celsius, 2))", dec!(3298.15))];

    test(tests, Some(Unit::Temperature(TemperatureUnit::Kelvin)));
}

#[test]
fn temperature_fahrenheit() {
    let tests = vec![
        ("10 celsius + pow(5,2) Fahrenheit", dec!(75.0)),
        ("10 celsius + (pow(5,2) Fahrenheit)", dec!(75.0)),
        ("10 celsius + Fahrenheit(pow(5,2)) ", dec!(75.0)),
    ];

    test(
        tests,
        Some(Unit::Temperature(TemperatureUnit::DegreesFahrenheit)),
    );
}

#[test]
fn in_operator() {
    let tests = vec![
        ("pow(5,2) Celsius to Fahrenheit", dec!(77.0)),
        (
            "10 kelvins to celsius to kelvins to Fahrenheit",
            dec!(-441.67),
        ),
    ];

    test(
        tests,
        Some(Unit::Temperature(TemperatureUnit::DegreesFahrenheit)),
    );
}

#[test]
fn list_fce() {
    let tests = vec![
        ("min(55 Celsius, 180 Kelvin) into celsius", dec!(-93.15)),
        (
            "sum(5 Celsius, 180 Kelvin, 10 Fahrenheit, 10 Celsius)",
            dec!(-90.37),
        ),
    ];

    test(
        tests,
        Some(Unit::Temperature(TemperatureUnit::DegreesCelsius)),
    );
}

#[test]
fn time() {
    let tests = vec![(
        "(5 days + 1 hour + (60 * 30) seconds + 15 min) in hours",
        dec!(121.75),
    )];

    test(tests, Some(Unit::Time(TimeUnit::Hour)));
}

#[test]
fn pow() {
    let tests = vec![("pow(5 hours, 2 hours)", dec!(25.0))];

    test(tests, Some(Unit::Time(TimeUnit::Hour)));
}
