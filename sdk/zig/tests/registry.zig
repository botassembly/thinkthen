const std = @import("std");
const tt = @import("thinkthen");
const builtin = @import("builtin");

// std.DynLib has no Windows implementation.
const WindowsLibrary = struct {
    const Module = std.os.windows.HMODULE;
    extern "kernel32" fn LoadLibraryW([*:0]const u16) callconv(.winapi) ?Module;
    extern "kernel32" fn GetProcAddress(Module, [*:0]const u8) callconv(.winapi) ?*anyopaque;
    extern "kernel32" fn FreeLibrary(Module) callconv(.winapi) c_int;

    handle: Module,
    fn open(comptime name: []const u8) !WindowsLibrary {
        return .{ .handle = LoadLibraryW(std.unicode.utf8ToUtf16LeStringLiteral(name)) orelse return error.FileNotFound };
    }
    fn lookup(self: *WindowsLibrary, comptime T: type, name: [:0]const u8) ?T {
        return @ptrCast(@alignCast(GetProcAddress(self.handle, name) orelse return null));
    }
    fn close(self: *WindowsLibrary) void {
        _ = FreeLibrary(self.handle);
    }
};
const Library = if (builtin.os.tag == .windows) WindowsLibrary else std.DynLib;

fn besideProgram(buffer: []u8) ![]const u8 {
    const file = "libthinkthen_ffi.so";
    const length = std.c.readlink("/proc/self/exe", buffer.ptr, buffer.len - file.len);
    if (length <= 0) return error.FileNotFound;
    const directory = 1 + (std.mem.findLastAny(u8, buffer[0..@intCast(length)], "/") orelse return error.FileNotFound);
    @memcpy(buffer[directory..][0..file.len], file);
    return buffer[0 .. directory + file.len];
}

test "generated error code integers and names match the loaded engine" {
    var buffer: [std.Io.Dir.max_path_bytes]u8 = undefined;
    var library = if (builtin.os.tag == .windows)
        try Library.open("thinkthen_ffi.dll")
    else
        try Library.open(if (builtin.os.tag == .macos) "@loader_path/libthinkthen_ffi.dylib" else try besideProgram(&buffer));
    defer library.close();
    const count = library.lookup(*const fn () callconv(.c) usize, "thinkthen_code_count").?;
    const at = library.lookup(*const fn (usize) callconv(.c) i32, "thinkthen_code_at").?;
    const name = library.lookup(*const fn (i32) callconv(.c) [*:0]const u8, "thinkthen_code_name").?;
    try std.testing.expectEqual(@typeInfo(tt.ErrorCode).@"enum".field_names.len, count());
    for (0..count()) |i| {
        const code = std.enums.fromInt(tt.ErrorCode, at(i)) orelse return error.UnknownCode;
        try std.testing.expectEqualStrings(std.mem.span(name(at(i))), code.name());
    }
}
