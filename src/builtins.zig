const std = @import("std");
const Allocator = std.mem.Allocator;
const Stack = std.SinglyLinkedList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const interpret = @import("interpret.zig");
const RockError = interpret.Error;
const RockVal = interpret.RockVal;
const RockMachine = interpret.RockMachine;

pub fn def(state: *RockMachine) !void {
    const usage = "USAGE: QUOTE TERM def ({any})\n";

    const vals = state.pop2() catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const name = vals.b.asCommand();
    const quote = vals.a.asQuote();

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

pub fn dotS(state: *RockMachine) !void {
    try stdout.print("[ ", .{});

    var valNode = state.nest.first.?.data.stack.first;
    var printOrder = Stack(RockVal){};

    while (valNode) |node| : (valNode = node.next) {
        try node.data.print();
        var printNode = Stack(RockVal).Node{ .data = node.data, .next = null };
        printOrder.prepend(&printNode);
    }

    var printme = printOrder.first;

    while (printme) |node| : (printme = node.next) {
        try node.data.print();
    }

    try stdout.print("]\n", .{});
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

pub fn subtract(state: *RockMachine) !void {
    const usage = "USAGE: a b - -> a+b ({any})\n";

    const ns = state.pop2() catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    { // Both integers
        const a = ns.a.asI64();
        const b = ns.b.asI64();

        if (a != null and b != null) {
            try state.push(.{ .i64 = a.? - b.? });
            return;
        }
    }

    { // Both floats
        const a = ns.a.asF64();
        const b = ns.b.asF64();

        if (a != null and b != null) {
            try state.push(.{ .f64 = a.? - b.? });
            return;
        }
    }
}
