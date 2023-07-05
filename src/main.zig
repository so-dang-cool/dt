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

pub const version = "0.9.0";

const stdlib = @embedFile("stdlib.dt");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    var machine = try RockMachine.init(arena.allocator());

    try builtins.defineAll(&machine);

    try loadStdlib(arena.allocator(), &machine);

    if (try readShebangFile(arena.allocator())) |fileContents| {
        var toks = Token.parse(arena.allocator(), fileContents);
        return while (try toks.next()) |token| try machine.interpret(token);
    } else if (!std.io.getStdIn().isTty()) {
        return handlePipedStdin(&machine);
    } else if (!std.io.getStdOut().isTty()) {
        return handlePipedStdoutOnly(&machine);
    }

    return readEvalPrintLoop(&machine);
}

// TODO: Can this be done at comptime somehow?
fn loadStdlib(allocator: Allocator, machine: *RockMachine) !void {
    var toks = Token.parse(allocator, stdlib);
    while (try toks.next()) |token| try machine.interpret(token);
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

    while (true) machine.interpret(.{ .term = "main-repl" }) catch |e| switch (e) {
        error.EndOfStream => {
            try stderr.print("\nSee you next time.\n", .{});
            return;
        },
        else => try stderr.print("Recovering from: {any}\n", .{e}),
    };
}

fn readShebangFile(allocator: Allocator) !?[]const u8 {
    var args = std.process.args();
    _ = args.skip();

    if (args.next()) |maybeFilepath| {
        // We get a Dir from CWD so we can resolve relative paths
        const theCwdPath = try std.process.getCwdAlloc(allocator);
        var theCwd = try std.fs.openDirAbsolute(theCwdPath, .{});

        const file = theCwd.openFile(maybeFilepath, .{}) catch return null;
        defer file.close();

        const contents = try file.readToEndAlloc(allocator, std.math.pow(usize, 2, 16));

        if (std.mem.startsWith(u8, contents, "#!")) {
            return contents;
        }
    }

    return null;
}
