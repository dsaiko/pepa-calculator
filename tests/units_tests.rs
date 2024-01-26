use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use pepa::{Calc, LengthUnit, NumericExpression, TemperatureUnit, TimeUnit, Unit, UnitPrefix};

fn test(test: &str, res: &[(Decimal, Option<Unit>)]) {
    let mut computer = Calc::default();
    let statement = computer.compute(test).unwrap();

    if let Err(e) = &statement.expression {
        panic!("Error in expression: '{:?}': {:?}", test, e);
    }

    match &statement.result {
        None => panic!("No result for: '{:?}'", test),
        Some(Err(e)) => panic!("Error in computation: '{:?}': {:?}", test, e),
        Some(Ok(n)) => {
            let t = NumericExpression::with_multiple_units(res.to_vec());
            let mut ok = true;

            let mut v1 = t.values();
            let mut v2 = n.values();

            if v1.len() != v2.len() {
                ok = false;
            } else {
                for i in 0..v1.len() {
                    if v1[i].1 != v2[i].1 {
                        ok = false;
                    }

                    let n1 = (v1[i].0 * dec!(100)).round() / dec!(100);
                    let n2 = (v2[i].0 * dec!(100)).round() / dec!(100);

                    v1[i].0 = n1;
                    v2[i].0 = n2;

                    if n1 != n2 {
                        ok = false;
                    }
                }
            }

            if !ok {
                panic!("{:?}: {:?} != {:?}", test, v1, v2);
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
fn functions() {
    test("(5 h) ^ 2", &[(dec!(25), Some(Unit::Time(TimeUnit::Hour)))]);
    test(
        "(5 m) ^ 2",
        &[
            (dec!(25), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(25), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );
    test(
        "(5 m) ^ 2 + 1m",
        &[
            (dec!(26), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(26), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );
    test(
        "((5 m) ^ 2 + 1km) in meters",
        &[(dec!(1025), Some(Unit::Length(LengthUnit::Meter(None))))],
    );

    test(
        "min(5 m, 4m, 1)",
        &[
            (dec!(1), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(1), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );

    test(
        "min(5 m, 4m)",
        &[
            (dec!(4), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(4), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );

    test(
        "max(5 m, 4m, 1km)",
        &[(
            dec!(1),
            Some(Unit::Length(LengthUnit::Meter(Some(UnitPrefix::Kilo)))),
        )],
    );

    test(
        "max(5 m, 4m, 6m)",
        &[
            (dec!(6), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(6), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );

    test(
        "max(500 m, 4m, 6m) in km",
        &[(
            dec!(0.5),
            Some(Unit::Length(LengthUnit::Meter(Some(UnitPrefix::Kilo)))),
        )],
    );

    test(
        "pow(60 m, 1 hour)",
        &[(dec!(1), Some(Unit::Time(TimeUnit::Hour)))],
    );
}

#[test]
fn multi_units() {
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

    test(
        "5m + 5m",
        &[
            (dec!(10), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(10), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );

    test(
        "500m + 5km",
        &[(
            dec!(5.5),
            Some(Unit::Length(LengthUnit::Meter(Some(UnitPrefix::Kilo)))),
        )],
    );

    test(
        "(33 + 3) m + 15",
        &[
            (dec!(51), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(51), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );

    test(
        "(33m + 3m) m + 15m",
        &[
            (dec!(51), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(51), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );

    test(
        "(33m + 3m) m + 3 + 15m + 1",
        &[
            (dec!(55), Some(Unit::Time(TimeUnit::Minute))),
            (dec!(55), Some(Unit::Length(LengthUnit::Meter(None)))),
        ],
    );
}
