use crate::pluralize;
use crate::utils::Pluralize;
use crate::utils::{flatten_lines, split_string_by_comma};

#[test]
fn test_split() {
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
fn test_pluralize() {
    assert_eq!(pluralize!("hour", 1), "hour");
    assert_eq!(pluralize!("hour", 0), "hours");
    assert_eq!(pluralize!("hour", -1), "hour");
    assert_eq!(pluralize!("foot", "feet", 1), "foot");
    assert_eq!(pluralize!("foot", "feet", 0), "feet");
}

#[test]
fn test_flatten_lines() {
    assert_eq!(
        flatten_lines(&vec![vec!["a"], vec!["b1", "b2"]]),
        vec![vec!["a", "b1"], vec!["a", "b2"]]
    );
    assert_eq!(
        flatten_lines(&vec![vec!["a1", "a2"], vec!["b"]]),
        vec![vec!["a1", "b"], vec!["a2", "b"]]
    );
    assert_eq!(
        flatten_lines(&vec![vec!["a1", "a2"], vec!["b1", "b2"], vec!["c"]]),
        vec![
            vec!["a1", "b1", "c"],
            vec!["a1", "b2", "c"],
            vec!["a2", "b1", "c"],
            vec!["a2", "b2", "c"],
        ]
    );
}
