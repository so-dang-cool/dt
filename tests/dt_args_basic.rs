mod dt_test_utils;
use dt_test_utils::dt;

#[test]
fn status() {
    assert_eq!("[ ]\n", dt(&["status"]).stdout);
}

#[test]
fn one_plus_one_is_two() {
    // dt 1 1 + print
    let res = dt(&["1", "1", "+", "println"]);
    assert_eq!("2\n", res.stdout);
}

#[test]
fn quoted_one_plus_one_is_two() {
    // dt "1 1 + print"
    let res = dt(&["1 1 + println"]);
    assert_eq!("2\n", res.stdout);
}
