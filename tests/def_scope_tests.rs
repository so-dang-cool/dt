mod rail_runner;
use rail_runner::rail;

#[test]
fn basic_def() {
    let source = r#"
        [ 1 + ] "inc" def
        1 inc print
        "inc" def? print
    "#;
    assert_eq!("2true", rail(&[source]).stdout)
}

#[test]
fn basic_arrow_do() {
    let source = r#"
        3 "banana"

        # Print banana three times
        [ [ "n" "str" ] -> [ str println ] n times ] do

        # These must be undefined. 'do' should not leak definitions.
        "n" def? println
        "str" def? println
    "#;
    assert_eq!(
        ["banana", "banana", "banana", "false", "false", ""].join("\n"),
        rail(&[source]).stdout
    )
}
