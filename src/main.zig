const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();
const File = std.fs.File;

const Token = @import("tokens.zig").Token;

const interpret = @import("interpret.zig");
const RockError = interpret.RockError;
const RockDictionary = interpret.Dictionary;
const RockCommand = interpret.RockCommand;
const RockAction = interpret.RockAction;
const RockMachine = interpret.RockMachine;

const builtins = @import("builtins.zig");

pub const version = "0.8.0";

const stdlib = @embedFile("stdlib.dt");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    var machine = try RockMachine.init(arena.allocator());

    try builtins.defineAll(&machine);

    // TODO: Can this be done at comptime somehow?
    var toks = Token.parse(stdlib);
    while (toks.next()) |token| try machine.interpret(token);

    if (!std.io.getStdIn().isTty()) {
        return handlePipedStdin(&machine);
    } else if (!std.io.getStdOut().isTty()) {
        return handlePipedStdoutOnly(&machine);
    } else if (try readShebangFile(arena.allocator())) |fileContents| {
        toks = Token.parse(fileContents);
        while (toks.next()) |token| try machine.interpret(token);
        return;
    }

    return readEvalPrintLoop(&machine);
}

fn handlePipedStdin(machine: *RockMachine) !void {
    machine.interpret(.{ .term = "pipe-thru-args" }) catch |e| {
        if (e == error.BrokenPipe) return;
        try stderr.print("RIP: {any}\n", .{e});
        std.os.exit(1);
    };
}

fn handlePipedStdoutOnly(machine: *RockMachine) !void {
    machine.interpret(.{ .term = "run-args" }) catch |e| {
        if (e == error.BrokenPipe) return;
        try stderr.print("RIP: {any}\n", .{e});
        std.os.exit(1);
    };
}

fn readEvalPrintLoop(machine: *RockMachine) !void {
    machine.interpret(.{ .term = "run-args" }) catch |e| {
        try stderr.print("RIP: {any}\n", .{e});
        std.os.exit(1);
    };

    while (true) machine.interpret(.{ .term = "main-repl" }) catch |e| if (e == error.EndOfStream) {
        try stderr.print("\nSee you next time.\n", .{});
        return;
    };
}

fn readShebangFile(allocator: Allocator) !?[]const u8 {
    var args = std.process.args();
    _ = args.skip();

    if (args.next()) |arg| {
        const file = std.fs.openFileAbsolute(arg, .{}) catch return null;
        defer file.close();

        const contents = try file.readToEndAlloc(allocator, std.math.pow(usize, 2, 16));

        if (std.mem.startsWith(u8, contents, "#!")) {
            return contents;
        }
    }

    return null;
}
