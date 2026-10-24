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
}

/// Link the bundled native engine and install it beside the caller executable.
pub fn linkNative(b: *std.Build, dependency: *std.Build.Dependency, executable: *std.Build.Step.Compile, installed: *std.Build.Step.InstallArtifact) void {
    const target = executable.rootModuleTarget();
    const platform, const name, const search = switch (target.os.tag) {
        .linux => if (target.cpu.arch == .x86_64 and target.abi == .gnu)
            .{ "linux-x64", "libthinkthen_ffi.so", "$ORIGIN" }
        else if (target.cpu.arch == .aarch64 and target.abi == .gnu)
            .{ "linux-arm64", "libthinkthen_ffi.so", "$ORIGIN" }
        else
            @panic("ThinkThen supports Linux x86-64/arm64 glibc and macOS arm64"),
        .windows => if (target.cpu.arch == .x86_64 and target.abi == .msvc)
            .{ "win-x64", "thinkthen_ffi.dll", "" }
        else
            @panic("ThinkThen supports Windows x86-64 MSVC"),
        .macos => if (target.cpu.arch == .aarch64)
            .{ "osx-arm64", "libthinkthen_ffi.dylib", "@loader_path" }
        else
            @panic("ThinkThen supports Linux x86-64/arm64 glibc and macOS arm64"),
        else => @panic("ThinkThen supports Linux x86-64/arm64 glibc and macOS arm64"),
    };
    executable.each_lib_rpath = false;
    if (target.os.tag == .windows) {
        executable.root_module.addObjectFile(dependency.path("native/win-x64/thinkthen_ffi.lib"));
    } else {
        executable.root_module.addLibraryPath(dependency.path(b.fmt("native/{s}", .{platform})));
        executable.root_module.linkSystemLibrary("thinkthen_ffi", .{ .use_pkg_config = .no });
        executable.root_module.addRPathSpecial(search);
    }
    const library = b.addInstallFileWithDir(dependency.path(b.fmt("native/{s}/{s}", .{ platform, name })), installed.dest_dir.?, name);
    installed.step.dependOn(&library.step);
}
