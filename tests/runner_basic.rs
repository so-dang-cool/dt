use std::process::{Command, Output};

pub const RAIL_PATH: &str = std::env!("CARGO_BIN_EXE_rail");

#[test]
pub fn say_hello() {
    let res = rail_run("tests/basic.rail");

    assert!(res.status.success());
    assert_eq!("Hello world!\n", String::from_utf8(res.stdout).unwrap());
    assert_eq!("", String::from_utf8(res.stderr).unwrap());
}

fn rail_run(file: &str) -> Output {
    Command::new(RAIL_PATH)
        .args(&["run", file])
        .output()
        .expect("Error running process")
}
