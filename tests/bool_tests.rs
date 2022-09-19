mod dt_test_utils;
use dt_test_utils::{dt, dt_oneliner};

pub const DT_PATH: &str = std::env!("CARGO_BIN_EXE_dt");

#[test]
fn test_true() {
    let res = dt(&["true", "print"]);
    assert_eq!("", &res.stderr);
    assert_eq!("true", &res.stdout);
    assert!(res.status.success());
}

#[test]
fn test_false() {
    let res = dt(&["false", "print"]);
    assert_eq!("", &res.stderr);
    assert_eq!("false", &res.stdout);
    assert!(res.status.success());
}

#[test]
fn test_not() {
    assert_eq!("true", &dt_oneliner("false not print").stdout);
    assert_eq!("false", &dt_oneliner("true not print").stdout);
}

#[test]
fn test_bool_equality() {
    let first_result = dt_oneliner("true true eq? print");
    assert_eq!("", &first_result.stderr);
    assert_eq!("true", &first_result.stdout);
    assert_eq!("true", &dt_oneliner("false false eq? print").stdout);
    assert_eq!("true", &dt_oneliner("true false neq? print").stdout);
    assert_eq!("true", &dt_oneliner("false true neq? print").stdout);

    assert_eq!("false", &dt_oneliner("true true neq? print").stdout);
    assert_eq!("false", &dt_oneliner("false false neq? print").stdout);
    assert_eq!("false", &dt_oneliner("true false eq? print").stdout);
    assert_eq!("false", &dt_oneliner("false true eq? print").stdout);
}

#[test]
fn test_numeric_equality() {
    assert_eq!("true", &dt_oneliner("1 1 eq? print").stdout);
    assert_eq!("true", &dt_oneliner("1 1.0 eq? print").stdout);
    assert_eq!("true", &dt_oneliner("1.0 1 eq? print").stdout);
    assert_eq!("true", &dt_oneliner("1.0 1.0 eq? print").stdout);

    assert_eq!("false", &dt_oneliner("1 2 eq? print").stdout);
    assert_eq!("false", &dt_oneliner("1 2.0 eq? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 2 eq? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 2.0 eq? print").stdout);

    assert_eq!("false", &dt_oneliner("1 1 neq? print").stdout);
    assert_eq!("false", &dt_oneliner("1 1.0 neq? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 1 neq? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 1.0 neq? print").stdout);

    assert_eq!("true", &dt_oneliner("1 2 neq? print").stdout);
    assert_eq!("true", &dt_oneliner("1 2.0 neq? print").stdout);
    assert_eq!("true", &dt_oneliner("1.0 2 neq? print").stdout);
    assert_eq!("true", &dt_oneliner("1.0 2.0 neq? print").stdout);
}

#[test]
fn test_string_equality() {
    assert_eq!("true", &dt_oneliner(r#""apple" "apple" eq? print"#).stdout);
    assert_eq!(
        "true",
        &dt_oneliner(r#""apple" "orange" neq? print"#).stdout
    );

    assert_eq!(
        "false",
        &dt_oneliner(r#""apple" "apple" neq? print"#).stdout
    );
    assert_eq!(
        "false",
        &dt_oneliner(r#""apple" "orange" eq? print"#).stdout
    );
}

#[test]
fn test_quote_of_many_types_equality() {
    assert_eq!(
        "true",
        &dt_oneliner(
            r#"
        [ true 2 -3.4e5 "6" [ print ] ]
        [ true 2 -3.4e5 "6" [ print ] ]
        eq? print
        "#
        )
        .stdout
    );
}

#[test]
fn test_comparison_gt() {
    assert_eq!("true", &dt_oneliner("1 2 gt? print").stdout);
    assert_eq!("true", &dt_oneliner("1 1.1 gt? print").stdout);
    assert_eq!("true", &dt_oneliner("1.1 2 gt? print").stdout);
    assert_eq!("true", &dt_oneliner("1.1 2.2 gt? print").stdout);

    assert_eq!("false", &dt_oneliner("1 1 gt? print").stdout);
    assert_eq!("false", &dt_oneliner("1 1.0 gt? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 1 gt? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 1.0 gt? print").stdout);
    assert_eq!("false", &dt_oneliner("1 0 gt? print").stdout);
    assert_eq!("false", &dt_oneliner("1 0.9 gt? print").stdout);
    assert_eq!("false", &dt_oneliner("1.1 1 gt? print").stdout);
    assert_eq!("false", &dt_oneliner("1.1 0.9 gt? print").stdout);
}

#[test]
fn test_comparison_gte() {
    assert_eq!("true", &dt_oneliner("1 2 gte? print").stdout);
    assert_eq!("true", &dt_oneliner("1 1 gte? print").stdout);
    assert_eq!("true", &dt_oneliner("1 1.1 gte? print").stdout);
    assert_eq!("true", &dt_oneliner("1.1 2 gte? print").stdout);
    assert_eq!("true", &dt_oneliner("1.1 1.1 gte? print").stdout);
    assert_eq!("true", &dt_oneliner("1 1.0 gte? print").stdout);
    assert_eq!("true", &dt_oneliner("1.0 1 gte? print").stdout);

    assert_eq!("false", &dt_oneliner("1 0 gte? print").stdout);
    assert_eq!("false", &dt_oneliner("1 0.9 gte? print").stdout);
    assert_eq!("false", &dt_oneliner("1.1 1 gte? print").stdout);
    assert_eq!("false", &dt_oneliner("1.1 0.9 gte? print").stdout);
}
#[test]
fn test_comparison_lt() {
    assert_eq!("true", &dt_oneliner("2 1 lt? print").stdout);
    assert_eq!("true", &dt_oneliner("1.1 1 lt? print").stdout);
    assert_eq!("true", &dt_oneliner("2 1.1 lt? print").stdout);
    assert_eq!("true", &dt_oneliner("2.2 1.1 lt? print").stdout);

    assert_eq!("false", &dt_oneliner("1 1 lt? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 1 lt? print").stdout);
    assert_eq!("false", &dt_oneliner("1 1.0 lt? print").stdout);
    assert_eq!("false", &dt_oneliner("1.0 1.0 lt? print").stdout);
    assert_eq!("false", &dt_oneliner("0 1 lt? print").stdout);
    assert_eq!("false", &dt_oneliner("0.9 1 lt? print").stdout);
    assert_eq!("false", &dt_oneliner("1 1.1 lt? print").stdout);
    assert_eq!("false", &dt_oneliner("0.9 1.1 lt? print").stdout);
}

#[test]
fn test_comparison_lte() {
    assert_eq!("true", &dt_oneliner("2 1 lte? print").stdout);
    assert_eq!("true", &dt_oneliner("1 1 lte? print").stdout);
    assert_eq!("true", &dt_oneliner("1.1 1 lte? print").stdout);
    assert_eq!("true", &dt_oneliner("2 1.1 lte? print").stdout);
    assert_eq!("true", &dt_oneliner("1.1 1.1 lte? print").stdout);
    assert_eq!("true", &dt_oneliner("1.0 1 lte? print").stdout);
    assert_eq!("true", &dt_oneliner("1 1.0 lte? print").stdout);

    assert_eq!("false", &dt_oneliner("0 1 lte? print").stdout);
    assert_eq!("false", &dt_oneliner("0.9 1 lte? print").stdout);
    assert_eq!("false", &dt_oneliner("1 1.1 lte? print").stdout);
    assert_eq!("false", &dt_oneliner("0.9 1.1 lte? print").stdout);
}
