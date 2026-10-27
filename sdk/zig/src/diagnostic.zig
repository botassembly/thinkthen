const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
pub const ErrorCode = @import("codes").ErrorCode;
pub const Error = error{ OutOfMemory, Usage, Setup, Transport, Cancelled, EngineVersion, Internal, UnknownCode, RecordSource };
pub const Failure = struct {
    code: ?ErrorCode,
    message: []const u8,
    http_status: ?u16 = null,
    raw_name: ?[]const u8 = null,
    pub fn name(self: Failure) []const u8 {
        return self.raw_name orelse if (self.code) |code| code.name() else "unknown";
    }
    pub fn jsonParse(allocator: std.mem.Allocator, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!Failure {
        return jsonParseFromValue(allocator, try std.json.innerParse(std.json.Value, allocator, source, options), options);
    }
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Failure {
        const WireFailure = struct { code: []const u8, message: []const u8, http_status: ?u16 = null };
        const wire = try std.json.innerParseFromValue(WireFailure, allocator, value, options);
        const code: ?ErrorCode = ErrorCode.fromName(wire.code) catch null;
        return .{ .code = code, .message = wire.message, .http_status = wire.http_status, .raw_name = if (code == null) wire.code else null };
    }
    pub fn jsonStringify(self: Failure, writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("code");
        try writer.write(self.name());
        try writer.objectField("message");
        try writer.write(self.message);
        if (self.http_status) |status| {
            try writer.objectField("http_status");
            try writer.write(status);
        }
        try writer.endObject();
    }
};
pub const Diagnostic = struct {
    buffer: [4096]u8 = undefined,
    name_buffer: [4096]u8 = undefined,
    name_length: usize = 0,
    code: ?ErrorCode = null,
    http_status: ?u16 = null,
    length: usize = 0,
    truncated: bool = false,
    pub fn message(self: *const Diagnostic) []const u8 {
        return self.buffer[0..self.length];
    }
    pub fn rawName(self: *const Diagnostic) ?[]const u8 {
        return if (self.name_length == 0) null else self.name_buffer[0..self.name_length];
    }
    pub fn clear(self: *Diagnostic) void {
        self.code = null;
        self.name_length = 0;
        self.http_status = null;
        self.length = 0;
        self.truncated = false;
    }
    pub fn set(self: *Diagnostic, failure: Failure) void {
        self.code = failure.code;
        const name = failure.raw_name orelse "";
        self.name_length = @min(self.name_buffer.len, name.len);
        @memcpy(self.name_buffer[0..self.name_length], name[0..self.name_length]);
        self.http_status = failure.http_status;
        self.length = @min(self.buffer.len, failure.message.len);
        self.truncated = failure.message.len > self.length or name.len > self.name_length;
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
    return if (failure.code) |code| code.operationError() else error.UnknownCode;
}
pub fn check(result: i32, err: *c.thinkthen_error, diag: ?*Diagnostic) Error!void {
    if (result == c.THINKTHEN_OK) return;
    defer native.function("thinkthen_error_free")(err);
    const code: ErrorCode = @fromBackingInt(@intCast(err.code));
    const name = native.function("thinkthen_code_name")(err.code);
    return promote(.{ .code = code, .raw_name = if (name == null) null else std.mem.span(name), .message = bytes(err.message), .http_status = if (err.http_status == 0) null else err.http_status }, diag);
}
