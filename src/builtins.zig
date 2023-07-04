const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Stack = std.SinglyLinkedList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const Token = @import("tokens.zig").Token;

const main = @import("main.zig");
const string = @import("string.zig");

const interpret = @import("interpret.zig");
const Quote = interpret.Quote;
const RockError = interpret.Error;
const RockVal = interpret.RockVal;
const RockMachine = interpret.RockMachine;

pub fn defineAll(machine: *RockMachine) !void {
    try machine.define(".q", "quit, printing a warning if there are any values left on stack", .{ .builtin = quit });
    try machine.define("exit", "exit with the specified exit code", .{ .builtin = exit });
    try machine.define("version", "print the version of the interpreter", .{ .builtin = version });

    try machine.define("cwd", "current working directory", .{ .builtin = cwd });
    try machine.define("cd", "change directory", .{ .builtin = cd });
    try machine.define("ls", "list contents of current directory", .{ .builtin = ls });
    try machine.define("readf", "read a file as a string", .{ .builtin = readf });
    try machine.define("writef", "write a string as a file", .{ .builtin = writef });

    try machine.define("exec", "execute a child process. When successful, returns stdout as a string. When unsuccessful, prints the child's stderr to stderr, and returns boolean false", .{ .builtin = exec });

    try machine.define("def!", "define a new command", .{ .builtin = def });
    try machine.define("defs", "produce a quote of all definition names", .{ .builtin = defs });
    try machine.define("def?", "return true if a name is defined", .{ .builtin = isDef });
    try machine.define("usage", "print the usage notes of a given command", .{ .builtin = cmdUsage });
    try machine.define(":", "bind variables", .{ .builtin = colon });

    try machine.define("do!", "execute a command or quote", .{ .builtin = doBang });
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

    try machine.define("get-line", "get a line from standard input (until newline)", .{ .builtin = getLine });
    try machine.define("get-lines", "get lines from standard input (until EOF)", .{ .builtin = getLines });
    try machine.define("get-args", "get command-line args", .{ .builtin = getArgs });
    try machine.define("eval", "evaluate a string as commands", .{ .builtin = eval });
    try machine.define("interactive?", "determine if the input mode is interactive (a TTY) or not", .{ .builtin = interactive });

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
    try machine.define("upcase", "convert a string to uppercase", .{ .builtin = upcase });
    try machine.define("downcase", "convert a string to lowercase", .{ .builtin = downcase });
    try machine.define("starts-with?", "consume a string and a prefix, and return true if the string has the prefix", .{ .builtin = startsWith });
    try machine.define("ends-with?", "consume a string and a suffix, and return true if the string has the suffix", .{ .builtin = endsWith });
    try machine.define("contains?", "consume a string and a substring, and return true if the string contains the substring", .{ .builtin = contains });

    try machine.define("map", "apply a command to all values in a quote", .{ .builtin = map });
    try machine.define("filter", "only keep values in that pass a predicate in a quote", .{ .builtin = filter });
    try machine.define("any?", "return true if any value in a quote passes a predicate", .{ .builtin = any });
    try machine.define("len", "the length of a quote or string", .{ .builtin = len });

    try machine.define("...", "expand a quote", .{ .builtin = ellipsis });
    try machine.define("rev", "reverse a quote or string", .{ .builtin = rev });
    try machine.define("quote", "quote a value", .{ .builtin = quoteVal });
    try machine.define("quote-all", "quote all current context", .{ .builtin = quoteAll });
    try machine.define("concat", "concatenate two quotes. Values are coerced into quotes. (For String concatenation, see join)", .{ .builtin = concat });
    try machine.define("push", "move an item into a quote", .{ .builtin = push });
    try machine.define("pop", "move the last item of a quote to top of stack", .{ .builtin = pop });
    try machine.define("enq", "move an item into the first position of a quote", .{ .builtin = enq });
    try machine.define("deq", "remove an item from the first position of a quote", .{ .builtin = deq });

    try machine.define("to-bool", "coerce value to boolean", .{ .builtin = toBool });
    try machine.define("to-int", "coerce value to integer", .{ .builtin = toInt });
    try machine.define("to-float", "coerce value to floating-point number", .{ .builtin = toFloat });
    try machine.define("to-string", "coerce value to string", .{ .builtin = toString });
    try machine.define("to-cmd", "coerce value to a deferred command", .{ .builtin = toCommand });
    try machine.define("to-quote", "coerce value to quote", .{ .builtin = toQuote });
}

pub fn quit(state: *RockMachine) !void {
    const ctx = try state.popContext();

    if (ctx.items.len > 0) {
        try stderr.print("Warning: Exited with unused values: [ ", .{});
        for (ctx.items) |item| {
            try item.print(state.alloc);
            try stderr.print(" ", .{});
        }
        try stderr.print("] \n", .{});
    }

    std.os.exit(0);
}

pub fn exit(state: *RockMachine) !void {
    const val = state.pop() catch RockVal{ .int = 255 };
    const i = try val.intoInt();

    if (i < 0) {
        return RockError.IntegerUnderflow;
    } else if (i > 255) {
        return RockError.IntegerOverflow;
    }

    const code: u8 = @intCast(i);
    std.os.exit(code);
}

pub fn version(state: *RockMachine) !void {
    try state.push(.{ .string = main.version });
}

pub fn cwd(state: *RockMachine) !void {
    const theCwd = try std.process.getCwdAlloc(state.alloc);
    try state.push(.{ .string = theCwd });
}

pub fn cd(state: *RockMachine) !void {
    const usage = "USAGE: path cd ({any})\n";
    _ = usage;

    const val = try state.pop();
    var path = try val.intoString(state);

    if (std.mem.eql(u8, path, "~")) {
        path = try std.process.getEnvVarOwned(state.alloc, "HOME");
    }

    std.os.chdir(path) catch |e| {
        try stderr.print("Unable to change directory: {any}\n", .{e});
        try state.push(val);
    };
}

pub fn ls(state: *RockMachine) !void {
    const theCwd = try std.process.getCwdAlloc(state.alloc);
    var dir = try std.fs.openIterableDirAbsolute(theCwd, .{});
    var entries = dir.iterate();

    var quote = Quote.init(state.alloc);
    while (try entries.next()) |entry| {
        var name = try state.alloc.dupe(u8, entry.name);
        try quote.append(.{ .string = name });
    }

    try state.push(.{ .quote = quote });

    dir.close();
}

pub fn readf(state: *RockMachine) !void {
    const val = try state.pop();
    const filename = val.intoString(state) catch |e| {
        try state.push(val);
        return e;
    };

    // We get a Dir from CWD so we can resolve relative paths
    const theCwdPath = try std.process.getCwdAlloc(state.alloc);
    var theCwd = try std.fs.openDirAbsolute(theCwdPath, .{});

    var file = try theCwd.openFile(filename, .{ .mode = .read_only });
    var contents = try file.readToEndAlloc(state.alloc, std.math.pow(usize, 2, 16));

    try state.push(.{ .string = contents });
    file.close();
}

pub fn writef(state: *RockMachine) !void {
    const vals = try state.popN(2);
    const filename = vals[1].intoString(state) catch |e| {
        try state.pushN(2, vals);
        return e;
    };
    const contents = vals[0].intoString(state) catch |e| {
        try state.pushN(2, vals);
        return e;
    };

    // We get a Dir from CWD so we can resolve relative paths
    const theCwdPath = try std.process.getCwdAlloc(state.alloc);
    var theCwd = try std.fs.openDirAbsolute(theCwdPath, .{});

    try theCwd.writeFile(filename, contents);
    theCwd.close();
}

pub fn exec(state: *RockMachine) !void {
    const val = try state.pop();
    const childProcess = try val.intoString(state);
    var args = std.mem.splitAny(u8, childProcess, " \t");
    var argv = ArrayList([]const u8).init(state.alloc);

    while (args.next()) |arg| try argv.append(arg);

    var result = std.process.Child.exec(.{
        .allocator = state.alloc,
        .argv = try argv.toOwnedSlice(),
    }) catch |e| {
        try stderr.print("Unable to launch child process: {any}\n", .{e});
        try state.push(.{ .bool = false });
        return;
    };

    switch (result.term) {
        .Exited => |code| if (code == 0) {
            const trimmed = std.mem.trimRight(u8, result.stdout, "\r\n");
            try state.push(.{ .string = trimmed });
            return;
        },
        else => {
            try stderr.print("{s}", .{result.stderr});
            try state.push(.{ .bool = false });
        },
    }
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

pub fn cmdUsage(state: *RockMachine) !void {
    const usage = "USAGE: term usage -> str ({any})\n";

    const val = state.pop() catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    const cmdName = try val.intoString(state);

    const cmd = state.defs.get(cmdName) orelse {
        const err = RockError.CommandUndefined;
        try stderr.print(usage, .{err});
        return err;
    };

    var description = try state.alloc.dupe(u8, cmd.description);

    try state.push(.{ .string = description });
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
        .string => |s| {
            var unescaped = try std.mem.replaceOwned(u8, state.alloc, s, "\\n", "\n");
            unescaped = try std.mem.replaceOwned(u8, state.alloc, unescaped, "\\\"", "\"");
            try stdout.print("{s}", .{unescaped});
        },
        else => {
            try val.print(state.alloc);
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
        try val.print(state.alloc);
        try stdout.print(" ", .{});
    }

    try stdout.print("]\n", .{});
}

pub fn getLine(state: *RockMachine) !void {
    var line = ArrayList(u8).init(state.alloc);
    try stdin.streamUntilDelimiter(line.writer(), '\n', null);
    const unescaped = try string.unescape(state.alloc, line.items);

    try state.push(.{ .string = unescaped });
}

pub fn getLines(state: *RockMachine) !void {
    var lines = Quote.init(state.alloc);

    while (true) {
        getLine(state) catch |err| switch (err) {
            error.EndOfStream => break,
            else => return err,
        };
        const val = try state.pop();
        const line = try val.intoString(state);
        try lines.append(.{ .string = line });
    }

    try state.push(.{ .quote = lines });
}

pub fn getArgs(state: *RockMachine) !void {
    var quote = Quote.init(state.alloc);
    var args = std.process.args();
    while (args.next()) |arg| {
        try quote.append(.{ .string = arg });
    }

    try state.push(.{ .quote = quote });
}

pub fn eval(state: *RockMachine) !void {
    var val = try state.pop();
    var code = try val.intoString(state);

    var tokens = Token.parse(state.alloc, code);
    while (try tokens.next()) |tok| {
        try state.interpret(tok);
    }
}

pub fn interactive(state: *RockMachine) !void {
    try state.push(.{ .bool = std.io.getStdIn().isTty() });
}

pub fn add(state: *RockMachine) !void {
    const usage = "USAGE: a b + -> a+b ({any})\n";

    const ns = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.WrongArguments;
    };

    if (ns[0].isInt() and ns[1].isInt()) {
        const a = try ns[0].intoInt();
        const b = try ns[1].intoInt();

        const res = @addWithOverflow(a, b);

        if (res[1] == 1) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Adding {} and {} would overflow.\n", .{ a, b });
            return RockError.IntegerOverflow;
        }

        try state.push(.{ .int = res[0] });
        return;
    }

    if (ns[0].isFloat() or ns[1].isFloat()) {
        const a = try ns[0].intoFloat();
        const b = try ns[1].intoFloat();

        try state.push(.{ .float = a + b });
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

    if (ns[0].isInt() and ns[1].isInt()) {
        const a = try ns[0].intoInt();
        const b = try ns[1].intoInt();

        const res = @subWithOverflow(a, b);

        if (res[1] == 1) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Subtracting {} from {} would overflow.\n", .{ b, a });
            return RockError.IntegerOverflow;
        }

        try state.push(.{ .int = res[0] });
        return;
    }

    if (ns[0].isFloat() or ns[1].isFloat()) {
        const a = try ns[0].intoFloat();
        const b = try ns[1].intoFloat();

        try state.push(.{ .float = a - b });
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

    if (ns[0].isInt() and ns[1].isInt()) {
        const a = try ns[0].intoInt();
        const b = try ns[1].intoInt();

        const res = @mulWithOverflow(a, b);

        if (res[1] == 1) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Multiplying {} by {} would overflow.\n", .{ a, b });
            return RockError.IntegerOverflow;
        }

        try state.push(.{ .int = res[0] });
        return;
    }

    if (ns[0].isFloat() or ns[1].isFloat()) {
        const a = try ns[0].intoFloat();
        const b = try ns[1].intoFloat();

        try state.push(.{ .float = a * b });
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

    if (ns[0].isInt() and ns[1].isInt()) {
        const a = try ns[0].intoInt();
        const b = try ns[1].intoInt();

        if (b == 0) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Cannot divide {} by zero.\n", .{a});
            return RockError.DivisionByZero;
        }

        try state.push(.{ .int = @divTrunc(a, b) });
        return;
    }

    if (ns[0].isFloat() or ns[1].isFloat()) {
        const a = try ns[0].intoFloat();
        const b = try ns[1].intoFloat();

        if (b == 0) {
            try state.pushN(2, ns);
            try stderr.print("ERROR: Cannot divide {} by zero.\n", .{a});
            return RockError.DivisionByZero;
        }

        try state.push(.{ .float = a / b });
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

    if (ns[0].isInt() and ns[1].isInt()) {
        const a = try ns[0].intoInt();
        const b = try ns[1].intoInt();

        try state.push(.{ .int = @mod(a, b) });
        return;
    }

    if (ns[0].isFloat() or ns[1].isFloat()) {
        const a = try ns[0].intoFloat();
        const b = try ns[1].intoFloat();

        try state.push(.{ .float = @mod(a, b) });
        return;
    }

    try state.pushN(2, ns);
    try stderr.print(usage, .{RockError.WrongArguments});
    return RockError.WrongArguments;
}

pub fn abs(state: *RockMachine) !void {
    const usage = "USAGE: n abs -> |n| ({any})\n";

    const n = try state.pop();

    if (n.isInt()) {
        const a = try n.intoInt();

        try state.push(.{ .int = try std.math.absInt(a) });
        return;
    }

    if (n.isFloat()) {
        const a = try n.intoFloat();

        try state.push(.{ .float = std.math.fabs(a) });
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

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try state.push(.{ .bool = a == b });
        return;
    }

    if ((vals[0].isInt() or vals[0].isFloat()) and (vals[1].isInt() or vals[1].isFloat())) {
        const a = try vals[0].intoFloat();
        const b = try vals[1].intoFloat();

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

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try state.push(.{ .bool = b > a });
        return;
    }

    const a = vals[0].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    const b = vals[1].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    try state.push(.{ .bool = b > a });
}

pub fn greaterThanEq(state: *RockMachine) !void {
    const usage = "USAGE: a b gt? -> b>=a ({any})\n";

    const vals = try state.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try state.push(.{ .bool = b >= a });
        return;
    }

    const a = vals[0].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    const b = vals[1].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    try state.push(.{ .bool = b >= a });
}

pub fn lessThan(state: *RockMachine) !void {
    const usage = "USAGE: a b lt? -> b<a ({any})\n";

    const vals = try state.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try state.push(.{ .bool = b < a });
        return;
    }

    const a = vals[0].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    const b = vals[1].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    try state.push(.{ .bool = b < a });
}

pub fn lessThanEq(state: *RockMachine) !void {
    const usage = "USAGE: a b lte? -> b<=a ({any})\n";

    const vals = try state.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try state.push(.{ .bool = b <= a });
        return;
    }

    const a = vals[0].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    const b = vals[1].intoFloat() catch {
        try state.pushN(2, vals);
        try stderr.print(usage, .{RockError.WrongArguments});
        return RockError.WrongArguments;
    };

    try state.push(.{ .bool = b <= a });
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

pub fn upcase(state: *RockMachine) !void {
    var val = try state.pop();
    const before = try val.intoString(state);
    const after = try std.ascii.allocUpperString(state.alloc, before);
    try state.push(.{ .string = after });
}

pub fn downcase(state: *RockMachine) !void {
    var val = try state.pop();
    const before = try val.intoString(state);
    const after = try std.ascii.allocLowerString(state.alloc, before);
    try state.push(.{ .string = after });
}

pub fn startsWith(state: *RockMachine) !void {
    var vals = try state.popN(2);
    var str = try vals[0].intoString(state);
    var prefix = try vals[1].intoString(state);
    try state.push(.{ .bool = std.mem.startsWith(u8, str, prefix) });
}

pub fn endsWith(state: *RockMachine) !void {
    var vals = try state.popN(2);
    var str = try vals[0].intoString(state);
    var suffix = try vals[1].intoString(state);
    try state.push(.{ .bool = std.mem.endsWith(u8, str, suffix) });
}

pub fn contains(state: *RockMachine) !void {
    var vals = try state.popN(2);
    var str = try vals[0].intoString(state);
    var substr = try vals[1].intoString(state);
    try state.push(.{ .bool = std.mem.containsAtLeast(u8, str, 1, substr) });
}

pub fn opt(state: *RockMachine) !void {
    const usage = "USAGE: ... term|quote bool ? -> ... ({any})\n";

    var val = state.pop() catch |e| switch (e) {
        error.StackUnderflow => {
            try stderr.print(usage, .{e});
            return;
        },
        else => return e,
    };

    const cond = val.intoBool(state);

    try if (cond) do(state) else drop(state) catch |e| {
        try state.push(val);
        try stderr.print(usage, .{e});
        return e;
    };
}

pub fn doBang(state: *RockMachine) !void {
    const usage = "USAGE: ... term|quote do! -> ... ({any})\n";
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

// Same as do! but does not uplevel any definitions
pub fn do(state: *RockMachine) !void {
    const usage = "USAGE: ... term|quote do -> ... ({any})\n";
    _ = usage;

    var val = try state.pop();

    var jail = try state.child();
    jail.nest = state.nest;

    if (val.isCommand() or val.isDeferredCommand() or val.isString()) {
        const cmdName = try val.intoString(state);

        try jail.handleCmd(cmdName);

        state.nest = jail.nest;
        return;
    }

    const quote = try val.intoQuote(state);

    for (quote.items) |v| try jail.handle(v);
    state.nest = jail.nest;
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

    const vals = try state.popN(2);

    const quote = try vals[0].intoQuote(state);
    const f = vals[1];

    _map(state, quote, f) catch |err| {
        if (err == error.BrokenPipe) return err;
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

pub fn any(state: *RockMachine) !void {
    const usage = "USAGE: [as] (a->bool) any? -> bool ({any})\n";

    const vals = state.popN(2) catch |e| {
        try stderr.print(usage, .{e});
        return RockError.StackUnderflow;
    };

    const quote = try vals[0].intoQuote(state);
    const f = vals[1];

    if (quote.items.len == 0) {
        try state.push(.{ .bool = false });
        return;
    }

    _any(state, quote, f) catch |err| {
        try stderr.print(usage, .{err});
        try state.pushN(2, vals);
    };
}

fn _any(state: *RockMachine, as: Quote, f: RockVal) !void {
    for (as.items) |a| {
        var child = try state.child();
        try child.push(a);
        try child.push(f);
        try do(&child);
        var lastVal = try child.pop();
        var cond = lastVal.intoBool(state);

        if (cond) {
            try state.push(RockVal{ .bool = true });
            return;
        }
    }

    try state.push(RockVal{ .bool = false });
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

pub fn len(state: *RockMachine) !void {
    const val = try state.pop();

    if (val.isQuote()) {
        const quote = try val.intoQuote(state);
        const length: i64 = @intCast(quote.items.len);
        try state.push(.{ .int = length });
        return;
    }

    if (val.isString()) {
        const s = try val.intoString(state);
        const length: i64 = @intCast(s.len);
        try state.push(.{ .int = length });
        return;
    }

    try stderr.print("USAGE: quote|string len -> int\n", .{});
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

pub fn rev(state: *RockMachine) !void {
    const val = try state.pop();

    if (val.isQuote()) {
        const quote = try val.intoQuote(state);
        const length = quote.items.len;

        var newItems = try state.alloc.alloc(RockVal, length);
        for (quote.items, 0..) |v, i| {
            newItems[length - i - 1] = v;
        }

        var newQuote = Quote.fromOwnedSlice(state.alloc, newItems);

        try state.push(.{ .quote = newQuote });
        return;
    }

    if (val.isString()) {
        const str = try val.intoString(state);
        var newStr = try state.alloc.alloc(u8, str.len);

        for (str, 0..) |c, i| {
            newStr[str.len - 1 - i] = c;
        }

        try state.push(.{ .string = newStr });
        return;
    }

    const err = RockError.WrongArguments;
    try stderr.print("USAGE: quote|str rev -> rts|etouq ({any})", .{err});
    return err;
}

pub fn quoteVal(state: *RockMachine) !void {
    const val = try state.pop();
    var quote = Quote.init(state.alloc);
    try quote.append(val);
    try state.push(.{ .quote = quote });
}

pub fn quoteAll(state: *RockMachine) !void {
    try state.quoteContext();
}

pub fn concat(state: *RockMachine) !void {
    const vals = try state.popN(2);

    var a = try vals[0].intoQuote(state);
    var b = try vals[1].intoQuote(state);

    try a.appendSlice(b.items);

    try state.push(.{ .quote = a });
}

pub fn toBool(state: *RockMachine) !void {
    const val = try state.pop();
    const b = val.intoBool(state);
    try state.push(.{ .bool = b });
}

pub fn toInt(state: *RockMachine) !void {
    const val = try state.pop();
    const i = try val.intoInt();
    try state.push(.{ .int = i });
}

pub fn toFloat(state: *RockMachine) !void {
    const val = try state.pop();
    const f = try val.intoFloat();
    try state.push(.{ .float = f });
}

pub fn toString(state: *RockMachine) !void {
    const val = try state.pop();
    const s = try val.intoString(state);
    try state.push(.{ .string = s });
}

pub fn toCommand(state: *RockMachine) !void {
    const val = try state.pop();
    const cmd = try val.intoString(state);
    try state.push(.{ .deferred_command = cmd });
}

pub fn toQuote(state: *RockMachine) !void {
    const val = try state.pop();
    const quote = try val.intoQuote(state);
    try state.push(.{ .quote = quote });
}
