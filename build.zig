const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const root_source_file = std.Build.FileSource.relative("src/main.zig");

    const exe = b.addExecutable(.{
        .name = "dt",
        .root_source_file = root_source_file,
        .optimize = optimize,
        .target = target,
    });

    b.installArtifact(exe);

    const test_step = b.step("test", "Run all tests");
    const test_exe = b.addTest(.{
        .root_source_file = root_source_file,
        .optimize = optimize,
        .target = target,
    });
    const run_test = b.addRunArtifact(test_exe);

    test_step.dependOn(&run_test.step);
}
