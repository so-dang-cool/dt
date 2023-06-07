const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const Token = @import("tokens.zig").Token;

const interpret = @import("interpret.zig");
const RockError = interpret.RockError;
const RockDictionary = interpret.RockDictionary;
const RockCommand = interpret.RockCommand;
const RockAction = interpret.RockAction;
const RockMachine = interpret.RockMachine;

const builtins = @import("builtins.zig");

const version = "0.1";

const helloFile = @embedFile("test/hello.rock");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    var dict = RockDictionary.init(arena.allocator());
    try dict.put("def", RockCommand{ .name = "def", .description = "define a new command", .action = RockAction{ .builtin = builtins.def } });
    try dict.put("pl", RockCommand{ .name = "pl", .description = "print a value and a newline", .action = RockAction{ .builtin = builtins.pl } });

    var machine = try RockMachine.init(dict);

    try stderr.print("rock {s}\n", .{version});

    const helloTokens = try Token.parseAlloc(arena.allocator(), helloFile);
    defer helloTokens.deinit();
    for (helloTokens.items) |token| {
        try stderr.print("Token: {any}\n", .{token});
        machine = try machine.interpret(token);
        // try stderr.print("STATE: {any}\n\n", .{machine.curr.});
    }

    var stop = false;
    while (!stop) {
        try stdout.print("> ", .{});

        const input = try prompt(arena.allocator());

        const tokens = try Token.parseAlloc(arena.allocator(), input);
        for (tokens.items) |token| {
            try stderr.print("Token: {any}\n", .{token});
            machine = machine.interpret(token) catch |e| {
                try stderr.print("OOPS {any} caused error: {any}\n", .{ token, e });
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
