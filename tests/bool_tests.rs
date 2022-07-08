use std::process::Command;

pub const RAIL_PATH: &str = std::env!("CARGO_BIN_EXE_rail");

#[test]
fn test_true() {
    assert_eq!("true", &rail_eval("true print"));
}

#[test]
fn test_false() {
    assert_eq!("false", &rail_eval("false print"));
}

#[test]
fn test_not() {
    assert_eq!("true", &rail_eval("false not print"));
    assert_eq!("false", &rail_eval("true not print"));
}

#[test]
fn test_bool_equality() {
    assert_eq!("true", &rail_eval("true true == print"));
    assert_eq!("true", &rail_eval("false false == print"));
    assert_eq!("true", &rail_eval("true false != print"));
    assert_eq!("true", &rail_eval("false true != print"));

    assert_eq!("false", &rail_eval("true true != print"));
    assert_eq!("false", &rail_eval("false false != print"));
    assert_eq!("false", &rail_eval("true false == print"));
    assert_eq!("false", &rail_eval("false true == print"));
}

#[test]
fn test_numeric_equality() {
    assert_eq!("true", &rail_eval("1 1 == print"));
    assert_eq!("true", &rail_eval("1 1.0 == print"));
    assert_eq!("true", &rail_eval("1.0 1 == print"));
    assert_eq!("true", &rail_eval("1.0 1.0 == print"));

    assert_eq!("false", &rail_eval("1 2 == print"));
    assert_eq!("false", &rail_eval("1 2.0 == print"));
    assert_eq!("false", &rail_eval("1.0 2 == print"));
    assert_eq!("false", &rail_eval("1.0 2.0 == print"));

    assert_eq!("false", &rail_eval("1 1 != print"));
    assert_eq!("false", &rail_eval("1 1.0 != print"));
    assert_eq!("false", &rail_eval("1.0 1 != print"));
    assert_eq!("false", &rail_eval("1.0 1.0 != print"));

    assert_eq!("true", &rail_eval("1 2 != print"));
    assert_eq!("true", &rail_eval("1 2.0 != print"));
    assert_eq!("true", &rail_eval("1.0 2 != print"));
    assert_eq!("true", &rail_eval("1.0 2.0 != print"));
}

#[test]
fn test_string_equality() {
    assert_eq!("true", &rail_eval(r#""apple" "apple" == print"#));
    assert_eq!("true", &rail_eval(r#""apple" "orange" != print"#));

    assert_eq!("false", &rail_eval(r#""apple" "apple" != print"#));
    assert_eq!("false", &rail_eval(r#""apple" "orange" == print"#));
}

#[test]
fn test_quote_of_many_types_equality() {
    assert_eq!(
        "true",
        &rail_eval(
            r#"
        [ true 2 -3.4e5 "6" [ print ] ]
        [ true 2 -3.4e5 "6" [ print ] ]
        == print
        "#
        )
    );
}

#[test]
fn test_comparison_lt() {
    assert_eq!("true", &rail_eval("1 2 < print"));
    assert_eq!("true", &rail_eval("1 1.1 < print"));
    assert_eq!("true", &rail_eval("1.1 2 < print"));
    assert_eq!("true", &rail_eval("1.1 2.2 < print"));

    assert_eq!("false", &rail_eval("1 1 < print"));
    assert_eq!("false", &rail_eval("1 1.0 < print"));
    assert_eq!("false", &rail_eval("1.0 1 < print"));
    assert_eq!("false", &rail_eval("1.0 1.0 < print"));
    assert_eq!("false", &rail_eval("1 0 < print"));
    assert_eq!("false", &rail_eval("1 0.9 < print"));
    assert_eq!("false", &rail_eval("1.1 1 < print"));
    assert_eq!("false", &rail_eval("1.1 0.9 < print"));
}

#[test]
fn test_comparison_le() {
    assert_eq!("true", &rail_eval("1 2 <= print"));
    assert_eq!("true", &rail_eval("1 1 <= print"));
    assert_eq!("true", &rail_eval("1 1.1 <= print"));
    assert_eq!("true", &rail_eval("1.1 2 <= print"));
    assert_eq!("true", &rail_eval("1.1 1.1 <= print"));
    assert_eq!("true", &rail_eval("1 1.0 <= print"));
    assert_eq!("true", &rail_eval("1.0 1 <= print"));

    assert_eq!("false", &rail_eval("1 0 <= print"));
    assert_eq!("false", &rail_eval("1 0.9 <= print"));
    assert_eq!("false", &rail_eval("1.1 1 <= print"));
    assert_eq!("false", &rail_eval("1.1 0.9 <= print"));
}
#[test]
fn test_comparison_gt() {
    assert_eq!("true", &rail_eval("2 1 > print"));
    assert_eq!("true", &rail_eval("1.1 1 > print"));
    assert_eq!("true", &rail_eval("2 1.1 > print"));
    assert_eq!("true", &rail_eval("2.2 1.1 > print"));

    assert_eq!("false", &rail_eval("1 1 > print"));
    assert_eq!("false", &rail_eval("1.0 1 > print"));
    assert_eq!("false", &rail_eval("1 1.0 > print"));
    assert_eq!("false", &rail_eval("1.0 1.0 > print"));
    assert_eq!("false", &rail_eval("0 1 > print"));
    assert_eq!("false", &rail_eval("0.9 1 > print"));
    assert_eq!("false", &rail_eval("1 1.1 > print"));
    assert_eq!("false", &rail_eval("0.9 1.1 > print"));
}

#[test]
fn test_comparison_ge() {
    assert_eq!("true", &rail_eval("2 1 >= print"));
    assert_eq!("true", &rail_eval("1 1 >= print"));
    assert_eq!("true", &rail_eval("1.1 1 >= print"));
    assert_eq!("true", &rail_eval("2 1.1 >= print"));
    assert_eq!("true", &rail_eval("1.1 1.1 >= print"));
    assert_eq!("true", &rail_eval("1.0 1 >= print"));
    assert_eq!("true", &rail_eval("1 1.0 >= print"));

    assert_eq!("false", &rail_eval("0 1 >= print"));
    assert_eq!("false", &rail_eval("0.9 1 >= print"));
    assert_eq!("false", &rail_eval("1 1.1 >= print"));
    assert_eq!("false", &rail_eval("0.9 1.1 >= print"));
}

fn rail_eval(source: &str) -> String {
    let output = Command::new(RAIL_PATH)
        .args(&["eval", source])
        .output()
        .expect("Error running process");

    String::from_utf8(output.stdout)
        .expect("Error reading stdout as utf8")
        .trim_end_matches('\n')
        .to_string()
}
