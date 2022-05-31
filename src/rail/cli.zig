const std = @import("std");
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const RunSettings = struct { program: []const u8 };

pub fn initialize(alloc: std.mem.Allocator) !RunSettings {
    var args = std.process.args();

    const program = try args.next(alloc).?;
    const next_arg = args.next(alloc);
    if (next_arg != null) {
        const arg = try next_arg.?;
        if (std.mem.eql(u8, "help", arg) or std.mem.eql(u8, "--help", arg)) {
            try stdout.print("TODO: Show HELP.\n", .{});
            std.os.exit(0);
        }
        try stderr.print("TODO: Show HELP.\n", .{});
        try stderr.print("Unknown command: {s}\n", .{arg});
        std.os.exit(1);
    }

    return RunSettings{.program = std.fs.path.basename(program)};
}
