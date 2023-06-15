const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const SinglyLinkedList = std.SinglyLinkedList;
const StringHashMap = std.StringHashMap;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const tokens = @import("tokens.zig");
const Token = tokens.Token;

const RockString = []const u8;
pub const RockDictionary = StringHashMap(RockCommand);
pub const RockNest = SinglyLinkedList(RockStack);
pub const RockStack = SinglyLinkedList(RockVal);
pub const RockNode = RockStack.Node;

pub const RockError = error{
    TooManyRightBrackets,
    CommandUndefined,
    ContextStackUnderflow,
    StackUnderflow,
    WrongArguments,
    ToDont, // Something is unimplemented
};

pub const RockMachine = struct {
    curr: RockStack,
    nest: RockNest,
    depth: u8,
    dictionary: RockDictionary,

    pub fn init(dict: RockDictionary) !RockMachine {
        return .{
            .curr = RockStack{},
            .nest = RockNest{},
            .depth = 0,
            .dictionary = dict,
        };
    }

    pub fn interpret(self: *RockMachine, tok: Token) !RockMachine {
        switch (tok) {
            .term => |cmdName| return self.handleCmd(cmdName),
            .left_bracket => {
                self.pushContext();
                self.depth += 1;
            },
            .right_bracket => {
                self.depth -= 1;
                if (self.depth < 0) {
                    return RockError.TooManyRightBrackets;
                }
                var quote = try self.popContext();
                self.push(RockVal{ .quote = quote });
            },
            .bool => |b| self.push(RockVal{ .bool = b }),
            .i64 => |i| self.push(RockVal{ .i64 = i }),
            .f64 => |f| self.push(RockVal{ .f64 = f }),
            .string => |s| self.push(RockVal{ .string = s[0..] }),
            .deferred_term => |cmd| self.push(RockVal{ .command = cmd }),
            .none => {},
        }
        return self.*;
    }

    fn handle(self: *RockMachine, val: RockVal) anyerror!RockMachine {
        switch (val) {
            .command => |cmdName| return self.handleCmd(cmdName),
            else => self.push(val),
        }
        return self.*;
    }

    fn handleCmd(self: *RockMachine, cmdName: RockString) !RockMachine {
        if (self.depth > 0) {
            self.push(RockVal{ .command = cmdName });
            return self.*;
        }

        const cmd = self.dictionary.get(cmdName) orelse {
            try stderr.print("Undefined: {s}\n", .{cmdName});
            return RockError.CommandUndefined;
        };

        try stderr.print("Running command: {s}\n", .{cmd.name});

        return cmd.run(self);
    }

    pub fn push(self: *RockMachine, val: RockVal) void {
        stderr.print("Pushn: {any}\n", .{val}) catch {};
        var node = RockNode{ .data = val };
        self.curr.prepend(&node);
    }

    pub fn push2(self: *RockMachine, vals: RockVal2) void {
        self.push(vals.b);
        self.push(vals.a);
    }

    pub fn pop(self: *RockMachine) !RockVal {
        var top = self.curr.popFirst() orelse return RockError.StackUnderflow;
        return top.data;
    }

    // Returns tuple with most recent val in zero, next most in 1.
    pub fn pop2(self: *RockMachine) !RockVal2 {
        const a = try self.pop();
        const b = self.pop() catch |e| {
            self.push(a);
            return e;
        };
        return .{ .a = a, .b = b };
    }

    pub fn pushContext(self: *RockMachine) void {
        var prev = self.curr;
        var node = RockNest.Node{ .data = prev };
        self.nest.prepend(&node);

        self.curr = RockStack{};
    }

    pub fn popContext(self: *RockMachine) !RockStack {
        const curr = self.curr;
        const next = self.nest.popFirst() orelse return RockError.ContextStackUnderflow;
        self.curr = next.data;
        return curr;
    }
};

pub const RockVal = union(enum) {
    bool: bool,
    i64: i64,
    f64: f64,
    command: RockString,
    quote: RockStack,
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

    pub fn asQuote(self: RockVal) ?RockStack {
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
                    node = n.next;
                }
                try stdout.print(" ]", .{});
            },
            .string => |s| try stdout.print("{s}", .{s}),
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

    fn run(self: RockCommand, state: *RockMachine) !RockMachine {
        switch (self.action) {
            .builtin => |b| return b(state),
            .quote => |q| {
                var nextState = state.*;
                var curr = q;
                while (curr.popFirst()) |val| {
                    nextState = try nextState.handle(val.data);
                }
                return nextState;
            },
        }
    }
};

pub const RockAction = union(enum) {
    builtin: *const fn (*RockMachine) anyerror!RockMachine,
    quote: RockStack,
};

test "can I even list?" {
    const S = RockStack;
    var stack = S{};

    try std.testing.expect(stack.len() == 0);

    var tok = Token{ .string = "sup" };

    var s = S.Node{ .data = RockVal{ .string = tok.string } };
    stack.prepend(&s);

    try std.testing.expect(stack.len() == 1);

    var c = S.Node{ .data = RockVal{ .command = "pl" } };
    stack.prepend(&c);

    try std.testing.expect(stack.len() == 2);

    var c2 = stack.popFirst();
    try std.testing.expect(stack.len() == 1);
    try std.testing.expect(std.mem.eql(u8, c2.?.data.command, "pl"));

    var s2 = stack.popFirst();
    try std.testing.expect(stack.len() == 0);
    try std.testing.expect(std.mem.eql(u8, s2.?.data.string, "sup"));

    var n = stack.popFirst();
    try std.testing.expect(stack.len() == 0);
    try std.testing.expect(n == null);
}
