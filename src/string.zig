const std = @import("std");
const Allocator = std.mem.Allocator;

pub const String = struct {
    str: []const u8,
    refs: *usize,

    pub fn ofAlloc(orig: []const u8, allocator: Allocator) !String {
        std.log.debug("STRING: \"{s}\", 1 (alloc)", .{orig});
        var str = try allocator.dupe(u8, orig);
        var refs = try allocator.create(usize);
        refs.* = 1;
        return String{
            .str = str,
            .refs = refs,
        };
    }

    pub fn of(str: []const u8, allocator: Allocator) !String {
        std.log.debug("STRING: \"{s}\", 1 (own)", .{str});
        var refs = try allocator.create(usize);
        refs.* = 1;
        return String{
            .str = str,
            .refs = refs,
        };
    }

    pub fn newRef(self: String) String {
        self.refs.* += 1;
        std.log.debug("STRING: \"{s}\", {} (+1)", .{ self.str, self.refs.* });
        return String{
            .str = self.str,
            .refs = self.refs,
        };
    }

    pub fn releaseRef(self: String, allocator: Allocator) void {
        _ = allocator;
        self.refs.* -= 1;
        std.log.debug("STRING: \"{s}\", {} (-1)", .{ self.str, self.refs.* });
        // if (self.refs.* < 1) {
        //     allocator.free(self.str);
        //     allocator.destroy(self.refs);
        // }
    }
};

const Escape = struct {
    from: []const u8,
    to: []const u8,
};

const escapes = [_]Escape{
    .{ .from = "\n", .to = "\\n" },
    .{ .from = "\r", .to = "\\r" },
    .{ .from = "\t", .to = "\\t" },
    .{ .from = "\"", .to = "\\\"" },
    .{ .from = "\\", .to = "\\\\" },
};

/// Allocates an escaped version of the input string.
/// Caller owns the memory.
pub fn escape(allocator: Allocator, unescaped: []const u8) ![]const u8 {
    var result = try allocator.dupe(u8, unescaped);

    for (escapes) |esc| {
        result = try std.mem.replaceOwned(u8, allocator, result, esc.from, esc.to);
    }

    return result;
}

/// Allocates an unescaped version of the input string.
/// Caller owns the memory.
pub fn unescape(allocator: Allocator, unescaped: []const u8) ![]const u8 {
    var result = try allocator.dupe(u8, unescaped);

    for (escapes) |esc| {
        result = try std.mem.replaceOwned(u8, allocator, result, esc.to, esc.from);
    }

    return result;
}
