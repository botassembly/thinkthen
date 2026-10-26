const std = @import("std");
const builtin = @import("builtin");
const c = @import("c");

const WindowsLibrary = struct {
    handle: std.os.windows.PVOID,
    fn open(_: []const u8) !WindowsLibrary {
        const windows = std.os.windows;
        const name = windows.UNICODE_STRING.init(std.unicode.utf8ToUtf16LeStringLiteral("thinkthen_ffi.dll"));
        var handle: windows.PVOID = undefined;
        if (windows.ntdll.LdrLoadDll(null, null, &name, &handle) != .SUCCESS) return error.FileNotFound;
        return .{ .handle = handle };
    }
    fn lookup(self: *WindowsLibrary, comptime T: type, name: [:0]const u8) ?T {
        const windows = std.os.windows;
        const symbol = windows.ANSI_STRING.init(std.mem.sliceTo(name, 0));
        var address: windows.PVOID = undefined;
        if (windows.ntdll.LdrGetProcedureAddress(self.handle, &symbol, 0, &address) != .SUCCESS) return null;
        return @ptrCast(@alignCast(address));
    }
    fn close(self: *WindowsLibrary) void {
        _ = std.os.windows.ntdll.LdrUnloadDll(self.handle);
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
