const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const root_source_file = std.Build.FileSource.relative("src/main.zig");

    // Dt executable
    const dt_step = b.step("dt", "Install dt executable");

    const dt = b.addExecutable(.{
        .name = "dt",
        .root_source_file = root_source_file,
        .optimize = optimize,
        .target = target,
    });

    const dt_install = b.addInstallArtifact(dt);
    dt_step.dependOn(&dt_install.step);
    b.default_step.dependOn(dt_step);

    // Tests
    const test_step = b.step("test", "Run tests");

    const test_exe = b.addTest(.{
        .root_source_file = root_source_file,
        .optimize = optimize,
        .target = target,
    });

    const test_run = b.addRunArtifact(test_exe);
    test_step.dependOn(&test_run.step);
    b.default_step.dependOn(test_step);
}
