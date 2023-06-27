const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Stack = std.SinglyLinkedList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const interpret = @import("interpret.zig");
const RockError = interpret.Error;
const RockVal = interpret.RockVal;
const RockMachine = interpret.RockMachine;

pub fn def(state: *RockMachine) !void {
    const usage = "USAGE: quote term def ({any})\n";

    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const quote = vals[0].asQuote();
    const name = vals[1].asCommand();

    if (name == null or quote == null) {
        try stderr.print(usage, .{.{ name, quote }});
        try state.pushN(2, vals);
        return RockError.WrongArguments;
    }

    try state.define(name.?, "TODO", .{ .quote = quote.? });
}

// Variable binding
pub fn colon(state: *RockMachine) !void {
    const usage = "USAGE: ...vals terms : ({any})\n";
    _ = usage;

    var terms = try state.pop();

    { // Single term
        const term = terms.asCommand();
        if (term != null) {
            const val = try state.pop();
            var quote = ArrayList(RockVal).init(state.alloc);
            try quote.append(val);
            try state.define(term.?, term.?, .{ .quote = quote });
            return;
        }
    }

    // Assume multiple terms
    for (terms.asQuote().?.items) |termVal| {
        const term = termVal.asCommand();
        const val = try state.pop();
        var quote = ArrayList(RockVal).init(state.alloc);
        try quote.append(val);
        try state.define(term.?, term.?, .{ .quote = quote });
    }
}

pub fn dup(state: *RockMachine) !void {
    const val = try state.pop();
    try state.push(val);
    try state.push(val);
}

pub fn drop(state: *RockMachine) !void {
    _ = try state.pop();
}

pub fn swap(state: *RockMachine) !void {
    const vals = try state.popN(2);
    try state.push(vals[1]);
    try state.push(vals[0]);
}

// ... a b c (rot) ... c a b
//   [ 0 1 2 ]       [ 2 0 1 ]
pub fn rot(state: *RockMachine) !void {
    const vals = try state.popN(3);
    try state.push(vals[2]);
    try state.push(vals[0]);
    try state.push(vals[1]);
}

pub fn pl(state: *RockMachine) !void {
    const val = try state.pop();

    switch (val) {
        .string => |s| try stdout.print("{s}\n", .{s}),
        else => {
            try val.print();
            try stdout.print("\n", .{});
        },
    }
}

pub fn dotS(state: *RockMachine) !void {
    try stdout.print("[ ", .{});

    var top = state.nest.first orelse return;

    for (top.data.stack.items) |val| {
        try val.print();
        try stdout.print(" ", .{});
    }

    try stdout.print("]\n", .{});
}

pub fn add(state: *RockMachine) !void {
    const usage = "USAGE: a b + -> a+b ({any})\n";

    const ns = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    { // Both integers
        const a = ns[0].asI64();
        const b = ns[1].asI64();

        if (a != null and b != null) {
            try state.push(.{ .i64 = a.? + b.? });
            return;
        }
    }

    { // Both floats
        const a = ns[0].asF64();
        const b = ns[1].asF64();

        if (a != null and b != null) {
            try state.push(.{ .f64 = a.? + b.? });
            return;
        }
    }

    try state.pushN(2, ns);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn subtract(state: *RockMachine) !void {
    const usage = "USAGE: a b - -> a+b ({any})\n";

    const ns = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    { // Both integers
        const a = ns[0].asI64();
        const b = ns[1].asI64();

        if (a != null and b != null) {
            try state.push(.{ .i64 = a.? - b.? });
            return;
        }
    }

    { // Both floats
        const a = ns[0].asF64();
        const b = ns[1].asF64();

        if (a != null and b != null) {
            try state.push(.{ .f64 = a.? - b.? });
            return;
        }
    }

    try state.pushN(2, ns);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn multiply(state: *RockMachine) !void {
    const usage = "USAGE: a b * -> a*b ({any})\n";

    const ns = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    { // Both integers
        const a = ns[0].asI64();
        const b = ns[1].asI64();

        if (a != null and b != null) {
            try state.push(.{ .i64 = a.? * b.? });
            return;
        }
    }

    { // Both floats
        const a = ns[0].asF64();
        const b = ns[1].asF64();

        if (a != null and b != null) {
            try state.push(.{ .f64 = a.? * b.? });
            return;
        }
    }

    try state.pushN(2, ns);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn divide(state: *RockMachine) !void {
    const usage = "USAGE: a b / -> a/b ({any})\n";

    const ns = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    { // Both integers
        const a = ns[0].asI64();
        const b = ns[1].asI64();

        if (a != null and b != null) {
            try state.push(.{ .i64 = @divTrunc(a.?, b.?) });
            return;
        }
    }

    { // Both floats
        const a = ns[0].asF64();
        const b = ns[1].asF64();

        if (a != null and b != null) {
            try state.push(.{ .f64 = a.? / b.? });
            return;
        }
    }

    try state.pushN(2, ns);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn modulo(state: *RockMachine) !void {
    const usage = "USAGE: a b % -> a%b ({any})\n";

    const ns = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    { // Both integers
        const a = ns[0].asI64();
        const b = ns[1].asI64();

        if (a != null and b != null) {
            try state.push(.{ .i64 = @mod(a.?, b.?) });
            return;
        }
    }

    { // Both floats
        const a = ns[0].asF64();
        const b = ns[1].asF64();

        if (a != null and b != null) {
            try state.push(.{ .f64 = @mod(a.?, b.?) });
            return;
        }
    }

    try state.pushN(2, ns);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn abs(state: *RockMachine) !void {
    const usage = "USAGE: n abs -> |n| ({any})\n";

    const n = try state.pop();

    { // Integers
        const a = n.asI64();

        if (a != null) {
            try state.push(.{ .i64 = try std.math.absInt(a.?) });
            return;
        }
    }

    { // Both floats
        const a = n.asF64();

        if (a != null) {
            try state.push(.{ .f64 = std.math.fabs(a.?) });
            return;
        }
    }

    try state.push(n);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn map(state: *RockMachine) !void {
    const usage = "USAGE: [as] term(a->b) map -> [bs] ({any})\n";

    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const quote = vals[0].asQuote();
    const f = vals[1].asCommand();

    if (quote != null and f != null) {
        var as = quote.?;

        var newQuote = ArrayList(RockVal).init(state.alloc);

        for (as.items) |a| {
            try state.push(a);
            try state.handleCmd(f.?); // This should be performed in a new context where only the first item is present
            var newVal = try state.pop();
            try newQuote.append(newVal);
        }

        try state.push(RockVal{ .quote = newQuote });
    }
}

pub fn pop(state: *RockMachine) !void {
    const val = try state.pop();
    var quote: ArrayList(RockVal) = val.asQuote() orelse {
        return RockError.WrongArguments;
    };

    if (quote.items.len > 0) {
        const lastVal = quote.pop();
        try state.push(RockVal{ .quote = quote });
        try state.push(lastVal);
        return;
    }

    try state.push(val);
}

pub fn push(state: *RockMachine) !void {
    const vals = try state.popN(2);

    var pushMe = vals[1];
    var quote: ArrayList(RockVal) = vals[0].asQuote() orelse {
        try state.pushN(2, vals);
        return RockError.WrongArguments;
    };

    try quote.append(pushMe);
    try state.push(RockVal{ .quote = quote });
}

pub fn enq(state: *RockMachine) !void {
    const vals = try state.popN(2);

    var pushMe = vals[0];
    var quote: ArrayList(RockVal) = vals[1].asQuote() orelse {
        try state.pushN(2, vals);
        return RockError.WrongArguments;
    };

    try quote.insert(0, pushMe);
    try state.push(RockVal{ .quote = quote });
}

pub fn deq(state: *RockMachine) !void {
    const val = try state.pop();
    var quote: ArrayList(RockVal) = val.asQuote() orelse {
        return RockError.WrongArguments;
    };

    if (quote.items.len > 0) {
        const firstVal = quote.orderedRemove(0);
        try state.push(firstVal);
        try state.push(RockVal{ .quote = quote });
        return;
    }

    try state.push(val);
}

pub fn ellipsis(state: *RockMachine) !void {
    const val = try state.pop();
    var quote: ArrayList(RockVal) = val.asQuote() orelse {
        return RockError.WrongArguments;
    };

    // TODO: Push as slice
    for (quote.items) |v| {
        try state.push(v);
    }
}
