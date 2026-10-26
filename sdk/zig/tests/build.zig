const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const dependency = b.dependency("thinkthen", .{ .target = target, .optimize = optimize });
    const executable = b.addExecutable(.{
        .name = "consumer",
        .root_module = b.createModule(.{
            .root_source_file = b.path("consumer.zig"),
            .target = target,
            .optimize = optimize,
            .link_libc = true,
        }),
    });
    executable.root_module.addImport("thinkthen", dependency.module("thinkthen"));
    const installed = b.addInstallArtifact(executable, .{});
    thinkthen.installNative(b, dependency, installed);
    b.getInstallStep().dependOn(&installed.step);
    const tests = b.addTest(.{ .root_module = b.createModule(.{ .root_source_file = b.path("public_api.zig"), .target = target, .optimize = optimize, .link_libc = true }) });
    tests.root_module.addImport("thinkthen", dependency.module("thinkthen"));
    const test_install = b.addInstallArtifact(tests, .{ .dest_sub_path = "public-api" });
    thinkthen.installNative(b, dependency, test_install);
    b.getInstallStep().dependOn(&test_install.step);
    inline for (.{ "invalid_annotation", "invalid_toolbox" }) |name| {
        const invalid = b.addExecutable(.{ .name = name, .root_module = b.createModule(.{ .root_source_file = b.path(name ++ ".zig"), .target = target, .optimize = optimize, .link_libc = true }) });
        invalid.root_module.addImport("thinkthen", dependency.module("thinkthen"));
        b.step(name, "Compile a rejected annotation consumer").dependOn(&invalid.step);
    }
}
