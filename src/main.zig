const std = @import("std");
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const railCli = @import("rail/cli.zig");

const version = "0.1.0";

pub fn main() anyerror!void {
    const alloc = std.heap.page_allocator;
    const runSettings = try railCli.initialize(alloc);
    try stdout.print("{s} {s}\n", .{runSettings.program, version});
}

test "basic test" {
    try std.testing.expectEqual(10, 3 + 7);
}
