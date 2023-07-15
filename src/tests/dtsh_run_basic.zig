const std = @import("std");
const expectEqual = std.testing.expectEqual;
const expectEqualStrings = std.testing.expectEqualStrings;

const dtRunFile = @import("dt_test_utils.zig").dtRunFile;

test "say_hello" {
    const res = try dtRunFile("./src/tests/basic.dt");
    try expectEqualStrings("", res.stderr);
    try expectEqual(@as(u8, 0), res.term.Exited);
    try expectEqualStrings("Hello world!\n", res.stdout);
}
