const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub const Token = union(enum) {
    left_bracket: void,
    right_bracket: void,
    bool: bool,
    i64: i64,
    f64: f64,
    term: []const u8,
    deferred_term: []const u8,
    string: []const u8,
    none: void,

    pub fn parseAlloc(alloc: Allocator, raw: []const u8) !ArrayList(Token) {
        var tokens = ArrayList(Token).init(alloc);

        var sections = std.mem.tokenize(u8, raw, "\"");
        var i: u64 = if (raw[0] == '"') 1 else 0;
        while (sections.next()) |contents| {
            if (i % 2 == 1) {
                try tokens.append(.{ .string = contents });
            } else {
                var parts = std.mem.tokenize(u8, contents, " \t,\r\n");

                while (parts.next()) |part| {
                    const tok = parse(part);
                    try tokens.append(tok);
                }
            }
            i += 1;
        }

        return tokens;
    }

    fn parse(part: []const u8) Token {
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
            return .{ .i64 = i };
        } else |_| {}
        if (std.fmt.parseFloat(f64, part)) |f| {
            return .{ .f64 = f };
        } else |_| {}

        return .{ .term = part };
    }

    fn assertEql(self: Token, other: Token) void {
        switch (self) {
            .left_bracket => std.debug.assert(other == Token.left_bracket),
            .right_bracket => std.debug.assert(other == Token.right_bracket),
            .bool => |b| std.debug.assert(other.bool == b),
            .i64 => |i| std.debug.assert(other.i64 == i),
            .f64 => |f| std.debug.assert(other.f64 == f),
            .string => |s| std.debug.assert(std.mem.eql(u8, other.string, s)),
            .term => |t| std.debug.assert(std.mem.eql(u8, other.term, t)),
            .deferred_term => |t| std.debug.assert(std.mem.eql(u8, other.deferred_term, t)),
            .none => std.debug.assert(other == Token.none),
        }
    }
};

// Testing!

test "parse hello.rock" {
    var expected = ArrayList(Token).init(std.testing.allocator);
    defer expected.deinit();
    try expected.append(Token.left_bracket);
    try expected.append(Token{ .string = "hello" });
    try expected.append(Token{ .term = "pl" });
    try expected.append(Token.right_bracket);
    try expected.append(Token{ .deferred_term = "greet" });
    try expected.append(Token{ .term = "def" });

    const helloFile = @embedFile("test/hello.rock");
    const tokens = try Token.parseAlloc(std.testing.allocator, helloFile);
    defer tokens.deinit();

    std.debug.assert(tokens.items.len == 6);
    var i: u8 = 0;
    while (i < 6) {
        std.debug.print("Expected: {any}, Actual: {any} ... ", .{ expected.items[i], tokens.items[i] });
        expected.items[i].assertEql(tokens.items[i]);
        std.debug.print("PASS\n", .{});
        i += 1;
    }
}
