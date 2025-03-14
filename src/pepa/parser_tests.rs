use crate::Calculator;

#[test]
fn test_explain() {
    let tests = [
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
        ("55 celsius", "55°C"),
        ("5 * 55 celsius + 3 Kelvins", "(5*55°C)+3K"),
        ("5 * celsius 55 + 3 Kelvins", "(5*55°C)+3K"),
        ("celsius(55)", "55→°C"),
        ("15 fahrenheits + 3 celsius", "15°F+3°C"),
        (
            "15 fahrenheits + 3 kelvins + 2 * 20 celsius",
            "15°F+3K+(2*20°C)",
        ),
        ("10 celsius + pow(5,2) Fahrenheit", "10°C+((pow(5,2))→°F)"),
        ("(33 + 3) celsius + 15", "((33+3)→°C)+15"),
        ("(10 kelvins) celsius", "10K→°C"),
        (
            "10 kelvins to celsius to kelvins to Fahrenheit",
            "10→K→°C→K→°F",
        ),
        ("pow(5,2) Fahrenheit to Celsius", "(pow(5,2))→°F→°C"),
        ("min(55 Celsius, 180 Kelvin) celsius", "(min(55°C,180K))→°C"),
        (
            "min(55 Celsius, 180 Kelvin) into celsius",
            "(min(55°C,180K))→°C",
        ),
        ("5d + 5m + 5h + 5s", "5d+5m+5h+5s"),
        ("5m + 1km", "5m+1km"),
        (
            "(5 days + 1 hour + (60 * 30) seconds + 15 min) in hours",
            "(5d+1h+((60*30)→s)+15m)→h",
        ),
        ("celsius((33 + 3) kelvins)", "((33+3)→K)→°C"),
        ("10 * celsius(5 + 5)", "10*((5+5)→°C)"),
        ("(5 Mm + 1000mm) to metres", "(5Mm+1000mm)→m"),
        ("(1 Mt) in kg", "1Mt→kg"),
        ("(1 degree) in gradians", "1°→gon"),
    ];

    for test in tests {
        let mut computer = Calculator::default();
        let statement = computer.prepare_statements(test.0).unwrap();
        let expression = statement.expression.as_ref().unwrap().clone();
        let actual = expression.explain();
        if actual != test.1 {
            panic!("{} != {}", test.1, actual)
        }
    }
}
