mod rail_runner;
use rail_runner::{railsh, RailRunResult};

fn assert_two(result: RailRunResult) {
    assert_eq!("", result.stdout);

    let stderr_lines = result.stderr.split('\n').collect::<Vec<_>>();
    assert!(stderr_lines[0].starts_with("rail"));
    assert_eq!("Derailed: End of input", stderr_lines[1]);
    assert_eq!("State dump: [ 2 ]", stderr_lines[2])
}

#[test]
pub fn one_plus_one_is_two() {
    let res = railsh("1 1 +\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_still_two() {
    let res = railsh("1 1 [ + ] do\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_definitely_two() {
    let res = railsh("1 [ 1 + ] do\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_positively_two() {
    let res = railsh("[ 1 ] 2 times +\n");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_never_not_two() {
    let res = railsh("[ 1 ] [ 1 ] [ + ] [ concat ] 2 times do\n");

    assert_two(res);
}
