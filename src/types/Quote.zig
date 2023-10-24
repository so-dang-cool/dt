const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const types = @import("../types.zig");
const Val = types.Val;
const Dictionary = types.Dictionary;

const Self = @This();

vals: ArrayList(Val),
defs: Dictionary,
allocator: Allocator,

pub fn init(allocator: Allocator) !Self {
    var vals = ArrayList(Val).init(allocator);
    var defs = Dictionary.init(allocator);
    return .{
        .vals = vals,
        .defs = defs,
        .allocator = allocator,
    };
}

pub fn child(self: *Self) !Self {
    var vals = try ArrayList(Val).init(self.allocator);
    return .{
        .vals = vals,
        .defs = self.defs.clone(),
        .allocator = self.allocator,
    };
}

pub fn deinit(self: Self) void {
    for (self.vals.items) |item| {
        item.deinit();
    }
    self.vals.deinit();

    self.defs.deinit();
}
