const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const string = @import("string.zig");

const stderr = std.io.getStdErr().writer(); // TODO: Remove

const specialChars = .{
    .alwaysSingle = "[]:",
    .whitespace = " ,\t\r\n",
};

pub const TokenIterator = struct {
    allocator: Allocator,
    buf: []const u8,
    index: usize,

    const Self = @This();

    pub fn next(self: *Self) !?Token {
        // If the index is out-of-bounds then we are done now and forever
        if (self.index >= self.buf.len) return null;

        // First, skip any whitespace. Return null if nothing else remains
        const start = std.mem.indexOfNonePos(u8, self.buf, self.index, specialChars.whitespace) orelse {
            self.index = self.buf.len;
            return null;
        };

        switch (self.buf[start]) {
            '"' => { // Parse a string
                var strStart = start + 1;
                var end = start + 1;
                var keepLookin = true;
                while (keepLookin) {
                    // Out-of-bounds: unterminated string. Return string as slice from start to end of buffer.
                    if (end >= self.buf.len) {
                        self.index = self.buf.len;
                        return .{ .string = self.buf[start..(self.buf.len)] };
                    }

                    end = std.mem.indexOfPos(u8, self.buf, end, "\"") orelse self.buf.len;

                    if (self.buf[end - 1] != '\\') {
                        // We found the end!
                        keepLookin = false;
                    } else {
                        // Found a quote, but it was escaped. Search from next character
                        end += 1;
                    }
                }
                self.index = end + 1;
                return .{ .string = self.buf[strStart..end] };
            },
            '~' => { // Parse an error
                const lastChar = std.mem.indexOfPos(u8, self.buf, start, "~") orelse self.buf.len;
                const end = lastChar + 1;
                self.index = end;
                return .{ .err = self.buf[start..end] };
            },
            '#' => { // Ignore a comment (by recursively returning the next non-comment token)
                self.index = std.mem.indexOfAnyPos(u8, self.buf, start, "\r\n") orelse self.buf.len;
                return self.next();
            },
            '[' => {
                self.index = start + 1;
                return .left_bracket;
            },
            ']' => {
                self.index = start + 1;
                return .right_bracket;
            },
            ':' => {
                self.index = start + 1;
                return .{ .term = ":" };
            },
            else => { // Parse a token
                var end = std.mem.indexOfAnyPos(u8, self.buf, start, specialChars.alwaysSingle ++ specialChars.whitespace) orelse self.buf.len;
                self.index = end;
                return Token.parseOneToken(self.buf[start..end]);
            },
        }
    }
};

pub const Token = union(enum) {
    left_bracket: void,
    right_bracket: void,
    bool: bool,
    int: i64,
    float: f64,
    term: []const u8,
    deferred_term: []const u8,
    string: []const u8,
    err: []const u8,
    none: void,

    pub fn parse(allocator: Allocator, code: []const u8) TokenIterator {
        return .{
            .allocator = allocator,
            .buf = code,
            .index = 0,
        };
    }

    fn parseOneToken(part: []const u8) Token {
        if (std.mem.eql(u8, part, "[")) {
            return .left_bracket;
        }
        if (std.mem.eql(u8, part, "]")) {
            return .right_bracket;
        }
        if (std.mem.eql(u8, part, "true")) {
            return .{ .bool = true };
        }
        if (std.mem.eql(u8, part, "false")) {
            return .{ .bool = false };
        }
        if (std.mem.startsWith(u8, part, "\\")) {
            const deferredTerm = part[1..];
            return .{ .deferred_term = deferredTerm };
        }
        if (std.fmt.parseInt(i64, part, 10)) |i| {
            return .{ .int = i };
        } else |_| {}
        if (std.fmt.parseFloat(f64, part)) |f| {
            return .{ .float = f };
        } else |_| {}

        return .{ .term = part };
    }

    fn assertEql(self: Token, other: Token) void {
        switch (self) {
            .left_bracket => std.debug.assert(other == Token.left_bracket),
            .right_bracket => std.debug.assert(other == Token.right_bracket),
            .bool => |b| std.debug.assert(other.bool == b),
            .int => |i| std.debug.assert(other.int == i),
            .float => |f| std.debug.assert(other.float == f),
            .string => |s| std.debug.assert(std.mem.eql(u8, other.string, s)),
            .term => |t| std.debug.assert(std.mem.eql(u8, other.term, t)),
            .deferred_term => |t| std.debug.assert(std.mem.eql(u8, other.deferred_term, t)),
            .err => |e| std.debug.assert(std.mem.eql(u8, other.err, e)),
            .none => std.debug.assert(other == Token.none),
        }
    }
};

// Testing!

test "parse hello.dt" {
    var expected = ArrayList(Token).init(std.testing.allocator);
    defer expected.deinit();
    try expected.append(Token.left_bracket);
    try expected.append(Token{ .string = "hello" });
    try expected.append(Token{ .term = "pl" });
    try expected.append(Token.right_bracket);
    try expected.append(Token{ .deferred_term = "greet" });
    try expected.append(Token{ .term = "def" });

    const helloFile = @embedFile("test/hello.dt");
    var tokens = Token.parse(std.testing.allocator, helloFile);

    var i: u8 = 0;
    while (try tokens.next()) |token| : (i += 1) {
        std.log.info("Expected: {any}, Actual: {any} ... ", .{ expected.items[i], token });
        expected.items[i].assertEql(token);
        std.log.info("PASS\n", .{});
    }
}
