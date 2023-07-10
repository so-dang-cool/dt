const std = @import("std");
const Allocator = std.mem.Allocator;

pub const String = []const u8;

const Escape = struct {
    from: String,
    to: String,
};

const escapes = [_]Escape{
    .{ .from = "\n", .to = "\\n" },
    .{ .from = "\r", .to = "\\r" },
    .{ .from = "\t", .to = "\\t" },
    .{ .from = "\"", .to = "\\\"" },
    .{ .from = "\\", .to = "\\\\" },
};

pub fn escape(allocator: Allocator, unescaped: String) !String {
    var result = try allocator.dupe(u8, unescaped);

    for (escapes) |esc| {
        result = try std.mem.replaceOwned(u8, allocator, result, esc.from, esc.to);
    }

    return result;
}

pub fn unescape(allocator: Allocator, unescaped: String) !String {
    var result = try allocator.dupe(u8, unescaped);

    for (escapes) |esc| {
        result = try std.mem.replaceOwned(u8, allocator, result, esc.to, esc.from);
    }

    return result;
}
