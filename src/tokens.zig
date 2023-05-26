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

        var parts = std.mem.split(u8, raw, " ");

        while (parts.next()) |part| {
            const token: Token = if (std.mem.eql(u8, part, "["))
                .left_bracket
            else if (std.mem.eql(u8, part, "]"))
                .right_bracket
            else if (std.mem.eql(u8, part, "true")) .{ .bool = true } else if (std.mem.eql(u8, part, "false")) .{ .bool = false } else .{ .term = part };
            try tokens.append(token);
        }

        return tokens;
    }
};
