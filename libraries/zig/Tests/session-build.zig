const std = @import("std");
const pkg = @import("thinkthen");
pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const dep = b.dependency("thinkthen", .{ .target = target });
    const module = dep.module("thinkthen");
    const exe = b.addExecutable(.{ .name = "session", .root_module = b.createModule(.{ .root_source_file = b.path("session.zig"), .target = target }) });
    exe.root_module.addImport("thinkthen", module);
    pkg.linkNative(b, exe, module, null, .static);
    b.installArtifact(exe);
}
