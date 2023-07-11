const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();
const File = std.fs.File;

const Token = @import("tokens.zig").Token;

const interpret = @import("interpret.zig");
const DtError = interpret.DtError;
const DtDictionary = interpret.Dictionary;
const DtCommand = interpret.Command;
const DtAction = interpret.Action;
const DtMachine = interpret.DtMachine;

const builtins = @import("builtins.zig");

// TODO: Change to @import when it's supported for zon
pub const version = "0.11.2"; // Update in build.zig.zon as well.

const stdlib = @embedFile("stdlib.dt");
const dtlib = @embedFile("dt.dt");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    var machine = try DtMachine.init(arena.allocator());

    try builtins.defineAll(&machine);
    try machine.loadFile(stdlib);
    try machine.loadFile(dtlib);

    const stdinPiped = !std.io.getStdIn().isTty();
    const stdoutPiped = !std.io.getStdOut().isTty();

    if (try readShebangFile(arena.allocator())) |fileContents| {
        return machine.loadFile(fileContents) catch |e| return doneOrDie(&machine, e);
    } else if (stdinPiped) {
        return handlePipedStdin(&machine);
    } else if (stdoutPiped) {
        return handlePipedStdoutOnly(&machine);
    }

    return readEvalPrintLoop(&machine);
}

fn handlePipedStdin(dt: *DtMachine) !void {
    dt.handleCmd("pipe-thru-args") catch |e| return doneOrDie(dt, e);
}

fn handlePipedStdoutOnly(dt: *DtMachine) !void {
    dt.handleCmd("run-args") catch |e| return doneOrDie(dt, e);
}

fn readEvalPrintLoop(dt: *DtMachine) !void {
    dt.handleCmd("run-args") catch |e| return doneOrDie(dt, e);

    // TODO: Can this catch be done in the stdlib? Other people need to catch errors too!
    while (true) dt.handleCmd("main-repl") catch |e| switch (e) {
        error.EndOfStream => {
            try stderr.print("\n", .{});
            return;
        },
        else => {
            try dt.red();
            try stderr.print("\nRestarting REPL after error: {s}\n\n", .{@errorName(e)});
            try dt.norm();
        },
    };
}

fn doneOrDie(dt: *DtMachine, reason: anyerror) !void {
    try stderr.print("\n", .{});
    switch (reason) {
        error.EndOfStream => return,
        error.BrokenPipe => return,
        else => {
            try dt.red();
            try stderr.print("RIP: {any}\n", .{reason});
            try dt.norm();

            std.os.exit(1);
        },
    }
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

test {
    std.testing.refAllDecls(@This());
}
