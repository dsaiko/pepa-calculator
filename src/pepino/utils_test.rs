use crate::pluralize;
use crate::utils::Pluralize;
use crate::utils::split_string_by_comma;

#[test]
fn split() {
    assert_eq!(split_string_by_comma("test"), vec!["test"]);
    assert_eq!(
        split_string_by_comma("test1, test2"),
        vec!["test1", "test2"]
    );
    assert_eq!(split_string_by_comma("test,,,"), vec!["test"]);
    assert_eq!(
        split_string_by_comma("(test),(test)"),
        vec!["(test)", "(test)"]
    );

    assert_eq!(
        split_string_by_comma("(test),pow(2, pow(1,1))"),
        vec!["(test)", "pow(2, pow(1,1))"]
    );

    assert!(split_string_by_comma(",,,").is_empty());
    assert!(split_string_by_comma(" ").is_empty());
}

#[test]
fn pluralize() {
    assert_eq!(pluralize!("hour", 1), "hour");
    assert_eq!(pluralize!("hour", 0), "hours");
    assert_eq!(pluralize!("hour", -1), "hour");
    assert_eq!(pluralize!("foot", "feet", 1), "foot");
    assert_eq!(pluralize!("foot", "feet", 0), "feet");
}
