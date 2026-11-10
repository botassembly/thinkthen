//! ThinkThen for Zig: ask questions about text and records, and read typed answers.
//!
//! Each ThinkThen release supports exactly one Zig version, the `minimum_zig_version`
//! in this package's build.zig.zon.
//!
//! One record: `const is_refund = try (try client.decide(arena, question, .{ .text = message }, .{}, &diag)).answer(&diag);`
//! Many records: `var rows = try client.decideAll(gpa, question, .{ .texts = &messages }, .{}, &diag);`
//! then `while (try rows.next(&diag)) |row| ...`. A not-sure row used as an answer returns
//! `error.Unsure`; it is never read as no.
const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const d = @import("diagnostic.zig");
const t = @import("types.zig");
const q = @import("questions.zig");
const r = @import("row.zig");
const calls = @import("call.zig");
const Allocator = std.mem.Allocator;
/// Every error the public interface returns.
pub const Error = d.Error;
/// Errors from reading a row as its answer.
pub const AnswerError = d.AnswerError;
/// Details of the last error; pass `&diag` to any call.
pub const Diagnostic = d.Diagnostic;
/// The engine's error codes.
pub const ErrorCode = d.ErrorCode;
/// One failed record or operation.
pub const Failure = d.Failure;
/// A many-record call.
pub const Call = calls.Call;
/// One record's result.
pub const Row = r.Row;
/// One question's outcome.
pub const Outcome = r.Outcome;
/// The answer type of each function.
pub const Answer = r.Answer;
/// The detail type of each function.
pub const Detail = r.Detail;
/// A filter row with its record.
pub const FilterRow = calls.FilterRow;
/// What `next` yields: a row, or for filter the kept record.
pub const Item = calls.Item;
/// A row of any function.
pub const AnyRow = r.AnyRow;
/// The service that answers questions.
pub const Backend = t.Backend;
/// The ten question functions.
pub const Function = t.Function;
/// When decide and filter answer.
pub const Threshold = t.Threshold;
/// A backend key that never prints.
pub const ApiKey = t.ApiKey;
/// Retry limits for backend requests.
pub const Retry = t.Retry;
/// Client settings.
pub const Settings = t.Settings;
/// Explicit local response-store configuration.
pub const ResponseStore = t.ResponseStore;
/// Local response-store execution mode.
pub const ResponseMode = t.ResponseMode;
/// One record.
pub const Record = t.Record;
/// Records pulled one at a time.
pub const RecordSource = t.RecordSource;
/// The records of a many-record call.
pub const Input = t.Input;
/// One label, level or kind.
pub const Choice = t.Choice;
/// Choices from label names, built at compile time.
pub const choices = t.choices;
/// One relation to find between names.
pub const RelationRule = t.RelationRule;
/// The question type of each function.
pub const QuestionType = q.QuestionType;
/// A saved question of any function.
pub const EntryQuestion = q.EntryQuestion;
/// An annotate question of any function but rank and annotate.
pub const AnnotationQuestion = q.AnnotationQuestion;
/// A saved question and its name.
pub const NamedQuestion = q.NamedQuestion;
/// Saved questions, loaded from or written to JSON.
pub const Toolbox = @import("toolbox.zig").Toolbox;
/// An annotate question and its name.
pub const NamedAnnotationQuestion = q.NamedAnnotationQuestion;
/// Annotate's questions.
pub const AnnotationQuestions = q.AnnotationQuestions;
/// A decide question, for `open`, `openAll` and saved toolboxes.
pub const Decide = q.Decide;
/// Options for decide: a name, the field pointer and the function's own settings.
pub const DecideOptions = q.DecideOptions;
/// A choose question, for `open`, `openAll` and saved toolboxes.
pub const Choose = q.Choose;
/// Options for choose: a name, the field pointer and the function's own settings.
pub const ChooseOptions = q.ChooseOptions;
/// A tag question, for `open`, `openAll` and saved toolboxes.
pub const Tag = q.Tag;
/// Options for tag: a name, the field pointer and the function's own settings.
pub const TagOptions = q.TagOptions;
/// A score question, for `open`, `openAll` and saved toolboxes.
pub const Score = q.Score;
/// Options for score: a name, the field pointer and the function's own settings.
pub const ScoreOptions = q.ScoreOptions;
/// A filter question, for `open`, `openAll` and saved toolboxes.
pub const Filter = q.Filter;
/// Options for filter: a name, the field pointer and the function's own settings.
pub const FilterOptions = q.FilterOptions;
/// A rank question, for `open`, `openAll` and saved toolboxes.
pub const Rank = q.Rank;
/// Options for rank: a name, the field pointer and the function's own settings.
pub const RankOptions = q.RankOptions;
/// A find question, for `open`, `openAll` and saved toolboxes.
pub const Find = q.Find;
/// Options for find: a name, the field pointer and the function's own settings.
pub const FindOptions = q.FindOptions;
/// A recognize question, for `open`, `openAll` and saved toolboxes.
pub const Recognize = q.Recognize;
/// Options for recognize: a name, the field pointer and the function's own settings.
pub const RecognizeOptions = q.RecognizeOptions;
/// A relate question, for `open`, `openAll` and saved toolboxes.
pub const Relate = q.Relate;
/// Options for relate: a name, the field pointer and the function's own settings.
pub const RelateOptions = q.RelateOptions;
/// A annotate question, for `open`, `openAll` and saved toolboxes.
pub const Annotate = q.Annotate;
/// Options for annotate: a name, the field pointer and the function's own settings.
pub const AnnotateOptions = q.AnnotateOptions;
/// A row's state.
pub const Status = r.Status;
/// A record's id.
pub const RecordId = r.RecordId;
/// Where an answer came from.
pub const Source = r.Source;
/// A piece of the record's text.
pub const Span = r.Span;
/// A recognized name.
pub const Name = r.Name;
/// One end of a relation.
pub const Endpoint = r.Endpoint;
/// A relation between two names.
pub const Relation = r.Relation;
/// Choose and tag detail.
pub const LabelDetail = r.LabelDetail;
/// Score and rank detail.
pub const LevelDetail = r.LevelDetail;
/// Recognize detail.
pub const RecognizeDetail = r.RecognizeDetail;
/// Relate detail.
pub const RelationDetail = r.RelationDetail;
/// One annotate question's name and outcome.
pub const NamedAnnotation = r.NamedAnnotation;
/// One annotate question's outcome.
pub const Annotation = r.Annotation;
/// Annotate's answer, read by name.
pub const Annotations = r.Annotations;
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
    cache_hits: u64,
    replay_hits: u64,
    input_tokens: TokenUsage,
    output_tokens: TokenUsage,
};
/// A connection to the engine and its backend. Open with `init`, close with `deinit`.
/// Each function has a one-record form, such as `decide`, which returns a row,
/// and a many-record form, such as `decideAll`, which returns a Call. One client may serve many threads.
pub const Client = struct {
    handle: ?*c.thinkthen_client,
    /// Open a client. Pass `&diag` to learn why it failed.
    pub fn init(settings: Settings, diag: ?*Diagnostic) (d.FailureError || error{OutOfMemory})!Client {
        d.clear(diag);
        native.ready() catch |err| switch (err) {
            error.EngineVersion => return d.incompatible(diag),
            error.LoadFailure => {
                if (diag) |value| value.set(.{ .code = .local_io, .message = native.failure() });
                return error.Setup;
            },
        };
        // Settings encode on the stack; a long model or URL falls back to the heap only for this call.
        var buffer: [1024]u8 = undefined;
        var fallback: std.heap.BufferFirstAllocator = .init(&buffer, std.heap.page_allocator);
        const a = fallback.allocator();
        // The engine admits the environment's backend name, so an unknown name fails as usage.backend.
        const backend: ?[]const u8 = if (settings.backend) |value| @tagName(value) else if (std.c.getenv("THINKTHEN_BACKEND")) |name| std.mem.span(name) else null;
        const encoded = try std.json.Stringify.valueAlloc(a, .{ .backend = backend, .model = settings.model, .url = settings.url, .batch = settings.batch, .max_requests = settings.max_requests, .response_store = settings.response_store, .retry = settings.retry }, .{ .emit_null_optional_fields = false });
        defer a.free(encoded);
        var builder: ?*c.thinkthen_settings = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_settings_from_json")(@ptrCast(encoded.ptr), encoded.len, &builder, &err), &err, diag);
        defer native.function("thinkthen_settings_free")(builder);
        if (settings.api_key) |key| {
            const bytes: [*]const u8 = @ptrCast(key.pointer);
            try d.check(native.function("thinkthen_settings_set_api_key")(builder, bytes, key.length, &err), &err, diag);
        }
        var handle: ?*c.thinkthen_client = null;
        try d.check(native.function("thinkthen_client_open")(builder, &handle, &err), &err, diag);
        return .{ .handle = handle };
    }
    /// Copy usage before deinit; the value remains valid independently of the client.
    pub fn usage(self: *const Client, allocator: Allocator, diag: ?*Diagnostic) Error!Usage {
        d.clear(diag);
        var output: c.thinkthen_buffer = std.mem.zeroes(c.thinkthen_buffer);
        defer native.function("thinkthen_buffer_free")(&output);
        var err = d.emptyError();
        try d.check(native.function("thinkthen_client_usage")(self.handle, &output, &err), &err, diag);
        const parsed = std.json.parseFromSlice(Usage, allocator, d.bytes(output), .{ .ignore_unknown_fields = true }) catch |failure| switch (failure) {
            error.OutOfMemory => return error.OutOfMemory,
            else => return d.incompatible(diag),
        };
        defer parsed.deinit();
        return parsed.value;
    }
    /// Close the client. Calls already open keep working until their own deinit.
    pub fn deinit(self: *Client) void {
        if (self.handle) |handle| native.function("thinkthen_client_free")(handle);
        self.handle = null;
    }
    /// Ask any question of many records. Free the call with `deinit`.
    /// allocator holds the call's bookkeeping and the rows `next` returns.
    pub fn openAll(self: *const Client, comptime function: Function, allocator: Allocator, question: QuestionType(function), input: Input, diag: ?*Diagnostic) Error!Call(function) {
        return calls.open(function, self.handle, allocator, question, input, diag);
    }
    /// Ask any question of one record. The row lives in arena, which must be an arena.
    /// For filter, the result is the record when kept, null when not, and `error.Unsure` when not sure;
    /// a failed record returns its error.
    pub fn open(self: *const Client, comptime function: Function, arena: Allocator, question: QuestionType(function), record: Record, diag: ?*Diagnostic) Error!One(function) {
        var call = try self.openAll(function, arena, question, .{ .records = &.{record} }, diag);
        defer call.deinit();
        if (function == .filter) {
            const item = (try call.nextRowInto(arena, diag)) orelse return error.Internal;
            return if (try item.row.answer(diag)) record else null;
        }
        const row = (try call.nextInto(arena, diag)) orelse return error.Internal;
        if (row.outcome == .failed and row.outcome.failed.code == .record_empty) return d.promote(row.outcome.failed, diag);
        return row;
    }
    /// Is the answer yes? The answer is a bool.
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn decide(self: *const Client, arena: Allocator, question: []const u8, record: Record, options: DecideOptions, diag: ?*Diagnostic) Error!Row(.decide) {
        return self.open(.decide, arena, .{ .question = question, .options = options }, record, diag);
    }
    /// Is the answer yes? The answer is a bool.
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn decideAll(self: *const Client, allocator: Allocator, question: []const u8, input: Input, options: DecideOptions, diag: ?*Diagnostic) Error!Call(.decide) {
        return self.openAll(.decide, allocator, .{ .question = question, .options = options }, input, diag);
    }
    /// Which one label fits? The answer is the label.
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn choose(self: *const Client, arena: Allocator, question: []const u8, labels: []const Choice, record: Record, options: ChooseOptions, diag: ?*Diagnostic) Error!Row(.choose) {
        return self.open(.choose, arena, .{ .question = question, .choices = labels, .options = options }, record, diag);
    }
    /// Which one label fits? The answer is the label.
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn chooseAll(self: *const Client, allocator: Allocator, question: []const u8, labels: []const Choice, input: Input, options: ChooseOptions, diag: ?*Diagnostic) Error!Call(.choose) {
        return self.openAll(.choose, allocator, .{ .question = question, .choices = labels, .options = options }, input, diag);
    }
    /// Which labels fit? The answer is every fitting label.
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn tag(self: *const Client, arena: Allocator, question: []const u8, labels: []const Choice, record: Record, options: TagOptions, diag: ?*Diagnostic) Error!Row(.tag) {
        return self.open(.tag, arena, .{ .question = question, .labels = labels, .options = options }, record, diag);
    }
    /// Which labels fit? The answer is every fitting label.
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn tagAll(self: *const Client, allocator: Allocator, question: []const u8, labels: []const Choice, input: Input, options: TagOptions, diag: ?*Diagnostic) Error!Call(.tag) {
        return self.openAll(.tag, allocator, .{ .question = question, .labels = labels, .options = options }, input, diag);
    }
    /// How much, from 0 to 1, across ordered levels?
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn score(self: *const Client, arena: Allocator, question: []const u8, levels: []const Choice, record: Record, options: ScoreOptions, diag: ?*Diagnostic) Error!Row(.score) {
        return self.open(.score, arena, .{ .question = question, .levels = levels, .options = options }, record, diag);
    }
    /// How much, from 0 to 1, across ordered levels?
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn scoreAll(self: *const Client, allocator: Allocator, question: []const u8, levels: []const Choice, input: Input, options: ScoreOptions, diag: ?*Diagnostic) Error!Call(.score) {
        return self.openAll(.score, allocator, .{ .question = question, .levels = levels, .options = options }, input, diag);
    }
    /// Keep the record?
    /// One record. Returns the record when kept and null when not; not sure returns `error.Unsure`, and a failed record its error.
    /// The result lives in arena, which must be an arena.
    pub fn filter(self: *const Client, arena: Allocator, question: []const u8, record: Record, options: FilterOptions, diag: ?*Diagnostic) Error!?Record {
        return self.open(.filter, arena, .{ .question = question, .options = options }, record, diag);
    }
    /// Keep the record?
    /// Many records. `next` yields only kept records; `nextRow` yields every row. Free the call with `deinit`.
    pub fn filterAll(self: *const Client, allocator: Allocator, question: []const u8, input: Input, options: FilterOptions, diag: ?*Diagnostic) Error!Call(.filter) {
        return self.openAll(.filter, allocator, .{ .question = question, .options = options }, input, diag);
    }
    /// Order records by level. The answer is the rank, counting from 1.
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn rank(self: *const Client, arena: Allocator, question: []const u8, levels: []const Choice, record: Record, options: RankOptions, diag: ?*Diagnostic) Error!Row(.rank) {
        return self.open(.rank, arena, .{ .question = question, .levels = levels, .options = options }, record, diag);
    }
    /// Order records by level. The answer is the rank, counting from 1.
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn rankAll(self: *const Client, allocator: Allocator, question: []const u8, levels: []const Choice, input: Input, options: RankOptions, diag: ?*Diagnostic) Error!Call(.rank) {
        return self.openAll(.rank, allocator, .{ .question = question, .levels = levels, .options = options }, input, diag);
    }
    /// Which line answers? The answer is the span, or null when no line does.
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn find(self: *const Client, arena: Allocator, question: []const u8, record: Record, options: FindOptions, diag: ?*Diagnostic) Error!Row(.find) {
        return self.open(.find, arena, .{ .question = question, .options = options }, record, diag);
    }
    /// Which line answers? The answer is the span, or null when no line does.
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn findAll(self: *const Client, allocator: Allocator, question: []const u8, input: Input, options: FindOptions, diag: ?*Diagnostic) Error!Call(.find) {
        return self.openAll(.find, allocator, .{ .question = question, .options = options }, input, diag);
    }
    /// Which names of these kinds appear?
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn recognize(self: *const Client, arena: Allocator, kinds: []const Choice, record: Record, options: RecognizeOptions, diag: ?*Diagnostic) Error!Row(.recognize) {
        return self.open(.recognize, arena, .{ .kinds = kinds, .options = options }, record, diag);
    }
    /// Which names of these kinds appear?
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn recognizeAll(self: *const Client, allocator: Allocator, kinds: []const Choice, input: Input, options: RecognizeOptions, diag: ?*Diagnostic) Error!Call(.recognize) {
        return self.openAll(.recognize, allocator, .{ .kinds = kinds, .options = options }, input, diag);
    }
    /// Which relations hold between recognized names?
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn relate(self: *const Client, arena: Allocator, rules: []const RelationRule, record: Record, options: RelateOptions, diag: ?*Diagnostic) Error!Row(.relate) {
        return self.open(.relate, arena, .{ .rules = rules, .options = options }, record, diag);
    }
    /// Which relations hold between recognized names?
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn relateAll(self: *const Client, allocator: Allocator, rules: []const RelationRule, input: Input, options: RelateOptions, diag: ?*Diagnostic) Error!Call(.relate) {
        return self.openAll(.relate, allocator, .{ .rules = rules, .options = options }, input, diag);
    }
    /// Answer several named questions at once. Read each with `answer.get(function, name)`.
    /// One record. Read the answer with `try row.answer(&diag)`.
    /// The result lives in arena, which must be an arena.
    pub fn annotate(self: *const Client, arena: Allocator, form: AnnotationQuestions, record: Record, options: AnnotateOptions, diag: ?*Diagnostic) Error!Row(.annotate) {
        return self.open(.annotate, arena, .{ .questions = form, .options = options }, record, diag);
    }
    /// Answer several named questions at once. Read each with `answer.get(function, name)`.
    /// Many records. Read rows with `while (try call.next(&diag)) |row|`. Free the call with `deinit`.
    pub fn annotateAll(self: *const Client, allocator: Allocator, form: AnnotationQuestions, input: Input, options: AnnotateOptions, diag: ?*Diagnostic) Error!Call(.annotate) {
        return self.openAll(.annotate, allocator, .{ .questions = form, .options = options }, input, diag);
    }
};
/// What a one-record `open` returns: a row, or for filter the record when kept.
pub fn One(comptime function: Function) type {
    return if (function == .filter) ?Record else Row(function);
}
