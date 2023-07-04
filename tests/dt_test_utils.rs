use std::fs::File;
use std::io::Write;
use std::process::{Command, ExitStatus, Output, Stdio};

const DT_PATH: &str = "/workplace/hiljusti/rock/zig-out/bin/dt"; // std::env!("CARGO_BIN_EXE_dt");
const DTSH_PATH: &str = "/workplace/hiljusti/rock/zig-out/bin/dt"; // std::env!("CARGO_BIN_EXE_dtsh");
// const DEV_MODE_ARGS: [&str; 3] = ["--no-stdlib", "--lib-list", "dt-src/dev.txt"];

#[derive(Debug)]
#[allow(dead_code)]
pub struct DtRunResult {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl From<Output> for DtRunResult {
    fn from(output: Output) -> Self {
        let Output {
            status,
            stdout,
            stderr,
        } = output;
        let stdout = String::from_utf8(stdout).expect("Unable to read stdout");
        let stderr = String::from_utf8(stderr).expect("Unable to read stderr");
        DtRunResult {
            status,
            stdout,
            stderr,
        }
    }
}

#[allow(dead_code)]
pub fn dt_stdin(stdin_input: &str) -> DtRunResult {
    let mut dt_proc = Command::new(DTSH_PATH)
        .args(["[\"#\" starts-with? not] filter", "unwords", "eval"])
        .stdin(Stdio ::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Error running process");

        let mut stdin = dt_proc.stdin.take().expect("Failed to open stdin");

        let stdin_input = String::from(stdin_input);
        std::thread::spawn(move || {
            stdin.write_all(stdin_input.as_bytes()).expect("Failed to write to stdin");
        });

    let output = dt_proc.wait_with_output().expect("Failed to read stdout");

    output.into()
}

#[allow(dead_code)]
pub fn dt_run_file(file: &str) -> DtRunResult {
    let file = File::open(file).expect("Unable to open file");
    let contents = std::io::read_to_string(file).expect("Unable to read file contents");

    dt_stdin(&contents)
}

#[allow(dead_code)]
pub fn dt_oneliner(source: &str) -> DtRunResult {
    dt(&[source])
}

#[allow(dead_code)]
pub fn dt(args: &[&str]) -> DtRunResult {
    Command::new(DT_PATH)
        // .args(DEV_MODE_ARGS)
        .args(args)
        .output()
        .expect("Error running process")
        .into()
}
