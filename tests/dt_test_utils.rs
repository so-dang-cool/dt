use std::io::{Read, Write};
use std::process::{Command, ExitStatus, Output, Stdio};

const DT_PATH: &str = std::env!("CARGO_BIN_EXE_dt");
const DTSH_PATH: &str = std::env!("CARGO_BIN_EXE_dtsh");
const DEV_MODE_ARGS: [&str; 3] = ["--no-stdlib", "--lib-list", "dt-src/dev.txt"];

#[allow(dead_code)]
pub struct DtPipedResult {
    pub stdout: String,
    pub stderr: String,
}

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
pub fn dtsh(stdin: &str) -> DtPipedResult {
    let dt_proc = Command::new(DTSH_PATH)
        .args(DEV_MODE_ARGS)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Error running process");

    dt_proc
        .stdin
        .expect("Error sending stdin")
        .write_all(stdin.as_bytes())
        .unwrap();

    let mut stdout = String::new();
    dt_proc.stdout.unwrap().read_to_string(&mut stdout).unwrap();

    let mut stderr = String::new();
    dt_proc.stderr.unwrap().read_to_string(&mut stderr).unwrap();

    DtPipedResult { stdout, stderr }
}

#[allow(dead_code)]
pub fn dtsh_run_file(file: &str) -> DtRunResult {
    Command::new(DTSH_PATH)
        .args(DEV_MODE_ARGS)
        .args(&["run", file])
        .output()
        .expect("Error running process")
        .into()
}

#[allow(dead_code)]
pub fn dt_oneliner(source: &str) -> DtRunResult {
    dt(&[source])
}

#[allow(dead_code)]
pub fn dt(args: &[&str]) -> DtRunResult {
    Command::new(DT_PATH)
        .args(DEV_MODE_ARGS)
        .args(args)
        .output()
        .expect("Error running process")
        .into()
}
