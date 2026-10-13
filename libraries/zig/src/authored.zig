//! Generated payloads and owned native admission. Constructors copy caller bytes.
const std = @import("std");
const tt = @import("thinkthen.zig");
const c = @import("native.zig").c;
const values = @import("results_generated.zig");
pub const inputs = @import("request_generated.zig");
pub const results = @import("plan_generated.zig");

pub const Role = enum(u32) { atomic = 1, set, dynamic_choose, recognize, relate, rank, rank_set, find };
pub const Question = struct {
    handle: *anyopaque,
    pub fn init(engine: *tt.Engine, role: Role, definition: inputs.RequestDefinition) !tt.Result(Question) {
        if (engine.closed) return error.ClosedEngine;
        const json = try std.json.Stringify.valueAlloc(engine.allocator, definition, .{ .emit_null_optional_fields = false });
        defer engine.allocator.free(json);
        var raw: ?*c.thinkthen_question = null;
        const code = c.thinkthen_question_parse(@as(*c.thinkthen_engine, @ptrCast(engine.handle)), @intFromEnum(role), .{ .data = json.ptr, .len = json.len }, &raw);
        if (code != c.THINKTHEN_OK) return .{ .failed = try tt.capture(engine.allocator, @as(*c.thinkthen_engine, @ptrCast(engine.handle)), code) };
        return .{ .ok = .{ .handle = raw orelse return error.Defect } };
    }
    pub fn name(self: Question) !?[]const u8 {
        var value: c.thinkthen_question_author_v1 = undefined;
        try tt.session.checked(c.thinkthen_question_author(@ptrCast(self.handle), &value));
        return if (value.name.present != 0) tt.session.bytes(value.name.value) else null;
    }
    pub fn deinit(self: Question) void {
        c.thinkthen_question_free(@ptrCast(self.handle));
    }
};

const RequestStorage = struct {
    allocator: std.mem.Allocator,
    function: inputs.RequestFunction,
    question: []const u8,
    input: []const u8,
    options: []const u8,
};
/// The request owns its arguments for preview and admission.
pub const Request = opaque {
    fn storage(self: *const Request) *const RequestStorage {
        return @ptrCast(@alignCast(self));
    }
    pub fn init(allocator: std.mem.Allocator, call: inputs.RequestCall) !*Request {
        const owned = try allocator.create(RequestStorage);
        errdefer allocator.destroy(owned);
        const question = try std.json.Stringify.valueAlloc(allocator, call.question, .{ .emit_null_optional_fields = false });
        errdefer allocator.free(question);
        const input = try std.json.Stringify.valueAlloc(allocator, call.input, .{ .emit_null_optional_fields = false });
        errdefer allocator.free(input);
        const options = if (call.options) |value| try std.json.Stringify.valueAlloc(allocator, value, .{ .emit_null_optional_fields = false }) else try allocator.dupe(u8, "{}");
        owned.* = .{ .allocator = allocator, .function = call.function, .question = question, .input = input, .options = options };
        return @ptrCast(owned);
    }
    pub fn deinit(self: *Request) void {
        const owned: *RequestStorage = @ptrCast(@alignCast(self));
        const allocator = owned.allocator;
        allocator.free(owned.question);
        allocator.free(owned.input);
        allocator.free(owned.options);
        allocator.destroy(owned);
    }
    fn descriptor(self: *const Request) c.thinkthen_named_request_v1 {
        const owned = self.storage();
        return .{ .function = utf8(@tagName(owned.function)), .question_json = utf8(owned.question), .input_json = utf8(owned.input), .options_json = utf8(owned.options) };
    }
    pub fn start(self: *const Request, engine: *tt.Engine) !tt.session.Session {
        if (engine.closed) return error.ClosedEngine;
        var raw: ?*c.thinkthen_session = null;
        const request = self.descriptor();
        try tt.session.checked(c.thinkthen_session_new_named_with_surface(@ptrCast(engine.handle), &request, utf8("zig"), &raw));
        return .{ .handle = raw orelse return error.Defect, .allocator = engine.allocator };
    }
    /// Native preview sends nothing and never reads credentials.
    pub fn plan(self: *const Request, engine: *tt.Engine) !Plan {
        if (engine.closed) return error.ClosedEngine;
        var raw: ?*c.thinkthen_value = null;
        const request = self.descriptor();
        try tt.session.checked(c.thinkthen_request_plan_named_value(@ptrCast(engine.handle), &request, &raw));
        const owner = raw orelse return error.Defect;
        defer c.thinkthen_value_free(owner);
        var view: [*c]const c.thinkthen_complete_json_v1 = null;
        try tt.session.checked(c.thinkthen_value_view(owner, &view));
        var arena = std.heap.ArenaAllocator.init(self.storage().allocator);
        errdefer arena.deinit();
        const value = try values.readValue(arena.allocator(), view[0]);
        return .{ .arena = arena, .value = try values.readAs(results.Plan, value) };
    }
};
fn utf8(value: []const u8) c.thinkthen_complete_utf8_v1 {
    return .{ .data = value.ptr, .len = value.len };
}
pub const Plan = struct {
    arena: std.heap.ArenaAllocator,
    value: results.Plan,
    pub fn deinit(self: *Plan) void {
        self.arena.deinit();
    }
};
