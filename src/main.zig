const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const Token = @import("tokens.zig").Token;

const interpret = @import("interpret.zig");
const RockDictionary = interpret.RockDictionary;
const RockCommand = interpret.RockCommand;
const RockAction = interpret.RockAction;
const RockMachine = interpret.RockMachine;
const RockStack = interpret.RockStack;
const RockNode = interpret.RockNode;

const version = "0.1";

const helloFile = @embedFile("test/hello.rock");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    var dict = RockDictionary.init(gpa.allocator());
    defer dict.deinit();
    try dict.put("def", RockCommand{ .name = "def", .description = "define a new command", .action = RockAction{ .builtin = def } });

    var machine = try RockMachine.init(gpa.allocator(), dict);

    try stderr.print("rock {s}\n", .{version});

    const helloTokens = try Token.parseAlloc(gpa.allocator(), helloFile);
    defer helloTokens.deinit();
    for (helloTokens.items) |token| {
        try stderr.print("Token: {any}\n", .{token});
        machine = try machine.interpret(token);
        // try stderr.print("STATE: {any}\n\n", .{machine.curr.});
    }

    var stop = false;
    while (!stop) {
        try stdout.print("> ", .{});

        const input = try prompt(gpa.allocator());

        const tokens = try Token.parseAlloc(gpa.allocator(), input);
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

fn def(state: *RockMachine) !RockMachine {
    const nameVal = (state.curr.popFirst() orelse {
        try stderr.print("USAGE: QUOTE TERM/STRING def\n", .{});
        return state.*;
    }).data;
    const name = switch (nameVal) {
        .command => |c| c,
        .string => |s| s,
        else => {
            state.push(nameVal);
            return state.*;
        },
    };

    const cmdVal = (state.curr.popFirst() orelse {
        try stderr.print("USAGE: QUOTE TERM/STRING def\n", .{});
        state.push(nameVal);
        return state.*;
    }).data;
    const cmd = switch (cmdVal) {
        .quote => |q| q,
        else => {
            try stderr.print("USAGE: QUOTE TERM/STRING def\n", .{});
            state.push(cmdVal);
            state.push(nameVal);
            return state.*;
        },
    };

    try state.dictionary.put(name, RockCommand{
        .name = name,
        .description = "TODO",
        .action = RockAction{
            .quote = cmd.*,
        },
    });

    return state.*;
}
