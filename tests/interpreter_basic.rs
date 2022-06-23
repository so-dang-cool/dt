use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub const RAIL_PATH: &str = std::env!("CARGO_BIN_EXE_rail");

#[test]
pub fn one_plus_one_is_two() {
    let res = rail_interpret("1 1 +\n");

    assert!(res.stdout.ends_with("[ 2 ]\n"));
    assert_eq!("Derailed: End of input\n", res.stderr);
}

#[test]
pub fn one_plus_one_is_still_two() {
    let res = rail_interpret("1 1 [ + ] call\n");

    assert!(res.stdout.ends_with("[ 2 ]\n"));
    assert_eq!("Derailed: End of input\n", res.stderr);
}

#[test]
pub fn one_plus_one_is_definitely_two() {
    let res = rail_interpret("1 [ 1 + ] call\n");

    assert!(res.stdout.ends_with("[ 2 ]\n"));
    assert_eq!("Derailed: End of input\n", res.stderr);
}

#[test]
pub fn one_plus_one_is_positively_two() {
    let res = rail_interpret("[ 1 ] 2 times +\n");

    assert!(res.stdout.ends_with("[ 2 ]\n"));
    assert_eq!("Derailed: End of input\n", res.stderr);
}

#[test]
pub fn one_plus_one_is_never_not_two() {
    let res = rail_interpret("[ 1 ] [ 1 ] [ + ] [ concat ] 2 times call\n");

    assert!(res.stdout.ends_with("[ 2 ]\n"));
    assert_eq!("Derailed: End of input\n", res.stderr);
}

pub struct RailRunResult {
    stdout: String,
    stderr: String,
}

fn rail_interpret(stdin: &str) -> RailRunResult {
    let rail_proc = Command::new(RAIL_PATH)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Error running process");

    rail_proc
        .stdin
        .expect("Error sending stdin")
        .write_all(stdin.as_bytes())
        .unwrap();

    let mut stdout = String::new();
    rail_proc
        .stdout
        .unwrap()
        .read_to_string(&mut stdout)
        .unwrap();

    let mut stderr = String::new();
    rail_proc
        .stderr
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();

    RailRunResult { stdout, stderr }
}
