const std = @import("std");
const Color = std.io.tty.Color;
const ArrayList = std.ArrayList;
const Stack = std.SinglyLinkedList;
const Allocator = std.mem.Allocator;
const StringHashMap = std.StringHashMap;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const string = @import("string.zig");
const String = string.String;

const Token = @import("tokens.zig").Token;

const inspiration = @embedFile("inspiration");

pub const Quote = ArrayList(DtVal);
pub const Dictionary = StringHashMap(Command);

pub const DtError = error{
    TooManyRightBrackets,

    ContextStackUnderflow,
    StackUnderflow,
    WrongArguments,
    CommandUndefined,

    DivisionByZero,
    IntegerOverflow,
    IntegerUnderflow,

    NoCoercionToInteger,
    NoCoercionToFloat,
    NoCoercionToString,
    NoCoercionToCommand,

    ProcessNameUnknown,
};

pub const DtMachine = struct {
    alloc: Allocator,

    nest: Stack(ArrayList(DtVal)),
    depth: u8,

    defs: Dictionary,

    stdoutConfig: std.io.tty.Config,
    stderrConfig: std.io.tty.Config,

    inspiration: []String,

    pub fn init(alloc: Allocator) !DtMachine {
        var nest = Stack(Quote){};
        var mainNode = try alloc.create(Stack(Quote).Node);
        mainNode.* = Stack(Quote).Node{ .data = Quote.init(alloc) };
        nest.prepend(mainNode);

        var inspirations = ArrayList(String).init(alloc);
        var lines = std.mem.tokenizeScalar(u8, inspiration, '\n');
        while (lines.next()) |line| {
            try inspirations.append(line);
        }

        return .{
            .alloc = alloc,
            .nest = nest,
            .depth = 0,
            .defs = Dictionary.init(alloc),
            .stdoutConfig = std.io.tty.detectConfig(std.io.getStdOut()),
            .stderrConfig = std.io.tty.detectConfig(std.io.getStdErr()),
            .inspiration = inspirations.items,
        };
    }

    pub fn interpret(self: *DtMachine, tok: Token) !void {
        switch (tok) {
            .term => |cmdName| try self.handleCmd(cmdName),
            .left_bracket => {
                try self.pushContext();
                self.depth += 1;
            },
            .right_bracket => {
                if (self.depth == 0) {
                    return DtError.TooManyRightBrackets;
                }

                self.depth -= 1;

                var context = try self.popContext();
                try self.push(DtVal{ .quote = context });
            },
            .bool => |b| try self.push(DtVal{ .bool = b }),
            .int => |i| try self.push(DtVal{ .int = i }),
            .float => |f| try self.push(DtVal{ .float = f }),
            .string => |s| {
                const unescaped = try string.unescape(self.alloc, s);
                try self.push(DtVal{ .string = unescaped });
            },
            .deferred_term => |cmd| try self.push(DtVal{ .deferred_command = cmd }),
            .none => {},
        }
    }

    pub fn handleVal(self: *DtMachine, val: DtVal) anyerror!void {
        const log = std.log.scoped(.@"dt.handleVal");

        switch (val) {
            .command => |name| {
                self.handleCmd(name) catch |e| {
                    if (e != error.CommandUndefined) return e;
                    try self.red();
                    log.warn("Undefined: {s}", .{name});
                    try self.norm();
                };
            },
            else => try self.push(val),
        }
    }

    pub fn handleCmd(self: *DtMachine, cmdName: String) !void {
        const log = std.log.scoped(.@"dt.handleCmd");

        if (self.depth > 0) {
            try self.push(DtVal{ .command = cmdName });
            return;
        }

        if (self.defs.get(cmdName)) |cmd| {
            try cmd.run(self);
            return;
        } else {
            try self.red();
            log.warn("Undefined: {s}", .{cmdName});
            try self.norm();
        }
    }

    pub fn loadFile(self: *DtMachine, code: []const u8) !void {
        var toks = Token.parse(self.alloc, code);
        while (try toks.next()) |token| try self.interpret(token);
    }

    pub fn red(self: *DtMachine) !void {
        try self.norm();
        try self.stdoutConfig.setColor(stdout, Color.red);
        try self.stderrConfig.setColor(stderr, Color.red);
    }

    pub fn green(self: *DtMachine) !void {
        try self.stdoutConfig.setColor(stdout, Color.green);
        try self.stdoutConfig.setColor(stdout, Color.bold);

        try self.stderrConfig.setColor(stderr, Color.green);
        try self.stdoutConfig.setColor(stdout, Color.bold);
    }

    pub fn norm(self: *DtMachine) !void {
        try self.stdoutConfig.setColor(stdout, Color.reset);
        try self.stderrConfig.setColor(stderr, Color.reset);
    }

    pub fn child(self: *DtMachine) !DtMachine {
        var newMachine = try DtMachine.init(self.alloc);

        // TODO: Persistent map for dictionary would make this much cheaper.
        newMachine.defs = try self.defs.clone();

        return newMachine;
    }

    pub fn define(self: *DtMachine, name: String, description: String, action: Action) !void {
        const cmd = Command{ .name = name, .description = description, .action = action };
        try self.defs.put(name, cmd);
    }

    pub fn rewind(self: *DtMachine, log: anytype, val: DtVal, err: anyerror) anyerror!void {
        log.warn("{s}, rewinding {any}", .{ @errorName(err), val });
        try self.push(val);
        return err;
    }

    pub fn rewindN(self: *DtMachine, comptime n: comptime_int, log: anytype, vals: [n]DtVal, err: anyerror) anyerror!void {
        log.warn("{s}, rewinding {any}", .{ @errorName(err), vals });
        for (vals) |val| try self.push(val);
        return err;
    }

    pub fn push(self: *DtMachine, val: DtVal) !void {
        var top = self.nest.first orelse return DtError.ContextStackUnderflow;
        try top.data.append(val);
    }

    pub fn pushN(self: *DtMachine, comptime n: comptime_int, vals: [n]DtVal) !void {
        // TODO: push as slice
        for (vals) |val| try self.push(val);
    }

    pub fn pop(self: *DtMachine) !DtVal {
        var top = self.nest.first orelse return DtError.ContextStackUnderflow;
        if (top.data.items.len < 1) {
            return DtError.StackUnderflow;
        }
        return top.data.pop();
    }

    // Removes and returns top N values from the stack from oldest to youngest. Last index is the most recent, 0 is the oldest.
    pub fn popN(self: *DtMachine, comptime n: comptime_int) ![n]DtVal {
        var vals: [n]DtVal = .{};

        comptime var i = n - 1;
        inline while (i >= 0) : (i -= 1) {
            vals[i] = self.pop() catch |e| {
                comptime var j = i + 1;
                inline while (j > n) : (j += 1) {
                    try self.push(vals[j]);
                }
                return e;
            };
        }

        return vals;
    }

    pub fn pushContext(self: *DtMachine) !void {
        var node = try self.alloc.create(Stack(Quote).Node);
        node.* = .{ .data = Quote.init(self.alloc) };
        self.nest.prepend(node);
    }

    pub fn popContext(self: *DtMachine) !Quote {
        var node = self.nest.popFirst() orelse return DtError.ContextStackUnderflow;
        return node.data;
    }

    pub fn quoteContext(self: *DtMachine) !void {
        var node = self.nest.popFirst();
        var quote = if (node) |n| n.data else Quote.init(self.alloc);

        if (self.nest.first == null) try self.pushContext();

        try self.push(.{ .quote = quote });
    }
};

pub const DtVal = union(enum) {
    bool: bool,
    int: i64, // TODO: std.math.big.int.Mutable?
    float: f64, // TODO: std.math.big.Rational?
    string: String,
    command: String,
    deferred_command: String,
    quote: Quote,

    pub fn isBool(self: DtVal) bool {
        return switch (self) {
            .bool => true,
            else => false,
        };
    }

    pub fn intoBool(self: DtVal, state: *DtMachine) bool {
        return switch (self) {
            .bool => |b| b,
            .int => |i| i > 0,
            .float => |f| f > 0,
            .string => |s| !std.mem.eql(u8, "", s),
            .quote => |q| q.items.len > 0,

            // Commands are truthy if defined
            .command => |cmd| state.defs.contains(cmd),
            .deferred_command => |cmd| state.defs.contains(cmd),
        };
    }

    pub fn isInt(self: DtVal) bool {
        return switch (self) {
            .int => true,
            else => false,
        };
    }

    pub fn intoInt(self: DtVal) !i64 {
        return switch (self) {
            .int => |i| i,

            .bool => |b| if (b) 1 else 0,
            .float => |f| @as(i64, @intFromFloat(f)),
            .string => |s| std.fmt.parseInt(i64, s, 10),
            else => DtError.NoCoercionToInteger,
        };
    }

    pub fn isFloat(self: DtVal) bool {
        return switch (self) {
            .float => true,
            else => false,
        };
    }

    pub fn intoFloat(self: DtVal) !f64 {
        return switch (self) {
            .float => |f| f,

            .bool => |b| if (b) 1 else 0,
            .int => |i| @as(f64, @floatFromInt(i)),
            .string => |s| std.fmt.parseFloat(f64, s),
            else => DtError.NoCoercionToInteger,
        };
    }

    pub fn isCommand(self: DtVal) bool {
        return switch (self) {
            .command => true,
            else => false,
        };
    }

    pub fn isDeferredCommand(self: DtVal) bool {
        return switch (self) {
            .deferred_command => true,
            else => false,
        };
    }

    pub fn isString(self: DtVal) bool {
        return switch (self) {
            .string => true,
            else => false,
        };
    }

    pub fn intoString(self: DtVal, state: *DtMachine) !String {
        return switch (self) {
            .command => |cmd| cmd,

            .deferred_command => |cmd| cmd,
            .string => |s| s,
            .bool => |b| if (b) "true" else "false",
            .int => |i| try std.fmt.allocPrint(state.alloc, "{}", .{i}),
            .float => |f| try std.fmt.allocPrint(state.alloc, "{}", .{f}),
            .quote => |q| switch (q.items.len) {
                0 => "",
                1 => q.items[0].intoString(state),
                else => DtError.NoCoercionToString,
            },
        };
    }

    pub fn isQuote(self: DtVal) bool {
        return switch (self) {
            .quote => true,
            else => false,
        };
    }

    pub fn intoQuote(self: DtVal, state: *DtMachine) !Quote {
        return switch (self) {
            .quote => |q| q,
            else => {
                var q = Quote.init(state.alloc);
                try q.append(self);
                return q;
            },
        };
    }

    pub fn isEqualTo(dt: *DtMachine, lhs: DtVal, rhs: DtVal) bool {
        if (lhs.isBool() and rhs.isBool()) {
            const a = lhs.intoBool(dt);
            const b = rhs.intoBool(dt);

            return a == b;
        }

        if (lhs.isInt() and rhs.isInt()) {
            const a = lhs.intoInt() catch unreachable;
            const b = rhs.intoInt() catch unreachable;

            return a == b;
        }

        if ((lhs.isInt() or lhs.isFloat()) and (rhs.isInt() or rhs.isFloat())) {
            const a = lhs.intoFloat() catch unreachable;
            const b = rhs.intoFloat() catch unreachable;

            return a == b;
        }

        if (lhs.isString() and rhs.isString()) {
            const a = lhs.intoString(dt) catch unreachable;
            const b = rhs.intoString(dt) catch unreachable;

            return std.mem.eql(u8, a, b);
        }

        if (lhs.isCommand() and rhs.isCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            const b = rhs.intoString(dt) catch unreachable;

            return std.mem.eql(u8, a, b);
        }

        if (lhs.isDeferredCommand() and rhs.isDeferredCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            const b = rhs.intoString(dt) catch unreachable;

            return std.mem.eql(u8, a, b);
        }

        if (lhs.isQuote() and rhs.isQuote()) {
            const quoteA = lhs.intoQuote(dt) catch unreachable;
            const quoteB = rhs.intoQuote(dt) catch unreachable;

            const as: []DtVal = quoteA.items;
            const bs: []DtVal = quoteB.items;

            if (as.len != bs.len) return false;

            for (as, bs) |a, b| {
                if (!DtVal.isEqualTo(dt, a, b)) return false;
            }

            // Length is equal and all values were equal
            return true;
        }

        return false;
    }

    /// This provides the following "natural" ordering when vals are different types:
    /// bool, int/float, string, command, deferred command, quote
    ///
    /// Strings are compared character-by-character (lexicographically), where
    /// capital letters are "less than" lowercase letters. When one string is a
    /// prefix of another, the shorter string is "less than" the other.
    ///
    /// (The same string rules apply to command and deferred command names.)
    ///
    /// Quotes are compared value-by-value. When one quote is a prefix of
    /// another, the shorter quote is "less than" the other.
    pub fn isLessThan(dt: *DtMachine, lhs: DtVal, rhs: DtVal) bool {

        // We'll consider a bool comparison "less than" when lhs = false and rhs = true
        if (lhs.isBool() and rhs.isBool()) return !lhs.intoBool(dt) and rhs.intoBool(dt);
        if (lhs.isBool()) return true;

        if (lhs.isInt() and rhs.isInt()) {
            const a = lhs.intoInt() catch unreachable;
            const b = rhs.intoInt() catch unreachable;
            return a < b;
        }
        if ((lhs.isInt() or lhs.isFloat()) and (rhs.isInt() or rhs.isFloat())) {
            const a = lhs.intoFloat() catch unreachable;
            const b = rhs.intoFloat() catch unreachable;
            return a < b;
        }
        if (lhs.isInt()) return true;
        if (lhs.isFloat()) return true;

        if (lhs.isString() and rhs.isString()) {
            const a = lhs.intoString(dt) catch unreachable;
            const b = rhs.intoString(dt) catch unreachable;
            return std.mem.lessThan(u8, a, b);
        }
        if (lhs.isString()) return true;

        if (lhs.isCommand() and rhs.isCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            const b = rhs.intoString(dt) catch unreachable;
            return std.mem.lessThan(u8, a, b);
        }
        if (lhs.isCommand()) return true;

        if (lhs.isDeferredCommand() and rhs.isDeferredCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            const b = rhs.intoString(dt) catch unreachable;
            return std.mem.lessThan(u8, a, b);
        }
        if (lhs.isDeferredCommand()) return true;

        if (lhs.isQuote() and rhs.isQuote()) {
            const as = lhs.intoQuote(dt) catch unreachable;
            const bs = rhs.intoQuote(dt) catch unreachable;

            for (as.items, bs.items) |a, b| {
                if (DtVal.isLessThan(dt, a, b)) return true;
            }

            if (as.items.len < bs.items.len) return true;
        }

        return false;
    }

    pub fn print(self: DtVal, writer: std.fs.File.Writer) !void {
        switch (self) {
            .bool => |b| try writer.print("{}", .{b}),
            .int => |i| try writer.print("{}", .{i}),
            .float => |f| try writer.print("{d}", .{f}),
            .command => |cmd| try writer.print("{s}", .{cmd}),
            .deferred_command => |cmd| try writer.print("\\{s}", .{cmd}),
            .quote => |q| {
                try writer.print("[ ", .{});
                for (q.items) |val| {
                    try val.print(writer);
                    try writer.print(" ", .{});
                }
                try writer.print("]", .{});
            },
            .string => |s| try writer.print("\"{s}\"", .{s}),
        }
    }
};

pub const Command = struct {
    name: String,
    description: String,
    action: Action,

    pub fn run(self: Command, state: *DtMachine) anyerror!void {
        switch (self.action) {
            .builtin => |b| return try b(state),
            .quote => |quote| {
                var again = true;

                var vals = quote.items;
                var lastIndex = vals.len - 1;

                while (again) {
                    again = false;

                    for (vals[0..lastIndex]) |val| {
                        try state.handleVal(val);
                    }

                    const lastVal = vals[lastIndex];

                    switch (lastVal) {
                        // Tail calls optimized, yay!
                        .command => |cmdName| {
                            // Even if this is the same command name, we should re-fetch in case it's been redefined
                            const cmd = state.defs.get(cmdName) orelse return DtError.CommandUndefined;
                            switch (cmd.action) {
                                .quote => |nextQuote| {
                                    again = true;
                                    vals = nextQuote.items;
                                    lastIndex = vals.len - 1;
                                },
                                .builtin => |b| return try b(state),
                            }
                        },
                        else => try state.handleVal(lastVal),
                    }
                }
            },
        }
    }
};

pub const Action = union(enum) {
    builtin: *const fn (*DtMachine) anyerror!void,
    quote: Quote,
};
