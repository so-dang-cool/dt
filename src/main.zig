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

pub const version = "0.1.1";

const stdlib = @embedFile("stdlib.dt");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    var machine = try RockMachine.init(arena.allocator());

    try builtins.defineAll(&machine);

    var toks = Token.parse(stdlib);
    while (toks.next()) |token| try machine.interpret(token);

    if (std.io.getStdIn().isTty()) {
        // REPL
        machine.interpret(.{ .term = "run-args" }) catch |e| {
            try stderr.print("RIP: {any}\n", .{e});
            std.os.exit(1);
        };

        while (true) machine.interpret(.{ .term = "main-repl" }) catch |e| if (e == error.EndOfStream) {
            try stderr.print("\nSee you next time.\n", .{});
            return;
        };
    } else {
        // PIPE
        machine.interpret(.{ .term = "pipe-thru-args" }) catch |e| {
            try stderr.print("RIP: {any}\n", .{e});
        };
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
