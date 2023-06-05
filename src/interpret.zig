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
            .bool => |b| {
                var node = RockNode{ .data = RockVal{ .boolean = b } };
                self.curr.prepend(&node);
            },
            .i64 => |i| {
                var node = RockNode{ .data = RockVal{ .i64 = i } };
                self.curr.prepend(&node);
            },
            .f64 => |f| {
                var node = RockNode{ .data = RockVal{ .f64 = f } };
                self.curr.prepend(&node);
            },
            .string => |s| {
                var node = RockNode{ .data = RockVal{ .string = s } };
                self.curr.prepend(&node);
            },
            .deferred_term => |cmd| {
                var node = RockNode{ .data = RockVal{ .command = cmd } };
                self.curr.prepend(&node);
            },
            .none => {},
        }
        return self.*;
    }

    fn handle(self: *RockMachine, val: RockVal) anyerror!RockMachine {
        switch (val) {
            .command => |cmdName| return self.handleCmd(cmdName),
            else => {
                var node = RockNode{ .data = val };
                self.curr.prepend(&node);
            },
        }
        return self.*;
    }

    fn handleCmd(self: *RockMachine, cmdName: RockString) !RockMachine {
        if (self.isNested()) {
            var node = RockNode{ .data = RockVal{ .command = cmdName } };
            self.curr.prepend(&node);
            return self.*;
        }

        const cmd = self.dictionary.get(cmdName) orelse {
            try stderr.print("Undefined: {s}\n", .{cmdName});
            return RockError.CommandUndefined;
        };
        return cmd.run(self);
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
