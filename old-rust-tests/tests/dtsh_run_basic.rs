mod dt_test_utils;
use dt_test_utils::dt_run_file;

#[test]
pub fn say_hello() {
    let res = dt_run_file("tests/basic.dt");

    assert_eq!("", res.stderr);
    assert!(res.status.success());
    assert_eq!("Hello world!\n", res.stdout);
}
