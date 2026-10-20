const std = @import("std");

pub const LinkMode = enum { shared, static };

/// Packaged callers use the bundled native root; source builders may override it.
pub fn linkNative(b: *std.Build, exe: *std.Build.Step.Compile, module: *std.Build.Module, native_override: ?[]const u8, mode: LinkMode) void {
    const target = exe.root_module.resolved_target.?.result;
    if (target.os.tag != .linux or target.cpu.arch != .x86_64 or target.abi != .gnu)
        @panic("thinkthen supports only x86_64-linux-gnu in this rehearsal");
    const archive = native_override orelse module.root_source_file.?.dirname().path(b, "../native/x86_64-unknown-linux-gnu").getPath(b);
    if (!std.fs.path.isAbsolute(archive)) @panic("-Dnative must name an absolute unpacked C archive directory");
    const header = b.pathJoin(&.{ archive, "include/thinkthen.h" });
    std.fs.cwd().access(header, .{}) catch @panic("thinkthen C header missing or unreadable");
    const lib = b.pathJoin(&.{ archive, "lib", if (mode == .shared) "libthinkthen.so" else "libthinkthen.a" });
    std.fs.cwd().access(lib, .{}) catch @panic("thinkthen selected native library missing");
    if (mode == .shared) {
        const soname = b.pathJoin(&.{ archive, "lib/libthinkthen.so.0" });
        std.fs.cwd().access(soname, .{}) catch @panic("thinkthen shared soname link missing");
    }
    const include = b.pathJoin(&.{ archive, "include" });
    const libdir = b.pathJoin(&.{ archive, "lib" });
    exe.addIncludePath(.{ .cwd_relative = include });
    module.addIncludePath(.{ .cwd_relative = include });
    exe.linkLibC();
    switch (mode) {
        .shared => {
            exe.addLibraryPath(.{ .cwd_relative = libdir });
            exe.linkSystemLibrary("thinkthen");
            exe.addRPath(.{ .cwd_relative = libdir });
        },
        .static => {
            // Zig 0.15.2's ELF linker misaligns Rust's 16-byte constants; LLD does not.
            // Issue: sdlc/issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md
            exe.use_llvm = true;
            exe.use_lld = true;
            exe.addObjectFile(.{ .cwd_relative = lib });
            for ([_][]const u8{ "gcc_s", "util", "rt", "pthread", "m", "dl" }) |name| exe.linkSystemLibrary(name);
        },
    }
}

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    _ = b.addModule("thinkthen", .{ .root_source_file = b.path("src/thinkthen.zig"), .target = target, .optimize = optimize });
    const archive = b.option([]const u8, "native", "Absolute path to unpacked matching C archive");
    {
        const native = archive;
        const mode = b.option(LinkMode, "link-mode", "shared (default) or static C library") orelse .static;
        const exe = b.addExecutable(.{ .name = "thinkthen-example", .root_module = b.createModule(.{ .root_source_file = b.path("examples/decide.zig"), .target = target, .optimize = optimize }) });
        exe.root_module.addImport("thinkthen", b.modules.get("thinkthen").?);
        linkNative(b, exe, b.modules.get("thinkthen").?, native, mode);
        b.installArtifact(exe);
        const run = b.addRunArtifact(exe);
        const step = b.step("example", "Run example using loopback or configured backend");
        step.dependOn(&run.step);
    }
}
