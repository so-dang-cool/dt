mod rail_runner;
use rail_runner::railsh_run_file;

#[test]
pub fn say_hello() {
    let res = railsh_run_file("tests/basic.rail");

    assert!(res.status.success());
    assert_eq!("Hello world!\n", res.stdout);
    assert_eq!("", res.stderr);
}
