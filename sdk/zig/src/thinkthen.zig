const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const d = @import("diagnostic.zig");
const t = @import("types.zig");
const q = @import("questions.zig");
const r = @import("row.zig");
const calls = @import("call.zig");
const Allocator = std.mem.Allocator;
pub const Error = d.Error;
pub const Diagnostic = d.Diagnostic;
pub const ErrorCode = d.ErrorCode;
pub const Failure = d.Failure;
pub const Call = calls.Call;
pub const Row = r.Row;
pub const Outcome = r.Outcome;
pub const Answer = r.Answer;
pub const Detail = r.Detail;
pub const FilterRow = calls.FilterRow;
pub const AnyRow = r.AnyRow;
pub const Backend = t.Backend;
pub const Function = t.Function;
pub const Threshold = t.Threshold;
pub const ApiKey = t.ApiKey;
pub const Retry = t.Retry;
pub const Settings = t.Settings;
pub const Record = t.Record;
pub const RecordSource = t.RecordSource;
pub const Input = t.Input;
pub const Choice = t.Choice;
pub const RelationRule = t.RelationRule;
pub const QuestionType = q.QuestionType;
pub const EntryQuestion = q.EntryQuestion;
pub const AnnotationQuestion = q.AnnotationQuestion;
pub const NamedQuestion = q.NamedQuestion;
pub const Toolbox = @import("toolbox.zig").Toolbox;
pub const NamedAnnotationQuestion = q.NamedAnnotationQuestion;
pub const AnnotationQuestions = q.AnnotationQuestions;
pub const Decide = q.Decide;
pub const DecideOptions = q.DecideOptions;
pub const Choose = q.Choose;
pub const ChooseOptions = q.ChooseOptions;
pub const Tag = q.Tag;
pub const TagOptions = q.TagOptions;
pub const Score = q.Score;
pub const ScoreOptions = q.ScoreOptions;
pub const Filter = q.Filter;
pub const FilterOptions = q.FilterOptions;
pub const Rank = q.Rank;
pub const RankOptions = q.RankOptions;
pub const Find = q.Find;
pub const FindOptions = q.FindOptions;
pub const Recognize = q.Recognize;
pub const RecognizeOptions = q.RecognizeOptions;
pub const Relate = q.Relate;
pub const RelateOptions = q.RelateOptions;
pub const Annotate = q.Annotate;
pub const AnnotateOptions = q.AnnotateOptions;
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
pub const NamedAnnotation = r.NamedAnnotation;
pub const Annotation = r.Annotation;
pub const Client = struct {
    handle: ?*c.thinkthen_client,
    pub fn init(settings: Settings, diag: ?*Diagnostic) Error!Client {
        d.clear(diag);
        native.ready() catch |err| switch (err) {
            error.EngineVersion => return d.incompatible(diag),
            error.LoadFailure => {
                if (diag) |value| value.set(.{ .code = .local_io, .message = "cannot load native engine: " ++ native.path });
                return error.Setup;
            },
        };
        // A temporary arena holds only admission encoding, never a returned row.
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        defer arena.deinit();
        const a = arena.allocator();
        const encoded = try std.json.Stringify.valueAlloc(a, settings, .{ .emit_null_optional_fields = false });
        var builder: ?*c.thinkthen_settings = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_settings_from_json")(encoded.ptr, encoded.len, &builder, &err), &err, diag);
        defer native.function("thinkthen_settings_free")(builder);
        if (settings.api_key) |key| {
            const bytes: [*]const u8 = @ptrCast(key.pointer);
            try d.check(native.function("thinkthen_settings_set_api_key")(builder, bytes, key.length, &err), &err, diag);
        }
        var handle: ?*c.thinkthen_client = null;
        try d.check(native.function("thinkthen_client_open")(builder, &handle, &err), &err, diag);
        return .{ .handle = handle };
    }
    pub fn deinit(self: *Client) void {
        if (self.handle) |handle| native.function("thinkthen_client_free")(handle);
        self.handle = null;
    }
    pub fn open(self: *const Client, comptime function: Function, allocator: Allocator, question: QuestionType(function), input: Input, diag: ?*Diagnostic) Error!Call(function) {
        return calls.open(function, self.handle, allocator, question, input, diag);
    }
    pub fn one(self: *const Client, comptime function: Function, allocator: Allocator, question: QuestionType(function), record: Record, diag: ?*Diagnostic) Error!calls.Item(function) {
        var call = try self.open(function, allocator, question, .{ .records = &.{record} }, diag);
        defer call.deinit();
        return (try call.next(allocator, diag)) orelse error.Internal;
    }
    pub fn decide(self: *const Client, allocator: Allocator, question: []const u8, input: Input, options: DecideOptions, diag: ?*Diagnostic) Error!Call(.decide) {
        return self.open(.decide, allocator, .{ .question = question, .options = options }, input, diag);
    }
    pub fn decideOne(self: *const Client, allocator: Allocator, question: []const u8, record: Record, options: DecideOptions, diag: ?*Diagnostic) Error!calls.Item(.decide) {
        return self.one(.decide, allocator, .{ .question = question, .options = options }, record, diag);
    }
    pub fn choose(self: *const Client, allocator: Allocator, question: []const u8, choices: []const Choice, input: Input, options: ChooseOptions, diag: ?*Diagnostic) Error!Call(.choose) {
        return self.open(.choose, allocator, .{ .question = question, .choices = choices, .options = options }, input, diag);
    }
    pub fn chooseOne(self: *const Client, allocator: Allocator, question: []const u8, choices: []const Choice, record: Record, options: ChooseOptions, diag: ?*Diagnostic) Error!calls.Item(.choose) {
        return self.one(.choose, allocator, .{ .question = question, .choices = choices, .options = options }, record, diag);
    }
    pub fn tag(self: *const Client, allocator: Allocator, question: []const u8, labels: []const Choice, input: Input, options: TagOptions, diag: ?*Diagnostic) Error!Call(.tag) {
        return self.open(.tag, allocator, .{ .question = question, .labels = labels, .options = options }, input, diag);
    }
    pub fn tagOne(self: *const Client, allocator: Allocator, question: []const u8, labels: []const Choice, record: Record, options: TagOptions, diag: ?*Diagnostic) Error!calls.Item(.tag) {
        return self.one(.tag, allocator, .{ .question = question, .labels = labels, .options = options }, record, diag);
    }
    pub fn score(self: *const Client, allocator: Allocator, question: []const u8, levels: []const Choice, input: Input, options: ScoreOptions, diag: ?*Diagnostic) Error!Call(.score) {
        return self.open(.score, allocator, .{ .question = question, .levels = levels, .options = options }, input, diag);
    }
    pub fn scoreOne(self: *const Client, allocator: Allocator, question: []const u8, levels: []const Choice, record: Record, options: ScoreOptions, diag: ?*Diagnostic) Error!calls.Item(.score) {
        return self.one(.score, allocator, .{ .question = question, .levels = levels, .options = options }, record, diag);
    }
    pub fn filter(self: *const Client, allocator: Allocator, question: []const u8, input: Input, options: FilterOptions, diag: ?*Diagnostic) Error!Call(.filter) {
        return self.open(.filter, allocator, .{ .question = question, .options = options }, input, diag);
    }
    pub fn filterOne(self: *const Client, allocator: Allocator, question: []const u8, record: Record, options: FilterOptions, diag: ?*Diagnostic) Error!calls.Item(.filter) {
        return self.one(.filter, allocator, .{ .question = question, .options = options }, record, diag);
    }
    pub fn rank(self: *const Client, allocator: Allocator, question: []const u8, levels: []const Choice, input: Input, options: RankOptions, diag: ?*Diagnostic) Error!Call(.rank) {
        return self.open(.rank, allocator, .{ .question = question, .levels = levels, .options = options }, input, diag);
    }
    pub fn rankOne(self: *const Client, allocator: Allocator, question: []const u8, levels: []const Choice, record: Record, options: RankOptions, diag: ?*Diagnostic) Error!calls.Item(.rank) {
        return self.one(.rank, allocator, .{ .question = question, .levels = levels, .options = options }, record, diag);
    }
    pub fn find(self: *const Client, allocator: Allocator, question: []const u8, input: Input, options: FindOptions, diag: ?*Diagnostic) Error!Call(.find) {
        return self.open(.find, allocator, .{ .question = question, .options = options }, input, diag);
    }
    pub fn findOne(self: *const Client, allocator: Allocator, question: []const u8, record: Record, options: FindOptions, diag: ?*Diagnostic) Error!calls.Item(.find) {
        return self.one(.find, allocator, .{ .question = question, .options = options }, record, diag);
    }
    pub fn recognize(self: *const Client, allocator: Allocator, kinds: []const Choice, input: Input, options: RecognizeOptions, diag: ?*Diagnostic) Error!Call(.recognize) {
        return self.open(.recognize, allocator, .{ .kinds = kinds, .options = options }, input, diag);
    }
    pub fn recognizeOne(self: *const Client, allocator: Allocator, kinds: []const Choice, record: Record, options: RecognizeOptions, diag: ?*Diagnostic) Error!calls.Item(.recognize) {
        return self.one(.recognize, allocator, .{ .kinds = kinds, .options = options }, record, diag);
    }
    pub fn relate(self: *const Client, allocator: Allocator, rules: []const RelationRule, input: Input, options: RelateOptions, diag: ?*Diagnostic) Error!Call(.relate) {
        return self.open(.relate, allocator, .{ .rules = rules, .options = options }, input, diag);
    }
    pub fn relateOne(self: *const Client, allocator: Allocator, rules: []const RelationRule, record: Record, options: RelateOptions, diag: ?*Diagnostic) Error!calls.Item(.relate) {
        return self.one(.relate, allocator, .{ .rules = rules, .options = options }, record, diag);
    }
    pub fn annotate(self: *const Client, allocator: Allocator, toolbox: AnnotationQuestions, input: Input, options: AnnotateOptions, diag: ?*Diagnostic) Error!Call(.annotate) {
        return self.open(.annotate, allocator, .{ .questions = toolbox, .options = options }, input, diag);
    }
    pub fn annotateOne(self: *const Client, allocator: Allocator, toolbox: AnnotationQuestions, record: Record, options: AnnotateOptions, diag: ?*Diagnostic) Error!calls.Item(.annotate) {
        return self.one(.annotate, allocator, .{ .questions = toolbox, .options = options }, record, diag);
    }
};
