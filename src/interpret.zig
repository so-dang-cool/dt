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

pub const RockError = error{
    TooManyRightBrackets,
    CommandUndefined,
    StackUnderflow,
    ToDont, // Something is unimplemented
};

pub const RockMachine = struct {
    curr: *RockStack,
    nest: ArrayList(*RockStack),
    dictionary: RockDictionary,

    pub fn init(alloc: Allocator, dict: RockDictionary) !RockMachine {
        var stack = RockStack{};
        return .{
            .curr = &stack,
            .nest = ArrayList(*RockStack).init(alloc),
            .dictionary = dict,
        };
    }

    pub fn interpret(self: *RockMachine, tok: Token) !RockMachine {
        switch (tok) {
            .term => |cmdName| return self.handleCmd(cmdName),
            .left_bracket => {
                try self.nest.append(self.curr);
                var newCurr = RockStack{};
                self.curr = &newCurr;
            },
            .right_bracket => {
                self.curr = self.nest.popOrNull() orelse return RockError.TooManyRightBrackets;
            },
            .bool => |b| self.push(RockVal{ .boolean = b }),
            .i64 => |i| self.push(RockVal{ .i64 = i }),
            .f64 => |f| self.push(RockVal{ .f64 = f }),
            .string => |s| self.push(RockVal{ .string = s }),
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
        if (self.isNested()) {
            self.push(RockVal{ .command = cmdName });
            return self.*;
        }

        const cmd = self.dictionary.get(cmdName) orelse {
            try stderr.print("Undefined: {s}\n", .{cmdName});
            return RockError.CommandUndefined;
        };
        return cmd.run(self);
    }

    pub fn push(self: *RockMachine, val: RockVal) void {
        var node = RockNode{ .data = val };
        self.curr.prepend(&node);
    }

    pub fn push2(self: *RockMachine, vals: RockVal2) void {
        self.push(vals.b);
        self.push(vals.a);
    }

    pub fn pop(self: *RockMachine) !RockVal {
        const top = self.curr.popFirst() orelse return RockError.StackUnderflow;
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

    fn isNested(self: RockMachine) bool {
        return self.nest.items.len > 0;
    }
};

pub const RockStack = SinglyLinkedList(RockVal);
pub const RockNode = RockStack.Node;

pub const RockVal = union(enum) {
    boolean: bool,
    i64: i64,
    f64: f64,
    command: RockString,
    quote: *RockStack,
    string: RockString,
    // TODO: HashMap<RockVal, RockVal, ..., ...>
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
