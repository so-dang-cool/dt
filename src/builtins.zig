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
const Error = interpret.DtError;
const DtVal = interpret.DtVal;
const DtMachine = interpret.DtMachine;

pub fn defineAll(machine: *DtMachine) !void {
    try machine.define(".q", "quit, printing a warning if there are any values left on stack", .{ .builtin = quit });
    try machine.define("exit", "exit with the specified exit code", .{ .builtin = exit });
    try machine.define("version", "print the version of the interpreter", .{ .builtin = version });

    try machine.define("cwd", "current working directory", .{ .builtin = cwd });
    try machine.define("cd", "change directory", .{ .builtin = cd });
    try machine.define("ls", "list contents of current directory", .{ .builtin = ls });
    try machine.define("readf", "read a file as a string", .{ .builtin = readf });
    try machine.define("writef", "write a string as a file", .{ .builtin = writef });
    // TODO: pathsep/filesep, env get, env set

    try machine.define("exec", "execute a child process. When successful, returns stdout as a string. When unsuccessful, prints the child's stderr to stderr, and returns boolean false", .{ .builtin = exec });

    try machine.define("def!", "define a new command", .{ .builtin = defBang });
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

    try machine.define("p", "print a value to stdout", .{ .builtin = p });
    try machine.define("ep", "print a value to stderr", .{ .builtin = ep });
    try machine.define("nl", "print a newline to stdout", .{ .builtin = nl });
    try machine.define("enl", "print a newline to stderr", .{ .builtin = enl });
    try machine.define(".s", "print the stack", .{ .builtin = dotS });

    try machine.define("read-line", "get a line from standard input (until newline)", .{ .builtin = readLine });
    try machine.define("read-lines", "get lines from standard input (until EOF)", .{ .builtin = readLines });
    try machine.define("procname", "get name of current process", .{ .builtin = procname });
    try machine.define("args", "get command-line args", .{ .builtin = args });
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
    try machine.define("len", "the length of a string or quote or 1 for single values", .{ .builtin = len });

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
    try machine.define("to-cmd", "coerce value to a command", .{ .builtin = toCommand });
    try machine.define("to-def", "coerce value to a deferred command", .{ .builtin = toDef });
    try machine.define("to-quote", "coerce value to quote", .{ .builtin = toQuote });
    try machine.define("to-error", "coerce value to an error", .{ .builtin = toError });
}

pub fn quit(dt: *DtMachine) !void {
    const ctx = try dt.popContext();

    if (ctx.items.len > 0) {
        try stderr.print("Warning: Exited with unused values: [ ", .{});
        for (ctx.items) |item| {
            try item.print(dt.alloc);
            try stderr.print(" ", .{});
        }
        try stderr.print("] \n", .{});
    }

    std.os.exit(0);
}

pub fn exit(dt: *DtMachine) !void {
    const val = dt.pop() catch DtVal{ .int = 255 };
    const i = try val.intoInt();

    if (i < 0) {
        return dt.rewind(val, Error.IntegerUnderflow);
    } else if (i > 255) {
        return dt.rewind(val, Error.IntegerOverflow);
    }

    const code: u8 = @intCast(i);
    std.os.exit(code);
}

pub fn version(dt: *DtMachine) !void {
    try dt.push(.{ .string = main.version });
}

pub fn cwd(dt: *DtMachine) !void {
    const theCwd = try std.process.getCwdAlloc(dt.alloc);
    try dt.push(.{ .string = theCwd });
}

pub fn cd(dt: *DtMachine) !void {
    const val = try dt.pop();
    var path = val.intoString(dt) catch |e| return dt.rewind(val, e);

    if (std.mem.eql(u8, path, "~")) {
        path = try std.process.getEnvVarOwned(dt.alloc, "HOME");
    }

    std.os.chdir(path) catch |e| {
        try stderr.print("Unable to change directory: {any}\n", .{e});
        try dt.push(val);
    };
}

pub fn ls(dt: *DtMachine) !void {
    const theCwd = try std.process.getCwdAlloc(dt.alloc);
    var dir = try std.fs.openIterableDirAbsolute(theCwd, .{});
    var entries = dir.iterate();

    var quote = Quote.init(dt.alloc);
    while (try entries.next()) |entry| {
        var name = try dt.alloc.dupe(u8, entry.name);
        try quote.append(.{ .string = name });
    }

    try dt.push(.{ .quote = quote });

    dir.close();
}

pub fn readf(dt: *DtMachine) !void {
    const val = try dt.pop();
    const filename = val.intoString(dt) catch |e| return dt.rewind(val, e);

    // We get a Dir from CWD so we can resolve relative paths
    const theCwdPath = try std.process.getCwdAlloc(dt.alloc);
    var theCwd = try std.fs.openDirAbsolute(theCwdPath, .{});

    var file = try theCwd.openFile(filename, .{ .mode = .read_only });
    var contents = try file.readToEndAlloc(dt.alloc, std.math.pow(usize, 2, 16));

    try dt.push(.{ .string = contents });
    file.close();
}

pub fn writef(dt: *DtMachine) !void {
    const vals = try dt.popN(2);
    const filename = vals[1].intoString(dt) catch |e| return dt.rewindN(2, vals, e);
    const contents = vals[0].intoString(dt) catch |e| return dt.rewindN(2, vals, e);

    // We get a Dir from CWD so we can resolve relative paths
    const theCwdPath = try std.process.getCwdAlloc(dt.alloc);
    var theCwd = try std.fs.openDirAbsolute(theCwdPath, .{});

    try theCwd.writeFile(filename, contents);
    theCwd.close();
}

pub fn exec(dt: *DtMachine) !void {
    const val = try dt.pop();
    const childProcess = try val.intoString(dt);
    var childArgs = std.mem.splitAny(u8, childProcess, " \t");
    var argv = ArrayList([]const u8).init(dt.alloc);

    while (childArgs.next()) |arg| try argv.append(arg);

    var result = std.process.Child.exec(.{
        .allocator = dt.alloc,
        .argv = try argv.toOwnedSlice(),
    }) catch |e| return dt.rewind(val, e);

    switch (result.term) {
        .Exited => |code| if (code == 0) {
            const trimmed = std.mem.trimRight(u8, result.stdout, "\r\n");
            try dt.push(.{ .string = trimmed });
            return;
        },
        else => {
            try stderr.print("{s}", .{result.stderr});
            try dt.push(.{ .bool = false });
        },
    }
}

pub fn defBang(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    const quote = try vals[0].intoQuote(dt);
    const name = vals[1].intoString(dt) catch |e| return dt.rewindN(2, vals, e);

    try dt.define(name, name, .{ .quote = quote });
}

pub fn defs(dt: *DtMachine) !void {
    var quote = Quote.init(dt.alloc);
    var defNames = dt.defs.keyIterator();

    while (defNames.next()) |defName| {
        var cmdName = try dt.alloc.dupe(u8, defName.*);
        try quote.append(.{ .string = cmdName });
    }

    try dt.push(.{ .quote = quote });
}

pub fn isDef(dt: *DtMachine) !void {
    const val = try dt.pop();

    const name = val.intoString(dt) catch |e| return dt.rewind(val, e);

    try dt.push(.{ .bool = dt.defs.contains(name) });
}

pub fn cmdUsage(dt: *DtMachine) !void {
    const val = try dt.pop();

    const cmdName = val.intoString(dt) catch |e| return dt.rewind(val, e);

    const cmd = dt.defs.get(cmdName) orelse return dt.rewind(val, Error.CommandUndefined);

    var description = try dt.alloc.dupe(u8, cmd.description);

    try dt.push(.{ .string = description });
}

// Variable binding
pub fn colon(dt: *DtMachine) !void {
    var termVal = try dt.pop();

    // Single term
    if (termVal.isCommand() or termVal.isDeferredCommand() or termVal.isString()) {
        const cmdName = try termVal.intoString(dt);

        const val = dt.pop() catch |e| return dt.rewind(termVal, e);

        var quote = Quote.init(dt.alloc);
        try quote.append(val);
        try dt.define(cmdName, cmdName, .{ .quote = quote });
        return;
    }

    // Multiple terms

    var terms = (try termVal.intoQuote(dt)).items;

    var vals = try dt.alloc.alloc(DtVal, terms.len);

    var i = terms.len;

    while (i > 0) : (i -= 1) {
        vals[i - 1] = dt.pop() catch |e| {
            while (i < terms.len) : (i += 1) {
                try dt.push(vals[i]);
            }
            return dt.rewind(termVal, e);
        };
    }

    for (terms, vals) |termV, val| {
        const term = try termV.intoString(dt);
        var quote = ArrayList(DtVal).init(dt.alloc);
        try quote.append(val);
        try dt.define(term, term, .{ .quote = quote });
    }
}

pub fn dup(dt: *DtMachine) !void {
    const val = try dt.pop();
    try dt.push(val);
    try dt.push(val);
}

pub fn drop(dt: *DtMachine) !void {
    _ = try dt.pop();
}

pub fn swap(dt: *DtMachine) !void {
    const vals = try dt.popN(2);
    try dt.push(vals[1]);
    try dt.push(vals[0]);
}

// ... a b c (rot) ... c a b
//   [ 0 1 2 ]       [ 2 0 1 ]
pub fn rot(dt: *DtMachine) !void {
    const vals = try dt.popN(3);
    try dt.push(vals[2]);
    try dt.push(vals[0]);
    try dt.push(vals[1]);
}

pub fn p(dt: *DtMachine) !void {
    const val = try dt.pop();
    try _p(val, dt.alloc, stdout);
}

pub fn ep(dt: *DtMachine) !void {
    const val = try dt.pop();
    try _p(val, dt.alloc, stderr);
}

fn _p(val: DtVal, allocator: Allocator, writer: std.fs.File.Writer) !void {
    switch (val) {
        .string => |s| {
            var unescaped = try std.mem.replaceOwned(u8, allocator, s, "\\n", "\n");
            unescaped = try std.mem.replaceOwned(u8, allocator, unescaped, "\\\"", "\"");
            try writer.print("{s}", .{unescaped});
        },
        else => {
            try val.print(allocator);
        },
    }
}

pub fn nl(_: *DtMachine) !void {
    try stdout.print("\n", .{});
}

pub fn enl(_: *DtMachine) !void {
    try stderr.print("\n", .{});
}

pub fn dotS(dt: *DtMachine) !void {
    try stdout.print("[ ", .{});

    var top = dt.nest.first orelse return;

    for (top.data.items) |val| {
        try val.print(dt.alloc);
        try stdout.print(" ", .{});
    }

    try stdout.print("]\n", .{});
}

pub fn readLine(dt: *DtMachine) !void {
    var line = ArrayList(u8).init(dt.alloc);
    try stdin.streamUntilDelimiter(line.writer(), '\n', null);

    try dt.push(.{ .string = line.items });
}

pub fn readLines(dt: *DtMachine) !void {
    var lines = Quote.init(dt.alloc);

    while (true) {
        readLine(dt) catch |err| switch (err) {
            error.EndOfStream => break,
            else => return err,
        };
        const val = try dt.pop();
        const line = try val.intoString(dt);
        try lines.append(.{ .string = line });
    }

    try dt.push(.{ .quote = lines });
}

pub fn procname(dt: *DtMachine) !void {
    var procArgs = std.process.args();
    var name = procArgs.next() orelse return Error.ProcessNameUnknown;
    try dt.push(.{ .string = name });
}

pub fn args(dt: *DtMachine) !void {
    var quote = Quote.init(dt.alloc);
    var procArgs = std.process.args();
    _ = procArgs.next(); // Discard process name

    while (procArgs.next()) |arg| {
        try quote.append(.{ .string = arg });
    }

    try dt.push(.{ .quote = quote });
}

pub fn eval(dt: *DtMachine) !void {
    var val = try dt.pop();
    var code = val.intoString(dt) catch |e| return dt.rewind(val, e);

    var tokens = Token.parse(dt.alloc, code);
    while (try tokens.next()) |tok| {
        try dt.interpret(tok);
    }
}

pub fn interactive(state: *DtMachine) !void {
    try state.push(.{ .bool = std.io.getStdIn().isTty() });
}

pub fn add(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        const res = @addWithOverflow(a, b);

        if (res[1] == 1) {
            try dt.pushN(2, vals);
            try stderr.print("ERROR: Adding {} and {} would overflow.\n", .{ a, b });
            return Error.IntegerOverflow;
        }

        try dt.push(.{ .int = res[0] });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .float = a + b });
}

pub fn subtract(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        const res = @subWithOverflow(a, b);

        if (res[1] == 1) {
            try dt.pushN(2, vals);
            try stderr.print("ERROR: Subtracting {} from {} would overflow.\n", .{ b, a });
            return Error.IntegerOverflow;
        }

        try dt.push(.{ .int = res[0] });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .float = a - b });
}

pub fn multiply(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        const res = @mulWithOverflow(a, b);

        if (res[1] == 1) {
            try dt.pushN(2, vals);
            try stderr.print("ERROR: Multiplying {} by {} would overflow.\n", .{ a, b });
            return Error.IntegerOverflow;
        }

        try dt.push(.{ .int = res[0] });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .float = a * b });
    return;
}

pub fn divide(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        if (b == 0) {
            try dt.pushN(2, vals);
            try stderr.print("ERROR: Cannot divide {} by zero.\n", .{a});
            return Error.DivisionByZero;
        }

        try dt.push(.{ .int = @divTrunc(a, b) });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    if (b == 0) {
        try dt.pushN(2, vals);
        try stderr.print("ERROR: Cannot divide {} by zero.\n", .{a});
        return Error.DivisionByZero;
    }

    try dt.push(.{ .float = a / b });
}

pub fn modulo(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try dt.push(.{ .int = @mod(a, b) });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .float = @mod(a, b) });
    return;
}

pub fn abs(dt: *DtMachine) !void {
    const val = try dt.pop();

    if (val.isInt()) {
        const a = try val.intoInt();

        try dt.push(.{ .int = try std.math.absInt(a) });
        return;
    }

    const a = val.intoFloat() catch |e| return dt.rewind(val, e);

    try dt.push(.{ .float = std.math.fabs(a) });
}

pub fn eq(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try dt.push(.{ .bool = a == b });
        return;
    }

    if ((vals[0].isInt() or vals[0].isFloat()) and (vals[1].isInt() or vals[1].isFloat())) {
        const a = try vals[0].intoFloat();
        const b = try vals[1].intoFloat();

        try dt.push(.{ .bool = a == b });
        return;
    }

    if (vals[0].isBool() and vals[1].isBool()) {
        const a = vals[0].intoBool(dt);
        const b = vals[1].intoBool(dt);

        try dt.push(.{ .bool = a == b });
        return;
    }

    if (vals[0].isQuote() and vals[1].isQuote()) {
        const a = try vals[0].intoQuote(dt);
        const b = try vals[1].intoQuote(dt);

        const as: []DtVal = a.items;
        const bs: []DtVal = b.items;

        if (as.len != bs.len) {
            try dt.push(.{ .bool = false });
            return;
        }

        var child = try dt.child();

        for (as, 0..) |val, i| {
            try child.push(val);
            try child.push(bs[i]);
            try eq(&child);
            const bv = try child.pop();
            const res = bv.intoBool(dt);
            if (!res) {
                try dt.push(.{ .bool = false });
                return;
            }
        }

        try dt.push(.{ .bool = true });
        return;
    }

    if (!vals[0].isQuote() and !vals[1].isQuote()) {
        const a = try vals[0].intoString(dt);
        const b = try vals[1].intoString(dt);

        try dt.push(.{ .bool = std.mem.eql(u8, a, b) });
        return;
    }

    try dt.push(.{ .bool = false });
}

pub fn greaterThan(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try dt.push(.{ .bool = b > a });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .bool = b > a });
}

pub fn greaterThanEq(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try dt.push(.{ .bool = b >= a });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .bool = b >= a });
}

pub fn lessThan(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try dt.push(.{ .bool = b < a });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .bool = b < a });
}

pub fn lessThanEq(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    if (vals[0].isInt() and vals[1].isInt()) {
        const a = try vals[0].intoInt();
        const b = try vals[1].intoInt();

        try dt.push(.{ .bool = b <= a });
        return;
    }

    const a = vals[0].intoFloat() catch |e| return dt.rewindN(2, vals, e);
    const b = vals[1].intoFloat() catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .bool = b <= a });
}

pub fn boolAnd(dt: *DtMachine) !void {
    var vals = try dt.popN(2);

    var a = vals[0].intoBool(dt);
    var b = vals[1].intoBool(dt);

    try dt.push(.{ .bool = a and b });
}

pub fn boolOr(dt: *DtMachine) !void {
    var vals = try dt.popN(2);

    var a = vals[0].intoBool(dt);
    var b = vals[1].intoBool(dt);

    try dt.push(.{ .bool = a or b });
}

pub fn not(dt: *DtMachine) !void {
    var val = try dt.pop();

    var a = val.intoBool(dt);
    try dt.push(.{ .bool = !a });
}

pub fn split(dt: *DtMachine) !void {
    var vals = try dt.popN(2);

    var str = vals[0].intoString(dt) catch |e| return dt.rewindN(2, vals, e);
    var delim = vals[1].intoString(dt) catch |e| return dt.rewindN(2, vals, e);

    if (delim.len > 0) {
        var parts = std.mem.split(u8, str, delim);
        var quote = Quote.init(dt.alloc);
        while (parts.next()) |part| {
            try quote.append(.{ .string = part });
        }
        try dt.push(.{ .quote = quote });
    } else {
        var quote = Quote.init(dt.alloc);
        for (str) |c| {
            var s = try dt.alloc.create([1]u8);
            s[0] = c;
            try quote.append(.{ .string = s });
        }
        try dt.push(.{ .quote = quote });
    }
}

pub fn join(dt: *DtMachine) !void {
    var vals = try dt.popN(2);

    if (!vals[0].isQuote()) {
        const str = vals[0].intoString(dt) catch |e| return dt.rewindN(2, vals, e);
        try dt.push(.{ .string = str });
        return;
    }

    var strs = try vals[0].intoQuote(dt);
    var delim = try vals[1].intoString(dt);

    var parts = try ArrayList([]const u8).initCapacity(dt.alloc, strs.items.len);
    for (strs.items) |part| {
        const s = try part.intoString(dt);
        try parts.append(s);
    }
    var acc = try std.mem.join(dt.alloc, delim, parts.items);
    try dt.push(.{ .string = acc });
}

pub fn upcase(dt: *DtMachine) !void {
    var val = try dt.pop();
    const before = val.intoString(dt) catch |e| return dt.rewind(val, e);

    const after = try std.ascii.allocUpperString(dt.alloc, before);

    try dt.push(.{ .string = after });
}

pub fn downcase(dt: *DtMachine) !void {
    var val = try dt.pop();
    const before = val.intoString(dt) catch |e| return dt.rewind(val, e);

    const after = try std.ascii.allocLowerString(dt.alloc, before);

    try dt.push(.{ .string = after });
}

pub fn startsWith(dt: *DtMachine) !void {
    var vals = try dt.popN(2);

    var str = vals[0].intoString(dt) catch |e| return dt.rewindN(2, vals, e);
    var prefix = vals[1].intoString(dt) catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .bool = std.mem.startsWith(u8, str, prefix) });
}

pub fn endsWith(dt: *DtMachine) !void {
    var vals = try dt.popN(2);

    var str = vals[0].intoString(dt) catch |e| return dt.rewindN(2, vals, e);
    var suffix = vals[1].intoString(dt) catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .bool = std.mem.endsWith(u8, str, suffix) });
}

pub fn contains(dt: *DtMachine) !void {
    var vals = try dt.popN(2);

    var str = vals[0].intoString(dt) catch |e| return dt.rewindN(2, vals, e);
    var substr = vals[1].intoString(dt) catch |e| return dt.rewindN(2, vals, e);

    try dt.push(.{ .bool = std.mem.containsAtLeast(u8, str, 1, substr) });
}

pub fn opt(dt: *DtMachine) !void {
    var val = try dt.pop();
    const cond = val.intoBool(dt);

    try if (cond) do(dt) else drop(dt) catch |e| return dt.rewind(val, e);
}

pub fn doBang(dt: *DtMachine) !void {
    var val = try dt.pop();

    if (val.isCommand() or val.isDeferredCommand() or val.isString()) {
        const cmdName = try val.intoString(dt);

        try dt.handleCmd(cmdName);
        return;
    }

    const quote = try val.intoQuote(dt);

    for (quote.items) |v| try dt.handle(v);
}

// Same as do! but does not uplevel any definitions
pub fn do(dt: *DtMachine) !void {
    var val = try dt.pop();

    var jail = try dt.child();
    jail.nest = dt.nest;

    if (val.isCommand() or val.isDeferredCommand()) {
        const cmdName = try val.intoString(dt);

        try jail.handleCmd(cmdName);

        dt.nest = jail.nest;
        return;
    }

    const quote = try val.intoQuote(dt);

    for (quote.items) |v| try jail.handle(v);
    dt.nest = jail.nest;
}

pub fn doin(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    const quote = try vals[0].intoQuote(dt);
    const f = vals[1];

    _doin(dt, quote, f) catch |e| return dt.rewindN(2, vals, e);
}

fn _doin(dt: *DtMachine, quote: Quote, f: DtVal) !void {
    var child = try dt.child();

    try child.push(.{ .quote = quote });
    try ellipsis(&child);
    try child.push(f);
    try do(&child);

    const resultQuote = try child.popContext();

    try dt.push(.{ .quote = resultQuote });
}

pub fn map(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    const quote = try vals[0].intoQuote(dt);
    const f = vals[1];

    _map(dt, quote, f) catch |e| return dt.rewindN(2, vals, e);
}

fn _map(dt: *DtMachine, as: Quote, f: DtVal) !void {
    var child = try dt.child();

    for (as.items) |a| {
        try child.push(a);
        try child.push(f);
        try do(&child);
    }

    const newQuote = try child.popContext();

    try dt.push(DtVal{ .quote = newQuote });
}

pub fn filter(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    const quote = try vals[0].intoQuote(dt);
    const f = vals[1];

    _filter(dt, quote, f) catch |e| return dt.rewindN(2, vals, e);
}

fn _filter(dt: *DtMachine, as: Quote, f: DtVal) !void {
    var quote = Quote.init(dt.alloc);

    for (as.items) |a| {
        var child = try dt.child();

        try child.push(a);
        try child.push(f);
        try do(&child);

        var lastVal = try child.pop();
        var cond = lastVal.intoBool(dt);

        if (cond) {
            try quote.append(a);
        }
    }

    try dt.push(DtVal{ .quote = quote });
}

pub fn any(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    const quote = try vals[0].intoQuote(dt);
    const f = vals[1];

    if (quote.items.len == 0) {
        try dt.push(.{ .bool = false });
        return;
    }

    _any(dt, quote, f) catch |e| return dt.rewindN(2, vals, e);
}

fn _any(dt: *DtMachine, as: Quote, f: DtVal) !void {
    for (as.items) |a| {
        var child = try dt.child();

        try child.push(a);
        try child.push(f);
        try do(&child);

        var lastVal = try child.pop();
        var cond = lastVal.intoBool(dt);

        if (cond) {
            try dt.push(DtVal{ .bool = true });
            return;
        }
    }

    try dt.push(DtVal{ .bool = false });
}

pub fn pop(dt: *DtMachine) !void {
    const val = try dt.pop();

    var quote = try val.intoQuote(dt);

    if (quote.items.len > 0) {
        const lastVal = quote.pop();
        try dt.push(DtVal{ .quote = quote });
        try dt.push(lastVal);
        return;
    }

    try dt.push(val);
}

pub fn push(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    var pushMe = vals[1];
    var quote = try vals[0].intoQuote(dt);

    try quote.append(pushMe);
    try dt.push(DtVal{ .quote = quote });
}

pub fn enq(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    var pushMe = vals[0];
    var quote = try vals[1].intoQuote(dt);

    var newQuote = Quote.init(dt.alloc);
    try newQuote.append(pushMe);
    try newQuote.appendSlice(quote.items);

    try dt.push(.{ .quote = newQuote });
}

pub fn deq(dt: *DtMachine) !void {
    const val = try dt.pop();

    var quote = try val.intoQuote(dt);

    if (quote.items.len == 0) {
        try dt.push(val);
        return;
    }

    const firstVal = quote.orderedRemove(0);
    try dt.push(firstVal);
    try dt.push(DtVal{ .quote = quote });
}

pub fn len(dt: *DtMachine) !void {
    const val = try dt.pop();

    if (val.isString()) {
        const s = try val.intoString(dt);
        const length: i64 = @intCast(s.len);
        try dt.push(.{ .int = length });
        return;
    }

    const quote = try val.intoQuote(dt);
    const length: i64 = @intCast(quote.items.len);
    try dt.push(.{ .int = length });
}

pub fn ellipsis(dt: *DtMachine) !void {
    const val = try dt.pop();

    var quote = val.intoQuote(dt) catch |e| return dt.rewind(val, e);

    // TODO: Push as slice
    for (quote.items) |v| {
        try dt.push(v);
    }
}

pub fn rev(dt: *DtMachine) !void {
    const val = try dt.pop();

    if (val.isQuote()) {
        const quote = try val.intoQuote(dt);
        const length = quote.items.len;

        var newItems = try dt.alloc.alloc(DtVal, length);
        for (quote.items, 0..) |v, i| {
            newItems[length - i - 1] = v;
        }

        var newQuote = Quote.fromOwnedSlice(dt.alloc, newItems);

        try dt.push(.{ .quote = newQuote });
        return;
    }

    if (val.isString()) {
        const str = try val.intoString(dt);
        var newStr = try dt.alloc.alloc(u8, str.len);

        for (str, 0..) |c, i| {
            newStr[str.len - 1 - i] = c;
        }

        try dt.push(.{ .string = newStr });
        return;
    }

    return dt.rewind(val, Error.WrongArguments);
}

pub fn quoteVal(dt: *DtMachine) !void {
    const val = try dt.pop();
    var quote = Quote.init(dt.alloc);
    try quote.append(val);
    try dt.push(.{ .quote = quote });
}

pub fn quoteAll(dt: *DtMachine) !void {
    try dt.quoteContext();
}

pub fn concat(dt: *DtMachine) !void {
    const vals = try dt.popN(2);

    var a = try vals[0].intoQuote(dt);
    var b = try vals[1].intoQuote(dt);

    try a.appendSlice(b.items);

    try dt.push(.{ .quote = a });
}

pub fn toBool(state: *DtMachine) !void {
    const val = try state.pop();
    const b = val.intoBool(state);
    try state.push(.{ .bool = b });
}

pub fn toInt(dt: *DtMachine) !void {
    const val = try dt.pop();
    const i = val.intoInt() catch |e| return dt.rewind(val, e);
    try dt.push(.{ .int = i });
}

pub fn toFloat(dt: *DtMachine) !void {
    const val = try dt.pop();
    const f = val.intoFloat() catch |e| return dt.rewind(val, e);
    try dt.push(.{ .float = f });
}

pub fn toString(dt: *DtMachine) !void {
    const val = try dt.pop();
    const s = val.intoString(dt) catch |e| return dt.rewind(val, e);
    try dt.push(.{ .string = s });
}

pub fn toCommand(dt: *DtMachine) !void {
    const val = try dt.pop();
    const cmd = val.intoString(dt) catch |e| return dt.rewind(val, e);
    try dt.push(.{ .command = cmd });
}

pub fn toDef(dt: *DtMachine) !void {
    const val = try dt.pop();
    const cmd = val.intoString(dt) catch |e| return dt.rewind(val, e);
    try dt.push(.{ .deferred_command = cmd });
}

pub fn toQuote(dt: *DtMachine) !void {
    const val = try dt.pop();
    const quote = try val.intoQuote(dt);
    try dt.push(.{ .quote = quote });
}

pub fn toError(dt: *DtMachine) !void {
    const val = try dt.pop();
    var errName = try val.intoString(dt);
    errName = std.mem.trim(u8, errName, "~");

    try dt.push(.{ .err = errName });
}
