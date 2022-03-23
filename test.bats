# Tested with BATS
# https://github.com/bats-core/bats-core/

setup() {
    load 'tests/test_helper/bats-support/load'
    load 'tests/test_helper/bats-assert/load'
}

@test "SETUP: 'cargo build' passes" {
    run cargo build
    assert_success
}

rail=./target/debug/rail

@test "SETUP: 'rail' is an executable" {
    [[ -x $rail ]]
}

@test "SETUP: 'rail --version' gives a version" {
    run $rail --version
    assert_success
    assert_output --regexp 'rail [0-9]\.[0-9]\.[0-9]'
}