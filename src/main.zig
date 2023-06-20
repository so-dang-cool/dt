const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const Token = @import("tokens.zig").Token;

const interpret = @import("interpret.zig");
const RockError = interpret.RockError;
const RockDictionary = interpret.Dictionary;
const RockCommand = interpret.RockCommand;
const RockAction = interpret.RockAction;
const RockMachine = interpret.RockMachine;

const builtins = @import("builtins.zig");

const version = "0.1";

const helloFile = @embedFile("test/hello.rock");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    var machine = try RockMachine.init(arena.allocator());

    try machine.define("def", "define a new command", .{ .builtin = builtins.def });
    try machine.define("pl", "print a value and a newline", .{ .builtin = builtins.pl });
    try machine.define("+", "add two numeric values", .{ .builtin = builtins.add });

    try stderr.print("rock {s}\n", .{version});

    const helloTokens = try Token.parseAlloc(arena.allocator(), helloFile);
    defer helloTokens.deinit();
    for (helloTokens.items) |token| {
        // try stderr.print("Token: {any}\n", .{token});
        try machine.interpret(token);
        // try stderr.print("STATE: {any}\n\n", .{machine.curr.});
    }

    var stop = false;
    while (!stop) {
        try stdout.print("> ", .{});

        const input = try prompt(arena.allocator());

        const tokens = try Token.parseAlloc(arena.allocator(), input);
        for (tokens.items) |token| {
            //try stderr.print("Token: {any}\n", .{token});
            machine.interpret(token) catch |e| {
                switch (token) {
                    .term => |term| try stderr.print("OOPS {s} caused error: {any}\n", .{ term, e }),
                    else => try stderr.print("OOPS {any} caused error: {any}\n", .{ token, e }),
                }
                break;
            };
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
