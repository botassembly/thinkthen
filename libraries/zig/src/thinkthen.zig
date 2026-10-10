const std = @import("std");
pub const authored = @import("authored.zig");
pub const inputs = authored.inputs;
pub const Question = authored.Question;
pub const Request = authored.Request;
pub const Plan = authored.Plan;
pub const session = @import("session.zig");
pub const c = @cImport({
    @cInclude("thinkthen.h");
});

/// Free message with the allocator passed to Engine.init, even when construction failed.
pub const FailureKind = enum(c_int) { usage = 1, backend, deadline, local, cancelled, defect };
pub const Failure = struct {
    code: c_int,
    kind: FailureKind,
    retryable: bool,
    message: []u8,
    facts_json: ?[]u8,
};
pub fn releaseFailure(allocator: std.mem.Allocator, failure: Failure) void {
    allocator.free(failure.message);
    if (failure.facts_json) |facts| allocator.free(facts);
}
pub fn Result(comptime T: type) type {
    return union(enum) { ok: T, failed: Failure };
}
pub const UsagePersistence = enum(u32) {
    disabled = c.THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1,
    pending = c.THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1,
    written = c.THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1,
    failed = c.THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1,
};
/// Owned advice uses the engine allocator and survives engine destruction.
pub const UsageStatus = struct {
    state: UsagePersistence,
    advice: ?[]const u8,
    pub fn deinit(self: UsageStatus, allocator: std.mem.Allocator) void {
        if (self.advice) |value| allocator.free(value);
    }
};

pub const Engine = struct {
    raw: *c.thinkthen_engine,
    closed: bool = false,
    allocator: std.mem.Allocator,
    pub fn init(allocator: std.mem.Allocator) error{OutOfMemory}!Result(Engine) {
        const raw = c.thinkthen_engine_new() orelse return .{ .failed = try capture(allocator, null, c.thinkthen_error_code(null)) };
        return .{ .ok = .{ .raw = raw, .allocator = allocator } };
    }
    pub fn initWithSettings(allocator: std.mem.Allocator, settings: [:0]const u8) error{ OutOfMemory, EmbeddedNul }!Result(Engine) {
        try checkCString(settings);
        const raw = c.thinkthen_engine_new_with(settings.ptr) orelse
            return .{ .failed = try capture(allocator, null, c.thinkthen_error_code(null)) };
        return .{ .ok = .{ .raw = raw, .allocator = allocator } };
    }
    pub fn deinit(self: *Engine) void {
        if (!self.closed) c.thinkthen_engine_free(self.raw);
        self.closed = true;
    }
    pub fn usagePersistence(self: *Engine) !UsageStatus {
        return self.usageStatus(c.thinkthen_engine_usage_persistence_v1);
    }
    pub fn finishUsageStatus(self: *Engine) !UsageStatus {
        return self.usageStatus(c.thinkthen_engine_finish_usage_status_v1);
    }
    fn usageStatus(self: *Engine, operation: anytype) !UsageStatus {
        if (self.closed) return error.ClosedEngine;
        var state: c.thinkthen_complete_usage_persistence_v1 = undefined;
        var advice: c.thinkthen_complete_utf8_v1 = undefined;
        try session.checked(operation(self.raw, &state, &advice));
        const kind = std.meta.intToEnum(UsagePersistence, state.kind) catch return error.InvalidNativeDiscriminator;
        if (advice.data == null and advice.len != 0) return error.InvalidNativeExtent;
        const copied = if (advice.data == null) null else try self.allocator.dupe(u8, advice.data[0..advice.len]);
        return .{ .state = kind, .advice = copied };
    }
};

pub fn capture(allocator: std.mem.Allocator, engine: ?*c.thinkthen_engine, code: c_int) error{OutOfMemory}!Failure {
    // Copy both borrowed fields on the calling native thread before another C call.
    const retryable = c.thinkthen_error_retryable(engine) != 0;
    const message = c.thinkthen_error_message(engine);
    const facts = c.thinkthen_error_facts_json(engine);
    const copied_message = try allocator.dupe(u8, std.mem.span(message));
    errdefer allocator.free(copied_message);
    const copied_facts = if (facts) |value| try allocator.dupe(u8, std.mem.span(value)) else null;
    const kind: FailureKind = std.meta.intToEnum(FailureKind, code) catch .defect;
    return .{ .code = code, .kind = kind, .retryable = retryable, .message = copied_message, .facts_json = copied_facts };
}
fn checkCString(text: [:0]const u8) error{EmbeddedNul}!void {
    if (std.mem.indexOfScalar(u8, text, 0) != null) return error.EmbeddedNul;
}
