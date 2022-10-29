mod dt_test_utils;
use dt_test_utils::dt;

#[test]
fn basic_def() {
    let source = r#"
        [1 +] [inc] def
        1 inc print
        [inc] def? print
    "#;

    assert_eq!("2true", dt(&[source]).stdout);
}

#[test]
fn basic_do_bang_def() {
    let source = r#"
        [[] [empty-quote] def] do!

        "empty-quote" def? "do! should define in parent context" assert-true
    "#;

    assert_eq!("", dt(&[source]).stderr);
    assert_eq!("", dt(&[source]).stdout);
}

#[test]
fn basic_do_def() {
    let source = r#"
        [[] [empty-quote] def] do

        "empty-quote" undef? "do should NOT define in parent context" assert-true
    "#;

    assert_eq!("", dt(&[source]).stderr);
    assert_eq!("", dt(&[source]).stdout);
}

#[test]
fn basic_arrow_do() {
    let source = r#"
        3 "banana"

        # Print banana three times
        [[n str]: [str print] n times] do

        "n" undef? "do must not leak definitions, but n was defined" assert-true
        "str" undef? "do must not leak definitions, but str was defined" assert-true
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(["banana", "banana", "banana"].join(""), res.stdout);
}

#[test]
fn arrow_in_times() {
    let source = r#"
        1
        [ [ n ]:
            n println
            n 2 *
        ] 7 times

        [ n ] undef? "times must not leak definitions, but n was defined" assert-true
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(
        ["1", "2", "4", "8", "16", "32", "64", ""].join("\n"),
        res.stdout
    );
}

#[test]
fn arrow_in_each() {
    let source = r#"
        [ 1 2 3 4 5 ]
        [ [ n ]: n n * println ] each

        [ n ] undef? "each must not leak definitions, but n was defined" assert-true
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(["1", "4", "9", "16", "25", ""].join("\n"), res.stdout);
}

#[test]
fn arrow_in_map() {
    let source = r#"
        ["apple" "banana" "cereal"]
        [[food]: food upcase] map print

        [food] undef? "map must not leak definitions, but food was defined" assert-true
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(r#"[ "APPLE" "BANANA" "CEREAL" ]"#, res.stdout);
}

#[test]
fn arrow_in_filter() {
    let source = r#"
        [[1 "banana"] [2 "banana"] [3 "banana"] [4 "bananas make a bunch!"]]
        [...[n str]: n even? str len odd?] filter unquote print

        [n]   undef? "filter must not leak definitions, but n was defined" assert-true
        [str] undef? "filter must not leak definitions, but str was defined" assert-true
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(r#"[ 4 "bananas make a bunch!" ]"#, res.stdout);
}

#[test]
fn shadowing_in_do() {
    let source = r#"
        [ 5 ] "fav-number" def

        fav-number println

        [
            [ 8 ] "fav-number" def
            fav-number println
        ] do

        fav-number println
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(["5", "8", "5", ""].join("\n"), res.stdout);
}

#[test]
fn shadowing_arrow_in_do() {
    let source = r#"
        [ 6 ] "fav-number" def

        fav-number println

        2 [ [ fav-number ] : fav-number println ] do

        fav-number println
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(["6", "2", "6", ""].join("\n"), res.stdout);
}

#[test]
fn shadowing_in_times() {
    let source = r#"
        [ 1 ] [ n ] def

        n [ [ n ] : n println n 1 + ] 3 times

        n println
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(["1", "2", "3", "1", ""].join("\n"), res.stdout);
}

#[test]
fn shadowing_in_each() {
    let source = r#"
        [ "pepperoni" ] [ pizza ] def

        pizza println

        [ "cheese" "bbq" "combo" ] [ [ pizza ] : pizza println ] each

        pizza println
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(
        ["pepperoni", "cheese", "bbq", "combo", "pepperoni", ""].join("\n"),
        res.stdout
    );
}

#[test]
fn shadowing_in_map() {
    let source = r#"
        [ "banana" ] [ x ] def

        x println

        [ 1 2 3 ] [ [ x ]: x 2.0 / ] map println

        x println
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(
        ["banana", "[ 0.5 1 1.5 ]", "banana", ""].join("\n"),
        res.stdout
    );
}

#[test]
fn shadowing_in_filter() {
    let source = r#"
        [ "whee" ] [ happy-word ] def

        happy-word println

        [ "yay" "hurray" "whoo" "huzzah" ] [ [ happy-word ] : 4 happy-word len gt? ] filter println

        happy-word println
    "#;

    let res = dt(&[source]);

    assert_eq!("", res.stderr);

    assert_eq!(
        ["whee", r#"[ "hurray" "huzzah" ]"#, "whee", ""].join("\n"),
        res.stdout
    );
}
