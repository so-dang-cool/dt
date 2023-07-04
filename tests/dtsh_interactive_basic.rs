mod dt_test_utils;
use dt_test_utils::{dt_stdin, DtRunResult, dt};

fn assert_two(result: DtRunResult) {
    assert_eq!("2\n", result.stdout);
}

#[test]
pub fn one_plus_one_how_hard_could_it_be() {
    let res = dt(&["1 1 + pl"]);

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_two() {
    let res = dt_stdin("1 1 + pl\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_still_two() {
    let res = dt_stdin("1 1 [ + ] do pl\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_definitely_two() {
    let res = dt_stdin("1 [ 1 + ] do pl\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_positively_two() {
    let res = dt_stdin("[ 1 ] 2 times + pl\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_never_not_two() {
    let res = dt_stdin("[ 1 ] [ 1 ] [ + ] [ concat ] 2 times do pl\n");

    assert_two(res);
}
