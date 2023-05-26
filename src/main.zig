const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();
const Token = @import("tokens.zig").Token;

const version = "0.1";

const helloFile = @embedFile("test/hello.rock");

pub fn main() !void {
    var alloc = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    try stderr.print("rock {s}\n", .{version});

    const helloTokens = try Token.parseAlloc(alloc.allocator(), helloFile);
    for (helloTokens.items) |token| {
        try stderr.print("Token: {any}\n", .{token});
    }

    var stop = false;
    while (!stop) {
        try stdout.print("> ", .{});

        const input = try prompt(alloc.allocator());

        const tokens = try Token.parseAlloc(alloc.allocator(), input);
        for (tokens.items) |token| {
            try stderr.print("Token: {any}\n", .{token});
        }
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
