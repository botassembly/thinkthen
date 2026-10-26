const std = @import("std");

pub fn build(b: *std.Build) void {
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
    if (target.result.os.tag == .linux) {
        const platform, _ = nativeAsset(target.result);
        module.addLibraryPath(b.path(b.fmt("native/{s}", .{platform})));
        module.linkSystemLibrary("thinkthen_ffi", .{ .use_pkg_config = .no });
        module.linkSystemLibrary("dl", .{ .use_pkg_config = .no });
    }
}

/// Link the bundled native engine and install it beside the caller executable.
pub fn installNative(b: *std.Build, dependency: *std.Build.Dependency, installed: *std.Build.Step.InstallArtifact) void {
    const target = installed.artifact.rootModuleTarget();
    const platform, const name = nativeAsset(target);
    if (target.os.tag == .linux) installed.artifact.each_lib_rpath = false;
    const library = b.addInstallFileWithDir(dependency.path(b.fmt("native/{s}/{s}", .{ platform, name })), installed.dest_dir.?, name);
    installed.step.dependOn(&library.step);
}

fn nativeAsset(target: std.Target) struct { []const u8, []const u8 } {
    return switch (target.os.tag) {
        .linux => if (target.cpu.arch == .x86_64 and target.abi == .gnu)
            .{ "linux-x64", "libthinkthen_ffi.so" }
        else if (target.cpu.arch == .aarch64 and target.abi == .gnu)
            .{ "linux-arm64", "libthinkthen_ffi.so" }
        else
            @panic("ThinkThen supports Linux x86-64/arm64 glibc and macOS arm64"),
        .windows => if (target.cpu.arch == .x86_64 and target.abi == .msvc)
            .{ "win-x64", "thinkthen_ffi.dll" }
        else
            @panic("ThinkThen supports Windows x86-64 MSVC"),
        .macos => if (target.cpu.arch == .aarch64)
            .{ "osx-arm64", "libthinkthen_ffi.dylib" }
        else
            @panic("ThinkThen supports Linux x86-64/arm64 glibc and macOS arm64"),
        else => @panic("ThinkThen supports Linux x86-64/arm64 glibc and macOS arm64"),
    };
}
