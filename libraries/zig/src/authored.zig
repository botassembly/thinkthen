//! Generated payloads and owned native admission. Constructors copy caller bytes.
const std = @import("std");
const tt = @import("thinkthen.zig");
const c = tt.c;
pub const inputs = @import("request_generated.zig");
pub const results = @import("plan_generated.zig");

pub const Question = struct {
    owner: tt.native.Question,
    pub fn init(engine: *tt.Engine, role: tt.native.Role, definition: inputs.RequestDefinition) !tt.native.Outcome(Question) {
        const json = try std.json.Stringify.valueAlloc(engine.allocator, definition, .{ .emit_null_optional_fields = false });
        defer engine.allocator.free(json);
        return switch (try tt.native.parse(engine, role, json)) {
            .ok => |owner| .{ .ok = .{ .owner = owner } },
            .failed => |failure| .{ .failed = failure },
        };
    }
    /// The generated native author view borrows this immutable question.
    pub fn author(self: Question) tt.session.Error!c.thinkthen_question_author_v1 {
        var value: c.thinkthen_question_author_v1 = undefined;
        try tt.session.checked(c.thinkthen_question_author(self.owner.raw, &value));
        return value;
    }
    pub fn deinit(self: Question) void {
        self.owner.deinit();
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
        var raw: ?*c.thinkthen_session = null;
        try tt.session.checked(c.thinkthen_session_new(engine.raw, self.json.ptr, self.json.len, &raw));
        return .{ .raw = raw orelse return error.Defect };
    }
    /// Native preview reads no key or cache and sends no request.
    pub fn plan(self: Request, engine: *tt.Engine) !Plan {
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
