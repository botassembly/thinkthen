const std = @import("std");
const c = @import("c");

pub const CallError = std.json.ParseError(std.json.Scanner) || error{EngineFailure};
pub const FailureView = struct { code: []const u8, message: []const u8 };

pub fn bytes(buffer: c.thinkthen_buffer) []const u8 {
    return if (buffer.len == 0) "" else buffer.data[0..buffer.len];
}

/// Owns the original ABI error even when decoding runs out of memory.
pub const Diagnostic = struct {
    allocator: std.mem.Allocator,
    raw: c.thinkthen_buffer = .{},
    parsed: ?std.json.Parsed(FailureView) = null,

    pub fn init(allocator: std.mem.Allocator) Diagnostic {
        return .{ .allocator = allocator };
    }

    pub fn clear(self: *Diagnostic) void {
        if (self.parsed) |value| value.deinit();
        self.parsed = null;
        c.thinkthen_buffer_free(&self.raw);
    }

    pub fn deinit(self: *Diagnostic) void {
        self.clear();
    }

    pub fn failure(self: *const Diagnostic) ?FailureView {
        return if (self.parsed) |value| value.value else null;
    }

    pub fn rawJson(self: *const Diagnostic) ?[]const u8 {
        return if (self.raw.len == 0) null else bytes(self.raw);
    }

    pub fn capture(self: *Diagnostic, raw: c.thinkthen_buffer) CallError {
        self.raw = raw;
        self.parsed = std.json.parseFromSlice(FailureView, self.allocator, bytes(raw), .{ .allocate = .alloc_always }) catch |err| return err;
        return error.EngineFailure;
    }
};
