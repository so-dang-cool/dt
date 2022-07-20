mod rail_runner;
use rail_runner::railsh_run_file;

#[test]
pub fn problem_01() {
    let res = railsh_run_file("tests/projecteuler/problem-01.rail");

    assert_eq!("", res.stderr);
    assert_eq!("233168\n", res.stdout);
    assert!(res.status.success());
}
