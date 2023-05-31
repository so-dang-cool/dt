const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    const mecha = b.dependency("mecha", .{});

    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "rock",
        .root_source_file = .{ .path = "src/main.zig" },
        .optimize = optimize,
        .target = target,
    });

    exe.addModule("mecha", mecha.module("mecha"));
    b.installArtifact(exe);

    // const run_cmd = exe.run();
    // run_cmd.step.dependOn(b.getInstallStep());
    // if (b.args) |args| {
    //     run_cmd.addArgs(args);
    // }

    // const run_step = b.step("run", "Run the app");
    // run_step.dependOn(&run_cmd.step);

    const test_step = b.step("test", "Run all tests");
    const the_test = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .optimize = optimize,
        .target = target,
    });

    test_step.dependOn(&the_test.step);
}
