//! Typed questions and three-state results for Zig.
const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const d = @import("diagnostic.zig");
const t = @import("types.zig");
const q = @import("questions.zig");
const r = @import("row.zig");
const result = @import("result.zig");
const calls = @import("call.zig");
const Allocator = std.mem.Allocator;
pub const Error = d.Error;
pub const NoAnswer = d.AnswerError;
pub const Diagnostic = d.Diagnostic;
pub const ErrorCode = d.ErrorCode;
pub const Failure = d.Failure;
pub const Row = result.Row;
pub const Backend = t.Backend;
pub const Threshold = t.Threshold;
pub const Cut = t.Cut;
pub const ApiKey = t.ApiKey;
pub const Retry = t.Retry;
pub const Settings = t.Settings;
pub const Record = t.Record;
pub const Function = t.Function;
pub const Call = calls.Call;
pub const QuestionType = q.QuestionType;
pub const EntryQuestion = q.EntryQuestion;
pub const NamedQuestion = q.NamedQuestion;
pub const RecordSource = t.RecordSource;
pub const Input = t.Input;
pub const RelationRule = t.RelationRule;
pub const Toolbox = @import("toolbox.zig").Toolbox;
pub const Decide = q.Decide;
pub const Choose = q.Choose;
pub const Tag = q.Tag;
pub const Score = q.Score;
pub const Filter = q.Filter;
pub const Rank = q.Rank;
pub const Find = q.Find;
pub const Recognize = q.Recognize;
pub const Relate = q.Relate;
pub const annotate = q.annotate;
pub const Status = r.Status;
pub const RecordId = r.RecordId;
pub const Source = r.Source;
pub const Span = r.Span;
pub const Name = r.Name;
pub const Endpoint = r.Endpoint;
pub const Relation = r.Relation;
pub const LabelDetail = r.LabelDetail;
pub const LevelDetail = r.LevelDetail;
pub const RecognizeDetail = r.RecognizeDetail;
pub const RelationDetail = r.RelationDetail;
pub const Cancel = struct {
    handle: ?*c.thinkthen_cancel,
    pub fn init(diag: ?*Diagnostic) Error!Cancel {
        d.clear(diag);
        try d.ready(diag);
        var handle: ?*c.thinkthen_cancel = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_cancel_new")(&handle, &err), &err, diag);
        return .{ .handle = handle };
    }
    pub fn cancel(self: *const Cancel) void {
        native.function("thinkthen_cancel_request")(self.handle);
    }
    pub fn deinit(self: *Cancel) void {
        native.function("thinkthen_cancel_free")(self.handle);
        self.handle = null;
    }
};
/// Reported provider tokens; null means overflow, unknown attempts remain separate.
pub const TokenUsage = struct {
    reported: ?u64,
    unknown_attempts: u64,
};
/// A copied cumulative snapshot; active work may still incur more usage.
pub const Usage = struct {
    attempts: u64,
    responses: u64,
    status_retries: u64,
    connection_retries: u64,
    request_body_bytes: u64,
    active_attempts: u64,
    active_calls: u64,
    input_tokens: TokenUsage,
    output_tokens: TokenUsage,
};
pub const Client = struct {
    handle: ?*c.thinkthen_client,
    pub fn init(settings: Settings, diag: ?*Diagnostic) (d.FailureError || error{OutOfMemory})!Client {
        d.clear(diag);
        try d.ready(diag);
        // Settings encode on the stack; a long model or URL falls back to the heap only for this call.
        var buffer: [1024]u8 = undefined;
        var fallback: std.heap.BufferFirstAllocator = .init(&buffer, std.heap.page_allocator);
        const a = fallback.allocator();
        // The engine admits the environment's backend name, so an unknown name fails as usage.backend.
        const backend: ?[]const u8 = if (settings.backend) |value| @tagName(value) else if (std.c.getenv("THINKTHEN_BACKEND")) |name| std.mem.span(name) else null;
        const encoded = try std.json.Stringify.valueAlloc(a, .{ .backend = backend, .model = settings.model, .url = settings.url, .batch = settings.batch, .max_requests = settings.max_requests, .retry = settings.retry }, .{ .emit_null_optional_fields = false });
        defer a.free(encoded);
        var builder: ?*c.thinkthen_settings = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_settings_from_json")(@ptrCast(encoded.ptr), encoded.len, &builder, &err), &err, diag);
        defer native.function("thinkthen_settings_free")(builder);
        if (settings.api_key) |key| {
            const bytes: [*]const u8 = @ptrCast(key.pointer);
            try d.check(native.function("thinkthen_settings_set_api_key")(builder, bytes, key.length, &err), &err, diag);
        }
        if (settings.cancel) |stop| try d.check(native.function("thinkthen_settings_set_cancel")(builder, stop.handle, &err), &err, diag);
        var handle: ?*c.thinkthen_client = null;
        try d.check(native.function("thinkthen_client_open")(builder, &handle, &err), &err, diag);
        return .{ .handle = handle };
    }
    pub fn usage(self: *const Client, diag: ?*Diagnostic) Error!Usage {
        d.clear(diag);
        var out: c.thinkthen_usage = .{ .struct_size = @sizeOf(c.thinkthen_usage) };
        var err = d.emptyError();
        try d.check(native.function("thinkthen_client_usage_values")(self.handle, &out, &err), &err, diag);
        return .{ .attempts = out.attempts, .responses = out.responses, .status_retries = out.status_retries, .connection_retries = out.connection_retries, .request_body_bytes = out.request_body_bytes, .active_attempts = out.active_attempts, .active_calls = out.active_calls, .input_tokens = .{ .reported = if (out.input_tokens.reported_known) out.input_tokens.reported else null, .unknown_attempts = out.input_tokens.unknown_attempts }, .output_tokens = .{ .reported = if (out.output_tokens.reported_known) out.output_tokens.reported else null, .unknown_attempts = out.output_tokens.unknown_attempts } };
    }
    pub fn deinit(self: *Client) void {
        if (self.handle) |handle| native.function("thinkthen_client_free")(handle);
        self.handle = null;
    }
    fn one(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        const T = calls.singleType(@TypeOf(record));
        const item: T = record;
        var call = try calls.open(self.handle, arena, question, &[_]T{item}, diag);
        defer call.deinit();
        const row = (try call.nextInto(arena, diag)) orelse return error.Internal;
        if (row.status == .failed) {
            if (diag) |target| {
                target.set(row.failure.?);
                target.position = row.position;
            }
            return error.Failed;
        }
        return row;
    }
    pub fn decide(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .decide) @compileError("decide requires a decide question");
        return self.one(arena, question, record, diag);
    }
    pub fn choose(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .choose) @compileError("choose requires a choose question");
        return self.one(arena, question, record, diag);
    }
    pub fn tag(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .tag) @compileError("tag requires a tag question");
        return self.one(arena, question, record, diag);
    }
    pub fn score(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .score) @compileError("score requires a score question");
        return self.one(arena, question, record, diag);
    }
    pub fn find(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .find) @compileError("find requires a find question");
        return self.one(arena, question, record, diag);
    }
    pub fn recognize(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .recognize) @compileError("recognize requires a recognize question");
        return self.one(arena, question, record, diag);
    }
    pub fn relate(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .relate) @compileError("relate requires a relate question");
        return self.one(arena, question, record, diag);
    }
    pub fn annotate(self: *const Client, arena: Allocator, question: anytype, record: anytype, diag: ?*Diagnostic) Error!Row(@TypeOf(question), calls.singleType(@TypeOf(record))) {
        if (@TypeOf(question).function != .annotate) @compileError("annotate requires a annotate question");
        return self.one(arena, question, record, diag);
    }
    pub fn decideAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .decide) @compileError("decideAll requires a decide question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn chooseAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .choose) @compileError("chooseAll requires a choose question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn tagAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .tag) @compileError("tagAll requires a tag question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn scoreAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .score) @compileError("scoreAll requires a score question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn findAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .find) @compileError("findAll requires a find question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn recognizeAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .recognize) @compileError("recognizeAll requires a recognize question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn relateAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .relate) @compileError("relateAll requires a relate question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn annotateAll(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .annotate) @compileError("annotateAll requires a annotate question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn filter(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .filter) @compileError("filter requires a filter question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
    pub fn rank(self: *const Client, allocator: Allocator, question: anytype, records: anytype, diag: ?*Diagnostic) Error!calls.Call(@TypeOf(question), calls.RecordType(@TypeOf(records))) {
        if (@TypeOf(question).function != .rank) @compileError("rank requires a rank question");
        return calls.open(self.handle, allocator, question, records, diag);
    }
};
