mod rail_runner;

use rail_runner::{rail, rail_oneliner};

pub const RAIL_PATH: &str = std::env!("CARGO_BIN_EXE_rail");

#[test]
fn test_true() {
    let res = rail(&["true", "print"]);
    assert_eq!("", &res.stderr);
    assert_eq!("true", &res.stdout);
    assert!(res.status.success());
}

#[test]
fn test_false() {
    let res = rail(&["false", "print"]);
    assert_eq!("", &res.stderr);
    assert_eq!("false", &res.stdout);
    assert!(res.status.success());
}

#[test]
fn test_not() {
    assert_eq!("true", &rail_oneliner("false not print").stdout);
    assert_eq!("false", &rail_oneliner("true not print").stdout);
}

#[test]
fn test_bool_equality() {
    assert_eq!("true", &rail_oneliner("true true == print").stdout);
    assert_eq!("true", &rail_oneliner("false false == print").stdout);
    assert_eq!("true", &rail_oneliner("true false != print").stdout);
    assert_eq!("true", &rail_oneliner("false true != print").stdout);

    assert_eq!("false", &rail_oneliner("true true != print").stdout);
    assert_eq!("false", &rail_oneliner("false false != print").stdout);
    assert_eq!("false", &rail_oneliner("true false == print").stdout);
    assert_eq!("false", &rail_oneliner("false true == print").stdout);
}

#[test]
fn test_numeric_equality() {
    assert_eq!("true", &rail_oneliner("1 1 == print").stdout);
    assert_eq!("true", &rail_oneliner("1 1.0 == print").stdout);
    assert_eq!("true", &rail_oneliner("1.0 1 == print").stdout);
    assert_eq!("true", &rail_oneliner("1.0 1.0 == print").stdout);

    assert_eq!("false", &rail_oneliner("1 2 == print").stdout);
    assert_eq!("false", &rail_oneliner("1 2.0 == print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 2 == print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 2.0 == print").stdout);

    assert_eq!("false", &rail_oneliner("1 1 != print").stdout);
    assert_eq!("false", &rail_oneliner("1 1.0 != print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 1 != print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 1.0 != print").stdout);

    assert_eq!("true", &rail_oneliner("1 2 != print").stdout);
    assert_eq!("true", &rail_oneliner("1 2.0 != print").stdout);
    assert_eq!("true", &rail_oneliner("1.0 2 != print").stdout);
    assert_eq!("true", &rail_oneliner("1.0 2.0 != print").stdout);
}

#[test]
fn test_string_equality() {
    assert_eq!("true", &rail_oneliner(r#""apple" "apple" == print"#).stdout);
    assert_eq!(
        "true",
        &rail_oneliner(r#""apple" "orange" != print"#).stdout
    );

    assert_eq!(
        "false",
        &rail_oneliner(r#""apple" "apple" != print"#).stdout
    );
    assert_eq!(
        "false",
        &rail_oneliner(r#""apple" "orange" == print"#).stdout
    );
}

#[test]
fn test_quote_of_many_types_equality() {
    assert_eq!(
        "true",
        &rail_oneliner(
            r#"
        [ true 2 -3.4e5 "6" [ print ] ]
        [ true 2 -3.4e5 "6" [ print ] ]
        == print
        "#
        )
        .stdout
    );
}

#[test]
fn test_comparison_lt() {
    assert_eq!("true", &rail_oneliner("1 2 < print").stdout);
    assert_eq!("true", &rail_oneliner("1 1.1 < print").stdout);
    assert_eq!("true", &rail_oneliner("1.1 2 < print").stdout);
    assert_eq!("true", &rail_oneliner("1.1 2.2 < print").stdout);

    assert_eq!("false", &rail_oneliner("1 1 < print").stdout);
    assert_eq!("false", &rail_oneliner("1 1.0 < print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 1 < print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 1.0 < print").stdout);
    assert_eq!("false", &rail_oneliner("1 0 < print").stdout);
    assert_eq!("false", &rail_oneliner("1 0.9 < print").stdout);
    assert_eq!("false", &rail_oneliner("1.1 1 < print").stdout);
    assert_eq!("false", &rail_oneliner("1.1 0.9 < print").stdout);
}

#[test]
fn test_comparison_le() {
    assert_eq!("true", &rail_oneliner("1 2 <= print").stdout);
    assert_eq!("true", &rail_oneliner("1 1 <= print").stdout);
    assert_eq!("true", &rail_oneliner("1 1.1 <= print").stdout);
    assert_eq!("true", &rail_oneliner("1.1 2 <= print").stdout);
    assert_eq!("true", &rail_oneliner("1.1 1.1 <= print").stdout);
    assert_eq!("true", &rail_oneliner("1 1.0 <= print").stdout);
    assert_eq!("true", &rail_oneliner("1.0 1 <= print").stdout);

    assert_eq!("false", &rail_oneliner("1 0 <= print").stdout);
    assert_eq!("false", &rail_oneliner("1 0.9 <= print").stdout);
    assert_eq!("false", &rail_oneliner("1.1 1 <= print").stdout);
    assert_eq!("false", &rail_oneliner("1.1 0.9 <= print").stdout);
}
#[test]
fn test_comparison_gt() {
    assert_eq!("true", &rail_oneliner("2 1 > print").stdout);
    assert_eq!("true", &rail_oneliner("1.1 1 > print").stdout);
    assert_eq!("true", &rail_oneliner("2 1.1 > print").stdout);
    assert_eq!("true", &rail_oneliner("2.2 1.1 > print").stdout);

    assert_eq!("false", &rail_oneliner("1 1 > print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 1 > print").stdout);
    assert_eq!("false", &rail_oneliner("1 1.0 > print").stdout);
    assert_eq!("false", &rail_oneliner("1.0 1.0 > print").stdout);
    assert_eq!("false", &rail_oneliner("0 1 > print").stdout);
    assert_eq!("false", &rail_oneliner("0.9 1 > print").stdout);
    assert_eq!("false", &rail_oneliner("1 1.1 > print").stdout);
    assert_eq!("false", &rail_oneliner("0.9 1.1 > print").stdout);
}

#[test]
fn test_comparison_ge() {
    assert_eq!("true", &rail_oneliner("2 1 >= print").stdout);
    assert_eq!("true", &rail_oneliner("1 1 >= print").stdout);
    assert_eq!("true", &rail_oneliner("1.1 1 >= print").stdout);
    assert_eq!("true", &rail_oneliner("2 1.1 >= print").stdout);
    assert_eq!("true", &rail_oneliner("1.1 1.1 >= print").stdout);
    assert_eq!("true", &rail_oneliner("1.0 1 >= print").stdout);
    assert_eq!("true", &rail_oneliner("1 1.0 >= print").stdout);

    assert_eq!("false", &rail_oneliner("0 1 >= print").stdout);
    assert_eq!("false", &rail_oneliner("0.9 1 >= print").stdout);
    assert_eq!("false", &rail_oneliner("1 1.1 >= print").stdout);
    assert_eq!("false", &rail_oneliner("0.9 1.1 >= print").stdout);
}
