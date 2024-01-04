use std::collections::HashSet;

use pepino::Calc;

fn test_operations(tests: Vec<(&str, f64)>) {
    for test in tests {
        let mut computer = Calc::default();
        let statement = computer.compute(test.0).unwrap();

        match statement.result {
            Err(e) => {
                panic!("Error in '{:?}': {:?}", test.0, e);
            }
            Ok(n) => {
                // round v
                let v = (n.value * 10.0).round() / 10.0;
                if v != test.1 {
                    panic!("{:?}: {:?} != {:?}", test.0, v, test.1);
                }
            }
        }
    }
}

fn test_errors(tests: Vec<&str>) {
    for test in tests {
        let mut computer = Calc::default();
        let statement = computer.compute(test).unwrap();
        if let Ok(result) = &statement.result {
            panic!("{:?} = {:?} expected to yield no result.", test, result)
        }
    }
}

#[test]
fn calc_plus_minus() {
    let tests: Vec<(&str, f64)> = vec![
        ("55", 55.0),
        ("+55", 55.0),
        ("55 +44", 99.0),
        ("+55 +44 + 1", 100.0),
        ("+55 -1", 54.0),
        ("-1 -2", -3.0),
        ("-1", -1.0),
        ("55 + -1", 54.0),
        ("55 + -1 - -1", 55.0),
        ("-5 --1", -4.0),
        ("55 +-+-+-+---1", 56.0),
        ("55 111 55", 5511155.0),
    ];

    test_operations(tests);
}

#[test]
fn calc_plus_minus_errors() {
    let tests: Vec<&str> = vec!["-", "+"];
    test_errors(tests);
}

#[test]
fn parentheses() {
    let tests: Vec<(&str, f64)> = vec![
        ("() + 2", 2.0),
        ("2 + ()", 2.0),
        ("2 + () - 2", 0.0),
        ("5 - ( 3 + 1) + 1", 2.0),
        ("-5 - ( 3 + 1) + 1", -8.0),
        ("2 + (-2)", 0.0),
        ("2 + (2 - (1 + 2) )", 1.0),
        ("((((3 - (4 - (3 - (2 - (-1))))))))", -1.0),
        ("(2+2)-(2-1)-(4)", -1.0),
        ("((((3 - (4 - (3 - (2 - (-1)))))))) - (5 + 1)", -7.0),
    ];

    test_operations(tests);
}

#[test]
fn parentheses_errors() {
    let tests: Vec<&str> = vec![") 2 + 1", "( 2 + 1))", "( 2 + 1)(", "()", "("];
    test_errors(tests);
}

#[test]
fn multiplication() {
    let tests: Vec<(&str, f64)> = vec![
        ("3 * 3", 9.0),
        ("3 * 3 * 3", 27.0),
        ("3 * (-(1))", -3.0),
        ("3 * -1", -3.0),
        ("3 * -(1+2)", -9.0),
        ("3 * -( 3 * -2)", 18.0),
        ("3 * +1", 3.0),
        ("3 * 3 * 3 * -1", -27.0),
        ("3 * 3 + 3", 12.0),
        ("3 * (3 + 3)", 18.0),
        ("(3 + 5) * 4", 32.0),
        ("3 + (5 * 4)", 23.0),
        ("3 + 5 * 4", 23.0),
        ("3 + 5 * 5 * 5 * 5", 628.0),
        ("3 + 5  *  3 + 3 * 5", 33.0),
        ("6 + 3 * 4 + 8", 26.0),
    ];

    test_operations(tests);
}

#[test]
fn multiplication_errors() {
    let tests: Vec<&str> = vec!["*", "2 ** 3", "*3", "3-*1"];
    test_errors(tests);
}

#[test]
fn division() {
    let tests: Vec<(&str, f64)> = vec![
        ("3 / 3", 1.0),
        ("9 / 3 / 3", 1.0),
        ("9 / 3 / 3 * 5 / 5 * 3", 3.0),
        ("9 + 3 / 3 - 3 - 3 * 1 / -3", 8.0),
        ("9 / 3 / 3", 1.0),
        ("9 / 3 / 3", 1.0),
        ("3 * 5 ^ 2.1", 88.1),
    ];

    test_operations(tests);
}

#[test]
fn pow() {
    let tests: Vec<(&str, f64)> = vec![
        ("3 ^ 3", 27.0),
        ("5 ^ 2 * 3", 75.0),
        ("(3 * 5) ^ 2", 225.0),
        ("pow(3 * 5, 2)", 225.0),
        ("3 * 5 ^ 2", 75.0),
        ("3 * 5 ^ 2 / -3", -25.0),
        ("27 / 3 ^ 2", 3.0),
        ("27 / 3 ^ 2 * 5", 15.0),
        ("27 + 3 / 3 + 2 ^ 2 + 3 * 5 + 1", 48.0),
    ];

    test_operations(tests);
}

#[test]
fn sqrt() {
    let tests: Vec<(&str, f64)> = vec![
        ("sqrt(25)", 5.0),
        ("sqr(5)", 25.0),
        ("sqrt(25)+sqr(5)", 30.0),
        ("sqrt(25 * 25)+sqr(5 + 5)", 125.0),
        ("sqrt(25)*5+sqr(5)/5", 30.0),
        ("sqrt 25", 5.0),
        ("sqrt 25 * 5 + sqr 5 / 5", 30.0),
        ("sqrt 25 * sqrt 25 + 10", 35.0),
    ];

    test_operations(tests);
}

#[test]
fn sqrt_errors() {
    let tests: Vec<&str> = vec!["sqrt", "5sqrt"];
    test_errors(tests);
}

#[test]
fn round() {
    let tests: Vec<(&str, f64)> = vec![
        ("round(1.6)", 2.0),
        ("round(sqrt(20))", 4.0),
        ("sqrt(round(sqrt(20)))", 2.0),
        ("round 1.4", 1.0),
        ("round 1.4 / 2", 0.5),
        ("sqrt round 4.4 / 2", 1.0),
    ];

    test_operations(tests);
}

#[test]
fn trunc_fract_floor_ceil() {
    let tests: Vec<(&str, f64)> = vec![
        ("trunc(1.9)", 1.0),
        ("trunc 0.9", 0.0),
        ("trunc 1.9", 1.0),
        ("trunc +1.9", 1.0),
        ("trunc(+1.9)", 1.0),
        ("fract(1.9)", 0.9),
        ("floor(3.7)", 3.0),
        ("ceil -1.9", -1.0),
    ];

    test_operations(tests);
}

#[test]
fn sinus() {
    let tests: Vec<(&str, f64)> = vec![
        ("sin(0)", 0.0),
        ("sin(PI/2)", 1.0),
        ("sin(3.141592684/2)", 1.0),
        ("sin(3.141592684/2)", 1.0),
        ("sin 0", 0.0),
    ];

    test_operations(tests);
}

#[test]
fn ln() {
    let tests: Vec<(&str, f64)> = vec![("ln(E)", 1.0)];

    test_operations(tests);
}

#[test]
fn random() {
    let mut numbers = HashSet::new();
    for _ in 0..10 {
        let mut computer = Calc::default();
        let statement = computer.compute("trunc(random() * 100000)").unwrap();

        match statement.result {
            Err(e) => {
                panic!("Error: {:?}", e);
            }
            Ok(n) => {
                numbers.insert(n.value as u32);
            }
        }
    }

    assert_eq!(numbers.len(), 10);
}

#[test]
fn random_errors() {
    let tests: Vec<&str> = vec!["random(5)"];
    test_errors(tests);
}

#[test]
fn min() {
    let tests: Vec<(&str, f64)> = vec![
        ("min(2,5)", 2.0),
        ("min(2)", 2.0),
        ("min2", 2.0),
        ("min(2,5,1,)", 1.0),
        ("min(5,(2*10))", 5.0),
        ("min(PI/2, PI^2)", 1.6),
    ];

    test_operations(tests);
}

#[test]
fn min_errors() {
    let tests: Vec<&str> = vec!["min2,5", "min * 5", "min"];
    test_errors(tests);
}

#[test]
fn max() {
    let tests: Vec<(&str, f64)> = vec![
        ("max(2,5)", 5.0),
        ("max(2)", 2.0),
        ("max2", 2.0),
        ("max(2,5,1,)", 5.0),
        ("max(5,(2*10))", 20.0),
        ("max(PI/2, PI^2 * 5 ^ 2)", 246.7),
    ];

    test_operations(tests);
}

#[test]
fn log() {
    let tests: Vec<(&str, f64)> = vec![("log(5 ^ 2, 10)", 1.4)];

    test_operations(tests);
}

#[test]
fn sum() {
    let tests: Vec<(&str, f64)> = vec![("sum(5,10,15)", 30.0)];

    test_operations(tests);
}

#[test]
fn average() {
    let tests: Vec<(&str, f64)> = vec![("average(10, 2, 38, 23, 38, 23, 21)", 22.1)];

    test_operations(tests);
}

#[test]
fn median() {
    let tests: Vec<(&str, f64)> = vec![("median(10, 2, 38, 23, 24, 38, 29, 21)", 23.5)];

    test_operations(tests);
}

#[test]
fn count() {
    let tests: Vec<(&str, f64)> = vec![("count(10, 2, 38, 23, 24, 38, 29, 21)", 8.0)];

    test_operations(tests);
}
