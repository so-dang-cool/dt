const std = @import("std");
const Atomic = std.atomic.Atomic;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const StringHashMap = std.StringHashMap;

const interpret = @import("interpret.zig");
const Command = interpret.Command;
const DtMachine = interpret.DtMachine;

pub const string = @import("types/string.zig");
pub const String = string.String;

pub const Quote = @import("types/Quote.zig");

pub const Dictionary = StringHashMap(Command);

pub const Val = union(enum) {
    bool: bool,
    int: i64, // TODO: std.math.big.int.Mutable?
    float: f64, // TODO: std.math.big.Rational?
    string: String,
    command: String,
    deferred_command: String,
    quote: Quote,

    pub fn deinit(self: *Val, state: *DtMachine) void {
        switch (self.*) {
            .string => |s| state.alloc.free(s),
            .command => |cmd| state.alloc.free(cmd),
            .deferred_command => |cmd| state.alloc.free(cmd),
            .quote => |q| {
                for (q.items) |*i| {
                    i.deinit(state);
                }
                q.deinit();
            },
            else => {},
        }
    }

    pub fn isBool(self: Val) bool {
        return switch (self) {
            .bool => true,
            else => false,
        };
    }

    pub fn intoBool(self: Val, state: *DtMachine) bool {
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

    pub fn isInt(self: Val) bool {
        return switch (self) {
            .int => true,
            else => false,
        };
    }

    pub fn intoInt(self: Val) !i64 {
        return switch (self) {
            .int => |i| i,

            .bool => |b| if (b) 1 else 0,
            .float => |f| @as(i64, @intFromFloat(f)),
            .string => |s| std.fmt.parseInt(i64, s, 10),
            else => Error.NoCoercionToInteger,
        };
    }

    pub fn isFloat(self: Val) bool {
        return switch (self) {
            .float => true,
            else => false,
        };
    }

    pub fn intoFloat(self: Val) !f64 {
        return switch (self) {
            .float => |f| f,

            .bool => |b| if (b) 1 else 0,
            .int => |i| @as(f64, @floatFromInt(i)),
            .string => |s| std.fmt.parseFloat(f64, s),
            else => Error.NoCoercionToInteger,
        };
    }

    pub fn isCommand(self: Val) bool {
        return switch (self) {
            .command => true,
            else => false,
        };
    }

    pub fn isDeferredCommand(self: Val) bool {
        return switch (self) {
            .deferred_command => true,
            else => false,
        };
    }

    pub fn isString(self: Val) bool {
        return switch (self) {
            .string => true,
            else => false,
        };
    }

    pub fn intoString(self: Val, state: *DtMachine) !String {
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
                else => Error.NoCoercionToString,
            },
        };
    }

    pub fn isQuote(self: Val) bool {
        return switch (self) {
            .quote => true,
            else => false,
        };
    }

    pub fn intoQuote(self: Val, state: *DtMachine) !Quote {
        return switch (self) {
            .quote => |q| q,
            else => {
                var q = Quote.init(state.alloc);
                try q.append(self);
                return q;
            },
        };
    }

    pub fn deepClone(self: Val, state: *DtMachine) anyerror!Val {
        switch (self) {
            .string => |s| {
                var cloned = try state.alloc.dupe(u8, s);
                return .{ .string = cloned };
            },
            .command => |cmd| {
                var cloned = try state.alloc.dupe(u8, cmd);
                return .{ .command = cloned };
            },
            .deferred_command => |cmd| {
                var cloned = try state.alloc.dupe(u8, cmd);
                return .{ .deferred_command = cloned };
            },
            .quote => |q| return .{ .quote = try _deepClone(q, state) },
            else => return self,
        }
    }

    fn _deepClone(quote: Quote, state: *DtMachine) anyerror!Quote {
        var cloned = try Quote.initCapacity(state.alloc, quote.items.len);
        for (quote.items) |item| {
            try cloned.append(try item.deepClone(state));
        }
        return cloned;
    }

    pub fn isEqualTo(dt: *DtMachine, lhs: Val, rhs: Val) bool {
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

            const as: []Val = quoteA.items;
            const bs: []Val = quoteB.items;

            if (as.len != bs.len) return false;

            for (as, bs) |a, b| {
                if (!Val.isEqualTo(dt, a, b)) return false;
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
    pub fn isLessThan(dt: *DtMachine, lhs: Val, rhs: Val) bool {

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
                if (Val.isLessThan(dt, a, b)) return true;
            }

            if (as.items.len < bs.items.len) return true;
        }

        return false;
    }

    pub fn print(self: Val, writer: std.fs.File.Writer) !void {
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

pub const Error = error{
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
