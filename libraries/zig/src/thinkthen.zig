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
pub fn releaseFailure(allocator: std.mem.Allocator, failure: Failure) void {
    allocator.free(failure.message);
    if (failure.facts_json) |facts| allocator.free(facts);
}
pub fn Result(comptime T: type) type {
    return union(enum) { ok: T, failed: Failure };
}
pub const CallFacts = struct {
    records: u64,
    requests_sent: u64,
    cache_answers: u64,
    seconds: f64,
    input_tokens: ?u64,
    output_tokens: ?u64,
    model: ?[]u8,
    pub fn deinit(self: CallFacts, allocator: std.mem.Allocator) void {
        if (self.model) |model| allocator.free(model);
    }
};
/// Release every successful typed result with the engine's original allocator,
/// even after the engine closes. Failure values use releaseFailure instead.
pub fn CallResult(comptime T: type) type {
    return struct {
        value: T,
        facts: CallFacts,
        pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
            self.facts.deinit(allocator);
            if (T == []Answer or T == []u8) allocator.free(self.value);
        }
    };
}
fn count(object: std.json.ObjectMap, key: []const u8) !u64 {
    const value = object.get(key) orelse return error.InvalidAbiFacts;
    if (value != .number_string) return error.InvalidAbiFacts;
    const digits = value.number_string;
    if (digits.len == 0) return error.InvalidAbiFacts;
    for (digits) |digit| if (digit < '0' or digit > '9') return error.InvalidAbiFacts;
    return std.fmt.parseInt(u64, digits, 10) catch error.InvalidAbiFacts;
}
fn optionalCount(object: std.json.ObjectMap, key: []const u8) !?u64 {
    if (!object.contains(key)) return null;
    return try count(object, key);
}
fn factsFromNative(allocator: std.mem.Allocator, raw: [*c]u8, len: usize) !CallFacts {
    if (raw == null) return error.InvalidAbiFacts;
    const parsed = std.json.parseFromSlice(std.json.Value, allocator, raw[0..len], .{ .parse_numbers = false }) catch |err| switch (err) {
        error.OutOfMemory => return err,
        else => return error.InvalidAbiFacts,
    };
    defer parsed.deinit();
    if (parsed.value != .object) return error.InvalidAbiFacts;
    const object = parsed.value.object;
    const seconds_value = object.get("seconds") orelse return error.InvalidAbiFacts;
    if (seconds_value != .number_string) return error.InvalidAbiFacts;
    const seconds = std.fmt.parseFloat(f64, seconds_value.number_string) catch return error.InvalidAbiFacts;
    if (!std.math.isFinite(seconds) or seconds < 0) return error.InvalidAbiFacts;
    var model: ?[]u8 = null;
    if (object.get("model")) |value| {
        if (value != .string) return error.InvalidAbiFacts;
        model = try allocator.dupe(u8, value.string);
    }
    errdefer if (model) |owned| allocator.free(owned);
    return .{
        .records = try count(object, "records"),
        .requests_sent = try count(object, "requests_sent"),
        .cache_answers = try count(object, "cache_answers"),
        .seconds = seconds,
        .input_tokens = try optionalCount(object, "input_tokens"),
        .output_tokens = try optionalCount(object, "output_tokens"),
        .model = model,
    };
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

test "owned facts reject malformed required and present optional fields" {
    const alloc = std.testing.allocator;
    const good = "{\"records\":18446744073709551615,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125}";
    const facts = try factsFromNative(alloc, @ptrCast(@constCast(good.ptr)), good.len);
    defer facts.deinit(alloc);
    try std.testing.expect(facts.records == std.math.maxInt(u64) and facts.seconds == 0.125 and facts.input_tokens == null and facts.model == null);
    const bad = [_][]const u8{
        "{\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125}",
        "{\"records\":null,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125}",
        "{\"records\":true,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125}",
        "{\"records\":-1,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125}",
        "{\"records\":0,\"requests_sent\":1.5,\"cache_answers\":0,\"seconds\":0.125}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":\"1\",\"seconds\":0.125}",
        "{\"records\":18446744073709551616,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":\"0.125\"}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":-0.125}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":true}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":null}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":1e999}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125,\"input_tokens\":null}",
        "{\"records\":0,\"requests_sent\":0,\"cache_answers\":0,\"seconds\":0.125,\"model\":1}",
    };
    for (bad) |json| try std.testing.expectError(error.InvalidAbiFacts, factsFromNative(alloc, @ptrCast(@constCast(json.ptr)), json.len));
}
