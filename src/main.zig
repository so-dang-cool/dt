const std = @import("std");
const Allocator = std.mem.Allocator;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const Token = union(enum) {
    left_bracket: void,
    right_bracket: void,
    bool: bool,
    i64: i64,
    f64: f64,
    term: []const u8,
    deferred_term: []const u8,
    string: []const u8,
    none: void,
};

pub fn main() !void {
    var stop = false;
    var alloc = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    while (!stop) {
        try stdout.print("> ", .{});

        const input = try prompt(alloc.allocator());

        try stdout.print("Ok smart guy, you said: {s}\n", .{input});
    }
}

fn prompt(alloc: Allocator) ![]const u8 {
    return stdin.readUntilDelimiterOrEofAlloc(alloc, '\n', 128) catch |err| {
        const message = switch (err) {
            error.StreamTooLong => "Response was too many characters.",
            else => "Unable to read response.",
        };
        try stderr.print("\nERROR: {s} ({any})\n", .{ message, err });
        std.os.exit(1);
    } orelse {
        try stderr.print("\nBye now.\n", .{});
        std.os.exit(0);
    };
}

test "simple test" {}
