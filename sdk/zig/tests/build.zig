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
    thinkthen.linkNative(b, dependency, executable, installed);
    b.getInstallStep().dependOn(&installed.step);
}
