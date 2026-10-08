const std = @import("std");
/// Host descriptors. Serialization is not the native request/result wire codec.
pub const Function = enum { decide, choose, tag, score, filter, rank, find, annotate, recognize, relate };
pub const FailureKind = enum(u8) { usage = 1, backend, deadline, local, cancelled, defect };
pub const ContentKind = enum { text, json };
pub const RuleKind = enum { default, null, cut, band };
pub const Media = enum { jpeg, png };
pub const SourceUnit = enum { line, window, file, image_file };
pub const ValueKind = enum { null, boolean, authored };
pub const AtomicKind = enum { yes_no, choice, tag, score, find };
pub const MemberState = enum { success, failure };
pub const MemberCause = enum { missing_answer, wrong_kind, missing_probability, invalid_probability, invalid_distribution, unexpected_probability };
pub const Origin = enum { live, cache, replay, proxy, memory };
pub const AttemptOutcome = enum { ok, status, transport };
pub const RelationMethod = enum { yes_no, choice };
pub const Direction = enum { source_to_target, either };
pub const Stage = enum { boundary, kind, edge, relation };
pub const IdentityKind = enum { observation, failure };
pub const StopCause = enum { usage, local, no_key, transport, status, too_large, reply, backend, cancelled, defect, deadline };
pub const BatchKind = enum { records, max };
pub const EventKind = enum { question, row };
fn Identity(comptime Tag: type) type {
    return struct {
        value: [64]u8,
        pub const Kind = Tag;
        pub fn jsonStringify(self: @This(), writer: *std.json.Stringify) !void {
            try writer.write(&self.value);
        }
        pub fn jsonParse(allocator: std.mem.Allocator, source: anytype, options: std.json.ParseOptions) !@This() {
            const value = try std.json.innerParse([]const u8, allocator, source, options);
            return init(value) catch return error.InvalidCharacter;
        }
        pub fn init(value: []const u8) error{InvalidIdentity}!@This() {
            if (value.len != 64) return error.InvalidIdentity;
            for (value) |c| if (!((c >= '0' and c <= '9') or (c >= 'a' and c <= 'f'))) return error.InvalidIdentity;
            var result: @This() = undefined;
            @memcpy(&result.value, value);
            return result;
        }
    };
}
pub const CallId = Identity(enum { call_id });
pub const SdkRequestId = Identity(enum { sdk_request_id });
pub const ObservationId = Identity(enum { observation_id });
pub const FailureId = Identity(enum { failure_id });
pub const AnswerId = Identity(enum { answer_id });
pub const Digest = Identity(enum { digest });
pub const Content = struct { kind: ContentKind, data: []const u8 };
pub const Rule = struct { kind: RuleKind, low: f64, high: f64 };
pub const Choice = struct { name: []const u8, description: ?Content, weight: ?f64 };
pub const Relation = struct { name: []const u8, source: []const u8, target: []const u8, reads: ?[]const u8, either: bool, single: bool };
pub const QuestionMember = struct { name: []const u8, question: Question };
pub const Question = struct { kind: Function, text: Content, yes: ?Content, no: ?Content, choices: []const Choice, threshold: Rule, relation_threshold: Rule, model: ?[]const u8, profile: ?[]const u8, batch: ?u64, batch_max: bool, none: bool, on: []const []const u8, members: []const QuestionMember, kinds: []const Choice, relations: []const Relation, name_pointer: ?[]const u8, kind_pointer: ?[]const u8, instructions: ?[]const u8 = null, entity_definition: ?[]const u8 = null };
pub const ImageInput = struct { media: Media, bytes: []const u8, filename: ?[]const u8 };
pub const ImageView = struct { media: Media, bytes: []const u8, width: u32, height: u32, filename: ?[]const u8 };
pub const RecordInput = struct { original: ?Content, context: ?Content, options: []const Choice, images: []const ImageInput };
pub const FileSource = struct { paths: []const []const u8, unit: SourceUnit, window: u64 };
pub const CallControls = struct { context: ?Content, batch: ?u64, batch_max: bool, attempts: bool };
pub const Probability = struct { name: []const u8, value: f64 };
pub const DecideValue = struct { kind: ValueKind, boolean: bool, authored: ?Content };
pub const AtomicAnswer = struct { kind: AtomicKind, probability: ?f64, pick: ?[]const u8, level: ?[]const u8, probabilities: []const Probability, confidence: ?f64 };
pub const Location = struct { file: ?[]const u8, first_line: ?u64, last_line: ?u64 };
pub const MemberValue = struct { function: Function, decide: ?DecideValue, choose: ?[]const u8, tag: ?[]const []const u8, score: ?f64 };
pub const MemberFailure = struct { failure_id: FailureId, cause: MemberCause };
pub const MemberSuccess = struct { answer_id: AnswerId, value: MemberValue, answer: AtomicAnswer, threshold: Rule };
pub const AnnotationMember = struct { name: []const u8, request: Digest, question: Question, state: MemberState, success: ?MemberSuccess, failure: ?MemberFailure };
pub const Entity = struct { text: []const u8, start: u64, end: u64, length: u64, kind: []const u8, strength: f64 };
pub const EntityEdge = struct { relation: []const u8, source: Entity, target: Entity, probability: f64, either: bool };
pub const Place = struct { start: u64, end: u64 };
pub const Piece = struct { start: u64, end: u64, tags: []const Probability };
pub const NameSpan = struct { start: u64, end: u64, kinds: ?[]const Probability, edges: ?[]const Probability };
pub const PairSpan = struct { relation: []const u8, source: Place, target: Place, probability: f64 };
pub const RecognizeValue = struct { entities: []const Entity, relations: ?[]const EntityEdge };
pub const RecognizeAnswer = struct { pieces: []const Piece, names: []const NameSpan, pairs: []const PairSpan };
pub const Endpoint = struct { name: []const u8, kind: []const u8 };
pub const Edge = struct { relation: []const u8, source: Endpoint, target: Endpoint, probability: f64, either: bool };
pub const RelationSuccess = struct { answer_id: AnswerId, probability: f64, accepted: bool };
pub const RelationAnswer = struct { relation: []const u8, reads: []const u8, method: RelationMethod, direction: Direction, source: Endpoint, target: ?Endpoint, request: Digest, state: MemberState, success: ?RelationSuccess, failure: ?MemberFailure };
pub const TokenUsage = struct { input_tokens: u64, output_tokens: u64 };
pub const QuestionSource = struct { origin: Origin, answered_by: []const u8 };
pub const ObservationIdentity = struct { kind: IdentityKind, observation_id: ?ObservationId, failure_id: ?FailureId };
pub const ProfileWarning = struct { tuned_for: []const u8, running: []const u8 };
pub const BatchSetting = struct { kind: BatchKind, records: u64 };
pub const BatchWarning = struct { tuned_for: BatchSetting, running: BatchSetting };
pub const Attempt = struct { ordinal: u64, request_sha256: Digest, wall_ms: u64, outcome: AttemptOutcome, sdk_request_id: SdkRequestId, status: ?u16, server_ms: ?u64, request_id: ?[]const u8 };
pub const Meta = struct { tool: []const u8, question_sha256: ?Digest, questions_sha256: ?Digest, url: []const u8, model: []const u8, usage: ?TokenUsage, requests_sent: u64, cached: bool, requests: []const Digest, failed_questions: u64, profile_warning: ?ProfileWarning, batch_setting: ?BatchSetting, batch_warning: ?BatchWarning, context_sha256: ?Digest, attempts: ?[]const Attempt, origin: ?Origin, question_sources: []const QuestionSource, observations: []const ObservationIdentity, answered_by: ?[]const u8 };
pub const CallFacts = struct { call_id: CallId, cache_answers: u64, estimated_cost_usd: ?[]const u8 = null, input_tokens: ?u64 = null, model: ?[]const u8 = null, output_tokens: ?u64 = null, records: u64, requests_sent: u64, seconds: f64, command_ms: ?u64 = null };
pub const Stopped = struct { at: ?u64, cause: StopCause, status: ?u16, retryable: bool };
pub const CompleteError = struct { code: FailureKind, message: []const u8, retryable: bool, stopped: ?Stopped, facts: ?CallFacts, attempts: ?[]const Attempt };
pub const CommonRow = struct { answer_id: AnswerId, input: ?Content, question: ?Question, answer: ?AtomicAnswer, threshold: ?Rule, position: ?Location, input_file: ?[]const u8, meta: Meta, images: ?[]const ImageView };
pub const DecideRow = struct { common: CommonRow, value: DecideValue };
pub const ChooseRow = struct { common: CommonRow, value: ?[]const u8 };
pub const TagRow = struct { common: CommonRow, value: []const []const u8 };
pub const ScoreRow = struct { common: CommonRow, value: f64 };
pub const FilterRow = struct { common: CommonRow, value: bool };
pub const RankRow = struct { common: CommonRow, value: ?u64, question_name: ?[]const u8 };
pub const FindRow = struct { common: CommonRow, value: ?Content, index: ?u64 };
pub const AnnotateRow = struct { common: CommonRow, answers: []const AnnotationMember };
pub const RecognizeRow = struct { common: CommonRow, value: RecognizeValue, answer: RecognizeAnswer };
pub const RelateRow = struct { common: CommonRow, value: []const Edge, questions: []const RelationAnswer };
pub const RowValue = struct { function: Function, decide: ?DecideRow, choose: ?ChooseRow, tag: ?TagRow, score: ?ScoreRow, filter: ?FilterRow, rank: ?RankRow, find: ?FindRow, annotate: ?AnnotateRow, recognize: ?RecognizeRow, relate: ?RelateRow };
pub const ObservedProbabilities = struct { yes: ?f64, named: ?[]const Probability };
pub const ObservationSuccess = struct { answer_id: AnswerId, observation_id: ObservationId, value: MemberValue, probabilities: ObservedProbabilities, confidence: ?f64 };
pub const QuestionObservation = struct { index: u64, member: ?[]const u8, stage: ?Stage, position: u64, question_sha256: Digest, model: []const u8, url: []const u8, requests: []const Digest, requests_sent: u64, cached: bool, failed_questions: u64, usage: ?TokenUsage, question_sources: []const QuestionSource, state: MemberState, success: ?ObservationSuccess, failure: ?MemberFailure };
pub const RowObservation = struct { index: u64, value: RowValue };
pub const ObservationEvent = struct { kind: EventKind, question: ?QuestionObservation, row: ?RowObservation };
pub const QuestionInput = struct {
    question: ?Question,
    file: ?[]const u8,
    pub fn asked(value: Question) QuestionInput {
        return .{ .question = value, .file = null };
    }
    pub fn questionFile(path: []const u8) QuestionInput {
        return .{ .question = null, .file = path };
    }
};
pub const InputSource = struct {
    records: ?[]const RecordInput,
    files: ?FileSource,
    pub fn fromRecords(values: []const RecordInput) InputSource {
        return .{ .records = values, .files = null };
    }
    pub fn fromFiles(value: FileSource) InputSource {
        return .{ .records = null, .files = value };
    }
};
pub const CompleteRequest = struct { function: Function, question: QuestionInput, source: InputSource, controls: CallControls };
pub const CompleteSummary = struct { count: u64, observation_count: u64, schema: []const u8, answer_id: AnswerId, function: Function, meta: Meta, facts: CallFacts, attempts: ?[]const Attempt };
pub const CompleteResult = struct { summary: CompleteSummary, rows: []const RowValue, observations: []const ObservationEvent };
pub fn text(value: []const u8) Content {
    return .{ .kind = .text, .data = value };
}
pub fn json(allocator: std.mem.Allocator, value: []const u8) !Content {
    var parsed = try std.json.parseFromSlice(std.json.Value, allocator, value, .{});
    defer parsed.deinit();
    return .{ .kind = .json, .data = value };
}
pub fn asked(kind: Function, content: Content) Question {
    return .{ .kind = kind, .text = content, .yes = null, .no = null, .choices = &.{}, .threshold = .{ .kind = .default, .low = 0, .high = 0 }, .relation_threshold = .{ .kind = .default, .low = 0, .high = 0 }, .model = null, .profile = null, .batch = null, .batch_max = false, .none = false, .on = &.{}, .members = &.{}, .kinds = &.{}, .relations = &.{}, .name_pointer = null, .kind_pointer = null };
}
pub const Requests = struct {
    pub fn decide(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .decide, .question = question, .source = source, .controls = controls };
    }
    pub fn choose(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .choose, .question = question, .source = source, .controls = controls };
    }
    pub fn tag(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .tag, .question = question, .source = source, .controls = controls };
    }
    pub fn score(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .score, .question = question, .source = source, .controls = controls };
    }
    pub fn filter(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .filter, .question = question, .source = source, .controls = controls };
    }
    pub fn rank(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .rank, .question = question, .source = source, .controls = controls };
    }
    pub fn find(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .find, .question = question, .source = source, .controls = controls };
    }
    pub fn annotate(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .annotate, .question = question, .source = source, .controls = controls };
    }
    pub fn recognize(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .recognize, .question = question, .source = source, .controls = controls };
    }
    pub fn relate(question: QuestionInput, source: InputSource, controls: CallControls) CompleteRequest {
        return .{ .function = .relate, .question = question, .source = source, .controls = controls };
    }
};
/// Deep clone a host descriptor into a parse arena. Deinit after all slices are unused.
pub fn clone(comptime T: type, allocator: std.mem.Allocator, value: T) !std.json.Parsed(T) {
    const encoded = try std.json.Stringify.valueAlloc(allocator, value, .{});
    defer allocator.free(encoded);
    return std.json.parseFromSlice(T, allocator, encoded, .{ .allocate = .alloc_always });
}
/// Owned serialized facts; deinit after every borrowed field is unused.
/// Requires a real call ID and refuses legacy facts; never supplies defaults.
pub fn readFacts(allocator: std.mem.Allocator, input: []const u8) !std.json.Parsed(CallFacts) {
    // JSON uses snake_case matching the host field names.
    var parsed = try std.json.parseFromSlice(CallFacts, allocator, input, .{ .allocate = .alloc_always, .ignore_unknown_fields = true });
    errdefer parsed.deinit();
    const value = parsed.value;
    if (!std.math.isFinite(value.seconds) or value.seconds < 0) return error.InvalidFacts;
    if (value.estimated_cost_usd) |cost| {
        if (cost.len < 8 or cost[cost.len - 7] != '.') return error.InvalidFacts;
        for (cost, 0..) |c, i| if (i != cost.len - 7 and (c < '0' or c > '9')) return error.InvalidFacts;
    }
    return parsed;
}
