use std::collections::HashSet;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::Calculator;

fn test_computation(tests: &[(&str, Decimal)]) {
    for test in tests {
        let mut computer = Calculator::default();
        let statement = computer.compute(test.0).unwrap();

        match &statement.result {
            None => panic!("No result: '{:?}'", test.0),
            Some(Err(e)) => panic!("Error in '{:?}': {:?}", test.0, e),
            Some(Ok(n)) => {
                // round v
                let v = (n.values()[0].0 * dec!(10.0)).round() / dec!(10.0);
                if v != test.1 {
                    panic!("{:?}: {:?} != {:?}", test.0, v, test.1);
                }
            }
        }
    }
}

fn test_errors(tests: &[&str]) {
    for test in tests {
        let mut computer = Calculator::default();
        let statement = computer.compute(test).unwrap();
        if let Some(Ok(result)) = &statement.result {
            panic!("{:?} = {:?} expected to yield no result.", test, result)
        }
    }
}

#[test]
fn test_plus_minus() {
    test_computation(&[
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
    ]);
}

#[test]
fn test_plus_minus_errors() {
    test_errors(&["-", "+"]);
}

#[test]
fn test_parentheses() {
    test_computation(&[
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
    ]);
}

#[test]
fn test_parentheses_errors() {
    test_errors(&[") 2 + 1", "( 2 + 1))", "( 2 + 1)(", "()", "("]);
}

#[test]
fn test_multiplication() {
    test_computation(&[
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
    ]);
}

#[test]
fn test_multiplication_errors() {
    test_errors(&["*", "2 ** 3", "*3", "3-*1"]);
}

#[test]
fn test_division() {
    test_computation(&[
        ("3 / 3", dec!(1.0)),
        ("9 / 3 / 3", dec!(1.0)),
        ("9 / 3 / 3 * 5 / 5 * 3", dec!(3.0)),
        ("9 + 3 / 3 - 3 - 3 * 1 / -3", dec!(8.0)),
        ("9 / 3 / 3", dec!(1.0)),
        ("9 / 3 / 3", dec!(1.0)),
        ("3 * 5 ^ 2.1", dec!(88.1)),
    ]);
}

#[test]
fn test_pow() {
    test_computation(&[
        ("3 ^ 3", dec!(27.0)),
        ("5 ^ 2 * 3", dec!(75.0)),
        ("(3 * 5) ^ 2", dec!(225.0)),
        ("pow(3 * 5, 2)", dec!(225.0)),
        ("3 * 5 ^ 2", dec!(75.0)),
        ("3 * 5 ^ 2 / -3", dec!(-25.0)),
        ("27 / 3 ^ 2", dec!(3.0)),
        ("27 / 3 ^ 2 * 5", dec!(15.0)),
        ("27 + 3 / 3 + 2 ^ 2 + 3 * 5 + 1", dec!(48.0)),
    ]);
}

#[test]
fn test_sqrt() {
    test_computation(&[
        ("sqrt(25)", dec!(5.0)),
        ("sqr(5)", dec!(25.0)),
        ("sqrt(25)+sqr(5)", dec!(30.0)),
        ("sqrt(25 * 25)+sqr(5 + 5)", dec!(125.0)),
        ("sqrt(25)*5+sqr(5)/5", dec!(30.0)),
        ("sqrt 25", dec!(5.0)),
        ("sqrt 25 * 5 + sqr 5 / 5", dec!(30.0)),
        ("sqrt 25 * sqrt 25 + 10", dec!(35.0)),
    ]);
}

#[test]
fn test_sqrt_errors() {
    test_errors(&["sqrt", "5sqrt"]);
}

#[test]
fn test_round() {
    test_computation(&[
        ("round(1.6)", dec!(2.0)),
        ("round(sqrt(20))", dec!(4.0)),
        ("sqrt(round(sqrt(20)))", dec!(2.0)),
        ("round 1.4", dec!(1.0)),
        ("round 1.4 / 2", dec!(0.5)),
        ("sqrt round 4.4 / 2", dec!(1.0)),
    ]);
}

#[test]
fn test_trunc_fract_floor_ceil() {
    test_computation(&[
        ("trunc(1.9)", dec!(1.0)),
        ("trunc 0.9", dec!(0.0)),
        ("trunc 1.9", dec!(1.0)),
        ("trunc +1.9", dec!(1.0)),
        ("trunc(+1.9)", dec!(1.0)),
        ("fract(1.9)", dec!(0.9)),
        ("floor(3.7)", dec!(3.0)),
        ("ceil -1.9", dec!(-1.0)),
    ]);
}

#[test]
fn test_sinus() {
    test_computation(&[
        ("sin(0)", dec!(0.0)),
        ("sin(PI/2)", dec!(1.0)),
        ("sin(3.141592684/2)", dec!(1.0)),
        ("sin(3.141592684/2)", dec!(1.0)),
        ("sin 0", dec!(0.0)),
    ]);
}

#[test]
fn test_ln() {
    test_computation(&[("ln(E)", dec!(1.0))]);
}

#[test]
fn test_random() {
    let mut numbers = HashSet::new();
    for _ in 0..10 {
        let mut computer = Calculator::default();
        let statement = computer.compute("trunc(random() * 100000)").unwrap();

        if let Some(Ok(n)) = &statement.result {
            numbers.insert(n.values()[0].0);
        }
    }

    assert_eq!(numbers.len(), 10);
}

#[test]
fn test_random_errors() {
    test_errors(&["random(5)"]);
}

#[test]
fn test_min() {
    test_computation(&[
        ("min(pow(5,2))", dec!(25.0)),
        ("min(200,pow(5,2))", dec!(25.0)),
        ("min(200,min(1,min(-2,5)))", dec!(-2.0)),
        ("min(2,5)", dec!(2.0)),
        ("min(2)", dec!(2.0)),
        ("min2", dec!(2.0)),
        ("min(2,5,1,)", dec!(1.0)),
        ("min(5,(2*10))", dec!(5.0)),
        ("min(PI/2, PI^2)", dec!(1.6)),
    ]);
}

#[test]
fn test_min_errors() {
    test_errors(&["min2,5", "min * 5", "min"]);
}

#[test]
fn test_max() {
    test_computation(&[
        ("max(2,5)", dec!(5.0)),
        ("max(2)", dec!(2.0)),
        ("max2", dec!(2.0)),
        ("max(2,5,1,)", dec!(5.0)),
        ("max(5,(2*10))", dec!(20.0)),
        ("max(PI/2, PI^2 * 5 ^ 2)", dec!(246.7)),
    ]);
}

#[test]
fn test_log() {
    test_computation(&[("log(5 ^ 2)", dec!(1.4))]);
}

#[test]
fn test_sum() {
    test_computation(&[("sum(5,10,15)", dec!(30.0))]);
}

#[test]
fn test_average() {
    test_computation(&[("average(10, 2, 38, 23, 38, 23, 21)", dec!(22.1))]);
}

#[test]
fn test_median() {
    test_computation(&[("median(10, 2, 38, 23, 24, 38, 29, 21)", dec!(23.5))]);
}

#[test]
fn test_count() {
    test_computation(&[("count(10, 2, 38, 23, 24, 38, 29, 21)", dec!(8.0))]);
}
