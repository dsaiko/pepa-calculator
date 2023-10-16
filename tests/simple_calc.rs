use pepino::ComputedResult;
use pepino::Computer;

fn test_operations(tests: Vec<(&str, f64)>) {
    for test in tests {
        let mut computer = Computer::default();
        computer.compute(test.0);
        let statement = computer.last_statement().unwrap();

        match statement.result {
            None => {
                panic!("{:?}: No result", test.0);
            }
            Some(ComputedResult::Numeric(v, _)) => {
                // round v
                let v = (v * 10.0).round() / 10.0;
                if v != test.1 {
                    panic!("{:?}: {:?} != {:?}", test.0, v, test.1);
                }
            }
            _ => panic!("{:?}: invalid result type!", test.0),
        }
    }
}

fn test_errors(tests: Vec<&str>) {
    for test in tests {
        let mut computer = Computer::default();
        computer.compute(test);
        let statement = computer.last_statement().unwrap();
        if let Some(result) = &statement.result {
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
        ("3 * 5 ^ 2", 75.0),
        ("3 * 5 ^ 2 / -3", -25.0),
        ("27 / 3 ^ 2", 3.0),
        ("27 / 3 ^ 2 * 5", 15.0),
        ("27 + 3 / 3 + 2 ^ 2 + 3 * 5 + 1", 48.0),
    ];

    test_operations(tests);
}
