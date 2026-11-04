const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
/// The engine's error codes, generated from its registry. New codes may appear in later engines.
pub const ErrorCode = @import("codes").ErrorCode;
/// Errors a failed engine operation or failed record can return. The Diagnostic holds the exact code.
pub const FailureError = error{ Usage, Setup, Cancelled, EngineVersion, Internal, UnknownCode, Record, Backend, Local };
/// Errors from reading a row as its answer: `Unsure` for a not-sure row, or the failed row's error.
pub const AnswerError = FailureError || error{Unsure};
/// Every error the public interface returns.
/// `Unsure` means a row was used as an answer while not sure; it is never an engine failure.
/// `RecordSource` is a caller's RecordSource failing to produce its next record.
pub const Error = AnswerError || error{ OutOfMemory, RecordSource };
/// One failed record or operation: the engine's code, its message, and any HTTP status.
pub const Failure = struct {
    /// The engine code, or null when this engine is newer and `raw_name` holds its name.
    code: ?ErrorCode,
    /// The engine's message.
    message: []const u8,
    /// The backend's HTTP status, when a backend reply failed.
    http_status: ?u16 = null,
    /// The code's name when it is unknown to this package.
    raw_name: ?[]const u8 = null,
    /// The code's dotted name, such as `record.missing`.
    pub fn name(self: Failure) []const u8 {
        return self.raw_name orelse if (self.code) |code| code.name() else "unknown";
    }
    /// Read JSON, for `std.json.parseFromSlice`.
    pub fn jsonParse(allocator: std.mem.Allocator, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!Failure {
        return jsonParseFromValue(allocator, try std.json.innerParse(std.json.Value, allocator, source, options), options);
    }
    /// Read a parsed JSON value, for `std.json.parseFromValue`.
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Failure {
        const WireFailure = struct { code: []const u8, message: []const u8, http_status: ?u16 = null };
        const wire = try std.json.innerParseFromValue(WireFailure, allocator, value, options);
        const code: ?ErrorCode = ErrorCode.fromName(wire.code) catch null;
        return .{ .code = code, .message = wire.message, .http_status = wire.http_status, .raw_name = if (code == null) wire.code else null };
    }
    /// Write as JSON, for `std.json.Stringify`.
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
/// Details of the last error, filled by any call given a pointer to it.
/// Each call clears it first. It owns its text, so it outlives the row or call that failed.
pub const Diagnostic = struct {
    buffer: [4096]u8 = undefined,
    name_buffer: [4096]u8 = undefined,
    name_length: usize = 0,
    /// The engine code of the last failure, or null for `Unsure` and for errors with no engine code.
    code: ?ErrorCode = null,
    /// The backend's HTTP status, when a backend reply failed.
    http_status: ?u16 = null,
    length: usize = 0,
    /// True when the message or code name was longer than the buffer and was cut.
    truncated: bool = false,
    /// The input position of the row that failed or was not sure, counting from 1.
    position: ?u64 = null,
    /// The probability of a row that was not sure.
    probability: ?f64 = null,
    /// The last failure's message.
    pub fn message(self: *const Diagnostic) []const u8 {
        return self.buffer[0..self.length];
    }
    /// The last failure's code name when this package does not know the code.
    pub fn rawName(self: *const Diagnostic) ?[]const u8 {
        return if (self.name_length == 0) null else self.name_buffer[0..self.name_length];
    }
    /// Forget the last error.
    pub fn clear(self: *Diagnostic) void {
        self.code = null;
        self.name_length = 0;
        self.http_status = null;
        self.length = 0;
        self.truncated = false;
        self.position = null;
        self.probability = null;
    }
    /// Record a not-sure row used as an answer.
    pub fn setUnsure(self: *Diagnostic, probability: f64) void {
        self.clear();
        self.probability = probability;
        const text = "not sure";
        @memcpy(self.buffer[0..text.len], text);
        self.length = text.len;
    }
    /// Record a failure.
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
pub fn incompatible(diag: ?*Diagnostic) error{EngineVersion} {
    if (diag) |d| d.set(.{ .code = .usage_abi, .message = c.THINKTHEN_ABI_ERROR_MESSAGE });
    return error.EngineVersion;
}
pub fn promote(failure: Failure, diag: ?*Diagnostic) FailureError {
    if (diag) |d| d.set(failure);
    const code = failure.code orelse return error.UnknownCode;
    if (code == .usage_abi) return error.EngineVersion;
    if (code == .local_cancelled) return error.Cancelled;
    native.ready() catch |failure_to_load| switch (failure_to_load) {
        error.EngineVersion => return incompatible(diag),
        error.LoadFailure => return error.Setup,
    };
    const raw = native.function("thinkthen_code_family")(@backingInt(code));
    if (raw == null) return error.UnknownCode;
    const family = std.mem.span(raw);
    if (std.mem.eql(u8, family, "usage")) return error.Usage;
    if (std.mem.eql(u8, family, "record")) return error.Record;
    if (std.mem.eql(u8, family, "backend")) return error.Backend;
    if (std.mem.eql(u8, family, "local")) return error.Local;
    return error.Internal;
}
pub fn check(result: i32, err: *c.thinkthen_error, diag: ?*Diagnostic) FailureError!void {
    if (result == c.THINKTHEN_OK) return;
    defer native.function("thinkthen_error_free")(err);
    const code: ErrorCode = @fromBackingInt(@intCast(err.code));
    const name = native.function("thinkthen_code_name")(err.code);
    return promote(.{ .code = code, .raw_name = if (name == null) null else std.mem.span(name), .message = bytes(err.message), .http_status = if (err.http_status == 0) null else err.http_status }, diag);
}
