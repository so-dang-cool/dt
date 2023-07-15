const std = @import("std");
const Child = std.process.Child;
const allocator = std.heap.page_allocator;

const MAX_FILE_SIZE = 1 << 12;

pub fn dtRunFile(file_path: []const u8) !Child.ExecResult {
    const cur_dir = std.fs.cwd();
    const contents = try cur_dir.readFileAlloc(allocator, file_path, MAX_FILE_SIZE);
    return try dtStdin(contents);
}

pub fn dtStdin(input: []const u8) !Child.ExecResult {
    return try Child.exec(.{ .allocator = allocator, .argv = &.{
        "./zig-out/bin/dt",
        "[\"#\" starts-with? not] filter",
        "unwords",
        "eval",
        input,
    } });
}

pub fn dt(argv: []const []const u8) !Child.ExecResult {
    var args = std.ArrayList([]const u8).init(allocator);
    defer args.deinit();

    try args.append("./zig-out/bin/dt");

    for (argv) |arg| {
        try args.append(arg);
    }

    return try Child.exec(.{ .allocator = allocator, .argv = args.items });
}
