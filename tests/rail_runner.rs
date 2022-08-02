use std::io::{Read, Write};
use std::process::{Command, ExitStatus, Output, Stdio};

const RAIL_PATH: &str = std::env!("CARGO_BIN_EXE_rail");
const RAILSH_PATH: &str = std::env!("CARGO_BIN_EXE_railsh");
const DEV_MODE_ARGS: [&str; 3] = ["--no-stdlib", "--lib-list", "rail-src/dev.txt"];

#[allow(dead_code)]
pub struct RailPipedResult {
    pub stdout: String,
    pub stderr: String,
}

#[allow(dead_code)]
pub struct RailRunResult {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl From<Output> for RailRunResult {
    fn from(output: Output) -> Self {
        let Output {
            status,
            stdout,
            stderr,
        } = output;
        let stdout = String::from_utf8(stdout).expect("Unable to read stdout");
        let stderr = String::from_utf8(stderr).expect("Unable to read stderr");
        RailRunResult {
            status,
            stdout,
            stderr,
        }
    }
}

#[allow(dead_code)]
pub fn railsh(stdin: &str) -> RailPipedResult {
    let rail_proc = Command::new(RAILSH_PATH)
        .args(DEV_MODE_ARGS)
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

    RailPipedResult { stdout, stderr }
}

#[allow(dead_code)]
pub fn railsh_run_file(file: &str) -> RailRunResult {
    Command::new(RAILSH_PATH)
        .args(DEV_MODE_ARGS)
        .args(&["run", file])
        .output()
        .expect("Error running process")
        .into()
}

#[allow(dead_code)]
pub fn rail_oneliner(source: &str) -> RailRunResult {
    rail(&[source])
}

#[allow(dead_code)]
pub fn rail(args: &[&str]) -> RailRunResult {
    Command::new(RAIL_PATH)
        .args(DEV_MODE_ARGS)
        .args(args)
        .output()
        .expect("Error running process")
        .into()
}
