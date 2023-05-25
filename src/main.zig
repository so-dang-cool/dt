const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const version = "0.1";

const helloFile = @embedFile("test/hello.rock");

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

    fn parseAlloc(alloc: Allocator, raw: []const u8) !ArrayList(Token) {
        var tokens = ArrayList(Token).init(alloc);

        var parts = std.mem.split(u8, raw, " ");

        while (parts.next()) |part| {
            const token: Token = if (std.mem.eql(u8, part, "["))
                .left_bracket
            else if (std.mem.eql(u8, part, "]"))
                .right_bracket
            else if (std.mem.eql(u8, part, "true")) .{ .bool = true } else if (std.mem.eql(u8, part, "false")) .{ .bool = false } else .{ .term = part };
            try tokens.append(token);
        }

        return tokens;
    }
};

pub fn main() !void {
    var stop = false;
    var alloc = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    try stderr.print("rock {s}\n", .{version});

    const tokens = try Token.parseAlloc(alloc.allocator(), helloFile);
    for (tokens.items) |token| {
        try stderr.print("Token: {any}\n", .{token});
    }

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
