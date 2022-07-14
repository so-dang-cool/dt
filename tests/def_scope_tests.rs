mod rail_runner;
use rail_runner::rail;

#[test]
fn basic_def() {
    let source = r#"
        [ 1 + ] "inc" def
        1 inc print
        "inc" def? print
    "#;

    assert_eq!("2true", rail(&[source]).stdout);
}

#[test]
fn basic_arrow_do() {
    let source = r#"
        3 "banana"

        # Print banana three times
        [ [ "n" "str" ] -> [ str print ] n times ] do

        "n" undef? "do must not leak definitions, but n was defined" assert-true
        "str" undef? "do must not leak definitions, but str was defined" assert-true
    "#;

    let res = rail(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(["banana", "banana", "banana"].join(""), res.stdout);
}

#[test]
#[ignore = "FIXME"]
fn arrow_in_each() {
    let source = r#"
        [ 1 2 3 4 5 ]
        [ [ "n" ] -> n n * println ] each

        "n" undef? "each must not leak definitions, but n was defined" assert-true
    "#;

    let res = rail(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(
        ["1", "4", "9", "16", "25", "false", ""].join("\n"),
        res.stdout
    );
}

#[test]
fn arrow_in_map() {
    let source = r#"
        [ "apple" "banana" "cereal" ]
        [ [ "food" ] -> food upcase ] map print

        "food" undef? "map must not leak definitions, but food was defined" assert-true
    "#;

    let res = rail(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(r#"[ "APPLE" "BANANA" "CEREAL" ]"#, res.stdout);
}

#[test]
fn arrow_in_filter() {
    let source = r#"
        [ [ 1 "banana" ] [ 2 "banana" ] [ 3 "banana" ] [ 4 "bananas make a bunch!" ] ]
        [ unquote [ "n" "str" ] -> n even? str len odd? ] filter unquote print

        "n" undef? "filter must not leak definitions, but n was defined" assert-true
        "str" undef? "filter must not leak definitions, but str was defined" assert-true
    "#;

    let res = rail(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(r#"[ 4 "bananas make a bunch!" ]"#, res.stdout);
}
