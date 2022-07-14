mod rail_runner;
use rail_runner::rail;

#[test]
fn status() {
    assert_eq!("[ ]\n", rail(&["status"]).stdout);
}

#[test]
fn one_plus_one_is_two() {
    // rail 1 1 + print
    let res = rail(&["1", "1", "+", "println"]);
    assert_eq!("2\n", res.stdout);
}

#[test]
fn quoted_one_plus_one_is_two() {
    // rail "1 1 + print"
    let res = rail(&["1 1 + println"]);
    assert_eq!("2\n", res.stdout);
}
