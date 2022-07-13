use std::io::{Read, Write};
use std::process::{Command, Output, Stdio};

const RAILSH_PATH: &str = std::env!("CARGO_BIN_EXE_railsh");

#[allow(dead_code)]
pub struct RailRunResult {
    pub stdout: String,
    pub stderr: String,
}

#[allow(dead_code)]
pub fn railsh(stdin: &str) -> RailRunResult {
    let rail_proc = Command::new(RAILSH_PATH)
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

#[allow(dead_code)]
pub fn railsh_run_file(file: &str) -> Output {
    Command::new(RAILSH_PATH)
        .args(&["run", file])
        .output()
        .expect("Error running process")
}
