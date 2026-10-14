const std = @import("std");
pub const authored = @import("authored.zig");
pub const inputs = authored.inputs;
pub const Question = authored.Question;
pub const Request = authored.Request;
pub const Plan = authored.Plan;
pub const session = @import("session.zig");
pub const calls = @import("calls_generated.zig");
const c = @import("native.zig").c;
const values = @import("results_generated.zig");
pub const results = values.Types;

/// Free message with the allocator passed to Engine.init, even when construction failed.
pub const FailureKind = enum(c_int) { usage = 1, backend, deadline, local, cancelled, defect };
pub const Failure = struct {
    code: c_int,
    kind: FailureKind,
    retryable: bool,
    message: []u8,
    facts: ?results.ImmediateFacts,
    arena: std.heap.ArenaAllocator,
    packets: []const *session.Packet = &.{},
    terminal: ?*const results.CallError = null,
};
pub fn releaseFailure(allocator: std.mem.Allocator, failure: Failure) void {
    allocator.free(failure.message);
    for (failure.packets) |packet| packet.deinit();
    var arena = failure.arena;
    arena.deinit();
}
pub fn Result(comptime T: type) type {
    return union(enum) { ok: T, failed: Failure };
}
pub const UsagePersistence = results.UsagePersistence;
/// Owned advice uses the engine allocator and survives engine destruction.
pub const UsageStatus = struct {
    state: UsagePersistence,
    advice: ?[]const u8,
    pub fn deinit(self: UsageStatus, allocator: std.mem.Allocator) void {
        if (self.advice) |value| allocator.free(value);
    }
};

pub const Engine = struct {
    handle: *anyopaque,
    closed: bool = false,
    allocator: std.mem.Allocator,
    pub fn init(allocator: std.mem.Allocator) !Result(Engine) {
        return initWithSettings(allocator, .{});
    }
    pub fn initWithSettings(allocator: std.mem.Allocator, settings: inputs.EngineSettings) !Result(Engine) {
        const encoded = try std.json.Stringify.valueAlloc(allocator, settings, .{ .emit_null_optional_fields = false });
        defer allocator.free(encoded);
        const terminated = try allocator.dupeZ(u8, encoded);
        defer allocator.free(terminated);
        const raw = c.thinkthen_engine_new_with(terminated.ptr) orelse return .{ .failed = try capture(allocator, null, c.thinkthen_error_code(null)) };
        return .{ .ok = .{ .handle = raw, .allocator = allocator } };
    }
    pub fn deinit(self: *Engine) void {
        if (!self.closed) c.thinkthen_engine_free(@as(*c.thinkthen_engine, @ptrCast(self.handle)));
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
        try session.checked(operation(@as(*c.thinkthen_engine, @ptrCast(self.handle)), &state, &advice));
        const kind = try values.readUsagePersistence(self.allocator, state);
        if (advice.data == null and advice.len != 0) return error.InvalidNativeExtent;
        const copied = if (advice.data == null) null else try self.allocator.dupe(u8, advice.data[0..advice.len]);
        return .{ .state = kind, .advice = copied };
    }
};

pub fn capture(allocator: std.mem.Allocator, engine: ?*Engine, code: c_int) !Failure {
    const raw = c.thinkthen_error_view(if (engine) |value| @ptrCast(value.handle) else null);
    if (raw == null) return error.Defect;
    const view = raw[0];
    var arena = std.heap.ArenaAllocator.init(allocator);
    errdefer arena.deinit();
    const facts = if (view.facts != null) try values.readImmediateFacts(arena.allocator(), view.facts[0]) else null;
    const message = try allocator.dupe(u8, if (view.message.len == 0) &.{} else view.message.data[0..view.message.len]);
    return .{ .code = code, .kind = std.meta.intToEnum(FailureKind, code) catch .defect, .retryable = view.retryable != 0, .message = message, .facts = facts, .arena = arena };
}
