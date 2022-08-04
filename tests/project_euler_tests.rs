mod rail_runner;
use rail_runner::railsh_run_file;

fn run_problem(n: &str) -> String {
    let filename = format!("tests/project_euler/problem-{}.rail", n);
    railsh_run_file(&filename).stdout.trim_end().to_string()
}

#[test]
pub fn problem_01() {
    assert_eq!("233168", run_problem("01"));
}

#[test]
pub fn problem_02() {
    assert_eq!("4613732", run_problem("02"));
}
