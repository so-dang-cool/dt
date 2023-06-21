const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Stack = std.SinglyLinkedList;
const StringHashMap = std.StringHashMap;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const tokens = @import("tokens.zig");
const Token = tokens.Token;

const RockString = []const u8;
pub const Dictionary = StringHashMap(RockCommand);

pub const Context = struct {
    stack: Stack(RockVal),
    defs: Dictionary,

    pub fn init(dict: Dictionary) Context {
        return .{
            .stack = .{},
            .defs = dict,
        };
    }
};

pub const Error = error{
    TooManyRightBrackets,
    CommandUndefined,
    ContextStackUnderflow,
    StackUnderflow,
    WrongArguments,
};

pub const RockMachine = struct {
    alloc: Allocator,
    nest: Stack(Context),
    depth: u8,

    pub fn init(alloc: Allocator) !RockMachine {
        var nest = Stack(Context){};
        var mainNode = try alloc.create(Stack(Context).Node);
        mainNode.* = Stack(Context).Node{ .data = Context.init(Dictionary.init(alloc)) };
        nest.prepend(mainNode);

        return .{
            .alloc = alloc,
            .nest = nest,
            .depth = 0,
        };
    }

    pub fn interpret(self: *RockMachine, tok: Token) !void {
        switch (tok) {
            .term => |cmdName| try self.handleCmd(cmdName),
            .left_bracket => {
                try self.pushContext();
                self.depth += 1;
            },
            .right_bracket => {
                self.depth -= 1;

                if (self.depth < 0) {
                    return Error.TooManyRightBrackets;
                }

                // TODO: Keep track of context length and store as array in RockVal (Avoid this reversal; faster forward traversal).
                // Or should it just be an ArrayList after all?
                var context = try self.popContext();
                var reversed = context.stack;
                var quote = Stack(RockVal){};

                while (reversed.popFirst()) |node| {
                    quote.prepend(node);
                }

                try self.push(RockVal{ .quote = quote });
            },
            .bool => |b| try self.push(RockVal{ .bool = b }),
            .i64 => |i| try self.push(RockVal{ .i64 = i }),
            .f64 => |f| try self.push(RockVal{ .f64 = f }),
            .string => |s| try self.push(RockVal{ .string = s[0..] }),
            .deferred_term => |cmd| try self.push(RockVal{ .command = cmd }),
            .none => {},
        }
    }

    fn handle(self: *RockMachine, val: RockVal) anyerror!void {
        switch (val) {
            .command => |cmdName| try self.handleCmd(cmdName),
            else => try self.push(val),
        }
    }

    fn handleCmd(self: *RockMachine, cmdName: RockString) !void {
        if (self.depth > 0) {
            try self.push(RockVal{ .command = cmdName });
            return;
        }

        var node = self.nest.first;
        while (node) |n| : (node = n.next) {
            if (n.data.defs.get(cmdName)) |cmd| {
                // try stderr.print("Running command: {s}\n", .{cmd.name});
                try cmd.run(self);
                return;
            }
        }

        try stderr.print("Undefined: {s}\n", .{cmdName});
        return Error.CommandUndefined;
    }

    pub fn define(self: *RockMachine, name: RockString, description: RockString, action: RockAction) !void {
        try self.nest.first.?.data.defs.put(name, RockCommand{ .name = name, .description = description, .action = action });
    }

    pub fn push(self: *RockMachine, val: RockVal) !void {
        var node = try self.alloc.create(Stack(RockVal).Node);
        node.* = .{ .data = val };
        var top = self.nest.first orelse return Error.ContextStackUnderflow;
        top.data.stack.prepend(node);
    }

    pub fn push2(self: *RockMachine, vals: RockVal2) !void {
        try self.push(vals.b);
        try self.push(vals.a);
    }

    pub fn pop(self: *RockMachine) !RockVal {
        var top = self.nest.first orelse return Error.ContextStackUnderflow;
        var topVal = top.data.stack.popFirst() orelse return Error.StackUnderflow;
        return topVal.data;
    }

    // Returns tuple with a=older, b=newer
    pub fn pop2(self: *RockMachine) !RockVal2 {
        const b = try self.pop();
        const a = self.pop() catch |e| {
            try self.push(b);
            return e;
        };
        return .{ .a = a, .b = b };
    }

    pub fn pushContext(self: *RockMachine) !void {
        var node = try self.alloc.create(Stack(Context).Node);
        node.* = .{ .data = .{ .stack = .{}, .defs = Dictionary.init(self.alloc) } };
        self.nest.prepend(node);
    }

    pub fn popContext(self: *RockMachine) !Context {
        var node = self.nest.popFirst() orelse return Error.ContextStackUnderflow;
        return node.data;
    }
};

pub const RockVal = union(enum) {
    bool: bool,
    i64: i64,
    f64: f64,
    command: RockString,
    quote: Stack(RockVal),
    string: RockString,
    // TODO: HashMap<RockVal, RockVal, ..., ...>

    pub fn asBool(self: RockVal) ?bool {
        return switch (self) {
            .bool => |b| b,
            else => null,
        };
    }

    pub fn asI64(self: RockVal) ?i64 {
        return switch (self) {
            .i64 => |i| i,
            else => null,
        };
    }

    pub fn asF64(self: RockVal) ?f64 {
        return switch (self) {
            .f64 => |f| f,
            else => null,
        };
    }

    pub fn asCommand(self: RockVal) ?RockString {
        return switch (self) {
            .command => |cmd| cmd,
            else => null,
        };
    }

    pub fn asQuote(self: RockVal) ?Stack(RockVal) {
        return switch (self) {
            .quote => |q| q,
            else => null,
        };
    }

    pub fn asString(self: RockVal) ?RockString {
        return switch (self) {
            .string => |s| s,
            else => null,
        };
    }

    pub fn print(self: RockVal) !void {
        switch (self) {
            .bool => |b| try stdout.print("{}", .{b}),
            .i64 => |i| try stdout.print("{}", .{i}),
            .f64 => |f| try stdout.print("{}", .{f}),
            .command => |cmd| try stdout.print("\\{s}", .{cmd}),
            .quote => |q| {
                try stdout.print("[ ", .{});
                var node = q.first;
                while (node) |n| {
                    try n.data.print();
                    try stdout.print(" ", .{});
                    node = n.next;
                }
                try stdout.print("]", .{});
            },
            .string => |s| try stdout.print("\"{s}\"", .{s}),
        }
    }
};

pub const RockVal2 = struct {
    a: RockVal,
    b: RockVal,
};

pub const RockCommand = struct {
    name: RockString,
    description: RockString,
    action: RockAction,

    fn run(self: RockCommand, state: *RockMachine) !void {
        switch (self.action) {
            .builtin => |b| return try b(state),
            .quote => |quote| {
                var node = quote.first;
                while (node) |n| : (node = n.next) {
                    state.handle(n.data) catch |e| {
                        return e;
                    };
                }
            },
        }
    }
};

pub const RockAction = union(enum) {
    builtin: *const fn (*RockMachine) anyerror!void,
    quote: Stack(RockVal),
};
