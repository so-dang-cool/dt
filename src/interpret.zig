const std = @import("std");
const Color = std.io.tty.Color;
const ArrayList = std.ArrayList;
const Stack = std.SinglyLinkedList;
const Allocator = std.mem.Allocator;
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

    inspiration: ArrayList(*[]const u8),

    pub fn init(alloc: Allocator) !DtMachine {
        var nest = Stack(Quote){};
        var mainNode = try alloc.create(Stack(Quote).Node);
        mainNode.* = Stack(Quote).Node{ .data = Quote.init(alloc) };
        nest.prepend(mainNode);

        var inspirations = ArrayList(*[]const u8).init(alloc);
        var lines = std.mem.tokenizeScalar(u8, inspiration, '\n');
        while (lines.next()) |line| {
            var copy = try alloc.dupe(u8, line);
            try inspirations.append(&copy);
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

        for (self.inspiration.items) |item| {
            self.alloc.free(item.*);
        }
        self.inspiration.deinit();

        var node = self.nest.first;
        while (node) |n| {
            node = n.next;
            n.data.deinit();
            self.alloc.destroy(n);
        }
    }

    pub fn interpret(self: *DtMachine, tok: Token) !void {
        tok.debugPrint();
        switch (tok) {
            .term => |cmdName| try self.handleCmd(try String.ofAlloc(cmdName, self.alloc)),
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
                try self.push(.{ .quote = context });
            },
            .bool => |b| try self.push(.{ .bool = b }),
            .int => |i| try self.push(.{ .int = i }),
            .float => |f| try self.push(.{ .float = f }),
            .string => |s| {
                var unescaped = try string.unescape(self.alloc, s);
                try self.push(.{ .string = try String.of(unescaped, self.alloc) });
            },
            .deferred_term => |cmd| try self.push(.{ .deferred_command = try String.ofAlloc(cmd, self.alloc) }),
            .none => {},
        }
    }

    pub fn handleVal(self: *DtMachine, val: Val) anyerror!void {
        const log = std.log.scoped(.@"dt.handleVal");

        switch (val) {
            .command => |name| {
                log.debug("DEBUG: name: {s}", .{name.str});
                self.handleCmd(name) catch |e| {
                    log.debug("DEBUG_ERR: name: {s}", .{name.str});
                    if (e != error.CommandUndefined) return e;
                    try self.red();
                    log.warn("Undefined: {s}", .{name.str});
                    name.releaseRef(self.alloc);
                    try self.norm();
                };
            },
            else => try self.push(val),
        }
    }

    /// Owns the cmdName.
    pub fn handleCmd(self: *DtMachine, cmdName: String) !void {
        const log = std.log.scoped(.@"dt.handleCmd");

        if (self.depth > 0) {
            try self.push(.{ .command = cmdName });
            return;
        }

        if (self.defs.get(cmdName.str)) |cmd| {
            try cmd.run(self);
            return;
        } else {
            try self.red();
            log.warn("Undefined: {s}", .{cmdName.str});
            try self.norm();
            cmdName.releaseRef(self.alloc);
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

    /// Clones the strings into dt Strings.
    pub fn define(self: *DtMachine, nameRaw: []const u8, descriptionRaw: []const u8, action: Action) !void {
        var allocator = self.alloc;

        const cmd = Command{
            .name = try String.ofAlloc(nameRaw, allocator),
            .description = try String.ofAlloc(descriptionRaw, allocator),
            .action = action,
        };
        try self.defs.put(nameRaw, cmd);
    }

    /// Causes the dictionary to own the name/description dt Strings.
    /// Caller should clone before calling if appropriate.
    pub fn defineDynamic(self: *DtMachine, name: String, description: String, action: Action) !void {
        const cmd = .{
            .name = name,
            .description = description,
            .action = action,
        };
        var key: []const u8 = name.str;
        try self.defs.put(key, cmd);
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

    pub fn run(self: Command, dt: *DtMachine) anyerror!void {
        switch (self.action) {
            .builtin => |b| return try b(dt),
            .quote => |quote| {
                var again = true;

                var vals = quote.items;
                var lastIndex = vals.len - 1;

                while (again) {
                    again = false;

                    for (vals[0..lastIndex]) |val| {
                        try dt.handleVal(val);
                    }

                    const lastVal = vals[lastIndex];

                    switch (lastVal) {
                        // Tail calls optimized, yay!
                        .command => |cmdName| {
                            // Even if this is the same command name, we should re-fetch in case it's been redefined
                            const cmd = dt.defs.get(cmdName.str) orelse return Error.CommandUndefined;
                            defer cmdName.releaseRef(dt.alloc);
                            switch (cmd.action) {
                                .quote => |nextQuote| {
                                    again = true;
                                    vals = nextQuote.items;
                                    lastIndex = vals.len - 1;
                                },
                                .builtin => |b| return try b(dt),
                            }
                        },
                        else => try dt.handleVal(lastVal),
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
