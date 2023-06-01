const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "rock",
        .root_source_file = .{ .path = "src/main.zig" },
        .optimize = optimize,
        .target = target,
    });

    b.installArtifact(exe);

    const test_step = b.step("test", "Run all tests");
    const main_test = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .optimize = optimize,
        .target = target,
    });
    const tokens_test = b.addTest(.{
        .root_source_file = .{ .path = "src/tokens.zig" },
        .optimize = optimize,
        .target = target,
    });

    test_step.dependOn(&main_test.step);
    test_step.dependOn(&tokens_test.step);
}
