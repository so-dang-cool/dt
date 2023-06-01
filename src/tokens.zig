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
        var i: u64 = 0;
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
            return .{ .deferred_term = part };
        }
        if (std.fmt.parseInt(i64, part, 10)) |i| {
            return .{ .i64 = i };
        } else |_| {}
        if (std.fmt.parseFloat(f64, part)) |f| {
            return .{ .f64 = f };
        } else |_| {}

        return .{ .term = part };
    }
};

// Testing!

test "parse hello.rock" {
    var expected = ArrayList(Token).init(std.testing.allocator);
    try expected.append(Token.left_bracket);
    try expected.append(Token{ .string = "hello" });
    try expected.append(Token{ .term = "pl" });
    try expected.append(Token.right_bracket);
    try expected.append(Token{ .deferred_term = "greet" });
    try expected.append(Token{ .term = "def" });

    const helloFile = @embedFile("test/hello.rock");
    const tokens = try Token.parseAlloc(std.testing.allocator, helloFile);

    std.debug.assert(tokens.items.len == 6);
    var i: u8 = 0;
    while (i < 6) {
        // TODO: Switch an test equality? How do tagged unions work?
        // std.debug.assert(tokens.items[i] == expected.items[i]);
        i += 1;
    }
}
