const std = @import("std");
const Color = std.io.tty.Color;
const ArrayList = std.ArrayList;
const Stack = std.SinglyLinkedList;
const Allocator = std.mem.Allocator;
const Integer = std.math.big.int.Managed;
const Rational = std.math.big.Rational;
// const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const string = @import("string.zig");
const String = string.String;

const Token = @import("tokens.zig").Token;

const inspiration = @embedFile("inspiration");

const types = @import("types.zig");
const Error = types.Error;
const Dictionary = types.Dictionary;
const Val = types.Val;
const Quote = types.Quote;

pub const DtMachine = struct {
    alloc: Allocator,

    nest: Stack(ArrayList(Val)),
    depth: u8,

    defs: Dictionary,

    stdoutConfig: std.io.tty.Config,
    stderrConfig: std.io.tty.Config,

    inspiration: ArrayList(String),

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
            .inspiration = inspirations,
        };
    }

    pub fn deinit(self: *DtMachine) void {
        self.defs.deinit();
        self.inspiration.deinit();

        var node = self.nest.first;
        while (node) |n| {
            node = n.next;
            n.data.deinit();
            self.alloc.destroy(n);
        }
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
                    return Error.TooManyRightBrackets;
                }

                self.depth -= 1;

                var context = try self.popContext();
                try self.push(Val{ .quote = context });
            },
            .bool => |b| try self.push(Val{ .bool = b }),
            .int => |i| try self.push(Val{ .int = i }),
            .float => |f| try self.push(Val{ .float = f }),
            .string => |s| {
                const unescaped = try string.unescape(self.alloc, s);
                try self.push(Val{ .string = unescaped });
            },
            .deferred_term => |cmd| try self.push(Val{ .deferred_command = cmd }),
            .none => {},
        }
    }

    pub fn handleVal(self: *DtMachine, val: Val) anyerror!void {
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
            // TODO: Ensure that this is never necessary; Clone immediately before words that mutate for efficiency.
            else => try self.push(try val.deepClone(self)),
        }
    }

    pub fn handleCmd(self: *DtMachine, cmdName: String) !void {
        const log = std.log.scoped(.@"dt.handleCmd");

        if (self.depth > 0) {
            try self.push(Val{ .command = cmdName });
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

    pub fn rewind(self: *DtMachine, log: anytype, val: Val, err: anyerror) anyerror!void {
        log.warn("{s}, rewinding {any}", .{ @errorName(err), val });
        try self.push(val);
        return err;
    }

    pub fn rewindN(self: *DtMachine, comptime n: comptime_int, log: anytype, vals: [n]Val, err: anyerror) anyerror!void {
        log.warn("{s}, rewinding {any}", .{ @errorName(err), vals });
        for (vals) |val| try self.push(val);
        return err;
    }

    pub fn push(self: *DtMachine, val: Val) !void {
        var top = self.nest.first orelse return Error.ContextStackUnderflow;
        try top.data.append(val);
    }

    pub fn pushN(self: *DtMachine, comptime n: comptime_int, vals: [n]Val) !void {
        // TODO: push as slice
        for (vals) |val| try self.push(val);
    }

    pub fn pop(self: *DtMachine) !Val {
        var top = self.nest.first orelse return Error.ContextStackUnderflow;
        if (top.data.items.len < 1) {
            return Error.StackUnderflow;
        }
        return top.data.pop();
    }

    // Removes and returns top N values from the stack from oldest to youngest. Last index is the most recent, 0 is the oldest.
    pub fn popN(self: *DtMachine, comptime n: comptime_int) ![n]Val {
        var vals: [n]Val = .{};

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
        var node = self.nest.popFirst() orelse return Error.ContextStackUnderflow;
        return node.data;
    }

    pub fn quoteContext(self: *DtMachine) !void {
        var node = self.nest.popFirst();
        var quote = if (node) |n| n.data else Quote.init(self.alloc);

        if (self.nest.first == null) try self.pushContext();

        try self.push(.{ .quote = quote });
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
                            const cmd = state.defs.get(cmdName) orelse return Error.CommandUndefined;
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
