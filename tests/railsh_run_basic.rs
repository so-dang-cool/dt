use std::process::{Command, Output};

pub const RAILSH_PATH: &str = std::env!("CARGO_BIN_EXE_railsh");

#[test]
pub fn say_hello() {
    let res = railsh_run_file("tests/basic.rail");

    assert!(res.status.success());
    assert_eq!("Hello world!\n", String::from_utf8(res.stdout).unwrap());
    assert_eq!("", String::from_utf8(res.stderr).unwrap());
}

fn railsh_run_file(file: &str) -> Output {
    Command::new(RAILSH_PATH)
        .args(&["run", file])
        .output()
        .expect("Error running process")
}
