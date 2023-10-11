const std = @import("std");
const Atomic = std.atomic.Atomic;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.hash_map.HashMap;
const StringHashMap = std.StringHashMap;

const string = @import("string.zig");
const String = string.String;

const interpret = @import("interpret.zig");
const Command = interpret.Command;
const DtMachine = interpret.DtMachine;

/// dt quote
pub const Quote = ArrayList(Val);

/// dt dictionary
pub const Dictionary = StringHashMap(Command);

pub const Val = union(enum) {
    bool: bool,
    int: i64, // TODO: std.math.big.int.Mutable?
    float: f64, // TODO: std.math.big.Rational?
    string: String,
    command: String,
    deferred_command: String,
    quote: Quote,

    pub fn deinit(self: *Val, dt: *DtMachine) void {
        switch (self.*) {
            .string => |s| s.releaseRef(dt.alloc),
            .command => |cmd| cmd.releaseRef(dt.alloc),
            .deferred_command => |cmd| cmd.releaseRef(dt.alloc),
            .quote => |q| {
                for (q.items) |*i| {
                    i.deinit(dt);
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

    pub fn intoBool(self: Val, dt: *DtMachine) bool {
        return switch (self) {
            .bool => |b| b,
            .int => |i| i > 0,
            .float => |f| f > 0,
            .string => |s| {
                defer s.releaseRef(dt.alloc);
                return !std.mem.eql(u8, "", s.str);
            },
            .quote => |q| q.items.len > 0,

            // Commands are truthy if defined
            .command => |cmd| {
                defer cmd.releaseRef(dt.alloc);
                return dt.defs.contains(cmd.str);
            },
            .deferred_command => |cmd| {
                defer cmd.releaseRef(dt.alloc);
                return dt.defs.contains(cmd.str);
            },
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
            .string => |s| std.fmt.parseInt(i64, s.str, 10), // TODO: releaseRef
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
            .string => |s| std.fmt.parseFloat(f64, s.str),
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

    pub fn intoByteString(self: Val, dt: *DtMachine) ![]const u8 {
        return switch (self) {
            .command => |cmd| cmd.str,

            .deferred_command => |cmd| cmd.str,
            .string => |s| s.str,
            .bool => |b| if (b) "true" else "false", // TODO: Could be constants from state
            .int => |i| try std.fmt.allocPrint(dt.alloc, "{}", .{i}),
            .float => |f| try std.fmt.allocPrint(dt.alloc, "{}", .{f}),
            .quote => |q| switch (q.items.len) {
                0 => "", // TODO: Could be constant from state
                1 => q.items[0].intoByteString(dt),
                else => Error.NoCoercionToString,
            },
        };
    }

    pub fn intoString(self: Val, dt: *DtMachine) !String {
        return switch (self) {
            .command => |cmd| cmd,

            .deferred_command => |cmd| cmd,
            .string => |s| s,
            .bool => |b| String.ofAlloc(if (b) "true" else "false", dt.alloc), // TODO: Could be constants from state
            .int => |i| String.ofAlloc(try std.fmt.allocPrint(dt.alloc, "{}", .{i}), dt.alloc),
            .float => |f| String.ofAlloc(try std.fmt.allocPrint(dt.alloc, "{}", .{f}), dt.alloc),
            .quote => |q| switch (q.items.len) {
                0 => String.ofAlloc("", dt.alloc), // TODO: Could be constant from state
                1 => q.items[0].intoString(dt),
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
        return switch (self) {
            .string => |s| .{ .string = s.newRef() },
            .command => |cmd| .{ .string = cmd.newRef() },
            .deferred_command => |cmd| .{ .string = cmd.newRef() },
            .quote => |q| .{ .quote = try _deepClone(q, state) },
            else => self,
        };
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
            defer a.releaseRef(dt.alloc);
            const b = rhs.intoString(dt) catch unreachable;
            defer b.releaseRef(dt.alloc);

            return std.mem.eql(u8, a.str, b.str);
        }

        if (lhs.isCommand() and rhs.isCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            defer a.releaseRef(dt.alloc);
            const b = rhs.intoString(dt) catch unreachable;
            defer b.releaseRef(dt.alloc);

            return std.mem.eql(u8, a.str, b.str);
        }

        if (lhs.isDeferredCommand() and rhs.isDeferredCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            defer a.releaseRef(dt.alloc);
            const b = rhs.intoString(dt) catch unreachable;
            defer b.releaseRef(dt.alloc);

            return std.mem.eql(u8, a.str, b.str);
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
            defer a.releaseRef(dt.alloc);
            const b = rhs.intoString(dt) catch unreachable;
            defer b.releaseRef(dt.alloc);
            return std.mem.lessThan(u8, a.str, b.str);
        }
        if (lhs.isString()) return true;

        if (lhs.isCommand() and rhs.isCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            defer a.releaseRef(dt.alloc);
            const b = rhs.intoString(dt) catch unreachable;
            defer b.releaseRef(dt.alloc);
            return std.mem.lessThan(u8, a.str, b.str);
        }
        if (lhs.isCommand()) return true;

        if (lhs.isDeferredCommand() and rhs.isDeferredCommand()) {
            const a = lhs.intoString(dt) catch unreachable;
            defer a.releaseRef(dt.alloc);
            const b = rhs.intoString(dt) catch unreachable;
            defer b.releaseRef(dt.alloc);
            return std.mem.lessThan(u8, a.str, b.str);
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
            .command => |cmd| try writer.print("{s}", .{cmd.str}),
            .deferred_command => |cmd| try writer.print("\\{s}", .{cmd.str}),
            .quote => |q| {
                try writer.print("[ ", .{});
                for (q.items) |val| {
                    try val.print(writer);
                    try writer.print(" ", .{});
                }
                try writer.print("]", .{});
            },
            .string => |s| try writer.print("\"{s}\"", .{s.str}),
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
