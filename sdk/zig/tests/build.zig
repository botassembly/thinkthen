const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn build(b: *std.Build) !void {
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
    try thinkthen.installNative(b, dependency, installed);
    b.getInstallStep().dependOn(&installed.step);
    const tests = b.addTest(.{ .root_module = b.createModule(.{ .root_source_file = b.path("public_api.zig"), .target = target, .optimize = optimize, .link_libc = true }) });
    const test_name = if (target.result.os.tag == .windows) "public-api.exe" else "public-api";
    tests.root_module.addImport("thinkthen", dependency.module("thinkthen"));
    const test_install = b.addInstallArtifact(tests, .{ .dest_sub_path = test_name });
    try thinkthen.installNative(b, dependency, test_install);
    b.getInstallStep().dependOn(&test_install.step);
    const run = std.Build.Step.Run.create(b, "run public SDK tests");
    run.addFileArg(.{ .relative = .{ .base = .install_bin, .sub_path = test_name } });
    run.step.dependOn(&test_install.step);
    b.step("test", "Run the installed public SDK tests").dependOn(&run.step);
    inline for (.{ "invalid_annotation", "invalid_toolbox" }) |name| {
        const invalid = b.addExecutable(.{ .name = name, .root_module = b.createModule(.{ .root_source_file = b.path(name ++ ".zig"), .target = target, .optimize = optimize, .link_libc = true }) });
        invalid.root_module.addImport("thinkthen", dependency.module("thinkthen"));
        b.step(name, "Compile a rejected annotation consumer").dependOn(&invalid.step);
    }
}
