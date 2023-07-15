const std = @import("std");
const expectEqualStrings = std.testing.expectEqualStrings;

const utils = @import("dt_test_utils.zig");
const dtStdin = utils.dtStdin;
const dt = utils.dt;

test "one_plus_one_how_hard_could_it_be" {
    try expectEqualStrings("2\n", (try dt(&.{"1 1 + pl"})).stdout);
}

test "one_plus_one_is_two" {
    try expectEqualStrings("2\n", (try dtStdin("1 1 + pl\n")).stdout);
}

test "one_plus_one_is_still_two" {
    try expectEqualStrings("2\n", (try dtStdin("1 1 [ + ] do pl\n")).stdout);
}

test "one_plus_one_is_definitely_two" {
    try expectEqualStrings("2\n", (try dtStdin("1 [ 1 + ] do pl\n")).stdout);
}

test "one_plus_one_is_positively_two" {
    try expectEqualStrings("2\n", (try dtStdin("[ 1 ] 2 times + pl\n")).stdout);
}

test "one_plus_one_is_never_not_two" {
    try expectEqualStrings("2\n", (try dtStdin("[ 1 ] [ 1 ] [ + ] [ concat ] 2 times do pl\n")).stdout);
}
