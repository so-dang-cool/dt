const std = @import("std");
const Atomic = std.atomic.Atomic;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const StringHashMap = std.StringHashMap;
const Integer = std.math.big.int.Managed;
const Rational = std.math.big.Rational;

const string = @import("string.zig");
const String = string.String;

const interpret = @import("interpret.zig");
const Command = interpret.Command;
const DtMachine = interpret.DtMachine;

pub const Quote = ArrayList(Val);
pub const Dictionary = StringHashMap(Command);

pub const Val = union(enum) {
    bool: bool,
    int: Integer,
    rat: Rational,
    string: String,
    command: String,
    deferred_command: String,
    quote: Quote,

    pub fn isBool(self: Val) bool {
        return switch (self) {
            .bool => true,
            else => false,
        };
    }

    pub fn intoBool(self: Val, state: *DtMachine) bool {
        return switch (self) {
            .bool => |b| b,
            .int => |i| i.isPositive(),
            .rat => |r| r.p.isPositive() == r.q.isPositive(),
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

    pub fn intoInt(self: Val) !Integer {
        return switch (self) {
            .int => |i| i,

            .bool => |b| if (b) 1 else 0,
            .float => |f| @as(i64, @intFromFloat(f)),
            .string => |s| std.fmt.parseInt(i64, s, 10),
            else => .NoCoercionToInteger,
        };
    }

    pub fn isRat(self: Val) bool {
        return switch (self) {
            .rat => true,
            else => false,
        };
    }

    pub fn intoRat(self: Val) !Rational {
        return switch (self) {
            .rat => |r| r,

            .bool => |b| if (b) 1 else 0,
            .int => |i| @as(f64, i.toConst().to(f64)),
            .string => |s| std.fmt.parseFloat(f64, s),
            else => .NoCoercionToInteger,
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
            .rat => |r| try std.fmt.allocPrint(state.alloc, "{}", .{r}),
            .quote => |q| switch (q.items.len) {
                0 => "",
                1 => q.items[0].intoString(state),
                else => .NoCoercionToString,
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

        if ((lhs.isInt() or lhs.isRat()) and (rhs.isInt() or rhs.isRat())) {
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
        if ((lhs.isInt() or lhs.isRat()) and (rhs.isInt() or rhs.isRat())) {
            const a = lhs.intoFloat() catch unreachable;
            const b = rhs.intoFloat() catch unreachable;
            return a < b;
        }
        if (lhs.isInt()) return true;
        if (lhs.isRat()) return true;

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
