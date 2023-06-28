const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Stack = std.SinglyLinkedList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const interpret = @import("interpret.zig");
const Quote = interpret.Quote;
const RockError = interpret.Error;
const RockVal = interpret.RockVal;
const RockMachine = interpret.RockMachine;

pub fn defineAll(machine: *RockMachine) !void {
    try machine.define(".q", "quit", .{ .builtin = quit });

    try machine.define("def", "define a new command", .{ .builtin = def });
    try machine.define(":", "bind variables", .{ .builtin = colon });

    try machine.define("do", "execute a command or quote", .{ .builtin = do });
    try machine.define("?", "consumes a command/quote and a boolean and performs the quote if the boolean is true", .{ .builtin = opt });

    try machine.define("dup", "duplicate the top value", .{ .builtin = dup });
    try machine.define("drop", "drop the top value", .{ .builtin = drop });
    try machine.define("swap", "swap the top two values", .{ .builtin = swap });
    try machine.define("rot", "rotate the top three values", .{ .builtin = rot });

    try machine.define("p", "print a value", .{ .builtin = p });
    try machine.define("nl", "print a newline", .{ .builtin = nl });
    try machine.define(".s", "print the stack", .{ .builtin = dotS });

    try machine.define("+", "add two numeric values", .{ .builtin = add });
    try machine.define("-", "subtract two numeric values", .{ .builtin = subtract });
    try machine.define("*", "multiply two numeric values", .{ .builtin = multiply });
    try machine.define("/", "divide two numeric values", .{ .builtin = divide });
    try machine.define("%", "modulo two numeric values", .{ .builtin = modulo });
    try machine.define("abs", "consume a number and produce its absolute value", .{ .builtin = abs });

    try machine.define("and", "consume two booleans and produce their logical and", .{ .builtin = boolAnd });
    try machine.define("or", "consume two booleans and produce their logical or", .{ .builtin = boolOr });
    try machine.define("not", "consume a booleans and produce its logical not", .{ .builtin = not });

    try machine.define("map", "apply a command to all values in a quote", .{ .builtin = map });
    try machine.define("filter", "only keep values in that pass a predicate in a quote", .{ .builtin = filter });

    try machine.define("...", "expand a quote", .{ .builtin = ellipsis });
    try machine.define("push", "move an item into a quote", .{ .builtin = push });
    try machine.define("pop", "move the last item of a quote to top of stack", .{ .builtin = pop });
    try machine.define("enq", "move an item into the first position of a quote", .{ .builtin = enq });
    try machine.define("deq", "remove an item from the first position of a quote", .{ .builtin = deq });
}

pub fn quit(state: *RockMachine) !void {
    const ctx = try state.popContext();

    if (ctx.items.len > 0) {
        try stderr.print("Warning: Exited with unused values: [ ", .{});
        for (ctx.items) |item| {
            try item.print();
            try stderr.print(" ", .{});
        }
        try stderr.print("] \n", .{});
    }

    std.os.exit(0);
}

pub fn def(state: *RockMachine) !void {
    const usage = "USAGE: quote term def ({any})\n";

    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const quote = vals[0].asQuote();
    const name = vals[1].asCommand() orelse vals[1].asDeferredCommand();

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
        const term = terms.asCommand() orelse terms.asDeferredCommand();
        if (term != null) {
            const val = try state.pop();
            var quote = ArrayList(RockVal).init(state.alloc);
            try quote.append(val);
            try state.define(term.?, term.?, .{ .quote = quote });
            return;
        }
    }

    // Multiple terms
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

pub fn p(state: *RockMachine) !void {
    const val = try state.pop();

    switch (val) {
        .string => |s| try stdout.print("{s}", .{s}),
        else => {
            try val.print();
        },
    }
}

pub fn nl(state: *RockMachine) !void {
    _ = state;
    try stdout.print("\n", .{});
}

pub fn dotS(state: *RockMachine) !void {
    try stdout.print("[ ", .{});

    var top = state.nest.first orelse return;

    for (top.data.items) |val| {
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

pub fn boolAnd(state: *RockMachine) !void {
    const usage = "USAGE: a b and -> a&b ({any})\n";

    var vals = try state.popN(2);

    var a = vals[0].asBool();
    var b = vals[1].asBool();

    if (a != null and b != null) {
        try state.push(.{ .bool = a.? and b.? });
    } else {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    }
}

pub fn boolOr(state: *RockMachine) !void {
    const usage = "USAGE: a b or -> a|b ({any})\n";

    var vals = try state.popN(2);

    var a = vals[0].asBool();
    var b = vals[1].asBool();

    if (a != null and b != null) {
        try state.push(.{ .bool = a.? or b.? });
    } else {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    }
}

pub fn not(state: *RockMachine) !void {
    const usage = "USAGE: a b or -> a|b ({any})\n";

    var val = try state.pop();

    var a = val.asBool();

    if (a != null) {
        try state.push(.{ .bool = !a.? });
    } else {
        try state.push(val);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    }
}

pub fn opt(state: *RockMachine) !void {
    const usage = "USAGE: cmd|quote b opt -> ... ({any})\n";

    var val = try state.pop();

    var cond = val.asBool();

    if (cond != null) {
        if (cond.?) {
            try do(state);
        } else {
            try drop(state);
        }
    } else {
        try state.push(val);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    }
}

pub fn do(state: *RockMachine) !void {
    const usage = "USAGE: ... term|quote do -> ... ({any})\n";

    var toDo = try state.pop();

    { // Command
        const cmd = toDo.asCommand() orelse toDo.asDeferredCommand();
        if (cmd != null) {
            try state.handleCmd(cmd.?);
            return;
        }
    }

    { // Quote
        const quote = toDo.asQuote();
        if (quote != null) {
            for (quote.?.items) |val| {
                try state.handle(val);
            }
            return;
        }
    }

    const err = RockError.WrongArguments;
    try stderr.print(usage, .{err});
    try state.push(toDo);
    return err;
}

pub fn map(state: *RockMachine) !void {
    const usage = "USAGE: [as] (a->b) map -> [bs] ({any})\n";

    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const quote = vals[0].asQuote();
    const f = vals[1];

    if (quote != null) {
        _map(state, quote.?, f) catch |err| {
            try stderr.print(usage, .{err});
            try state.pushN(2, vals);
        };
    } else {
        try stderr.print(usage, .{RockError.WrongArguments});
        try state.pushN(2, vals);
    }
}

fn _map(state: *RockMachine, as: Quote, f: RockVal) !void {
    var child = try state.child();

    for (as.items) |a| {
        try child.push(a);
        try child.push(f);
        try do(&child);
    }

    const newQuote = try child.popContext();

    try state.push(RockVal{ .quote = newQuote });
}

pub fn filter(state: *RockMachine) !void {
    const usage = "USAGE: [as] (a->bool) filter -> [bs] ({any})\n";

    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const quote = vals[0].asQuote();
    const f = vals[1];

    if (quote != null) {
        _filter(state, quote.?, f) catch |err| {
            try stderr.print(usage, .{err});
            try state.pushN(2, vals);
        };
    } else {
        try stderr.print(usage, .{RockError.WrongArguments});
        try state.pushN(2, vals);
    }
}

fn _filter(state: *RockMachine, as: Quote, f: RockVal) !void {
    var quote = Quote.init(state.alloc);

    for (as.items) |a| {
        var child = try state.child();
        try child.push(a);
        try child.push(f);
        try do(&child);
        var lastVal = try child.pop();
        var cond = lastVal.asBool();

        if (cond != null and cond.?) {
            try quote.append(a);
        }
    }

    try state.push(RockVal{ .quote = quote });
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
