//! Generated payloads and owned native admission. Constructors copy caller bytes.
const std = @import("std");
const tt = @import("thinkthen.zig");
const c = tt.c;
pub const inputs = @import("request_generated.zig");
pub const results = @import("plan_generated.zig");

pub const Role = enum(u32) { atomic = 1, set, dynamic_choose, recognize, relate, rank, rank_set, find };
pub const Question = struct {
    raw: *c.thinkthen_question,
    pub fn init(engine: *tt.Engine, role: Role, definition: inputs.RequestDefinition) !tt.Result(Question) {
        if (engine.closed) return error.ClosedEngine;
        const json = try std.json.Stringify.valueAlloc(engine.allocator, definition, .{ .emit_null_optional_fields = false });
        defer engine.allocator.free(json);
        var raw: ?*c.thinkthen_question = null;
        const code = c.thinkthen_question_parse(engine.raw, @intFromEnum(role), .{ .data = json.ptr, .len = json.len }, &raw);
        if (code != c.THINKTHEN_OK) return .{ .failed = try tt.capture(engine.allocator, engine.raw, code) };
        return .{ .ok = .{ .raw = raw orelse return error.Defect } };
    }
    /// The generated native author view borrows this immutable question.
    pub fn author(self: Question) tt.session.Error!c.thinkthen_question_author_v1 {
        var value: c.thinkthen_question_author_v1 = undefined;
        try tt.session.checked(c.thinkthen_question_author(self.raw, &value));
        return value;
    }
    pub fn deinit(self: Question) void {
        c.thinkthen_question_free(self.raw);
    }
};

/// One owned canonical request, usable for preview or session admission.
pub const Request = struct {
    allocator: std.mem.Allocator,
    json: []const u8,
    pub fn init(allocator: std.mem.Allocator, call: inputs.RequestCall) !Request {
        return .{ .allocator = allocator, .json = try std.json.Stringify.valueAlloc(allocator, inputs.Request{ .call = call }, .{ .emit_null_optional_fields = false }) };
    }
    pub fn deinit(self: Request) void {
        self.allocator.free(self.json);
    }
    pub fn start(self: Request, engine: *tt.Engine) tt.session.Error!tt.session.Session {
        if (engine.closed) return error.ClosedEngine;
        var raw: ?*c.thinkthen_session = null;
        try tt.session.checked(c.thinkthen_session_new_with_surface(engine.raw, self.json.ptr, self.json.len, "zig", "zig".len, &raw));
        return .{ .raw = raw orelse return error.Defect };
    }
    /// Native preview reads no key or cache and sends no request.
    pub fn plan(self: Request, engine: *tt.Engine) !Plan {
        if (engine.closed) return error.ClosedEngine;
        var raw: [*c]u8 = null;
        var len: usize = 0;
        try tt.session.checked(c.thinkthen_request_plan_json(engine.raw, self.json.ptr, self.json.len, &raw, &len));
        if (raw == null) return error.Defect;
        errdefer c.thinkthen_free_string(raw);
        const parsed = try std.json.parseFromSlice(results.Plan, self.allocator, raw[0..len], .{ .allocate = .alloc_always, .ignore_unknown_fields = true });
        return .{ .raw = raw, .len = len, .parsed = parsed, .value = parsed.value };
    }
};

/// Typed fields and extension JSON remain live until explicit release.
pub const Plan = struct {
    raw: [*c]u8,
    len: usize,
    parsed: std.json.Parsed(results.Plan),
    value: results.Plan,
    pub fn json(self: Plan) []const u8 {
        return self.raw[0..self.len];
    }
    pub fn deinit(self: Plan) void {
        self.parsed.deinit();
        c.thinkthen_free_string(self.raw);
    }
};
