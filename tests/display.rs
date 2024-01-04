use pepino::Calc;

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
        ("sqrt(25)+sqr(5)", "(sqrt25)+(sqr5)"),
        ("sqrt25+sqr5", "(sqrt25)+(sqr5)"),
        ("sqrt round 4.4 / 2", "(sqrt(round4.4))/2"),
        ("ceil -1.9", "ceil(-1.9)"),
        ("random() * 10000 * 5", "(random()*10000)*5"),
        ("min(2,5)", "min(2,5)"),
        ("min(2,5 * 2 * 3)", "min(2,(5*2)*3)"),
    ];

    for test in tests {
        let mut computer = Calc::default();
        let statement = computer.compute(test.0).unwrap();
        let expression = statement.expression.as_ref().unwrap().clone();
        let actual = expression.to_string();
        if actual != test.1 {
            panic!("{} != {}", test.1, actual)
        }
    }
}
