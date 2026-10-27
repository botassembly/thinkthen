const std = @import("std");
const builtin = @import("builtin");
const c = @import("c");

// Pin the engine to the application folder and allow its system dependencies.
// https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw
const WindowsLibrary = struct {
    const windows = std.os.windows;
    const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 0x00000100;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x00000800;
    extern "kernel32" fn GetModuleFileNameW(?windows.HMODULE, [*]u16, u32) callconv(.winapi) u32;
    extern "kernel32" fn LoadLibraryExW([*:0]const u16, ?windows.HANDLE, u32) callconv(.winapi) ?windows.HMODULE;
    extern "kernel32" fn GetProcAddress(windows.HMODULE, [*:0]const u8) callconv(.winapi) ?*anyopaque;
    extern "kernel32" fn FreeLibrary(windows.HMODULE) callconv(.winapi) windows.BOOL;

    handle: windows.HMODULE,
    fn open(comptime name: []const u8) !WindowsLibrary {
        var filename: [windows.PATH_MAX_WIDE:0]u16 = undefined;
        const length = GetModuleFileNameW(null, &filename, filename.len);
        if (length == 0 or length >= filename.len) return error.FileNotFound;
        const directory = 1 + (std.mem.findLastAny(u16, filename[0..length], &.{ '\\', '/' }) orelse return error.FileNotFound);
        const basename = std.unicode.utf8ToUtf16LeStringLiteral(name);
        const end = directory + basename.len;
        if (end >= filename.len) return error.FileNotFound;
        std.mem.replaceScalar(u16, filename[0..directory], '/', '\\');
        std.mem.copyForwards(u16, filename[directory..end], basename);
        filename[end] = 0;
        const handle = LoadLibraryExW(filename[0..end :0].ptr, null, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32) orelse return error.FileNotFound;
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
const Library = if (builtin.os.tag == .windows) WindowsLibrary else std.DynLib;
var library: ?Library = null;
var lock: std.atomic.Mutex = .unlocked;

fn Pointer(comptime name: []const u8) type {
    return *const @TypeOf(@field(c, name));
}

/// The engine stays loaded for the lifetime of its allocations and workers.
/// Resolve the generated interface before any engine operation or layout access.
pub const path = switch (builtin.os.tag) {
    .macos => "@loader_path/libthinkthen_ffi.dylib",
    .windows => "thinkthen_ffi.dll",
    else => "libthinkthen_ffi.so",
};
pub fn ready() error{ LoadFailure, EngineVersion }!void {
    @setEvalBranchQuota(100000);
    while (!lock.tryLock()) std.atomic.spinLoopHint();
    defer lock.unlock();
    if (library != null) return;
    var candidate = Library.open(path) catch return error.LoadFailure;
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

/// Called only after ready has admitted and retained this library.
pub fn function(comptime name: []const u8) Pointer(name) {
    // Serialize lookup as DynLib.lookup takes a mutable handle on POSIX.
    while (!lock.tryLock()) std.atomic.spinLoopHint();
    defer lock.unlock();
    return library.?.lookup(Pointer(name), name ++ "\x00").?;
}
