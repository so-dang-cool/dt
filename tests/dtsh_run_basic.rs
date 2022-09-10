mod dt_test_utils;
use dt_test_utils::dtsh_run_file;

#[test]
pub fn say_hello() {
    let res = dtsh_run_file("tests/basic.dt");

    assert!(res.status.success());
    assert_eq!("Hello world!\n", res.stdout);
    assert_eq!("", res.stderr);
}
