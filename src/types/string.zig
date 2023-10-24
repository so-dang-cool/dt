const std = @import("std");
const Allocator = std.mem.Allocator;

pub const String = struct {
    data: []const u8,
    refcount: *usize,

    const Self = @This();

    /// Owns the string data it's passed. Allocates a refcount.
    pub fn init(data: []const u8, allocator: Allocator) !Self {
        const refcount = try allocator.create(usize);
        return .{
            .data = data,
            .refcount = refcount,
        };
    }

    /// Allocates a copy of the string data it's passed, and allocates a refcount.
    pub fn initAlloc(data: []const u8, allocator: Allocator) !Self {
        const dataCopy = try allocator.dupe(u8, data);
        const refcount = try allocator.create(usize);
        return .{
            .data = dataCopy,
            .refcount = refcount,
        };
    }
};

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
