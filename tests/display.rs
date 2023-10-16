use pepino::Computer;

#[test]
fn display() {
    let tests: Vec<(&str, &str)> = vec![
        ("-5 - -1", "-5+1"),
        ("-5 * -1", "-(5*(-1))"),
        ("((((3 - (4 - (3 - (2 - (-1))))))))", "3-(4-(3-(2-(-1))))"),
        ("(((3))) + 1", "3+1"),
        ("(((3)))", "3"),
        ("3 + 5 * 4", "3+(5*4)"),
        ("5 * 4", "5*4"),
        ("5 * 4 * 3 * 2 * 1", "(((5*4)*3)*2)*1"),
        ("3 + 5  *  3 + 3 * 5", "3+(5*3)+(3*5)"),
        ("9 / 3 / 3", "(9/3)/3"),
        ("9 + 3 / 3 - 3 - 3 * 1 / -3", "9+(3/3)-3-((3*1)/(-3))"),
        ("3 * 5 ^ 2", "3*(5^2)"),
        ("27 + 3 / 3 + 2 ^ 2 + 3 * 5 + 1", "27+(3/3)+(2^2)+(3*5)+1"),
    ];

    for test in tests {
        let mut computer = Computer::default();
        computer.compute(test.0);
        let statement = computer.last_statement().unwrap();
        let expression = statement.expression.as_ref().unwrap().clone();
        let actual = expression.to_string();
        if actual != test.1 {
            panic!("{} != {}", test.1, actual)
        }
    }
}
