use std::collections::HashSet;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use pepino::Calc;

fn test(tests: Vec<(&str, Decimal)>) {
    for test in tests {
        let mut computer = Calc::default();
        let statement = computer.compute(test.0).unwrap();

        match statement.result {
            Err(e) => {
                panic!("Error in '{:?}': {:?}", test.0, e);
            }
            Ok(n) => {
                // round v
                let v = (n.value * dec!(10.0)).round() / dec!(10.0);
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
    let tests = vec![
        ("55", dec!(55.0)),
        ("+55", dec!(55.0)),
        ("55 +44", dec!(99.0)),
        ("+55 +44 + 1", dec!(100.0)),
        ("+55 -1", dec!(54.0)),
        ("-1 -2", dec!(-3.0)),
        ("-1", dec!(-1.0)),
        ("55 + -1", dec!(54.0)),
        ("55 + -1 - -1", dec!(55.0)),
        ("-5 --1", dec!(-4.0)),
        ("55 +-+-+-+---1", dec!(56.0)),
        ("55 111 55", dec!(5511155.0)),
    ];

    test(tests);
}

#[test]
fn calc_plus_minus_errors() {
    let tests: Vec<&str> = vec!["-", "+"];
    test_errors(tests);
}

#[test]
fn parentheses() {
    let tests = vec![
        ("() + 2", dec!(2.0)),
        ("2 + ()", dec!(2.0)),
        ("2 + () - 2", dec!(0.0)),
        ("5 - ( 3 + 1) + 1", dec!(2.0)),
        ("-5 - ( 3 + 1) + 1", dec!(-8.0)),
        ("2 + (-2)", dec!(0.0)),
        ("2 + (2 - (1 + 2) )", dec!(1.0)),
        ("((((3 - (4 - (3 - (2 - (-1))))))))", dec!(-1.0)),
        ("(2+2)-(2-1)-(4)", dec!(-1.0)),
        ("((((3 - (4 - (3 - (2 - (-1)))))))) - (5 + 1)", dec!(-7.0)),
    ];

    test(tests);
}

#[test]
fn parentheses_errors() {
    let tests: Vec<&str> = vec![") 2 + 1", "( 2 + 1))", "( 2 + 1)(", "()", "("];
    test_errors(tests);
}

#[test]
fn multiplication() {
    let tests = vec![
        ("3 * 3", dec!(9.0)),
        ("3 * 3 * 3", dec!(27.0)),
        ("3 * (-(1))", dec!(-3.0)),
        ("3 * -1", dec!(-3.0)),
        ("3 * -(1+2)", dec!(-9.0)),
        ("3 * -( 3 * -2)", dec!(18.0)),
        ("3 * +1", dec!(3.0)),
        ("3 * 3 * 3 * -1", dec!(-27.0)),
        ("3 * 3 + 3", dec!(12.0)),
        ("3 * (3 + 3)", dec!(18.0)),
        ("(3 + 5) * 4", dec!(32.0)),
        ("3 + (5 * 4)", dec!(23.0)),
        ("3 + 5 * 4", dec!(23.0)),
        ("3 + 5 * 5 * 5 * 5", dec!(628.0)),
        ("3 + 5  *  3 + 3 * 5", dec!(33.0)),
        ("6 + 3 * 4 + 8", dec!(26.0)),
    ];

    test(tests);
}

#[test]
fn multiplication_errors() {
    let tests: Vec<&str> = vec!["*", "2 ** 3", "*3", "3-*1"];
    test_errors(tests);
}

#[test]
fn division() {
    let tests = vec![
        ("3 / 3", dec!(1.0)),
        ("9 / 3 / 3", dec!(1.0)),
        ("9 / 3 / 3 * 5 / 5 * 3", dec!(3.0)),
        ("9 + 3 / 3 - 3 - 3 * 1 / -3", dec!(8.0)),
        ("9 / 3 / 3", dec!(1.0)),
        ("9 / 3 / 3", dec!(1.0)),
        ("3 * 5 ^ 2.1", dec!(88.1)),
    ];

    test(tests);
}

#[test]
fn pow() {
    let tests = vec![
        ("3 ^ 3", dec!(27.0)),
        ("5 ^ 2 * 3", dec!(75.0)),
        ("(3 * 5) ^ 2", dec!(225.0)),
        ("pow(3 * 5, 2)", dec!(225.0)),
        ("3 * 5 ^ 2", dec!(75.0)),
        ("3 * 5 ^ 2 / -3", dec!(-25.0)),
        ("27 / 3 ^ 2", dec!(3.0)),
        ("27 / 3 ^ 2 * 5", dec!(15.0)),
        ("27 + 3 / 3 + 2 ^ 2 + 3 * 5 + 1", dec!(48.0)),
    ];

    test(tests);
}

#[test]
fn sqrt() {
    let tests = vec![
        ("sqrt(25)", dec!(5.0)),
        ("sqr(5)", dec!(25.0)),
        ("sqrt(25)+sqr(5)", dec!(30.0)),
        ("sqrt(25 * 25)+sqr(5 + 5)", dec!(125.0)),
        ("sqrt(25)*5+sqr(5)/5", dec!(30.0)),
        ("sqrt 25", dec!(5.0)),
        ("sqrt 25 * 5 + sqr 5 / 5", dec!(30.0)),
        ("sqrt 25 * sqrt 25 + 10", dec!(35.0)),
    ];

    test(tests);
}

#[test]
fn sqrt_errors() {
    let tests: Vec<&str> = vec!["sqrt", "5sqrt"];
    test_errors(tests);
}

#[test]
fn round() {
    let tests = vec![
        ("round(1.6)", dec!(2.0)),
        ("round(sqrt(20))", dec!(4.0)),
        ("sqrt(round(sqrt(20)))", dec!(2.0)),
        ("round 1.4", dec!(1.0)),
        ("round 1.4 / 2", dec!(0.5)),
        ("sqrt round 4.4 / 2", dec!(1.0)),
    ];

    test(tests);
}

#[test]
fn trunc_fract_floor_ceil() {
    let tests = vec![
        ("trunc(1.9)", dec!(1.0)),
        ("trunc 0.9", dec!(0.0)),
        ("trunc 1.9", dec!(1.0)),
        ("trunc +1.9", dec!(1.0)),
        ("trunc(+1.9)", dec!(1.0)),
        ("fract(1.9)", dec!(0.9)),
        ("floor(3.7)", dec!(3.0)),
        ("ceil -1.9", dec!(-1.0)),
    ];

    test(tests);
}

#[test]
fn sinus() {
    let tests = vec![
        ("sin(0)", dec!(0.0)),
        ("sin(PI/2)", dec!(1.0)),
        ("sin(3.141592684/2)", dec!(1.0)),
        ("sin(3.141592684/2)", dec!(1.0)),
        ("sin 0", dec!(0.0)),
    ];

    test(tests);
}

#[test]
fn ln() {
    let tests = vec![("ln(E)", dec!(1.0))];

    test(tests);
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
                numbers.insert(n.value);
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
    let tests = vec![
        ("min(pow(5,2))", dec!(25.0)),
        ("min(200,pow(5,2))", dec!(25.0)),
        ("min(200,min(1,min(-2,5)))", dec!(-2.0)),
        ("min(2,5)", dec!(2.0)),
        ("min(2)", dec!(2.0)),
        ("min2", dec!(2.0)),
        ("min(2,5,1,)", dec!(1.0)),
        ("min(5,(2*10))", dec!(5.0)),
        ("min(PI/2, PI^2)", dec!(1.6)),
    ];

    test(tests);
}

#[test]
fn min_errors() {
    let tests: Vec<&str> = vec!["min2,5", "min * 5", "min"];
    test_errors(tests);
}

#[test]
fn max() {
    let tests = vec![
        ("max(2,5)", dec!(5.0)),
        ("max(2)", dec!(2.0)),
        ("max2", dec!(2.0)),
        ("max(2,5,1,)", dec!(5.0)),
        ("max(5,(2*10))", dec!(20.0)),
        ("max(PI/2, PI^2 * 5 ^ 2)", dec!(246.7)),
    ];

    test(tests);
}

#[test]
fn log() {
    let tests = vec![("log(5 ^ 2)", dec!(1.4))];

    test(tests);
}

#[test]
fn sum() {
    let tests = vec![("sum(5,10,15)", dec!(30.0))];

    test(tests);
}

#[test]
fn average() {
    let tests = vec![("average(10, 2, 38, 23, 38, 23, 21)", dec!(22.1))];

    test(tests);
}

#[test]
fn median() {
    let tests = vec![("median(10, 2, 38, 23, 24, 38, 29, 21)", dec!(23.5))];

    test(tests);
}

#[test]
fn count() {
    let tests = vec![("count(10, 2, 38, 23, 24, 38, 29, 21)", dec!(8.0))];

    test(tests);
}
