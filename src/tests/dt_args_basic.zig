const std = @import("std");
const expectEqualStrings = std.testing.expectEqualStrings;

const dt = @import("dt_test_utils.zig").dt;

test "status" {
    try expectEqualStrings("[ ]\n", (try dt(&.{ "quote-all", "drop", "status" })).stderr);
}

test "one_plus_one_is_two" {
    try expectEqualStrings("2\n", (try dt(&.{ "1", "1", "+", "println" })).stdout);
}

test "quoted_one_plus_one_is_two" {
    try expectEqualStrings("2\n", (try dt(&.{"1 1 + println"})).stdout);
}
