// SPDX-License-Identifier: MPL-2.0
//! Build graph for @@PROJECT_NAME@@.
//!
//! Zero dependencies and no network use: `zig build` compiles from this
//! tree alone, which is why the scaffold works air-gapped.
//!
//! NOTE ON build.zig.zon: there is none. Zig 0.16 requires a package
//! `fingerprint` in it, and — correctly — refuses a fingerprint that was
//! not generated for that package name, so a template cannot ship one
//! without guessing. It is not needed to build: run
//! `zig fetch --save <url>` when you add your first dependency and Zig
//! will create build.zig.zon with a valid fingerprint for you.

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // The library module — business logic, importable and testable.
    const lib = b.addModule("@@MOD_NAME@@", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    // The CLI, which imports the library under the project's own name.
    const exe = b.addExecutable(.{
        .name = "@@PROJECT_NAME@@",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "@@MOD_NAME@@", .module = lib },
            },
        }),
    });
    b.installArtifact(exe);

    const run_step = b.step("run", "Run the app");
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| run_cmd.addArgs(args);
    run_step.dependOn(&run_cmd.step);

    // Unit tests live beside the code they exercise.
    const lib_tests = b.addTest(.{ .root_module = lib });
    const run_lib_tests = b.addRunArtifact(lib_tests);

    const exe_tests = b.addTest(.{ .root_module = exe.root_module });
    const run_exe_tests = b.addRunArtifact(exe_tests);

    // Integration tests exercise the public API as a consumer would.
    const integration_tests = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("test/integration_test.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "@@MOD_NAME@@", .module = lib },
            },
        }),
    });
    const run_integration_tests = b.addRunArtifact(integration_tests);

    const test_step = b.step("test", "Run unit and integration tests");
    test_step.dependOn(&run_lib_tests.step);
    test_step.dependOn(&run_exe_tests.step);
    test_step.dependOn(&run_integration_tests.step);
}
