const std = @import("std");
const expectEqualStrings = std.testing.expectEqualStrings;

const dtRunFile = @import("dt_test_utils.zig").dtRunFile;

test "problem_01" {
    try expectEqualStrings("233168\n", (try dtRunFile("./src/tests/project_euler/problem-01.dt")).stdout);
}

// Broken by zig impl changes

// test "problem_02a" {
//     try expectEqualStrings("4613732\n", (try dtRunFile("./src/tests/project_euler/problem-02a.dt")).stdout);
// }

// test "problem_02b" {
//     try expectEqualStrings("4613732\n", (try dtRunFile("./src/tests/project_euler/problem-02b.dt")).stdout);
// }

// test "problem_03" {
//     try expectEqualStrings("6857\n", (try dtRunFile("./src/tests/project_euler/problem-03.dt")).stdout);
// }

// test "problem_04" {
//     try expectEqualStrings("906609\n", (try dtRunFile("./src/tests/project_euler/problem-04.dt")).stdout);
// }
