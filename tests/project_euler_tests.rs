mod dt_test_utils;
use dt_test_utils::dtsh_run_file;

fn run_problem(n: &str) -> String {
    let filename = format!("tests/project_euler/problem-{}.dt", n);
    dtsh_run_file(&filename).stdout.trim_end().to_string()
}

#[test]
pub fn problem_01() {
    assert_eq!("233168", run_problem("01"));
}

#[test]
pub fn problem_02a() {
    assert_eq!("4613732", run_problem("02a"));
}

#[test]
pub fn problem_02b() {
    assert_eq!("4613732", run_problem("02b"));
}

#[test]
pub fn problem_03() {
    assert_eq!("6857", run_problem("03"));
}

#[test]
pub fn problem_04() {
    assert_eq!("906609", run_problem("04"));
}
