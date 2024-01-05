use pepino::{Calc, TemperatureUnit, TimeUnit, Unit};

fn test(tests: Vec<(&str, f64)>, unit: Option<Unit>) {
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
                let v = (n.value * 100.0).round() / 100.0;
                if v != test.1 || n.unit != unit {
                    panic!("{:?}: {:?} != {:?} {:?}", test.0, v, test.1, unit);
                }
            }
        }
    }
}

#[test]
fn none() {
    let tests = vec![("55 + 55", 110.0)];

    test(tests, None);
}

#[test]
fn temperature_celsius() {
    let tests = vec![
        // ("(33 + 3) celsius + 15", 51.0),
        // ("celsius((33 + 3) kelvins)", -237.15),
        ("celsius(33 + 3 kelvins)", -237.15),
        // ("celsius(55)", 55.0),
        // ("pow(55 celsius, 2)", 3025.0),
        // ("55 celsius", 55.0),
        // ("celsius 55", 55.0),
        // ("55°C", 55.0),
        // ("°c55", 55.0),
        // ("degrees celsius 55", 55.0),
        // ("55 degrees celsius", 55.0),
        // ("10 * 5 celsius", 50.0),
        // ("10 * celsius(5 + 5)", 100.0),
        // ("15 fahrenheits + 3 celsius", -6.44),
        // ("15 fahrenheits + 3 kelvins + 2 * 20 celsius", 33.56),
    ];

    test(tests, Some(Unit::Temperature(TemperatureUnit::Celsius)));
}

#[test]
fn temperature_kelvin() {
    let tests = vec![("kelvin(pow(55 celsius, 2))", 3298.15)];

    test(tests, Some(Unit::Temperature(TemperatureUnit::Kelvin)));
}

#[test]
fn temperature_fahrenheit() {
    let tests = vec![
        ("10 celsius + pow(5,2) Fahrenheit", 75.0),
        ("10 celsius + (pow(5,2) Fahrenheit)", 75.0),
        ("10 celsius + Fahrenheit(pow(5,2)) ", 75.0),
    ];

    test(tests, Some(Unit::Temperature(TemperatureUnit::Fahrenheit)));
}

#[test]
fn in_operator() {
    let tests = vec![
        ("pow(5,2) Celsius to Fahrenheit", 77.0),
        ("10 kelvins to celsius to kelvins to Fahrenheit", -441.67),
    ];

    test(tests, Some(Unit::Temperature(TemperatureUnit::Fahrenheit)));
}

#[test]
fn list_fce() {
    let tests = vec![
        ("min(55 Celsius, 180 Kelvin) into celsius", -93.15),
        (
            "sum(5 Celsius, 180 Kelvin, 10 Fahrenheit, 10 Celsius)",
            -90.37,
        ),
    ];

    test(tests, Some(Unit::Temperature(TemperatureUnit::Celsius)));
}

#[test]
fn time() {
    let tests = vec![(
        "(5 days + 1 hour + (60 * 30) seconds + 15 min) in hours",
        121.75,
    )];

    test(tests, Some(Unit::Time(TimeUnit::Hour)));
}

#[test]
fn pow() {
    let tests = vec![("pow(5 hours, 2 hours)", 25.0)];

    test(tests, Some(Unit::Time(TimeUnit::Hour)));
}
