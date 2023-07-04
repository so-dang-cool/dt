mod dt_test_utils;
use dt_test_utils::dt_run_file;

fn run_problem(n: &str) -> String {
    let filename = format!("tests/project_euler/problem-{}.dt", n);
    let res = dt_run_file(&filename);

    eprintln!("=== STDOUT:\n{}\n===\n", res.stdout);
    eprintln!("=== STDERR:\n{}\n===", res.stderr);

    res.stdout.trim_end().to_string()
}

#[test]
pub fn problem_01() {
    assert_eq!("233168", run_problem("01"));
}

#[test]
#[ignore = "Broken by zig impl changes"]
pub fn problem_02a() {
    assert_eq!("4613732", run_problem("02a"));
}

#[test]
#[ignore = "Broken by zig impl changes"]
pub fn problem_02b() {
    assert_eq!("4613732", run_problem("02b"));
}

#[test]
#[ignore = "Broken by zig impl changes"]
pub fn problem_03() {
    assert_eq!("6857", run_problem("03"));
}

#[test]
#[ignore = "Broken by zig impl changes"]
pub fn problem_04() {
    assert_eq!("906609", run_problem("04"));
}
