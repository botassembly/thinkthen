const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const header = b.addTranslateC(.{
        .root_source_file = b.path("include/thinkthen.h"),
        .target = target,
        .optimize = optimize,
    });
    const module = b.addModule("thinkthen", .{
        .root_source_file = b.path("src/thinkthen.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    module.addImport("c", header.createModule());
    module.addImport("codes", b.createModule(.{ .root_source_file = b.path("src/error_codes.zig"), .target = target, .optimize = optimize }));
    if (target.result.os.tag != .windows) module.addRPathSpecial(if (target.result.os.tag == .macos) "@loader_path" else "$ORIGIN");
    if (target.result.os.tag == .linux) module.linkSystemLibrary("dl", .{ .use_pkg_config = .no });
    const tests = b.addTest(.{ .root_module = b.createModule(.{
        .root_source_file = b.path("tests/public_api.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    }) });
    if (target.result.os.tag == .linux) {
        tests.use_llvm = true;
        tests.use_lld = true;
    }
    const test_name = if (target.result.os.tag == .windows) "public-api.exe" else "public-api";
    tests.root_module.addImport("thinkthen", module);
    const installed = b.addInstallArtifact(tests, .{ .dest_sub_path = test_name });
    const platform, const name = try nativeAsset(target.result);
    const library = b.addInstallFileWithDir(b.path(b.fmt("native/{s}/{s}", .{ platform, name })), .bin, name);
    installed.step.dependOn(&library.step);
    const run = std.Build.Step.Run.create(b, "run public SDK tests");
    run.addFileArg(.{ .relative = .{ .base = .install_bin, .sub_path = test_name } });
    run.step.dependOn(&installed.step);
    b.step("test", "Run the public SDK tests with the bundled engine").dependOn(&run.step);
}

/// Install the bundled native engine beside the caller executable.
pub fn installNative(b: *std.Build, dependency: *std.Build.Dependency, installed: *std.Build.Step.InstallArtifact) !void {
    const target = installed.artifact.rootModuleTarget();
    const platform, const name = try nativeAsset(target);
    if (target.os.tag == .linux) {
        // LLVM ignores unresolved addresses used only by translated C debug metadata.
        installed.artifact.use_llvm = true;
        installed.artifact.use_lld = true;
        installed.artifact.each_lib_rpath = false;
    }
    const library = b.addInstallFileWithDir(dependency.path(b.fmt("native/{s}/{s}", .{ platform, name })), installed.dest_dir.?, name);
    installed.step.dependOn(&library.step);
}

fn nativeAsset(target: std.Target) error{UnsupportedTarget}!struct { []const u8, []const u8 } {
    return switch (target.os.tag) {
        .linux => if (target.cpu.arch == .x86_64 and target.abi == .gnu)
            .{ "linux-x64", "libthinkthen_ffi.so" }
        else if (target.cpu.arch == .aarch64 and target.abi == .gnu)
            .{ "linux-arm64", "libthinkthen_ffi.so" }
        else
            error.UnsupportedTarget,
        .windows => if (target.cpu.arch == .x86_64 and target.abi == .msvc)
            .{ "win-x64", "thinkthen_ffi.dll" }
        else
            error.UnsupportedTarget,
        .macos => if (target.cpu.arch == .aarch64)
            .{ "osx-arm64", "libthinkthen_ffi.dylib" }
        else
            error.UnsupportedTarget,
        else => error.UnsupportedTarget,
    };
}
