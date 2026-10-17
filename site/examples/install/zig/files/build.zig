const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const dep = b.dependency("thinkthen", .{
        .target = target,
        .optimize = optimize,
    });
    const module = dep.module("thinkthen");
    const exe = b.addExecutable(.{
        .name = "first-call",
        .root_module = b.createModule(.{
            .root_source_file = b.path("first-call.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    exe.root_module.addImport("thinkthen", module);
    thinkthen.linkNative(b, exe, module, null, .static);
    b.installArtifact(exe);
}
