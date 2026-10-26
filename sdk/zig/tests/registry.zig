const std = @import("std");
const tt = @import("thinkthen");
const builtin = @import("builtin");
test "generated error code integers and names match the loaded engine" {
    if (builtin.os.tag == .windows) return error.SkipZigTest;
    var library = try std.DynLib.open(if (builtin.os.tag == .macos) "@loader_path/libthinkthen_ffi.dylib" else "libthinkthen_ffi.so");
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
