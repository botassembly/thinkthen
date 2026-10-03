const std = @import("std");

pub const LinkMode = enum { shared, static };

fn headerVersionMatches(header: []const u8) bool {
    const names = [_][]const u8{ "THINKTHEN_VERSION_MAJOR", "THINKTHEN_VERSION_MINOR", "THINKTHEN_VERSION_PATCH" };
    const expected = [_]u32{ 0, 1, 2 };
    var seen = [_]bool{ false, false, false };
    var in_comment = false;
    var lines = std.mem.splitScalar(u8, header, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r");
        if (in_comment) {
            if (std.mem.indexOf(u8, trimmed, "*/") != null) in_comment = false;
            continue;
        }
        if (std.mem.startsWith(u8, trimmed, "/*")) {
            in_comment = std.mem.indexOf(u8, trimmed, "*/") == null;
            continue;
        }
        if (std.mem.startsWith(u8, trimmed, "//")) continue;
        var words = std.mem.tokenizeAny(u8, trimmed, " \t\r");
        if (!std.mem.eql(u8, words.next() orelse continue, "#define")) continue;
        const name = words.next() orelse continue;
        for (names, expected, 0..) |version_name, value, i| {
            if (!std.mem.eql(u8, name, version_name)) continue;
            if (seen[i]) return false;
            const written = std.fmt.parseInt(u32, words.next() orelse return false, 10) catch return false;
            if (written != value) return false;
            if (words.next()) |extra| {
                if (!std.mem.startsWith(u8, extra, "//") and !std.mem.startsWith(u8, extra, "/*")) return false;
            }
            seen[i] = true;
        }
    }
    for (seen) |found| if (!found) return false;
    return true;
}

/// The caller supplies the absolute path of an unpacked matching C release archive.
/// This checks the header and layout; it cannot prove an opaque library's runtime ABI identity.
pub fn linkNative(b: *std.Build, exe: *std.Build.Step.Compile, module: *std.Build.Module, archive: []const u8, mode: LinkMode) void {
    const target = exe.root_module.resolved_target.?.result;
    if (target.os.tag != .linux or target.cpu.arch != .x86_64 or target.abi != .gnu)
        @panic("thinkthen supports only x86_64-linux-gnu in this rehearsal");
    if (!std.fs.path.isAbsolute(archive)) @panic("-Dnative must name an absolute unpacked C archive directory");
    const header = b.pathJoin(&.{ archive, "include/thinkthen.h" });
    const bytes = std.fs.cwd().readFileAlloc(b.allocator, header, 65536) catch @panic("thinkthen C header missing or unreadable");
    if (!headerVersionMatches(bytes)) @panic("thinkthen C header does not match package ABI 0.1.2");
    for ([_][]const u8{ "typedef struct thinkthen_answer", "thinkthen_decide_many_opts", "thinkthen_call_opts" }) |needle| {
        if (std.mem.indexOf(u8, bytes, needle) == null) @panic("thinkthen C header does not match package ABI 0.1.2");
    }
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
    if (archive) |native| {
        const mode = b.option(LinkMode, "link-mode", "shared (default) or static C library") orelse .shared;
        const exe = b.addExecutable(.{ .name = "thinkthen-example", .root_module = b.createModule(.{ .root_source_file = b.path("examples/decide.zig"), .target = target, .optimize = optimize }) });
        exe.root_module.addImport("thinkthen", b.modules.get("thinkthen").?);
        linkNative(b, exe, b.modules.get("thinkthen").?, native, mode);
        b.installArtifact(exe);
        const run = b.addRunArtifact(exe);
        const step = b.step("example", "Run example using loopback or configured backend");
        step.dependOn(&run.step);
    }
}
