const std = @import("std");
const Allocator = std.mem.Allocator;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const builtins = @import("builtins.zig");

const String = @import("string.zig").String;

const interpret = @import("interpret.zig");
const DtMachine = interpret.DtMachine;

// TODO: Change to @import when it's supported for zon
pub const version = "1.2.5"; // Update in build.zig.zon as well.

const stdlib = @embedFile("stdlib.dt");
const dtlib = @embedFile("dt.dt");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{
        // .never_unmap = true,
    }){};
    var allocator = gpa.allocator();
    // var arena = std.heap.ArenaAllocator{ .child_allocator = std.heap.page_allocator };
    // var allocator = arena.allocator();

    var machine = try DtMachine.init(allocator);

    try builtins.defineAll(&machine);
    try machine.loadFile(stdlib);
    try machine.loadFile(dtlib);

    const stdinPiped = !std.io.getStdIn().isTty();
    const stdoutPiped = !std.io.getStdOut().isTty();

    const firstArgMaybe = try readFirstArg(allocator);

    if (firstArgMaybe) |firstArg| {
        if (try readShebangFile(allocator, firstArg)) |fileContents| {
            return machine.loadFile(fileContents) catch |e| return doneOrDie(&machine, e);
        } else if ((std.mem.eql(u8, firstArg, "stream") or std.mem.startsWith(u8, firstArg, "stream ")) and (stdinPiped or stdoutPiped)) {
            return handlePipedStdoutOnly(&machine);
        }
    }

    if (stdinPiped) {
        return handlePipedStdin(&machine);
    } else if (stdoutPiped) {
        return handlePipedStdoutOnly(&machine);
    }

    return readEvalPrintLoop(&machine);
}

fn readFirstArg(allocator: Allocator) !?[]const u8 {
    var args = try std.process.argsWithAllocator(allocator);
    _ = args.skip(); // Discard process name
    return if (args.next()) |arg| try allocator.dupe(u8, arg) else null;
}

fn handlePipedStdin(dt: *DtMachine) !void {
    dt.handleCmd(try String.ofAlloc("dt/pipe-thru-args", dt.alloc)) catch |e| return doneOrDie(dt, e);
}

fn handlePipedStdoutOnly(dt: *DtMachine) !void {
    dt.handleCmd(try String.ofAlloc("dt/run-args", dt.alloc)) catch |e| return doneOrDie(dt, e);
}

fn readEvalPrintLoop(dt: *DtMachine) !void {
    dt.handleCmd(try String.ofAlloc("dt/run-args", dt.alloc)) catch |e| return doneOrDie(dt, e);

    // TODO: Can this catch be done in the stdlib? Other people need to catch errors too!
    while (true) dt.handleCmd(try String.ofAlloc("dt/main-repl", dt.alloc)) catch |e| switch (e) {
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

fn readShebangFile(allocator: Allocator, maybeFilepath: []const u8) !?[]const u8 {
    // We get a Dir from CWD so we can resolve relative paths
    const theCwdPath = try std.process.getCwdAlloc(allocator);
    var theCwd = try std.fs.openDirAbsolute(theCwdPath, .{});

    const file = theCwd.openFile(maybeFilepath, .{}) catch return null;
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, std.math.pow(usize, 2, 16));

    if (std.mem.startsWith(u8, contents, "#!")) {
        return contents;
    }

    return null;
}

test {
    std.testing.refAllDecls(@This());
    _ = @import("tests/bool_tests.zig");
    _ = @import("tests/dt_args_basic.zig");
    _ = @import("tests/dtsh_run_basic.zig");
    _ = @import("tests/def_scope_tests.zig");
    _ = @import("tests/project_euler_tests.zig");
    _ = @import("tests/dtsh_interactive_basic.zig");
}
