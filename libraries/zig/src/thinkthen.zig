const std = @import("std");
pub const authored = @import("authored.zig");
pub const inputs = authored.inputs;
pub const Question = authored.Question;
pub const Request = authored.Request;
pub const Plan = authored.Plan;
pub const session = @import("session.zig");
pub const complete = @import("complete.zig");
pub const native = @import("native.zig");
pub const c = @cImport({
    @cInclude("thinkthen.h");
});

pub const Outcome = enum(c_int) { yes = 1, no = 0, unsure = 2 };
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
pub fn releaseFailure(allocator: std.mem.Allocator, failure: Failure) void {
    allocator.free(failure.message);
    if (failure.facts_json) |facts| allocator.free(facts);
}
pub fn Result(comptime T: type) type {
    return union(enum) { ok: T, failed: Failure };
}
/// Facts, and a plan, are the engine's JSON as a host value per
/// specification/result.schema.json. A reader ignores members it does not know.
pub const Json = std.json.Parsed(std.json.Value);
/// Release every successful typed result with the engine's original allocator,
/// even after the engine closes. Failure values use releaseFailure instead.
pub fn CallResult(comptime T: type) type {
    return struct {
        value: T,
        facts: Json,
        pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
            self.facts.deinit();
            if (T == []Answer or T == []u8) allocator.free(self.value);
        }
    };
}
fn factsFromNative(allocator: std.mem.Allocator, raw: [*c]u8, len: usize) !Json {
    if (raw == null) return error.InvalidAbiFacts;
    // Copy every string: the native text is freed when the call returns.
    return std.json.parseFromSlice(std.json.Value, allocator, raw[0..len], .{ .allocate = .alloc_always }) catch |err| switch (err) {
        error.OutOfMemory => err,
        else => error.InvalidAbiFacts,
    };
}

/// One annotate member, per ADR 0112 section 4: JSON null is unresolved,
/// `{"failed": {"kind", "cause"}}` is a failure, and any other value is an
/// answer. No answered value is an object.
pub const Field = union(enum) {
    unresolved,
    answered: std.json.Value,
    failed: struct { kind: FailureKind, cause: []const u8 },
};
/// Any other object is error.NotAField. Members of the failure other than
/// kind and cause are ignored.
pub fn readField(member: std.json.Value) error{NotAField}!Field {
    const object = switch (member) {
        .null => return .unresolved,
        .object => |object| object,
        else => return .{ .answered = member },
    };
    if (object.count() != 1) return error.NotAField;
    const failure = object.get("failed") orelse return error.NotAField;
    if (failure != .object) return error.NotAField;
    const kind = failure.object.get("kind") orelse return error.NotAField;
    const cause = failure.object.get("cause") orelse return error.NotAField;
    if (kind != .string or cause != .string) return error.NotAField;
    return .{ .failed = .{ .kind = std.meta.stringToEnum(FailureKind, kind.string) orelse return error.NotAField, .cause = cause.string } };
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
    pub fn freeFailure(self: *Engine, failure: Failure) void {
        releaseFailure(self.allocator, failure);
    }
    fn failed(self: *Engine, code: c_int) error{OutOfMemory}!Failure {
        return capture(self.allocator, self.raw, code);
    }
    pub fn decide(self: *Engine, question: [:0]const u8, evidence: []const u8, options: Options) !Result(CallResult(Answer)) {
        try checkCString(question);
        var raw: c.thinkthen_answer = undefined;
        var facts: [*c]u8 = null;
        var facts_len: usize = 0;
        const code = c.thinkthen_decide_with_facts_opts(self.raw, question.ptr, evidence.ptr, evidence.len, options.deadline_ms, options.cancel, &raw, &facts, &facts_len);
        defer c.thinkthen_free_string(facts);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        return .{ .ok = .{ .value = try convert(raw), .facts = try factsFromNative(self.allocator, facts, facts_len) } };
    }
    /// Free successful answer slices with the engine allocator.
    pub fn decideMany(self: *Engine, question: [:0]const u8, evidence: []const []const u8, options: Options) !Result(CallResult([]Answer)) {
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
        var facts: [*c]u8 = null;
        var facts_len: usize = 0;
        const code = c.thinkthen_decide_many_with_facts_opts(self.raw, question.ptr, pointers.ptr, lengths.ptr, evidence.len, options.deadline_ms, options.cancel, raw.ptr, &facts, &facts_len);
        defer c.thinkthen_free_string(facts);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        const answers = try self.allocator.alloc(Answer, evidence.len);
        errdefer self.allocator.free(answers);
        for (raw, 0..) |answer, i| answers[i] = try convert(answer);
        return .{ .ok = .{ .value = answers, .facts = try factsFromNative(self.allocator, facts, facts_len) } };
    }
    /// Explicit native reader options, never an implicit path in evidence.
    pub const ReaderOptions = struct { unit: []const u8 = "line", window: ?usize = null };
    /// All ten question grammars through the explicit source envelope.
    pub fn files(self: *Engine, question: []const u8, paths: []const []const u8, reader: ReaderOptions, options: Options) !Result([]u8) {
        const parsed = try std.json.parseFromSlice(std.json.Value, self.allocator, question, .{});
        defer parsed.deinit();
        if (parsed.value != .object or parsed.value.object.contains("source")) return error.InvalidQuestion;
        const selection = try std.json.Stringify.valueAlloc(self.allocator, .{ .paths = paths, .unit = reader.unit, .window = reader.window }, .{});
        defer self.allocator.free(selection);
        const held = std.mem.trim(u8, question, " \t\r\n");
        const request = try std.fmt.allocPrintSentinel(self.allocator, "{s}{s}\"source\":{s}}}", .{ held[0 .. held.len - 1], if (parsed.value.object.count() == 0) "" else ",", selection }, 0);
        defer self.allocator.free(request);
        return self.call(request, options);
    }
    /// The JSON door supports all ten verbs. Free successful bytes with the engine allocator.
    pub fn call(self: *Engine, request: [:0]const u8, options: Options) !Result([]u8) {
        try checkCString(request);
        const raw = c.thinkthen_call_opts(self.raw, request.ptr, options.deadline_ms, options.cancel) orelse
            return .{ .failed = try self.failed(c.thinkthen_error_code(self.raw)) };
        defer c.thinkthen_free_string(raw);
        return .{ .ok = try self.allocator.dupe(u8, std.mem.span(raw)) };
    }
    pub fn recognize(self: *Engine, spec: [:0]const u8, evidence: []const u8, options: Options) !Result(CallResult([]u8)) {
        try checkCString(spec);
        var raw: [*c]u8 = null;
        var len: usize = 0;
        var facts: [*c]u8 = null;
        var facts_len: usize = 0;
        const code = c.thinkthen_recognize_with_facts_opts(self.raw, spec.ptr, evidence.ptr, evidence.len, options.deadline_ms, options.cancel, &raw, &len, &facts, &facts_len);
        defer c.thinkthen_free_string(facts);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        defer c.thinkthen_free_string(raw);
        const value = try self.allocator.dupe(u8, raw[0..len]);
        errdefer self.allocator.free(value);
        return .{ .ok = .{ .value = value, .facts = try factsFromNative(self.allocator, facts, facts_len) } };
    }
    /// Preview a decide, choose, score or tag call through thinkthen_plan_json
    /// without a key, a cache read or a send. A question starting with `{` is a
    /// question object and is sent as written; other text is the bare question.
    /// `settings` is one thinkthen.settings/1 object. Free a successful plan
    /// with deinit.
    pub fn plan(self: *Engine, verb: []const u8, question: []const u8, input: []const []const u8, settings: ?[]const u8) !Result(Json) {
        // Both spliced texts must each be one JSON object.
        for ([_]?[]const u8{ if (questionObject(question)) question else null, settings }) |given| {
            const text = std.mem.trim(u8, given orelse continue, " \t\r\n");
            if (!std.mem.startsWith(u8, text, "{") or !try std.json.validate(self.allocator, text)) return error.NotJsonObject;
        }
        var body: std.Io.Writer.Allocating = .init(self.allocator);
        defer body.deinit();
        planInput(&body.writer, verb, question, input, settings) catch return error.OutOfMemory;
        const request = try body.toOwnedSliceSentinel(0);
        defer self.allocator.free(request);
        try checkCString(request);
        var raw: [*c]u8 = null;
        var len: usize = 0;
        const code = c.thinkthen_plan_json(self.raw, request.ptr, &raw, &len);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        defer c.thinkthen_free_string(raw);
        return .{ .ok = std.json.parseFromSlice(std.json.Value, self.allocator, raw[0..len], .{ .allocate = .alloc_always }) catch |err| switch (err) {
            error.OutOfMemory => return err,
            else => return error.InvalidAbiJson,
        } };
    }
    pub fn relate(self: *Engine, spec: [:0]const u8, records: []const []const u8, options: Options) !Result(CallResult([]u8)) {
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
        var facts: [*c]u8 = null;
        var facts_len: usize = 0;
        const code = c.thinkthen_relate_with_facts_opts(self.raw, spec.ptr, pointers.ptr, lengths.ptr, records.len, options.deadline_ms, options.cancel, &raw, &len, &facts, &facts_len);
        defer c.thinkthen_free_string(facts);
        if (code != c.THINKTHEN_OK) return .{ .failed = try self.failed(code) };
        defer c.thinkthen_free_string(raw);
        const value = try self.allocator.dupe(u8, raw[0..len]);
        errdefer self.allocator.free(value);
        return .{ .ok = .{ .value = value, .facts = try factsFromNative(self.allocator, facts, facts_len) } };
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
    const outcome = std.meta.intToEnum(Outcome, raw.outcome) catch return error.InvalidAbiAnswer;
    return .{ .outcome = outcome, .probability = raw.probability };
}
/// The closed thinkthen.plan-input/1 object. As Go's Engine.Plan, a question
/// starting with `{` is spliced as written, so its key order is the caller's.
fn planInput(w: *std.Io.Writer, verb: []const u8, question: []const u8, input: []const []const u8, settings: ?[]const u8) std.Io.Writer.Error!void {
    try w.writeAll("{\"verb\":");
    try std.json.Stringify.encodeJsonString(verb, .{}, w);
    try w.writeAll(",\"question\":");
    if (questionObject(question)) try w.writeAll(std.mem.trim(u8, question, " \t\r\n")) else try std.json.Stringify.encodeJsonString(question, .{}, w);
    try w.writeAll(",\"input\":");
    try std.json.Stringify.value(input, .{}, w);
    if (settings) |text| {
        try w.writeAll(",\"settings\":");
        try w.writeAll(std.mem.trim(u8, text, " \t\r\n"));
    }
    try w.writeByte('}');
}
fn questionObject(question: []const u8) bool {
    return std.mem.startsWith(u8, std.mem.trimLeft(u8, question, " \t\r\n"), "{");
}
fn checkCString(text: [:0]const u8) error{EmbeddedNul}!void {
    if (std.mem.indexOfScalar(u8, text, 0) != null) return error.EmbeddedNul;
}
