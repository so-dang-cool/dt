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
    try machine.define("defs", "produce a quote of all definition names", .{ .builtin = defs });
    try machine.define("def?", "return true if a name is defined", .{ .builtin = isDef });
    try machine.define(":", "bind variables", .{ .builtin = colon });

    try machine.define("do", "execute a command or quote", .{ .builtin = do });
    try machine.define("doin", "execute a command or quote in a previous quote", .{ .builtin = doin });
    try machine.define("?", "consumes a command/quote and a value and performs it if the value is truthy", .{ .builtin = opt });

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

    try machine.define("eq?", "consume two values and return true if they are equal", .{ .builtin = eq });
    try machine.define("gt?", "consume two numbers and return true if the most recent is greater", .{ .builtin = greaterThan });
    try machine.define("gte?", "consume two numbers and return true if the most recent is greater", .{ .builtin = greaterThanEq });
    try machine.define("lt?", "consume two numbers and return true if the most recent is less", .{ .builtin = lessThan });
    try machine.define("lte?", "consume two numbers and return true if the most recent is less", .{ .builtin = lessThanEq });

    try machine.define("and", "consume two booleans and produce their logical and", .{ .builtin = boolAnd });
    try machine.define("or", "consume two booleans and produce their logical or", .{ .builtin = boolOr });
    try machine.define("not", "consume a booleans and produce its logical not", .{ .builtin = not });

    try machine.define("split", "consume a string and a delimiter, and produce a quote of the string split on all occurrences of the substring", .{ .builtin = split });
    try machine.define("join", "consume a quote of strings and a delimiter, and produce a string with the delimiter interspersed", .{ .builtin = join });

    try machine.define("map", "apply a command to all values in a quote", .{ .builtin = map });
    try machine.define("filter", "only keep values in that pass a predicate in a quote", .{ .builtin = filter });

    try machine.define("...", "expand a quote", .{ .builtin = ellipsis });
    try machine.define("quote-all", "quote all current context", .{ .builtin = quoteAll });
    try machine.define("push", "move an item into a quote", .{ .builtin = push });
    try machine.define("pop", "move the last item of a quote to top of stack", .{ .builtin = pop });
    try machine.define("enq", "move an item into the first position of a quote", .{ .builtin = enq });
    try machine.define("deq", "remove an item from the first position of a quote", .{ .builtin = deq });

    try machine.define("to-bool", "coerce value to boolean", .{ .builtin = toBool });
    try machine.define("to-quote", "coerce value to quote", .{ .builtin = toQuote });
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

    const quote = try vals[0].intoQuote(state);
    const name = try vals[1].intoString(state);

    try state.define(name, name, .{ .quote = quote });
}

pub fn defs(state: *RockMachine) !void {
    const usage = "USAGE: defs -> [cmdnames...] ({any})\n";
    _ = usage;

    var quote = Quote.init(state.alloc);
    var defNames = state.defs.keyIterator();

    while (defNames.next()) |defName| {
        var cmdName = try state.alloc.dupe(u8, defName.*);
        try quote.append(.{ .string = cmdName });
    }

    try state.push(.{ .quote = quote });
}

pub fn isDef(state: *RockMachine) !void {
    const usage = "USAGE: term def? ({any})\n";

    const val = state.pop() catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const name = try val.intoString(state);

    try state.push(.{ .bool = state.defs.contains(name) });
}

// Variable binding
pub fn colon(state: *RockMachine) !void {
    const usage = "USAGE: ...vals term(s) : ({any})\n";

    var termVal = try state.pop();

    // Single term
    if (termVal.isCommand() or termVal.isDeferredCommand() or termVal.isString()) {
        const cmdName = try termVal.intoString(state);

        const val = state.pop() catch |e| {
            try stderr.print(usage, .{e});
            try state.push(termVal);
            return e;
        };

        var quote = ArrayList(RockVal).init(state.alloc);
        try quote.append(val);
        try state.define(cmdName, cmdName, .{ .quote = quote });
        return;
    }

    // Multiple terms

    var terms = (try termVal.intoQuote(state)).items;
    var vals = try state.alloc.alloc(RockVal, terms.len);

    var i = terms.len;

    while (i > 0) : (i -= 1) {
        vals[i - 1] = try state.pop();
    }

    for (terms, vals) |termV, val| {
        const term = try termV.intoString(state);
        var quote = ArrayList(RockVal).init(state.alloc);
        try quote.append(val);
        try state.define(term, term, .{ .quote = quote });
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

    if (ns[0].isI64() and ns[1].isI64()) {
        const a = try ns[0].intoI64();
        const b = try ns[1].intoI64();

        const res = @addWithOverflow(a, b);

        if (res[1] == 1) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Adding {} and {} would overflow.\n", .{ a, b });
            return RockError.IntegerOverflow;
        }

        try state.push(.{ .i64 = res[0] });
        return;
    }

    if (ns[0].isF64() or ns[1].isF64()) {
        const a = try ns[0].intoF64();
        const b = try ns[1].intoF64();

        try state.push(.{ .f64 = a + b });
        return;
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

    if (ns[0].isI64() and ns[1].isI64()) {
        const a = try ns[0].intoI64();
        const b = try ns[1].intoI64();

        const res = @subWithOverflow(a, b);

        if (res[1] == 1) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Subtracting {} from {} would overflow.\n", .{ b, a });
            return RockError.IntegerOverflow;
        }

        try state.push(.{ .i64 = res[0] });
        return;
    }

    if (ns[0].isF64() or ns[1].isF64()) {
        const a = try ns[0].intoF64();
        const b = try ns[1].intoF64();

        try state.push(.{ .f64 = a - b });
        return;
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

    if (ns[0].isI64() and ns[1].isI64()) {
        const a = try ns[0].intoI64();
        const b = try ns[1].intoI64();

        const res = @mulWithOverflow(a, b);

        if (res[1] == 1) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Multiplying {} by {} would overflow.\n", .{ a, b });
            return RockError.IntegerOverflow;
        }

        try state.push(.{ .i64 = res[0] });
        return;
    }

    if (ns[0].isF64() or ns[1].isF64()) {
        const a = try ns[0].intoF64();
        const b = try ns[1].intoF64();

        try state.push(.{ .f64 = a * b });
        return;
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

    if (ns[0].isI64() and ns[1].isI64()) {
        const a = try ns[0].intoI64();
        const b = try ns[1].intoI64();

        if (b == 0) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Cannot divide {} by zero.\n", .{a});
            return RockError.DivisionByZero;
        }

        try state.push(.{ .i64 = @divTrunc(a, b) });
        return;
    }

    if (ns[0].isF64() or ns[1].isF64()) {
        const a = try ns[0].intoF64();
        const b = try ns[1].intoF64();

        if (b == 0) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Cannot divide {} by zero.\n", .{a});
            return RockError.DivisionByZero;
        }

        try state.push(.{ .f64 = a / b });
        return;
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

    if (ns[0].isI64() and ns[1].isI64()) {
        const a = try ns[0].intoI64();
        const b = try ns[1].intoI64();

        try state.push(.{ .i64 = @mod(a, b) });
        return;
    }

    if (ns[0].isF64() or ns[1].isF64()) {
        const a = try ns[0].intoF64();
        const b = try ns[1].intoF64();

        try state.push(.{ .f64 = @mod(a, b) });
        return;
    }

    try state.pushN(2, ns);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn abs(state: *RockMachine) !void {
    const usage = "USAGE: n abs -> |n| ({any})\n";

    const n = try state.pop();

    if (n.isI64()) {
        const a = try n.intoI64();

        try state.push(.{ .i64 = try std.math.absInt(a) });
        return;
    }

    if (n.isF64()) {
        const a = try n.intoF64();

        try state.push(.{ .f64 = std.math.fabs(a) });
        return;
    }

    try state.push(n);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn eq(state: *RockMachine) !void {
    const usage = "USAGE: a b eq? -> bool ({any})\n";
    _ = usage;

    const vals = try state.popN(2);

    if (vals[0].isI64() and vals[1].isI64()) {
        const a = try vals[0].intoI64();
        const b = try vals[1].intoI64();

        try state.push(.{ .bool = a == b });
        return;
    }

    if (vals[0].isF64() and vals[1].isF64()) {
        const a = try vals[0].intoF64();
        const b = try vals[1].intoF64();

        try state.push(.{ .bool = a == b });
        return;
    }

    if (vals[0].isBool() and vals[1].isBool()) {
        const a = vals[0].intoBool(state);
        const b = vals[1].intoBool(state);

        try state.push(.{ .bool = a == b });
        return;
    }

    if (vals[0].isQuote() and vals[1].isQuote()) {
        const a = try vals[0].intoQuote(state);
        const b = try vals[1].intoQuote(state);

        const as: []RockVal = a.items;
        const bs: []RockVal = b.items;

        if (as.len != bs.len) {
            try state.push(.{ .bool = false });
            return;
        }

        var child = try state.child();

        for (as, 0..) |val, i| {
            try child.push(val);
            try child.push(bs[i]);
            try eq(&child);
            const bv = try child.pop();
            const res = bv.intoBool(state);
            if (!res) {
                try state.push(.{ .bool = false });
                return;
            }
        }

        try state.push(.{ .bool = true });
        return;
    }

    if (!vals[0].isQuote() and !vals[1].isQuote()) {
        const a = try vals[0].intoString(state);
        const b = try vals[1].intoString(state);

        try state.push(.{ .bool = std.mem.eql(u8, a, b) });
        return;
    }

    try state.push(.{ .bool = false });
}

pub fn greaterThan(state: *RockMachine) !void {
    const usage = "USAGE: a b gt? -> b>a ({any})\n";

    const vals = try state.popN(2);

    if (vals[0].isI64() and vals[1].isI64()) {
        const a = try vals[0].intoI64();
        const b = try vals[1].intoI64();

        try state.push(.{ .bool = b > a });
        return;
    }

    if (vals[0].isF64() and vals[1].isF64()) {
        const a = try vals[0].intoF64();
        const b = try vals[1].intoF64();

        try state.push(.{ .bool = b > a });
        return;
    }

    try state.pushN(2, vals);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn greaterThanEq(state: *RockMachine) !void {
    const usage = "USAGE: a b gt? -> b>a ({any})\n";

    const vals = try state.popN(2);

    if (vals[0].isI64() and vals[1].isI64()) {
        const a = try vals[0].intoI64();
        const b = try vals[1].intoI64();

        try state.push(.{ .bool = b >= a });
        return;
    }

    if (vals[0].isF64() and vals[1].isF64()) {
        const a = try vals[0].intoF64();
        const b = try vals[1].intoF64();

        try state.push(.{ .bool = b >= a });
        return;
    }

    try state.pushN(2, vals);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn lessThan(state: *RockMachine) !void {
    const usage = "USAGE: a b lt? -> b>a ({any})\n";

    const vals = try state.popN(2);

    if (vals[0].isI64() and vals[1].isI64()) {
        const a = try vals[0].intoI64();
        const b = try vals[1].intoI64();

        try state.push(.{ .bool = b < a });
        return;
    }

    if (vals[0].isF64() and vals[1].isF64()) {
        const a = try vals[0].intoF64();
        const b = try vals[1].intoF64();

        try state.push(.{ .bool = b < a });
        return;
    }

    try state.pushN(2, vals);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn lessThanEq(state: *RockMachine) !void {
    const usage = "USAGE: a b lte? -> b>a ({any})\n";

    const vals = try state.popN(2);

    if (vals[0].isI64() and vals[1].isI64()) {
        const a = try vals[0].intoI64();
        const b = try vals[1].intoI64();

        try state.push(.{ .bool = b <= a });
        return;
    }

    if (vals[0].isF64() and vals[1].isF64()) {
        const a = try vals[0].intoF64();
        const b = try vals[1].intoF64();

        try state.push(.{ .bool = b <= a });
        return;
    }

    try state.pushN(2, vals);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn boolAnd(state: *RockMachine) !void {
    var vals = try state.popN(2);

    var a = vals[0].intoBool(state);
    var b = vals[1].intoBool(state);

    try state.push(.{ .bool = a and b });
}

pub fn boolOr(state: *RockMachine) !void {
    var vals = try state.popN(2);

    var a = vals[0].intoBool(state);
    var b = vals[1].intoBool(state);

    try state.push(.{ .bool = a or b });
}

pub fn not(state: *RockMachine) !void {
    var val = try state.pop();

    var a = val.intoBool(state);
    try state.push(.{ .bool = !a });
}

pub fn split(state: *RockMachine) !void {
    const usage = "USAGE: str delim split -> [substrs...] ({any})\n";
    _ = usage;

    var vals = try state.popN(2);

    var str = try vals[0].intoString(state);
    var delim = try vals[1].intoString(state);

    if (delim.len > 0) {
        var parts = std.mem.split(u8, str, delim);
        var quote = Quote.init(state.alloc);
        while (parts.next()) |part| {
            try quote.append(.{ .string = part });
        }
        try state.push(.{ .quote = quote });
    } else {
        var quote = Quote.init(state.alloc);
        for (str) |c| {
            var s = try state.alloc.create([1]u8);
            s[0] = c;
            try quote.append(.{ .string = s });
        }
        try state.push(.{ .quote = quote });
    }
}

pub fn join(state: *RockMachine) !void {
    const usage = "USAGE: [strs...] delim join -> str ({any})\n";
    _ = usage;

    var vals = try state.popN(2);

    if (!vals[0].isQuote()) {
        const str = try vals[0].intoString(state);
        try state.push(.{ .string = str });
        return;
    }

    var strs = try vals[0].intoQuote(state);
    var delim = try vals[1].intoString(state);

    var parts = try ArrayList([]const u8).initCapacity(state.alloc, strs.items.len);
    for (strs.items) |part| {
        const s = try part.intoString(state);
        try parts.append(s);
    }
    var acc = try std.mem.join(state.alloc, delim, parts.items);
    try state.push(.{ .string = acc });
}

pub fn opt(state: *RockMachine) !void {
    var val = try state.pop();
    const cond = val.intoBool(state);

    if (cond) try do(state) else try drop(state);
}

pub fn do(state: *RockMachine) !void {
    const usage = "USAGE: ... term|quote do -> ... ({any})\n";
    _ = usage;

    var val = try state.pop();

    if (val.isCommand() or val.isDeferredCommand() or val.isString()) {
        const cmdName = try val.intoString(state);

        try state.handleCmd(cmdName);
        return;
    }

    const quote = try val.intoQuote(state);

    for (quote.items) |v| try state.handle(v);
}

pub fn doin(state: *RockMachine) !void {
    const usage = "USAGE: [as...] cmd|quote doin -> [bs...] ({any})\n";
    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const quote = try vals[0].intoQuote(state);
    const f = vals[1];

    _doin(state, quote, f) catch {
        try stderr.print(usage, .{RockError.WrongArguments});
        try state.pushN(2, vals);
    };
}

fn _doin(state: *RockMachine, quote: Quote, f: RockVal) !void {
    var child = try state.child();

    try child.push(.{ .quote = quote });
    try ellipsis(&child);
    try child.push(f);
    try do(&child);
    const resultQuote = try child.popContext();

    try state.push(.{ .quote = resultQuote });
}

pub fn map(state: *RockMachine) !void {
    const usage = "USAGE: [as] (a->b) map -> [bs] ({any})\n";

    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    if (!vals[0].isQuote()) {
        const err = RockError.WrongArguments;
        try stderr.print(usage, .{err});
        return err;
    }

    const quote = try vals[0].intoQuote(state);
    const f = vals[1];

    _map(state, quote, f) catch |err| {
        try stderr.print(usage, .{err});
        try state.pushN(2, vals);
    };
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

    if (!vals[0].isQuote()) {
        const err = RockError.WrongArguments;
        try stderr.print(usage, .{err});
        return err;
    }

    const quote = try vals[0].intoQuote(state);
    const f = vals[1];

    _filter(state, quote, f) catch |err| {
        try stderr.print(usage, .{err});
        try state.pushN(2, vals);
    };
}

fn _filter(state: *RockMachine, as: Quote, f: RockVal) !void {
    var quote = Quote.init(state.alloc);

    for (as.items) |a| {
        var child = try state.child();
        try child.push(a);
        try child.push(f);
        try do(&child);
        var lastVal = try child.pop();
        var cond = lastVal.intoBool(state);

        if (cond) {
            try quote.append(a);
        }
    }

    try state.push(RockVal{ .quote = quote });
}

pub fn pop(state: *RockMachine) !void {
    const val = try state.pop();

    if (!val.isQuote()) {
        try state.push(val);
        return RockError.WrongArguments;
    }

    var quote = try val.intoQuote(state);

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

    if (!vals[0].isQuote()) {
        try state.pushN(2, vals);
        return RockError.WrongArguments;
    }

    var pushMe = vals[1];
    var quote: ArrayList(RockVal) = try vals[0].intoQuote(state);

    try quote.append(pushMe);
    try state.push(RockVal{ .quote = quote });
}

pub fn enq(state: *RockMachine) !void {
    const vals = try state.popN(2);

    if (!vals[1].isQuote()) {
        try state.pushN(2, vals);
        return RockError.WrongArguments;
    }

    var pushMe = vals[0];
    var quote: ArrayList(RockVal) = try vals[1].intoQuote(state);

    try quote.insert(0, pushMe);
    try state.push(RockVal{ .quote = quote });
}

pub fn deq(state: *RockMachine) !void {
    const val = try state.pop();

    if (!val.isQuote()) {
        try state.push(val);
        return RockError.WrongArguments;
    }

    var quote: ArrayList(RockVal) = try val.intoQuote(state);

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

    var quote = try val.intoQuote(state);

    // TODO: Push as slice
    for (quote.items) |v| {
        try state.push(v);
    }
}

pub fn quoteAll(state: *RockMachine) !void {
    try state.quoteContext();
}

pub fn toBool(state: *RockMachine) !void {
    const val = try state.pop();

    try state.push(.{ .bool = val.intoBool(state) });
}

pub fn toQuote(state: *RockMachine) !void {
    const val = try state.pop();
    const quote = try val.intoQuote(state);

    try state.push(.{ .quote = quote });
}
