const std = @import("std");
pub const c = @cImport({
    @cInclude("thinkthen.h");
});

pub const Outcome = enum { yes, no, unsure };
pub const Answer = struct { outcome: Outcome, probability: f64 };

/// Free message with the allocator passed to Engine.init, even when construction failed.
pub const FailureKind = enum(c_int) { usage = 1, backend, deadline, local, cancelled, defect };
pub const Failure = struct {
    code: c_int,
    kind: FailureKind,
    retryable: bool,
    message: []u8,
    facts_json: ?[]u8,
};
pub fn Result(comptime T: type) type {
    return union(enum) { ok: T, failed: Failure };
}
pub const Options = struct { deadline_ms: i64 = c.THINKTHEN_NO_DEADLINE, cancel: ?*c.thinkthen_cancel_token = null };

/// Do not copy or free while calls are running. A token must outlive every call carrying it.
pub const CancelToken = struct {
    raw: *c.thinkthen_cancel_token,
    pub fn init() error{TokenUnavailable}!CancelToken {
        return .{ .raw = c.thinkthen_cancel_token_new() orelse return error.TokenUnavailable };
    }
    pub fn cancel(self: *CancelToken) void {
        c.thinkthen_cancel(self.raw);
    }
    pub fn deinit(self: *CancelToken) void {
        c.thinkthen_cancel_token_free(self.raw);
    }
};

pub const Engine = struct {
    raw: *c.thinkthen_engine,
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
        c.thinkthen_engine_free(self.raw);
    }
    pub fn freeFailure(self: *Engine, failure: Failure) void {
        self.allocator.free(failure.message);
        if (failure.facts_json) |facts| self.allocator.free(facts);
    }
    fn failed(self: *Engine, code: c_int) error{OutOfMemory}!Failure {
        return capture(self.allocator, self.raw, code);
    }
    pub fn decide(self: *Engine, question: [:0]const u8, evidence: []const u8, options: Options) !Result(Answer) {
        try checkCString(question);
        var raw: c.thinkthen_answer = undefined;
        const code = c.thinkthen_decide_opts(self.raw, question.ptr, evidence.ptr, evidence.len, options.deadline_ms, options.cancel, &raw);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        return .{ .ok = try convert(raw) };
    }
    /// Free successful answer slices with the engine allocator.
    pub fn decideMany(self: *Engine, question: [:0]const u8, evidence: []const []const u8, options: Options) !Result([]Answer) {
        try checkCString(question);
        const pointers = try self.allocator.alloc([*c]const u8, evidence.len);
        defer self.allocator.free(pointers);
        const lengths = try self.allocator.alloc(usize, evidence.len);
        defer self.allocator.free(lengths);
        const raw = try self.allocator.alloc(c.thinkthen_answer, evidence.len);
        defer self.allocator.free(raw);
        for (evidence, 0..) |text, i| {
            pointers[i] = text.ptr;
            lengths[i] = text.len;
        }
        const code = c.thinkthen_decide_many_opts(self.raw, question.ptr, pointers.ptr, lengths.ptr, evidence.len, options.deadline_ms, options.cancel, raw.ptr);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        const answers = try self.allocator.alloc(Answer, evidence.len);
        errdefer self.allocator.free(answers);
        for (raw, 0..) |answer, i| answers[i] = try convert(answer);
        return .{ .ok = answers };
    }
    /// The JSON door supports all ten verbs. Free successful bytes with the engine allocator.
    pub fn call(self: *Engine, request: [:0]const u8, options: Options) !Result([]u8) {
        try checkCString(request);
        const raw = c.thinkthen_call_opts(self.raw, request.ptr, options.deadline_ms, options.cancel) orelse
            return .{ .failed = try self.failed(c.thinkthen_error_code(self.raw)) };
        defer c.thinkthen_free_string(raw);
        return .{ .ok = try self.allocator.dupe(u8, std.mem.span(raw)) };
    }
    pub fn recognize(self: *Engine, spec: [:0]const u8, evidence: []const u8, options: Options) !Result([]u8) {
        try checkCString(spec);
        var raw: [*c]u8 = null;
        var len: usize = 0;
        const code = c.thinkthen_recognize_opts(self.raw, spec.ptr, evidence.ptr, evidence.len, options.deadline_ms, options.cancel, &raw, &len);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        defer c.thinkthen_free_string(raw);
        return .{ .ok = try self.allocator.dupe(u8, raw[0..len]) };
    }
    pub fn relate(self: *Engine, spec: [:0]const u8, records: []const []const u8, options: Options) !Result([]u8) {
        try checkCString(spec);
        const pointers = try self.allocator.alloc([*c]const u8, records.len);
        defer self.allocator.free(pointers);
        const lengths = try self.allocator.alloc(usize, records.len);
        defer self.allocator.free(lengths);
        for (records, 0..) |text, i| {
            pointers[i] = text.ptr;
            lengths[i] = text.len;
        }
        var raw: [*c]u8 = null;
        var len: usize = 0;
        const code = c.thinkthen_relate_opts(self.raw, spec.ptr, pointers.ptr, lengths.ptr, records.len, options.deadline_ms, options.cancel, &raw, &len);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        defer c.thinkthen_free_string(raw);
        return .{ .ok = try self.allocator.dupe(u8, raw[0..len]) };
    }
};

fn capture(allocator: std.mem.Allocator, engine: ?*c.thinkthen_engine, code: c_int) error{OutOfMemory}!Failure {
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
fn convert(raw: c.thinkthen_answer) error{InvalidAbiAnswer}!Answer {
    const outcome: Outcome = switch (raw.outcome) {
        c.THINKTHEN_YES => .yes,
        c.THINKTHEN_NO => .no,
        c.THINKTHEN_UNSURE => .unsure,
        else => return error.InvalidAbiAnswer,
    };
    return .{ .outcome = outcome, .probability = raw.probability };
}
fn checkCString(text: [:0]const u8) error{EmbeddedNul}!void {
    if (std.mem.indexOfScalar(u8, text, 0) != null) return error.EmbeddedNul;
}
