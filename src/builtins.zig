const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const interpret = @import("interpret.zig");
const RockError = interpret.Error;
const RockMachine = interpret.RockMachine;

pub fn def(state: *RockMachine) !void {
    const usage = "USAGE: QUOTE TERM def ({any})\n";

    const vals = state.pop2() catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const name = vals.a.asCommand();
    const quote = vals.b.asQuote();

    if (name == null or quote == null) {
        try stderr.print(usage, .{.{ name, quote }});
        try state.push2(vals);
        return RockError.WrongArguments;
    }

    try state.define(name.?, "TODO", .{ .quote = quote.? });
}

pub fn pl(state: *RockMachine) !void {
    const val = try state.pop();
    try val.print();
    try stdout.print("\n", .{});
}

pub fn add(state: *RockMachine) !void {
    const usage = "USAGE: a b + -> a+b ({any})\n";

    const ns = state.pop2() catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    { // Both integers
        const a = ns.a.asI64();
        const b = ns.b.asI64();

        if (a != null and b != null) {
            try state.push(.{ .i64 = a.? + b.? });
            return;
        }
    }

    { // Both floats
        const a = ns.a.asF64();
        const b = ns.b.asF64();

        if (a != null and b != null) {
            try state.push(.{ .f64 = a.? + b.? });
            return;
        }
    }
}
