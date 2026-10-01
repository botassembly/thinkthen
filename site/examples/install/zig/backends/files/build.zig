const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const native = b.option(
        []const u8,
        "native",
        "The unpacked C archive",
    ) orelse @panic("give -Dnative");
    const dep = b.dependency("thinkthen", .{
        .target = target,
        .optimize = optimize,
    });
    const module = dep.module("thinkthen");
    const exe = b.addExecutable(.{
        .name = "backends",
        .root_module = b.createModule(.{
            .root_source_file = b.path("backends.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    exe.root_module.addImport("thinkthen", module);
    thinkthen.linkNative(b, exe, module, native, .shared);
    b.installArtifact(exe);
}
