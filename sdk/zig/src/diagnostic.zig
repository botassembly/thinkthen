const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
pub const ErrorCode = @import("codes").ErrorCode;
pub const Error = error{ OutOfMemory, Usage, Setup, Transport, Cancelled, EngineVersion, Internal, RecordSource };
pub const Failure = struct { code: ErrorCode, message: []const u8, http_status: ?u16 = null };
pub const Diagnostic = struct {
    buffer: [4096]u8 = undefined,
    code: ?ErrorCode = null,
    http_status: ?u16 = null,
    length: usize = 0,
    truncated: bool = false,
    pub fn message(self: *const Diagnostic) []const u8 {
        return self.buffer[0..self.length];
    }
    pub fn clear(self: *Diagnostic) void {
        self.code = null;
        self.http_status = null;
        self.length = 0;
        self.truncated = false;
    }
    pub fn set(self: *Diagnostic, failure: Failure) void {
        self.code = failure.code;
        self.http_status = failure.http_status;
        self.length = @min(self.buffer.len, failure.message.len);
        self.truncated = failure.message.len > self.length;
        @memcpy(self.buffer[0..self.length], failure.message[0..self.length]);
    }
};
pub fn clear(diag: ?*Diagnostic) void {
    if (diag) |d| d.clear();
}
pub fn bytes(buffer: c.thinkthen_buffer) []const u8 {
    return if (buffer.len == 0) "" else buffer.data[0..buffer.len];
}
pub fn emptyError() c.thinkthen_error {
    return .{ .struct_size = @sizeOf(c.thinkthen_error) };
}
pub fn incompatible(diag: ?*Diagnostic) Error {
    if (diag) |d| d.set(.{ .code = .usage_abi, .message = c.THINKTHEN_ABI_ERROR_MESSAGE });
    return error.EngineVersion;
}
pub fn promote(failure: Failure, diag: ?*Diagnostic) Error {
    if (diag) |d| d.set(failure);
    return failure.code.operationError();
}
pub fn check(result: i32, err: *c.thinkthen_error, diag: ?*Diagnostic) Error!void {
    if (result == c.THINKTHEN_OK) return;
    defer native.function("thinkthen_error_free")(err);
    const code = std.enums.fromInt(ErrorCode, err.code) orelse return error.Internal;
    return promote(.{ .code = code, .message = bytes(err.message), .http_status = if (err.http_status == 0) null else err.http_status }, diag);
}
