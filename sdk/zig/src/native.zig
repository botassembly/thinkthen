const std = @import("std");
const builtin = @import("builtin");
const c = @import("c");

const file = switch (builtin.os.tag) {
    .macos => "libthinkthen_ffi.dylib",
    .windows => "thinkthen_ffi.dll",
    else => "libthinkthen_ffi.so",
};
var attempted: [std.Io.Dir.max_path_bytes]u8 = undefined;
var failure_text: [std.Io.Dir.max_path_bytes + 128]u8 = undefined;
var failure_length: usize = 0;

/// Keep the first load failure's text. Call with `lock` held.
fn unloadable(path: []const u8) error{LoadFailure} {
    if (failure_length == 0) {
        const text = std.fmt.bufPrint(&failure_text, "cannot load native engine {s}; install it beside the executable with thinkthen.installNative in build.zig", .{path}) catch return error.LoadFailure;
        failure_length = text.len;
    }
    return error.LoadFailure;
}

fn refused() error{LoadFailure} {
    if (failure_length == 0) {
        @memcpy(failure_text[0..c.THINKTHEN_LIBRARY_PATH_MESSAGE.len], c.THINKTHEN_LIBRARY_PATH_MESSAGE);
        failure_length = c.THINKTHEN_LIBRARY_PATH_MESSAGE.len;
    }
    return error.LoadFailure;
}

// Pin the engine to the application folder and allow its system dependencies.
// https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw
const WindowsLibrary = struct {
    const windows = std.os.windows;
    const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 0x00000100;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x00000800;
    extern "kernel32" fn GetEnvironmentVariableW([*:0]const u16, [*]u16, u32) callconv(.winapi) u32;
    extern "kernel32" fn GetModuleFileNameW(?windows.HMODULE, [*]u16, u32) callconv(.winapi) u32;
    extern "kernel32" fn LoadLibraryExW([*:0]const u16, ?windows.HANDLE, u32) callconv(.winapi) ?windows.HMODULE;
    extern "kernel32" fn GetProcAddress(windows.HMODULE, [*:0]const u8) callconv(.winapi) ?*anyopaque;
    extern "kernel32" fn FreeLibrary(windows.HMODULE) callconv(.winapi) windows.BOOL;

    handle: windows.HMODULE,
    fn open() error{LoadFailure}!WindowsLibrary {
        var filename: [windows.PATH_MAX_WIDE:0]u16 = undefined;
        var end: usize = GetEnvironmentVariableW(std.unicode.utf8ToUtf16LeStringLiteral(c.THINKTHEN_LIBRARY_VARIABLE), &filename, filename.len);
        if (end >= filename.len) return unloadable(file);
        if (end != 0) {
            switch (std.fs.path.getWin32PathType(u16, filename[0..end])) {
                .rooted, .drive_relative, .relative => return refused(),
                else => {},
            }
        } else {
            const length = GetModuleFileNameW(null, &filename, filename.len);
            if (length == 0 or length >= filename.len) return unloadable(file);
            const directory = 1 + (std.mem.findLastAny(u16, filename[0..length], &.{ '\\', '/' }) orelse return unloadable(file));
            const basename = std.unicode.utf8ToUtf16LeStringLiteral(file);
            end = directory + basename.len;
            if (end >= filename.len) return unloadable(file);
            std.mem.copyForwards(u16, filename[directory..end], basename);
        }
        std.mem.replaceScalar(u16, filename[0..end], '/', '\\');
        filename[end] = 0;
        const handle = LoadLibraryExW(filename[0..end :0].ptr, null, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32) orelse
            return unloadable(attempted[0..std.unicode.wtf16LeToWtf8(&attempted, filename[0..end])]);
        return .{ .handle = handle };
    }
    fn lookup(self: *WindowsLibrary, comptime T: type, name: [:0]const u8) ?T {
        const address = GetProcAddress(self.handle, name) orelse return null;
        return @ptrCast(@alignCast(address));
    }
    fn close(self: *WindowsLibrary) void {
        _ = FreeLibrary(self.handle);
    }
};

/// The override, or the file beside the program. A bare name would use the system search.
fn openPosix() error{LoadFailure}!std.DynLib {
    const chosen = if (std.c.getenv(c.THINKTHEN_LIBRARY_VARIABLE)) |value| std.mem.span(value) else "";
    if (chosen.len != 0) {
        if (!std.fs.path.isAbsolutePosix(chosen)) return refused();
        return std.DynLib.open(chosen) catch return unloadable(chosen);
    }
    if (builtin.os.tag == .macos) {
        const packaged = "@loader_path/" ++ file;
        return std.DynLib.open(packaged) catch return unloadable(packaged);
    }
    const length = std.c.readlink("/proc/self/exe", &attempted, attempted.len - file.len);
    if (length <= 0) return unloadable(file);
    const directory = 1 + (std.mem.findLastAny(u8, attempted[0..@intCast(length)], "/") orelse return unloadable(file));
    @memcpy(attempted[directory..][0..file.len], file);
    const packaged = attempted[0 .. directory + file.len];
    return std.DynLib.open(packaged) catch return unloadable(packaged);
}

const Library = if (builtin.os.tag == .windows) WindowsLibrary else std.DynLib;
const openLibrary = if (builtin.os.tag == .windows) WindowsLibrary.open else openPosix;
var library: ?Library = null;
var lock: std.atomic.Mutex = .unlocked;

fn Pointer(comptime name: []const u8) type {
    return *const @TypeOf(@field(c, name));
}

/// The engine stays loaded for the lifetime of its allocations and workers.
/// Resolve the generated interface before any engine operation or layout access.
pub fn ready() error{ LoadFailure, EngineVersion }!void {
    @setEvalBranchQuota(100000);
    while (!lock.tryLock()) std.atomic.spinLoopHint();
    defer lock.unlock();
    if (library != null) return;
    var candidate = try openLibrary();
    var accepted = false;
    defer if (!accepted) candidate.close();
    inline for (comptime std.meta.declarations(c)) |decl| {
        if (comptime std.mem.startsWith(u8, decl, "thinkthen_") and @typeInfo(@TypeOf(@field(c, decl))) == .@"fn") {
            if (candidate.lookup(Pointer(decl), decl ++ "\x00") == null) return error.EngineVersion;
        }
    }
    const version = candidate.lookup(Pointer("thinkthen_abi_version"), "thinkthen_abi_version").?;
    const compatible = candidate.lookup(Pointer("thinkthen_abi_compatible"), "thinkthen_abi_compatible").?;
    if (compatible(c.THINKTHEN_ABI_VERSION, version()) == 0) return error.EngineVersion;
    library = candidate;
    accepted = true;
}

/// Why the first load failed. The text never changes once written.
pub fn failure() []const u8 {
    while (!lock.tryLock()) std.atomic.spinLoopHint();
    defer lock.unlock();
    return failure_text[0..failure_length];
}

/// Called only after ready has admitted and retained this library.
pub fn function(comptime name: []const u8) Pointer(name) {
    // Serialize lookup as DynLib.lookup takes a mutable handle on POSIX.
    while (!lock.tryLock()) std.atomic.spinLoopHint();
    defer lock.unlock();
    return library.?.lookup(Pointer(name), name ++ "\x00").?;
}
