// Generated owned values from the canonical native packet graph; do not edit.
const std = @import("std");
const c = @import("native.zig").c;
pub fn Presence(comptime T: type) type {
    return union(enum) {
        missing,
        null_value,
        value: T,
        pub fn optional(self: @This()) ?T {
            return switch (self) {
                .value => |v| v,
                else => null,
            };
        }
    };
}
pub fn Map(comptime T: type) type {
    return struct {
        pub const Entry = struct { key: []const u8, value: T };
        entries: []const Entry,
    };
}
pub const Number = union(enum) { signed: i64, unsigned: u64, decimal: f64 };
pub const Value = union(enum) {
    null_value,
    boolean: bool,
    number: Number,
    string: []const u8,
    array: []const Value,
    object: Map(Value),
};
fn text(v: c.thinkthen_complete_utf8_v1) []const u8 {
    return if (v.len == 0) &.{} else v.data[0..v.len];
}
fn readNumber(v: c.thinkthen_complete_utf8_v1) !Number {
    const source = text(v);
    if (std.mem.indexOfAny(u8, source, ".eE") != null) return .{ .decimal = try std.fmt.parseFloat(f64, source) };
    if (source.len != 0 and source[0] == '-') return .{ .signed = try std.fmt.parseInt(i64, source, 10) };
    return .{ .unsigned = try std.fmt.parseInt(u64, source, 10) };
}
/// Project a decoded native value into a generated typed record.
pub fn readAs(comptime T: type, v: Value) anyerror!T {
    switch (@typeInfo(T)) {
        .optional => |info| return if (v == .null_value) null else try readAs(info.child, v),
        .pointer => return if (T == []const u8 and v == .string) v.string else error.InvalidNativeValue,
        .bool => return if (v == .boolean) v.boolean else error.InvalidNativeValue,
        .int => return switch (v.number) {
            .unsigned => |n| std.math.cast(T, n) orelse error.InvalidNativeValue,
            .signed => |n| std.math.cast(T, n) orelse error.InvalidNativeValue,
            else => error.InvalidNativeValue,
        },
        .@"struct" => {
            if (v != .object) return error.InvalidNativeValue;
            var result: T = undefined;
            inline for (std.meta.fields(T)) |field| {
                var found = false;
                for (v.object.entries) |entry| if (std.mem.eql(u8, entry.key, field.name)) {
                    @field(result, field.name) = try readAs(field.type, entry.value);
                    found = true;
                    break;
                };
                if (!found) return error.MissingNativeField;
            }
            return result;
        },
        else => return error.InvalidNativeValue,
    }
}
pub fn readObject(a: std.mem.Allocator, v: anytype) anyerror!Map(Value) {
    const entries = try a.alloc(Map(Value).Entry, v.len);
    for (entries, 0..) |*out, i| out.* = .{ .key = try a.dupe(u8, text(v.data[i].name)), .value = try readValue(a, v.data[i].value[0]) };
    return .{ .entries = entries };
}
pub fn readValue(a: std.mem.Allocator, v: c.thinkthen_complete_json_v1) anyerror!Value {
    return switch (v.kind) {
        1 => .null_value,
        2 => .{ .boolean = v.data.boolean != 0 },
        3 => .{ .number = try readNumber(v.data.number) },
        4 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        5 => blk: {
            const items = try a.alloc(Value, v.data.array.len);
            for (items, 0..) |*out, i| out.* = try readValue(a, v.data.array.data[i][0]);
            break :blk .{ .array = items };
        },
        6 => .{ .object = try readObject(a, v.data.object) },
        else => error.InvalidNativeDiscriminator,
    };
}
pub const ImmediateFacts = struct {
    attempts: Presence([]const Attempt),
    cache_answers: u64,
    call_id: Presence(CallId),
    estimated_cost_usd: Presence([]const u8),
    held_model_mismatch: Presence(bool),
    input_tokens: Presence(u64),
    largest_request_bytes: u64,
    largest_request_estimated_input_tokens: Presence(u64),
    model: Presence([]const u8),
    output_tokens: Presence(u64),
    records: u64,
    requests_sent: u64,
    seconds: f64,
    token_estimate_method: []const u8,
    usage_persistence: Presence(PersistenceObservation),
    extensions: Map(Value),
};

pub const PersistenceObservation = struct {
    advice: Presence([]const u8),
    observed_at: []const u8,
    state: UsagePersistence,
    extensions: Map(Value),
};

pub const UsagePersistence = enum { disabled, pending, written, failed };

pub const CallId = []const u8;

pub const Attempt = struct {
    ordinal: u64,
    outcome: AttemptOutcome,
    request_id: Presence([]const u8),
    request_sha256: []const u8,
    sdk_request_id: SdkRequestId,
    server_ms: Presence(u64),
    status: Presence(u64),
    wall_ms: u64,
    extensions: Map(Value),
};

pub const SdkRequestId = []const u8;

pub const AttemptOutcome = enum { ok, status, transport };

pub const SessionPacket = union(enum) {
    decide_row: SessionPacketDecideRow,
    choose_row: SessionPacketChooseRow,
    tag_row: SessionPacketTagRow,
    score_row: SessionPacketScoreRow,
    filter_row: SessionPacketFilterRow,
    annotate_row: SessionPacketAnnotateRow,
    decide_aggregate: SessionPacketDecideAggregate,
    choose_aggregate: SessionPacketChooseAggregate,
    tag_aggregate: SessionPacketTagAggregate,
    score_aggregate: SessionPacketScoreAggregate,
    filter_aggregate: SessionPacketFilterAggregate,
    rank_aggregate: SessionPacketRankAggregate,
    find_aggregate: SessionPacketFindAggregate,
    annotate_aggregate: SessionPacketAnnotateAggregate,
    recognize_aggregate: SessionPacketRecognizeAggregate,
    relate_aggregate: SessionPacketRelateAggregate,
    observation: SessionPacketObservation,
    terminal: SessionPacketTerminal,
};

pub const SessionPacketTerminal = struct {
    answer: Presence(SettledAnswer),
    facts: Presence(Facts),
    failure: Presence(CallError),
    kind: []const u8,
    resolution: AnswerResolution,
    extensions: Map(Value),
};

pub const AnswerResolution = enum { resolved, unresolved, failed };

pub const CallError = struct {
    @"error": Error,
    facts: Presence(Facts),
    resolution: AnswerResolution,
    extensions: Map(Value),
};

pub const Facts = struct {
    attempts: Presence([]const Attempt),
    cache_answers: u64,
    call_id: CallId,
    estimated_cost_usd: Presence([]const u8),
    held_model_mismatch: Presence(bool),
    input_tokens: Presence(u64),
    largest_request_bytes: u64,
    largest_request_estimated_input_tokens: Presence(u64),
    model: Presence([]const u8),
    output_tokens: Presence(u64),
    records: u64,
    requests_sent: u64,
    seconds: f64,
    token_estimate_method: []const u8,
    usage_persistence: Presence(PersistenceObservation),
    extensions: Map(Value),
};

pub const Error = struct {
    estimated_input_denial: Presence(EstimatedInputDenial),
    kind: FailureKind,
    message: []const u8,
    retryable: bool,
    send_budget_denial: Presence(SendBudgetDenial),
    stopped: Stopped,
    extensions: Map(Value),
};

pub const Stopped = struct {
    at: Presence(u64),
    cause: StopCause,
    retryable: bool,
    status: Presence(u64),
    extensions: Map(Value),
};

pub const StopCause = enum { usage, local, no_key, transport, status, too_large, reply, backend, cancelled, deadline, defect };

pub const SendBudgetDenial = union(enum) {
    before_first_send: SendBudgetDenialBeforeFirstSend,
    before_additional_send: SendBudgetDenialBeforeAdditionalSend,
    before_retry: SendBudgetDenialBeforeRetry,
};

pub const SendBudgetDenialBeforeRetry = struct {
    kind: []const u8,
    last_status: u64,
    extensions: Map(Value),
};

pub const SendBudgetDenialBeforeAdditionalSend = struct {
    kind: []const u8,
    extensions: Map(Value),
};

pub const SendBudgetDenialBeforeFirstSend = struct {
    kind: []const u8,
    extensions: Map(Value),
};

pub const FailureKind = enum { usage, backend, local, cancelled, deadline, defect };

pub const EstimatedInputDenial = union(enum) {
    initial_request: EstimatedInputDenialInitialRequest,
    additional_request: EstimatedInputDenialAdditionalRequest,
    retry: EstimatedInputDenialRetry,
};

pub const EstimatedInputDenialRetry = struct {
    kind: []const u8,
    last_status: u64,
    limit: u64,
    extensions: Map(Value),
};

pub const EstimatedInputDenialAdditionalRequest = struct {
    kind: []const u8,
    limit: u64,
    extensions: Map(Value),
};

pub const EstimatedInputDenialInitialRequest = struct {
    kind: []const u8,
    limit: u64,
    extensions: Map(Value),
};

pub const SettledAnswer = union(enum) {
    decide: SettledAnswerDecide,
    choose: SettledAnswerChoose,
    tag: SettledAnswerTag,
    score: SettledAnswerScore,
    filter: SettledAnswerFilter,
    rank: SettledAnswerRank,
    find: SettledAnswerFind,
    recognize: SettledAnswerRecognize,
    relate: SettledAnswerRelate,
    annotate: SettledAnswerAnnotate,
};

pub const SettledAnswerAnnotate = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape2),
    resolution: AnswerShape5,
    value: Presence(AnswerShape15),
    extensions: Map(Value),
};

pub const AnswerShape15 = union(enum) {
    object: Map(SettledJudgment),
    null_value,
    array: []const AnswerShape15AnyOf1Items,
};

pub const AnswerShape15AnyOf1Items = union(enum) {
    object: Map(SettledJudgment),
    null_value,
};

pub const SettledJudgment = union(enum) {
    boolean: bool,
    null_value,
    string: []const u8,
    number: f64,
    array: []const []const u8,
};

pub const AnswerShape5 = union(enum) {
    string: AnswerResolution,
    object: Map(AnswerResolution),
    array: []const AnswerResolutions,
};

pub const AnswerResolutions = union(enum) {
    string: AnswerResolution,
    object: Map(AnswerResolution),
};

pub const AnswerShape2 = union(enum) {
    number: f64,
    null_value,
    array: []const AnswerShape8AnyOf1Items,
};

pub const AnswerShape8AnyOf1Items = union(enum) {
    number: f64,
    null_value,
};

pub const AnswerShape3 = union(enum) {
    string: []const u8,
    null_value,
    array: []const AnswerShape6AnyOf1Items,
};

pub const AnswerShape6AnyOf1Items = union(enum) {
    string: []const u8,
    null_value,
};

pub const AnswerShape4 = union(enum) {
    integer: u64,
    null_value,
    array: []const AnswerShape4AnyOf1Items,
};

pub const AnswerShape4AnyOf1Items = union(enum) {
    integer: u64,
    null_value,
};

pub const SettledAnswerRelate = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape10),
    resolution: AnswerShape5,
    value: Presence(AnswerShape14),
    extensions: Map(Value),
};

pub const AnswerShape14 = union(enum) {
    array: SettledRelations,
    null_value,
    array2: []const AnswerShape14AnyOf1Items,
};

pub const AnswerShape14AnyOf1Items = union(enum) {
    array: SettledRelations,
    null_value,
};

pub const SettledRelations = []const RelatedEntityEdge;

pub const RelatedEntityEdge = struct {
    either: Presence(bool),
    probability: f64,
    relation: []const u8,
    source: RelationEndpoint,
    target: RelationEndpoint,
    extensions: Map(Value),
};

pub const RelationEndpoint = union(enum) {
    related_entity: RelationEndpointRelatedEntity,
    source: RelationEndpointSource,
};

pub const RelationEndpointSource = struct {
    file: Presence([]const u8),
    first_line: Presence(u64),
    kind: []const u8,
    last_line: Presence(u64),
    name: []const u8,
    ordinal: u64,
    record: Presence(Value),
    extensions: Map(Value),
};

pub const RelationEndpointRelatedEntity = struct {
    kind: []const u8,
    name: []const u8,
    extensions: Map(Value),
};

pub const AnswerShape10 = union(enum) {
    array: []const AnswerShape8AnyOf1Items,
    null_value,
    array2: []const AnswerShape10AnyOf1Items,
};

pub const AnswerShape10AnyOf1Items = union(enum) {
    array: []const AnswerShape8AnyOf1Items,
    null_value,
};

pub const SettledAnswerRecognize = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape10),
    resolution: AnswerShape5,
    value: Presence(AnswerShape13),
    extensions: Map(Value),
};

pub const AnswerShape13 = union(enum) {
    object: SettledRecognition,
    null_value,
    array: []const AnswerShape13AnyOf1Items,
};

pub const AnswerShape13AnyOf1Items = union(enum) {
    object: SettledRecognition,
    null_value,
};

pub const SettledRecognition = union(enum) {
    entities: SettledRecognitionEntities,
    mode_proposals: SettledRecognitionModeProposals,
};

pub const SettledRecognitionModeProposals = struct {
    mode: BoundaryMode,
    proposals: []const BoundaryProposal,
    extensions: Map(Value),
};

pub const BoundaryProposal = struct {
    end: u64,
    file: Presence([]const u8),
    first_line: Presence(u64),
    last_line: Presence(u64),
    length: u64,
    probability: f64,
    start: u64,
    text: []const u8,
    extensions: Map(Value),
};

pub const BoundaryMode = enum { boundary_only };

pub const SettledRecognitionEntities = struct {
    entities: []const Entity,
    relations: Presence([]const EntityEdge),
    extensions: Map(Value),
};

pub const EntityEdge = struct {
    either: Presence(bool),
    probability: f64,
    relation: []const u8,
    source: Entity,
    target: Entity,
    extensions: Map(Value),
};

pub const Entity = struct {
    end: u64,
    file: Presence([]const u8),
    first_line: Presence(u64),
    kind: []const u8,
    last_line: Presence(u64),
    length: u64,
    start: u64,
    strength: f64,
    text: []const u8,
    extensions: Map(Value),
};

pub const SettledAnswerFind = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape2),
    resolution: AnswerShape5,
    value: Presence(AnswerShape12),
    extensions: Map(Value),
};

pub const AnswerShape12 = Value;

pub const SettledAnswerRank = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape10),
    resolution: AnswerShape5,
    value: Presence(AnswerShape11),
    extensions: Map(Value),
};

pub const AnswerShape11 = union(enum) {
    array: []const u64,
    null_value,
    array2: []const AnswerShape11AnyOf1Items,
};

pub const AnswerShape11AnyOf1Items = union(enum) {
    array: []const u64,
    null_value,
};

pub const SettledAnswerFilter = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape10),
    resolution: AnswerShape5,
    value: Presence(AnswerShape9),
    extensions: Map(Value),
};

pub const AnswerShape9 = union(enum) {
    array: []const SettledOriginal,
    null_value,
    array2: []const AnswerShape9AnyOf1Items,
};

pub const AnswerShape9AnyOf1Items = union(enum) {
    array: []const SettledOriginal,
    null_value,
};

pub const SettledOriginal = Value;

pub const SettledAnswerScore = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape2),
    resolution: AnswerShape5,
    value: Presence(AnswerShape8),
    extensions: Map(Value),
};

pub const AnswerShape8 = union(enum) {
    number: f64,
    null_value,
    array: []const AnswerShape8AnyOf1Items,
};

pub const SettledAnswerTag = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape2),
    resolution: AnswerShape5,
    value: Presence(AnswerShape7),
    extensions: Map(Value),
};

pub const AnswerShape7 = union(enum) {
    array: []const []const u8,
    null_value,
    array2: []const AnswerShape7AnyOf1Items,
};

pub const AnswerShape7AnyOf1Items = union(enum) {
    array: []const []const u8,
    null_value,
};

pub const SettledAnswerChoose = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape2),
    resolution: AnswerShape5,
    value: Presence(AnswerShape6),
    extensions: Map(Value),
};

pub const AnswerShape6 = union(enum) {
    string: []const u8,
    null_value,
    array: []const AnswerShape6AnyOf1Items,
};

pub const SettledAnswerDecide = struct {
    function: []const u8,
    index: Presence(AnswerShape4),
    pick: Presence(AnswerShape3),
    probability: Presence(AnswerShape2),
    resolution: AnswerShape5,
    value: Presence(AnswerShape),
    extensions: Map(Value),
};

pub const AnswerShape = Value;

pub const SessionPacketObservation = struct {
    function: RequestFunction,
    kind: []const u8,
    value: SessionObservation,
    extensions: Map(Value),
};

pub const SessionObservation = union(enum) {
    question: SessionObservationQuestion,
    row: SessionObservationRow,
};

pub const SessionObservationRow = struct {
    index: u64,
    kind: []const u8,
    value: SessionObservedRow,
    extensions: Map(Value),
};

pub const SessionObservedRow = union(enum) {
    judgment: SessionObservedRowJudgment,
    annotated: SessionObservedRowAnnotated,
    recognized: SessionObservedRowRecognized,
    find: SessionObservedRowFind,
    relations: SessionObservedRowRelations,
};

pub const SessionObservedRowRelations = struct {
    kind: []const u8,
    value: []const SessionRelationEdge,
    extensions: Map(Value),
};

pub const SessionRelationEdge = struct {
    either: bool,
    probability: f64,
    relation: []const u8,
    source: EntityDocument,
    target: EntityDocument,
    extensions: Map(Value),
};

pub const EntityDocument = struct {
    kind: []const u8,
    name: []const u8,
    extensions: Map(Value),
};

pub const SessionObservedRowFind = struct {
    kind: []const u8,
    value: Presence(u64),
    extensions: Map(Value),
};

pub const SessionObservedRowRecognized = struct {
    kind: []const u8,
    value: SessionRecognition,
    extensions: Map(Value),
};

pub const SessionRecognition = struct {
    entities: []const Entity,
    mode: RecognitionMode,
    proposals: Presence([]const BoundaryProposal),
    relations: Presence([]const RecognitionEdgeDocument),
    extensions: Map(Value),
};

pub const RecognitionEdgeDocument = struct {
    either: bool,
    probability: f64,
    relation: []const u8,
    source: Entity,
    target: Entity,
    extensions: Map(Value),
};

pub const RecognitionMode = enum { whole, boundary_only };

pub const SessionObservedRowAnnotated = struct {
    kind: []const u8,
    value: []const SessionAnnotation,
    extensions: Map(Value),
};

pub const SessionAnnotation = struct {
    name: []const u8,
    value: AnnotationValue,
    extensions: Map(Value),
};

pub const AnnotationValue = union(enum) {
    decision: AnnotationValueDecision,
    choice: AnnotationValueChoice,
    score: AnnotationValueScore,
    tags: AnnotationValueTags,
    failed: AnnotationValueFailed,
};

pub const AnnotationValueFailed = struct {
    kind: []const u8,
    value: Failure,
    extensions: Map(Value),
};

pub const Failure = struct {
    cause: FailureCause,
    kind: FailureFieldKind,
    extensions: Map(Value),
};

pub const FailureCause = enum { missing_answer, wrong_kind, missing_probability, invalid_probability, invalid_distribution, unexpected_probability };

pub const AnnotationValueTags = struct {
    kind: []const u8,
    value: []const []const u8,
    extensions: Map(Value),
};

pub const AnnotationValueScore = struct {
    kind: []const u8,
    value: f64,
    extensions: Map(Value),
};

pub const AnnotationValueChoice = struct {
    kind: []const u8,
    value: Presence([]const u8),
    extensions: Map(Value),
};

pub const AnnotationValueDecision = struct {
    kind: []const u8,
    value: Presence(bool),
    extensions: Map(Value),
};

pub const SessionObservedRowJudgment = struct {
    kind: []const u8,
    value: SessionJudgment,
    extensions: Map(Value),
};

pub const SessionJudgment = union(enum) {
    decision: SessionJudgmentDecision,
    choice: SessionJudgmentChoice,
    score: SessionJudgmentScore,
    tags: SessionJudgmentTags,
};

pub const SessionJudgmentTags = struct {
    kind: []const u8,
    value: []const []const u8,
    extensions: Map(Value),
};

pub const SessionJudgmentScore = struct {
    kind: []const u8,
    value: f64,
    extensions: Map(Value),
};

pub const SessionJudgmentChoice = struct {
    kind: []const u8,
    value: Presence([]const u8),
    extensions: Map(Value),
};

pub const SessionJudgmentDecision = struct {
    kind: []const u8,
    value: Presence(bool),
    extensions: Map(Value),
};

pub const SessionObservationQuestion = struct {
    detail: SessionQuestionDetail,
    index: u64,
    kind: []const u8,
    member: Presence([]const u8),
    position: u64,
    stage: Presence([]const u8),
    extensions: Map(Value),
};

pub const SessionQuestionDetail = struct {
    answer_id: Presence(AnswerId),
    cached: bool,
    confidence: Presence(f64),
    failed_questions: u64,
    failure: Presence(Failure),
    failure_id: Presence(FailureId),
    input: Presence(Value),
    input_source: Presence(PhysicalSource),
    input_sources: []const SessionInputSource,
    inputs: []const Value,
    model: []const u8,
    observations: []const Observation,
    probabilities: Presence(SessionProbabilities),
    question: QuestionDocument,
    question_sha256: []const u8,
    question_sources: []const QuestionSource,
    raw_pick: Presence([]const u8),
    reported_usage: Presence(Usage),
    requests: []const []const u8,
    requests_sent: u64,
    threshold: Presence(Threshold),
    url: []const u8,
    usage: Presence(TokenUsage),
    value: Presence(JudgmentValue),
    extensions: Map(Value),
};

pub const JudgmentValue = union(enum) {
    boolean: bool,
    null_value,
    string: []const u8,
    array: []const []const u8,
    number: f64,
};

pub const TokenUsage = struct {
    input_tokens: u64,
    output_tokens: u64,
    extensions: Map(Value),
};

pub const Threshold = union(enum) {
    number: f64,
    string: []const u8,
};

pub const Usage = struct {
    input_tokens: Presence(u64),
    output_tokens: Presence(u64),
    extensions: Map(Value),
};

pub const QuestionSource = struct {
    answered_by: []const u8,
    batch_size: Presence(u64),
    origin: Origin,
    extensions: Map(Value),
};

pub const Origin = enum { live, cache, replay, proxy, memory };

pub const QuestionDocument = union(enum) {
    decide: QuestionDocumentDecide,
    choose: QuestionDocumentChoose,
    tag: QuestionDocumentTag,
    score: QuestionDocumentScore,
};

pub const QuestionDocumentScore = struct {
    batch: Presence(Batch),
    context_schema: Presence(InputDeclaration),
    item_schema: Presence(InputDeclaration),
    label_details: Presence([]const Label),
    model: Presence([]const u8),
    name: Presence(QuestionName),
    on: Presence([]const []const u8),
    profile: Presence([]const u8),
    wording_version: Presence(WordingVersion),
    levels: []const []const u8,
    text: Presence(Value),
    verb: []const u8,
    extensions: Map(Value),
};

pub const WordingVersion = u64;

pub const QuestionName = []const u8;

pub const Label = struct {
    description: Presence(Value),
    name: []const u8,
    extensions: Map(Value),
};

pub const InputDeclaration = union(enum) {
    string_root: InputDeclarationStringRoot,
    object_root: InputDeclarationObjectRoot,
};

pub const InputDeclarationObjectRoot = struct {
    properties: Map(InputPropertyType),
    required: Presence([]const []const u8),
    type: ObjectType,
    extensions: Map(Value),
};

pub const ObjectType = enum { object };

pub const InputPropertyType = union(enum) {
    string: InputPropertyTypeString,
    number: InputPropertyTypeNumber,
    boolean: InputPropertyTypeBoolean,
    array: InputPropertyTypeArray,
};

pub const InputPropertyTypeArray = struct {
    items: StringRoot,
    type: []const u8,
    extensions: Map(Value),
};

pub const StringRoot = struct {
    type: StringType,
    extensions: Map(Value),
};

pub const StringType = enum { string };

pub const InputPropertyTypeBoolean = struct {
    type: []const u8,
    extensions: Map(Value),
};

pub const InputPropertyTypeNumber = struct {
    type: []const u8,
    extensions: Map(Value),
};

pub const InputPropertyTypeString = struct {
    type: []const u8,
    extensions: Map(Value),
};

pub const InputDeclarationStringRoot = struct {
    type: StringType,
    extensions: Map(Value),
};

pub const Batch = union(enum) {
    integer: u64,
    string: BatchString,
};

pub const QuestionDocumentTag = struct {
    batch: Presence(Batch),
    context_schema: Presence(InputDeclaration),
    item_schema: Presence(InputDeclaration),
    label_details: Presence([]const Label),
    model: Presence([]const u8),
    name: Presence(QuestionName),
    on: Presence([]const []const u8),
    profile: Presence([]const u8),
    wording_version: Presence(WordingVersion),
    labels: []const []const u8,
    text: Presence(Value),
    verb: []const u8,
    extensions: Map(Value),
};

pub const QuestionDocumentChoose = struct {
    batch: Presence(Batch),
    context_schema: Presence(InputDeclaration),
    item_schema: Presence(InputDeclaration),
    label_details: Presence([]const Label),
    model: Presence([]const u8),
    name: Presence(QuestionName),
    on: Presence([]const []const u8),
    profile: Presence([]const u8),
    wording_version: Presence(WordingVersion),
    options: []const []const u8,
    text: Presence(Value),
    verb: []const u8,
    extensions: Map(Value),
};

pub const QuestionDocumentDecide = struct {
    batch: Presence(Batch),
    context_schema: Presence(InputDeclaration),
    item_schema: Presence(InputDeclaration),
    label_details: Presence([]const Label),
    model: Presence([]const u8),
    name: Presence(QuestionName),
    on: Presence([]const []const u8),
    profile: Presence([]const u8),
    wording_version: Presence(WordingVersion),
    false: Presence(Value),
    text: Presence(Value),
    true: Presence(Value),
    verb: []const u8,
    extensions: Map(Value),
};

pub const SessionProbabilities = union(enum) {
    yes_no: SessionProbabilitiesYesNo,
    named: SessionProbabilitiesNamed,
};

pub const SessionProbabilitiesNamed = struct {
    kind: []const u8,
    value: []const SessionNamedProbability,
    extensions: Map(Value),
};

pub const SessionNamedProbability = struct {
    name: []const u8,
    probability: f64,
    extensions: Map(Value),
};

pub const SessionProbabilitiesYesNo = struct {
    kind: []const u8,
    value: f64,
    extensions: Map(Value),
};

pub const Observation = union(enum) {
    observation_id: ObservationObservationId,
    failure_id: ObservationFailureId,
};

pub const ObservationFailureId = struct {
    failure_id: FailureId,
    extensions: Map(Value),
};

pub const FailureId = []const u8;

pub const ObservationObservationId = struct {
    observation_id: ObservationId,
    extensions: Map(Value),
};

pub const ObservationId = []const u8;

pub const SessionInputSource = struct {
    index: u64,
    source: PhysicalSource,
    extensions: Map(Value),
};

pub const PhysicalSource = struct {
    file: []const u8,
    first_line: Presence(u64),
    last_line: Presence(u64),
    extensions: Map(Value),
};

pub const AnswerId = []const u8;

pub const RequestFunction = enum { decide, choose, tag, score, filter, rank, find, annotate, recognize, relate };

pub const SessionPacketRelateAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: Relation,
    extensions: Map(Value),
};

pub const Relation = struct {
    answer: Answers,
    answer_id: AnswerId,
    file: Presence([]const u8),
    first_line: Presence(u64),
    index: Presence(u64),
    input: Presence(Value),
    input_sources: Presence([]const SessionInputSource),
    last_line: Presence(u64),
    meta: Meta,
    position: Presence(Position),
    question: RelateQuestionDocument,
    schema: Version,
    value: []const RelatedEntityEdge,
    extensions: Map(Value),
};

pub const Version = enum { @"thinkthen.result/2" };

pub const RelateQuestionDocument = struct {
    batch: Presence(Batch),
    context_schema: Presence(InputDeclaration),
    fields: Presence(RelateFields),
    item_schema: Presence(InputDeclaration),
    label_details: Presence([]const Label),
    model: Presence([]const u8),
    name: Presence(QuestionName),
    on: Presence([]const []const u8),
    profile: Presence([]const u8),
    relations: []const RelationRule,
    threshold: Threshold,
    verb: []const u8,
    wording_version: Presence(WordingVersion),
    extensions: Map(Value),
};

pub const RelationRule = struct {
    either: bool,
    name: []const u8,
    reads: []const u8,
    single: Presence(bool),
    source: []const u8,
    target: []const u8,
    extensions: Map(Value),
};

pub const RelateFields = struct {
    kind: []const u8,
    name: []const u8,
    extensions: Map(Value),
};

pub const Position = struct {
    file: Presence([]const u8),
    first: Presence(u64),
    images: Presence([]const []const u8),
    last: Presence(u64),
    extensions: Map(Value),
};

pub const Meta = struct {
    answered_by: Presence([]const u8),
    attempts: Presence([]const Attempt),
    batch_setting: Presence(BatchSetting),
    batch_warning: Presence(BatchWarning),
    cached: bool,
    context_sha256: Presence([]const u8),
    failed_questions: u64,
    model: []const u8,
    observations: []const Observation,
    origin: Presence(Origin),
    profile_warning: Presence(ProfileWarning),
    question_sha256: Presence([]const u8),
    question_sources: []const QuestionSource,
    questions_sha256: Presence([]const u8),
    requests: []const []const u8,
    requests_sent: u64,
    tool: []const u8,
    url: []const u8,
    usage: Presence(Usage),
    extensions: Map(Value),
};

pub const ProfileWarning = struct {
    running: []const u8,
    tuned_for: []const u8,
    extensions: Map(Value),
};

pub const BatchWarning = struct {
    running: BatchSetting,
    tuned_for: BatchSetting,
    extensions: Map(Value),
};

pub const BatchSetting = union(enum) {
    integer: u64,
    string: []const u8,
};

pub const Answers = struct {
    questions: []const RelationMember,
    extensions: Map(Value),
};

pub const RelationMember = union(enum) {
    answer_id: RelationMemberAnswerId,
    failure_id: RelationMemberFailureId,
};

pub const RelationMemberFailureId = struct {
    direction: RelationDirection,
    method: RelationMethod,
    observations: []const Observation,
    question: QuestionDocument,
    question_sources: []const QuestionSource,
    reads: []const u8,
    relation: []const u8,
    request: []const u8,
    source: RelatedEntity,
    target: Presence(RelatedEntity),
    threshold: Threshold,
    usage: Presence(Usage),
    failure: Failure,
    failure_id: FailureId,
    extensions: Map(Value),
};

pub const RelatedEntity = struct {
    kind: []const u8,
    name: []const u8,
    extensions: Map(Value),
};

pub const RelationMethod = enum { yes_no, choice };

pub const RelationDirection = enum { source_to_target, either };

pub const RelationMemberAnswerId = struct {
    direction: RelationDirection,
    method: RelationMethod,
    observations: []const Observation,
    question: QuestionDocument,
    question_sources: []const QuestionSource,
    reads: []const u8,
    relation: []const u8,
    request: []const u8,
    source: RelatedEntity,
    target: Presence(RelatedEntity),
    threshold: Threshold,
    usage: Presence(Usage),
    accepted: bool,
    answer: Answer,
    answer_id: AnswerId,
    probability: f64,
    extensions: Map(Value),
};

pub const Answer = union(enum) {
    yes_no: AnswerYesNo,
    choice: AnswerChoice,
    tag: AnswerTag,
    score: AnswerScore,
};

pub const AnswerScore = struct {
    confidence: Presence(f64),
    kind: []const u8,
    level: []const u8,
    probabilities: Map(f64),
    extensions: Map(Value),
};

pub const AnswerTag = struct {
    kind: []const u8,
    probabilities: Map(f64),
    extensions: Map(Value),
};

pub const AnswerChoice = struct {
    confidence: Presence(f64),
    kind: []const u8,
    pick: []const u8,
    probabilities: Map(f64),
    extensions: Map(Value),
};

pub const AnswerYesNo = struct {
    kind: []const u8,
    probability: f64,
    extensions: Map(Value),
};

pub const SessionPacketRecognizeAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const Recognition,
    extensions: Map(Value),
};

pub const Recognition = struct {
    answer: RecognitionOdds,
    answer_id: AnswerId,
    file: Presence([]const u8),
    first_line: Presence(u64),
    index: Presence(u64),
    input: Presence(Value),
    last_line: Presence(u64),
    meta: Meta,
    position: Presence(Position),
    question: RecognizeQuestionDocument,
    schema: Version,
    source: Presence(PhysicalSource),
    value: Recognize,
    extensions: Map(Value),
};

pub const Recognize = union(enum) {
    entities: RecognizeEntities,
    mode_proposals: RecognizeModeProposals,
};

pub const RecognizeModeProposals = struct {
    mode: BoundaryMode,
    proposals: []const BoundaryProposal,
    extensions: Map(Value),
};

pub const RecognizeEntities = struct {
    entities: []const Entity,
    relations: Presence([]const EntityEdge),
    extensions: Map(Value),
};

pub const RecognizeQuestionDocument = struct {
    batch: Presence(Batch),
    context_schema: Presence(InputDeclaration),
    entity_definition: Presence(Value),
    instructions: Presence(Value),
    item_schema: Presence(InputDeclaration),
    kinds: Map(Value),
    label_details: Presence([]const Label),
    mode: Presence(RecognitionMode),
    model: Presence([]const u8),
    name: Presence(QuestionName),
    on: Presence([]const []const u8),
    profile: Presence([]const u8),
    relation_threshold: Presence(Threshold),
    relations: Presence([]const RelationRule),
    snippet_pieces: Presence(u64),
    stage_context: Presence(RecognitionStageContext),
    threshold: Threshold,
    verb: Verb,
    wording_version: Presence(WordingVersion),
    extensions: Map(Value),
};

pub const Verb = enum { recognize };

pub const RecognitionStageContext = struct {
    boundary: Presence([]const u8),
    kind_edge: Presence([]const u8),
    relation: Presence([]const u8),
    extensions: Map(Value),
};

pub const RecognitionOdds = union(enum) {
    recognize_answer: RecognitionOddsRecognizeAnswer,
    boundary_odds: RecognitionOddsBoundaryOdds,
};

pub const RecognitionOddsBoundaryOdds = struct {
    pieces: []const PieceOdds,
    proposals: []const BoundaryProposal,
    extensions: Map(Value),
};

pub const PieceOdds = struct {
    end: u64,
    start: u64,
    tags: Map(f64),
    extensions: Map(Value),
};

pub const RecognitionOddsRecognizeAnswer = struct {
    names: []const NameOdds,
    pairs: []const PairOdds,
    pieces: []const PieceOdds,
    proposals: []const RecognitionProposal,
    extensions: Map(Value),
};

pub const RecognitionProposal = struct {
    end: u64,
    kept: bool,
    kind: Presence([]const u8),
    selected: Presence(Place),
    span_probability: f64,
    start: u64,
    strength: Presence(f64),
    extensions: Map(Value),
};

pub const Place = struct {
    end: u64,
    start: u64,
    extensions: Map(Value),
};

pub const PairOdds = struct {
    probability: f64,
    relation: []const u8,
    source: Place,
    target: Place,
    extensions: Map(Value),
};

pub const NameOdds = struct {
    edges: Presence(Map(f64)),
    end: u64,
    kinds: Presence(Map(f64)),
    start: u64,
    extensions: Map(Value),
};

pub const SessionPacketAnnotateAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const Annotation,
    extensions: Map(Value),
};

pub const Annotation = struct {
    answer_id: AnswerId,
    answers: Map(AnnotationMember),
    file: Presence([]const u8),
    first_line: Presence(u64),
    index: Presence(u64),
    input: Presence(Value),
    last_line: Presence(u64),
    meta: Meta,
    position: Presence(Position),
    schema: Version,
    source: Presence(PhysicalSource),
    value: AnnotatedRow,
    extensions: Map(Value),
};

pub const AnnotatedRow = Map(AnnotatedField);

pub const AnnotatedField = union(enum) {
    boolean: bool,
    null_value,
    string: []const u8,
    array: []const []const u8,
    number: f64,
    object: Failed,
};

pub const Failed = struct {
    failed: Failure,
    extensions: Map(Value),
};

pub const AnnotationMember = union(enum) {
    answer_id: AnnotationMemberAnswerId,
    failure_id: AnnotationMemberFailureId,
};

pub const AnnotationMemberFailureId = struct {
    failure: Failure,
    failure_id: FailureId,
    observations: []const Observation,
    question: QuestionDocument,
    question_sources: []const QuestionSource,
    request: []const u8,
    threshold: Presence(Threshold),
    usage: Presence(Usage),
    extensions: Map(Value),
};

pub const AnnotationMemberAnswerId = struct {
    answer: Answer,
    answer_id: AnswerId,
    observations: []const Observation,
    question: QuestionDocument,
    question_sources: []const QuestionSource,
    request: []const u8,
    threshold: Presence(Threshold),
    usage: Presence(Usage),
    value: Presence(JudgmentValue),
    extensions: Map(Value),
};

pub const SessionPacketFindAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: Find,
    extensions: Map(Value),
};

pub const Find = struct {
    answer: FindAnswer,
    answer_id: AnswerId,
    candidates: Presence([]const FindCandidate),
    file: Presence([]const u8),
    first_line: Presence(u64),
    index: Presence(u64),
    last_line: Presence(u64),
    meta: Meta,
    position: Presence(Position),
    question: FindQuestionDocument,
    schema: Version,
    threshold: Presence(Value),
    value: Presence(Value),
    extensions: Map(Value),
};

pub const FindQuestionDocument = struct {
    batch: Presence(Batch),
    context_schema: Presence(InputDeclaration),
    item_schema: Presence(InputDeclaration),
    label_details: Presence([]const Label),
    model: Presence([]const u8),
    name: Presence(QuestionName),
    none: bool,
    on: Presence([]const []const u8),
    profile: Presence([]const u8),
    text: Presence(Value),
    verb: []const u8,
    wording_version: Presence(WordingVersion),
    extensions: Map(Value),
};

pub const FindCandidate = struct {
    index: Presence(u64),
    input: Presence(Value),
    probability: f64,
    source: Presence(PhysicalSource),
    extensions: Map(Value),
};

pub const FindAnswer = struct {
    confidence: Presence(f64),
    kind: []const u8,
    pick: []const u8,
    probabilities: Map(f64),
    extensions: Map(Value),
};

pub const SessionPacketRankAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const AtomicNonZeroUsize,
    extensions: Map(Value),
};

pub const AtomicNonZeroUsize = struct {
    answer: Answer,
    answer_id: AnswerId,
    images: Presence([]const Image),
    index: Presence(u64),
    input: Presence(Value),
    members: Presence([]const RankMember),
    meta: Meta,
    question: QuestionDocument,
    question_name: Presence([]const u8),
    schema: Version,
    source: Presence(PhysicalSource),
    threshold: Presence(Threshold),
    value: u64,
    extensions: Map(Value),
};

pub const RankMember = struct {
    name: []const u8,
    result: RankMemberResult,
    extensions: Map(Value),
};

pub const RankMemberResult = struct {
    answer: Answer,
    answer_id: AnswerId,
    images: Presence([]const Image),
    meta: Meta,
    question: QuestionDocument,
    schema: Version,
    source: Presence(PhysicalSource),
    threshold: Presence(Value),
    value: u64,
    extensions: Map(Value),
};

pub const Image = struct {
    base64: []const u8,
    height: u64,
    media: ImageMedia,
    width: u64,
    extensions: Map(Value),
};

pub const ImageMedia = enum { @"image/jpeg", @"image/png" };

pub const SessionPacketFilterAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const AtomicBoolean,
    extensions: Map(Value),
};

pub const AtomicBoolean = struct {
    answer: Answer,
    answer_id: AnswerId,
    images: Presence([]const Image),
    index: Presence(u64),
    input: Presence(Value),
    members: Presence([]const RankMember),
    meta: Meta,
    question: QuestionDocument,
    question_name: Presence([]const u8),
    schema: Version,
    source: Presence(PhysicalSource),
    threshold: Presence(Threshold),
    value: bool,
    extensions: Map(Value),
};

pub const SessionPacketScoreAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const AtomicDouble,
    extensions: Map(Value),
};

pub const AtomicDouble = struct {
    answer: Answer,
    answer_id: AnswerId,
    images: Presence([]const Image),
    index: Presence(u64),
    input: Presence(Value),
    members: Presence([]const RankMember),
    meta: Meta,
    question: QuestionDocument,
    question_name: Presence([]const u8),
    schema: Version,
    source: Presence(PhysicalSource),
    threshold: Presence(Threshold),
    value: f64,
    extensions: Map(Value),
};

pub const SessionPacketTagAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const AtomicArrayOfString,
    extensions: Map(Value),
};

pub const AtomicArrayOfString = struct {
    answer: Answer,
    answer_id: AnswerId,
    images: Presence([]const Image),
    index: Presence(u64),
    input: Presence(Value),
    members: Presence([]const RankMember),
    meta: Meta,
    question: QuestionDocument,
    question_name: Presence([]const u8),
    schema: Version,
    source: Presence(PhysicalSource),
    threshold: Presence(Threshold),
    value: []const []const u8,
    extensions: Map(Value),
};

pub const SessionPacketChooseAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const AtomicNullableString,
    extensions: Map(Value),
};

pub const AtomicNullableString = struct {
    answer: Answer,
    answer_id: AnswerId,
    images: Presence([]const Image),
    index: Presence(u64),
    input: Presence(Value),
    members: Presence([]const RankMember),
    meta: Meta,
    question: QuestionDocument,
    question_name: Presence([]const u8),
    schema: Version,
    source: Presence(PhysicalSource),
    threshold: Presence(Threshold),
    value: Presence([]const u8),
    extensions: Map(Value),
};

pub const SessionPacketDecideAggregate = struct {
    function: []const u8,
    kind: []const u8,
    value: []const AtomicDecideValue,
    extensions: Map(Value),
};

pub const AtomicDecideValue = struct {
    answer: Answer,
    answer_id: AnswerId,
    images: Presence([]const Image),
    index: Presence(u64),
    input: Presence(Value),
    members: Presence([]const RankMember),
    meta: Meta,
    question: QuestionDocument,
    question_name: Presence([]const u8),
    schema: Version,
    source: Presence(PhysicalSource),
    threshold: Presence(Threshold),
    value: Presence(DecideValue),
    extensions: Map(Value),
};

pub const DecideValue = Value;

pub const SessionPacketAnnotateRow = struct {
    function: []const u8,
    kind: []const u8,
    value: Annotation,
    extensions: Map(Value),
};

pub const SessionPacketFilterRow = struct {
    function: []const u8,
    kind: []const u8,
    value: AtomicBoolean,
    extensions: Map(Value),
};

pub const SessionPacketScoreRow = struct {
    function: []const u8,
    kind: []const u8,
    value: AtomicDouble,
    extensions: Map(Value),
};

pub const SessionPacketTagRow = struct {
    function: []const u8,
    kind: []const u8,
    value: AtomicArrayOfString,
    extensions: Map(Value),
};

pub const SessionPacketChooseRow = struct {
    function: []const u8,
    kind: []const u8,
    value: AtomicNullableString,
    extensions: Map(Value),
};

pub const SessionPacketDecideRow = struct {
    function: []const u8,
    kind: []const u8,
    value: AtomicDecideValue,
    extensions: Map(Value),
};

pub const FailureFieldKind = enum { backend };

pub const BatchString = enum { max };

pub fn readImmediateFacts(a: std.mem.Allocator, v: c.thinkthen_complete_immediate_facts_v1) anyerror!ImmediateFacts {
    return .{
        .attempts = switch (v.attempts.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Attempt, v.attempts.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.attempts.value.data[i];
                    out.* = try readAttempt(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .cache_answers = v.cache_answers,
        .call_id = switch (v.call_id.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readCallId(a, v.call_id.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .estimated_cost_usd = switch (v.estimated_cost_usd.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.estimated_cost_usd.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .held_model_mismatch = switch (v.held_model_mismatch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.held_model_mismatch.value != 0 },
            else => return error.InvalidNativeDiscriminator,
        },
        .input_tokens = switch (v.input_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.input_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .largest_request_bytes = v.largest_request_bytes,
        .largest_request_estimated_input_tokens = switch (v.largest_request_estimated_input_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.largest_request_estimated_input_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .output_tokens = switch (v.output_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.output_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .records = v.records,
        .requests_sent = v.requests_sent,
        .seconds = v.seconds,
        .token_estimate_method = try a.dupe(u8, text(v.token_estimate_method)),
        .usage_persistence = switch (v.usage_persistence.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPersistenceObservation(a, v.usage_persistence.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readPersistenceObservation(a: std.mem.Allocator, v: c.thinkthen_complete_persistence_observation_v1) anyerror!PersistenceObservation {
    return .{
        .advice = switch (v.advice.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.advice.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .observed_at = try a.dupe(u8, text(v.observed_at)),
        .state = try readUsagePersistence(a, v.state[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readUsagePersistence(a: std.mem.Allocator, v: c.thinkthen_complete_usage_persistence_v1) anyerror!UsagePersistence {
    _ = a;
    return switch (v.kind) {
        1 => .disabled,
        2 => .pending,
        3 => .written,
        4 => .failed,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readCallId(a: std.mem.Allocator, v: c.thinkthen_complete_call_id_v1) anyerror!CallId {
    return try a.dupe(u8, text(v.value));
}

pub fn readAttempt(a: std.mem.Allocator, v: c.thinkthen_complete_attempt_v1) anyerror!Attempt {
    return .{
        .ordinal = v.ordinal,
        .outcome = try readAttemptOutcome(a, v.outcome[0]),
        .request_id = switch (v.request_id.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.request_id.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .request_sha256 = try a.dupe(u8, text(v.request_sha256)),
        .sdk_request_id = try readSdkRequestId(a, v.sdk_request_id[0]),
        .server_ms = switch (v.server_ms.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.server_ms.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .status = switch (v.status.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.status.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .wall_ms = v.wall_ms,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSdkRequestId(a: std.mem.Allocator, v: c.thinkthen_complete_sdk_request_id_v1) anyerror!SdkRequestId {
    return try a.dupe(u8, text(v.value));
}

pub fn readAttemptOutcome(a: std.mem.Allocator, v: c.thinkthen_complete_attempt_outcome_v1) anyerror!AttemptOutcome {
    _ = a;
    return switch (v.kind) {
        1 => .ok,
        2 => .status,
        3 => .transport,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionPacket(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_v1) anyerror!SessionPacket {
    return switch (v.kind) {
        1 => .{ .decide_row = try readSessionPacketDecideRow(a, v.data.decide_row[0]) },
        2 => .{ .choose_row = try readSessionPacketChooseRow(a, v.data.choose_row[0]) },
        3 => .{ .tag_row = try readSessionPacketTagRow(a, v.data.tag_row[0]) },
        4 => .{ .score_row = try readSessionPacketScoreRow(a, v.data.score_row[0]) },
        5 => .{ .filter_row = try readSessionPacketFilterRow(a, v.data.filter_row[0]) },
        6 => .{ .annotate_row = try readSessionPacketAnnotateRow(a, v.data.annotate_row[0]) },
        7 => .{ .decide_aggregate = try readSessionPacketDecideAggregate(a, v.data.decide_aggregate[0]) },
        8 => .{ .choose_aggregate = try readSessionPacketChooseAggregate(a, v.data.choose_aggregate[0]) },
        9 => .{ .tag_aggregate = try readSessionPacketTagAggregate(a, v.data.tag_aggregate[0]) },
        10 => .{ .score_aggregate = try readSessionPacketScoreAggregate(a, v.data.score_aggregate[0]) },
        11 => .{ .filter_aggregate = try readSessionPacketFilterAggregate(a, v.data.filter_aggregate[0]) },
        12 => .{ .rank_aggregate = try readSessionPacketRankAggregate(a, v.data.rank_aggregate[0]) },
        13 => .{ .find_aggregate = try readSessionPacketFindAggregate(a, v.data.find_aggregate[0]) },
        14 => .{ .annotate_aggregate = try readSessionPacketAnnotateAggregate(a, v.data.annotate_aggregate[0]) },
        15 => .{ .recognize_aggregate = try readSessionPacketRecognizeAggregate(a, v.data.recognize_aggregate[0]) },
        16 => .{ .relate_aggregate = try readSessionPacketRelateAggregate(a, v.data.relate_aggregate[0]) },
        17 => .{ .observation = try readSessionPacketObservation(a, v.data.observation[0]) },
        18 => .{ .terminal = try readSessionPacketTerminal(a, v.data.terminal[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionPacketTerminal(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_terminal_v1) anyerror!SessionPacketTerminal {
    return .{
        .answer = switch (v.answer.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readSettledAnswer(a, v.answer.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .facts = switch (v.facts.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readFacts(a, v.facts.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .failure = switch (v.failure.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readCallError(a, v.failure.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind = try a.dupe(u8, text(v.kind)),
        .resolution = try readAnswerResolution(a, v.resolution[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerResolution(a: std.mem.Allocator, v: c.thinkthen_complete_answer_resolution_v1) anyerror!AnswerResolution {
    _ = a;
    return switch (v.kind) {
        1 => .resolved,
        2 => .unresolved,
        3 => .failed,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readCallError(a: std.mem.Allocator, v: c.thinkthen_complete_call_error_v1) anyerror!CallError {
    return .{
        .@"error" = try readError(a, v.@"error"[0]),
        .facts = switch (v.facts.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readFacts(a, v.facts.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerResolution(a, v.resolution[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFacts(a: std.mem.Allocator, v: c.thinkthen_complete_facts_v1) anyerror!Facts {
    return .{
        .attempts = switch (v.attempts.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Attempt, v.attempts.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.attempts.value.data[i];
                    out.* = try readAttempt(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .cache_answers = v.cache_answers,
        .call_id = try readCallId(a, v.call_id[0]),
        .estimated_cost_usd = switch (v.estimated_cost_usd.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.estimated_cost_usd.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .held_model_mismatch = switch (v.held_model_mismatch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.held_model_mismatch.value != 0 },
            else => return error.InvalidNativeDiscriminator,
        },
        .input_tokens = switch (v.input_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.input_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .largest_request_bytes = v.largest_request_bytes,
        .largest_request_estimated_input_tokens = switch (v.largest_request_estimated_input_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.largest_request_estimated_input_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .output_tokens = switch (v.output_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.output_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .records = v.records,
        .requests_sent = v.requests_sent,
        .seconds = v.seconds,
        .token_estimate_method = try a.dupe(u8, text(v.token_estimate_method)),
        .usage_persistence = switch (v.usage_persistence.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPersistenceObservation(a, v.usage_persistence.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readError(a: std.mem.Allocator, v: c.thinkthen_complete_error_v1) anyerror!Error {
    return .{
        .estimated_input_denial = switch (v.estimated_input_denial.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readEstimatedInputDenial(a, v.estimated_input_denial.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind = try readFailureKind(a, v.kind[0]),
        .message = try a.dupe(u8, text(v.message)),
        .retryable = v.retryable != 0,
        .send_budget_denial = switch (v.send_budget_denial.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readSendBudgetDenial(a, v.send_budget_denial.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .stopped = try readStopped(a, v.stopped[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readStopped(a: std.mem.Allocator, v: c.thinkthen_complete_stopped_v1) anyerror!Stopped {
    return .{
        .at = switch (v.at.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.at.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .cause = try readStopCause(a, v.cause[0]),
        .retryable = v.retryable != 0,
        .status = switch (v.status.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.status.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readStopCause(a: std.mem.Allocator, v: c.thinkthen_complete_stop_cause_v1) anyerror!StopCause {
    _ = a;
    return switch (v.kind) {
        1 => .usage,
        2 => .local,
        3 => .no_key,
        4 => .transport,
        5 => .status,
        6 => .too_large,
        7 => .reply,
        8 => .backend,
        9 => .cancelled,
        10 => .deadline,
        11 => .defect,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSendBudgetDenial(a: std.mem.Allocator, v: c.thinkthen_complete_send_denial_v1) anyerror!SendBudgetDenial {
    return switch (v.kind) {
        1 => .{ .before_first_send = try readSendBudgetDenialBeforeFirstSend(a, v.data.before_first_send[0]) },
        2 => .{ .before_additional_send = try readSendBudgetDenialBeforeAdditionalSend(a, v.data.before_additional_send[0]) },
        3 => .{ .before_retry = try readSendBudgetDenialBeforeRetry(a, v.data.before_retry[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSendBudgetDenialBeforeRetry(a: std.mem.Allocator, v: c.thinkthen_complete_send_denial_before_retry_v1) anyerror!SendBudgetDenialBeforeRetry {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .last_status = v.last_status,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSendBudgetDenialBeforeAdditionalSend(a: std.mem.Allocator, v: c.thinkthen_complete_send_denial_before_additional_send_v1) anyerror!SendBudgetDenialBeforeAdditionalSend {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSendBudgetDenialBeforeFirstSend(a: std.mem.Allocator, v: c.thinkthen_complete_send_denial_before_first_send_v1) anyerror!SendBudgetDenialBeforeFirstSend {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFailureKind(a: std.mem.Allocator, v: c.thinkthen_complete_failure_kind_v1) anyerror!FailureKind {
    _ = a;
    return switch (v.kind) {
        1 => .usage,
        2 => .backend,
        3 => .local,
        4 => .cancelled,
        5 => .deadline,
        6 => .defect,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readEstimatedInputDenial(a: std.mem.Allocator, v: c.thinkthen_complete_input_denial_v1) anyerror!EstimatedInputDenial {
    return switch (v.kind) {
        1 => .{ .initial_request = try readEstimatedInputDenialInitialRequest(a, v.data.initial_request[0]) },
        2 => .{ .additional_request = try readEstimatedInputDenialAdditionalRequest(a, v.data.additional_request[0]) },
        3 => .{ .retry = try readEstimatedInputDenialRetry(a, v.data.retry[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readEstimatedInputDenialRetry(a: std.mem.Allocator, v: c.thinkthen_complete_input_denial_retry_v1) anyerror!EstimatedInputDenialRetry {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .last_status = v.last_status,
        .limit = v.limit,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readEstimatedInputDenialAdditionalRequest(a: std.mem.Allocator, v: c.thinkthen_complete_input_denial_additional_request_v1) anyerror!EstimatedInputDenialAdditionalRequest {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .limit = v.limit,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readEstimatedInputDenialInitialRequest(a: std.mem.Allocator, v: c.thinkthen_complete_input_denial_initial_request_v1) anyerror!EstimatedInputDenialInitialRequest {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .limit = v.limit,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSettledAnswer(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_v1) anyerror!SettledAnswer {
    return switch (v.kind) {
        1 => .{ .decide = try readSettledAnswerDecide(a, v.data.decide[0]) },
        2 => .{ .choose = try readSettledAnswerChoose(a, v.data.choose[0]) },
        3 => .{ .tag = try readSettledAnswerTag(a, v.data.tag[0]) },
        4 => .{ .score = try readSettledAnswerScore(a, v.data.score[0]) },
        5 => .{ .filter = try readSettledAnswerFilter(a, v.data.filter[0]) },
        6 => .{ .rank = try readSettledAnswerRank(a, v.data.rank[0]) },
        7 => .{ .find = try readSettledAnswerFind(a, v.data.find[0]) },
        8 => .{ .recognize = try readSettledAnswerRecognize(a, v.data.recognize[0]) },
        9 => .{ .relate = try readSettledAnswerRelate(a, v.data.relate[0]) },
        10 => .{ .annotate = try readSettledAnswerAnnotate(a, v.data.annotate[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledAnswerAnnotate(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_annotate_v1) anyerror!SettledAnswerAnnotate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape2(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape15(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape15(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_15_v1) anyerror!AnswerShape15 {
    return switch (v.kind) {
        1 => .{ .object = blk: {
            const entries = try a.alloc(Map(SettledJudgment).Entry, v.data.object.len);
            for (entries, 0..) |*out, i| {
                const entry = v.data.object.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readSettledJudgment(a, entry.value[0]) };
            }
            break :blk .{ .entries = entries };
        } },
        2 => .null_value,
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape15AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape15AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape15AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_15_any_of_1_items_v1) anyerror!AnswerShape15AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .object = blk: {
            const entries = try a.alloc(Map(SettledJudgment).Entry, v.data.object.len);
            for (entries, 0..) |*out, i| {
                const entry = v.data.object.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readSettledJudgment(a, entry.value[0]) };
            }
            break :blk .{ .entries = entries };
        } },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledJudgment(a: std.mem.Allocator, v: c.thinkthen_complete_settled_judgment_v1) anyerror!SettledJudgment {
    return switch (v.kind) {
        1 => .{ .boolean = v.data.boolean != 0 },
        2 => .null_value,
        3 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        4 => .{ .number = v.data.number },
        5 => .{ .array = blk: {
            const items = try a.alloc([]const u8, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape5(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_5_v1) anyerror!AnswerShape5 {
    return switch (v.kind) {
        1 => .{ .string = try readAnswerResolution(a, v.data.string[0]) },
        2 => .{ .object = blk: {
            const entries = try a.alloc(Map(AnswerResolution).Entry, v.data.object.len);
            for (entries, 0..) |*out, i| {
                const entry = v.data.object.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readAnswerResolution(a, entry.value[0]) };
            }
            break :blk .{ .entries = entries };
        } },
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerResolutions, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerResolutions(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerResolutions(a: std.mem.Allocator, v: c.thinkthen_complete_answer_resolutions_v1) anyerror!AnswerResolutions {
    return switch (v.kind) {
        1 => .{ .string = try readAnswerResolution(a, v.data.string[0]) },
        2 => .{ .object = blk: {
            const entries = try a.alloc(Map(AnswerResolution).Entry, v.data.object.len);
            for (entries, 0..) |*out, i| {
                const entry = v.data.object.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readAnswerResolution(a, entry.value[0]) };
            }
            break :blk .{ .entries = entries };
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape2(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_2_v1) anyerror!AnswerShape2 {
    return switch (v.kind) {
        1 => .{ .number = v.data.number },
        2 => .null_value,
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape8AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape8AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape8AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_8_any_of_1_items_v1) anyerror!AnswerShape8AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .number = v.data.number },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape3(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_3_v1) anyerror!AnswerShape3 {
    return switch (v.kind) {
        1 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        2 => .null_value,
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape6AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape6AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape6AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_6_any_of_1_items_v1) anyerror!AnswerShape6AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape4(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_4_v1) anyerror!AnswerShape4 {
    return switch (v.kind) {
        1 => .{ .integer = v.data.integer },
        2 => .null_value,
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape4AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape4AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape4AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_4_any_of_1_items_v1) anyerror!AnswerShape4AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .integer = v.data.integer },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledAnswerRelate(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_relate_v1) anyerror!SettledAnswerRelate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape10(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape14(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape14(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_14_v1) anyerror!AnswerShape14 {
    return switch (v.kind) {
        1 => .{ .array = try readSettledRelations(a, v.data.array[0]) },
        2 => .null_value,
        3 => .{ .array2 = blk: {
            const items = try a.alloc(AnswerShape14AnyOf1Items, v.data.array2.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array2.data[i];
                out.* = try readAnswerShape14AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape14AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_14_any_of_1_items_v1) anyerror!AnswerShape14AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .array = try readSettledRelations(a, v.data.array[0]) },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledRelations(a: std.mem.Allocator, v: c.thinkthen_complete_settled_relations_v1) anyerror!SettledRelations {
    return blk: {
        const items = try a.alloc(RelatedEntityEdge, v.value.len);
        for (items, 0..) |*out, i| {
            const entry = v.value.data[i];
            out.* = try readRelatedEntityEdge(a, entry[0]);
        }
        break :blk items;
    };
}

pub fn readRelatedEntityEdge(a: std.mem.Allocator, v: c.thinkthen_complete_related_entity_edge_v1) anyerror!RelatedEntityEdge {
    return .{
        .either = switch (v.either.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.either.value != 0 },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = v.probability,
        .relation = try a.dupe(u8, text(v.relation)),
        .source = try readRelationEndpoint(a, v.source[0]),
        .target = try readRelationEndpoint(a, v.target[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelationEndpoint(a: std.mem.Allocator, v: c.thinkthen_complete_relation_endpoint_v1) anyerror!RelationEndpoint {
    return switch (v.kind) {
        1 => .{ .related_entity = try readRelationEndpointRelatedEntity(a, v.data.related_entity[0]) },
        2 => .{ .source = try readRelationEndpointSource(a, v.data.source[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRelationEndpointSource(a: std.mem.Allocator, v: c.thinkthen_complete_relation_endpoint_source_v1) anyerror!RelationEndpointSource {
    return .{
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind = try a.dupe(u8, text(v.kind)),
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = try a.dupe(u8, text(v.name)),
        .ordinal = v.ordinal,
        .record = switch (v.record.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.record.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelationEndpointRelatedEntity(a: std.mem.Allocator, v: c.thinkthen_complete_relation_endpoint_related_entity_v1) anyerror!RelationEndpointRelatedEntity {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .name = try a.dupe(u8, text(v.name)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape10(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_10_v1) anyerror!AnswerShape10 {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape8AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape8AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        2 => .null_value,
        3 => .{ .array2 = blk: {
            const items = try a.alloc(AnswerShape10AnyOf1Items, v.data.array2.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array2.data[i];
                out.* = try readAnswerShape10AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape10AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_10_any_of_1_items_v1) anyerror!AnswerShape10AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape8AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape8AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledAnswerRecognize(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_recognize_v1) anyerror!SettledAnswerRecognize {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape10(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape13(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape13(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_13_v1) anyerror!AnswerShape13 {
    return switch (v.kind) {
        1 => .{ .object = try readSettledRecognition(a, v.data.object[0]) },
        2 => .null_value,
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape13AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape13AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape13AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_13_any_of_1_items_v1) anyerror!AnswerShape13AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .object = try readSettledRecognition(a, v.data.object[0]) },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledRecognition(a: std.mem.Allocator, v: c.thinkthen_complete_settled_recognition_v1) anyerror!SettledRecognition {
    return switch (v.kind) {
        1 => .{ .entities = try readSettledRecognitionEntities(a, v.data.entities[0]) },
        2 => .{ .mode_proposals = try readSettledRecognitionModeProposals(a, v.data.mode_proposals[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledRecognitionModeProposals(a: std.mem.Allocator, v: c.thinkthen_complete_settled_recognition_mode_proposals_v1) anyerror!SettledRecognitionModeProposals {
    return .{
        .mode = try readBoundaryMode(a, v.mode[0]),
        .proposals = blk: {
            const items = try a.alloc(BoundaryProposal, v.proposals.len);
            for (items, 0..) |*out, i| {
                const entry = v.proposals.data[i];
                out.* = try readBoundaryProposal(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readBoundaryProposal(a: std.mem.Allocator, v: c.thinkthen_complete_boundary_proposal_v1) anyerror!BoundaryProposal {
    return .{
        .end = v.end,
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .length = v.length,
        .probability = v.probability,
        .start = v.start,
        .text = try a.dupe(u8, text(v.text)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readBoundaryMode(a: std.mem.Allocator, v: c.thinkthen_complete_boundary_mode_v1) anyerror!BoundaryMode {
    _ = a;
    return switch (v.kind) {
        1 => .boundary_only,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledRecognitionEntities(a: std.mem.Allocator, v: c.thinkthen_complete_settled_recognition_entities_v1) anyerror!SettledRecognitionEntities {
    return .{
        .entities = blk: {
            const items = try a.alloc(Entity, v.entities.len);
            for (items, 0..) |*out, i| {
                const entry = v.entities.data[i];
                out.* = try readEntity(a, entry[0]);
            }
            break :blk items;
        },
        .relations = switch (v.relations.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(EntityEdge, v.relations.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.relations.value.data[i];
                    out.* = try readEntityEdge(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readEntityEdge(a: std.mem.Allocator, v: c.thinkthen_complete_entity_edge_v1) anyerror!EntityEdge {
    return .{
        .either = switch (v.either.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.either.value != 0 },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = v.probability,
        .relation = try a.dupe(u8, text(v.relation)),
        .source = try readEntity(a, v.source[0]),
        .target = try readEntity(a, v.target[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readEntity(a: std.mem.Allocator, v: c.thinkthen_complete_entity_v1) anyerror!Entity {
    return .{
        .end = v.end,
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind = try a.dupe(u8, text(v.kind)),
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .length = v.length,
        .start = v.start,
        .strength = v.strength,
        .text = try a.dupe(u8, text(v.text)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSettledAnswerFind(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_find_v1) anyerror!SettledAnswerFind {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape2(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape12(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape12(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_12_v1) anyerror!AnswerShape12 {
    return try readValue(a, v.value[0]);
}

pub fn readSettledAnswerRank(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_rank_v1) anyerror!SettledAnswerRank {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape10(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape11(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape11(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_11_v1) anyerror!AnswerShape11 {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc(u64, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = entry;
            }
            break :blk items;
        } },
        2 => .null_value,
        3 => .{ .array2 = blk: {
            const items = try a.alloc(AnswerShape11AnyOf1Items, v.data.array2.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array2.data[i];
                out.* = try readAnswerShape11AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape11AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_11_any_of_1_items_v1) anyerror!AnswerShape11AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc(u64, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = entry;
            }
            break :blk items;
        } },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledAnswerFilter(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_filter_v1) anyerror!SettledAnswerFilter {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape10(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape9(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape9(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_9_v1) anyerror!AnswerShape9 {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc(SettledOriginal, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readSettledOriginal(a, entry[0]);
            }
            break :blk items;
        } },
        2 => .null_value,
        3 => .{ .array2 = blk: {
            const items = try a.alloc(AnswerShape9AnyOf1Items, v.data.array2.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array2.data[i];
                out.* = try readAnswerShape9AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape9AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_9_any_of_1_items_v1) anyerror!AnswerShape9AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc(SettledOriginal, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readSettledOriginal(a, entry[0]);
            }
            break :blk items;
        } },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledOriginal(a: std.mem.Allocator, v: c.thinkthen_complete_settled_original_v1) anyerror!SettledOriginal {
    return try readValue(a, v.value[0]);
}

pub fn readSettledAnswerScore(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_score_v1) anyerror!SettledAnswerScore {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape2(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape8(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape8(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_8_v1) anyerror!AnswerShape8 {
    return switch (v.kind) {
        1 => .{ .number = v.data.number },
        2 => .null_value,
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape8AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape8AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledAnswerTag(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_tag_v1) anyerror!SettledAnswerTag {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape2(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape7(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape7(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_7_v1) anyerror!AnswerShape7 {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc([]const u8, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        } },
        2 => .null_value,
        3 => .{ .array2 = blk: {
            const items = try a.alloc(AnswerShape7AnyOf1Items, v.data.array2.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array2.data[i];
                out.* = try readAnswerShape7AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerShape7AnyOf1Items(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_7_any_of_1_items_v1) anyerror!AnswerShape7AnyOf1Items {
    return switch (v.kind) {
        1 => .{ .array = blk: {
            const items = try a.alloc([]const u8, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        } },
        2 => .null_value,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledAnswerChoose(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_choose_v1) anyerror!SettledAnswerChoose {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape2(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape6(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape6(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_6_v1) anyerror!AnswerShape6 {
    return switch (v.kind) {
        1 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        2 => .null_value,
        3 => .{ .array = blk: {
            const items = try a.alloc(AnswerShape6AnyOf1Items, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try readAnswerShape6AnyOf1Items(a, entry[0]);
            }
            break :blk items;
        } },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSettledAnswerDecide(a: std.mem.Allocator, v: c.thinkthen_complete_settled_answer_decide_v1) anyerror!SettledAnswerDecide {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape4(a, v.index.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .pick = switch (v.pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape3(a, v.pick.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = switch (v.probability.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape2(a, v.probability.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .resolution = try readAnswerShape5(a, v.resolution[0]),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerShape(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerShape(a: std.mem.Allocator, v: c.thinkthen_complete_answer_shape_v1) anyerror!AnswerShape {
    return try readValue(a, v.value[0]);
}

pub fn readSessionPacketObservation(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_observation_v1) anyerror!SessionPacketObservation {
    return .{
        .function = try readRequestFunction(a, v.function[0]),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readSessionObservation(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionObservation(a: std.mem.Allocator, v: c.thinkthen_complete_session_observation_v1) anyerror!SessionObservation {
    return switch (v.kind) {
        1 => .{ .question = try readSessionObservationQuestion(a, v.data.question[0]) },
        2 => .{ .row = try readSessionObservationRow(a, v.data.row[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionObservationRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_observation_row_v1) anyerror!SessionObservationRow {
    return .{
        .index = v.index,
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readSessionObservedRow(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionObservedRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_observed_row_v1) anyerror!SessionObservedRow {
    return switch (v.kind) {
        1 => .{ .judgment = try readSessionObservedRowJudgment(a, v.data.judgment[0]) },
        2 => .{ .annotated = try readSessionObservedRowAnnotated(a, v.data.annotated[0]) },
        3 => .{ .recognized = try readSessionObservedRowRecognized(a, v.data.recognized[0]) },
        4 => .{ .find = try readSessionObservedRowFind(a, v.data.find[0]) },
        5 => .{ .relations = try readSessionObservedRowRelations(a, v.data.relations[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionObservedRowRelations(a: std.mem.Allocator, v: c.thinkthen_complete_session_observed_row_relations_v1) anyerror!SessionObservedRowRelations {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(SessionRelationEdge, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readSessionRelationEdge(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionRelationEdge(a: std.mem.Allocator, v: c.thinkthen_complete_session_relation_edge_v1) anyerror!SessionRelationEdge {
    return .{
        .either = v.either != 0,
        .probability = v.probability,
        .relation = try a.dupe(u8, text(v.relation)),
        .source = try readEntityDocument(a, v.source[0]),
        .target = try readEntityDocument(a, v.target[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readEntityDocument(a: std.mem.Allocator, v: c.thinkthen_complete_entity_document_v1) anyerror!EntityDocument {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .name = try a.dupe(u8, text(v.name)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionObservedRowFind(a: std.mem.Allocator, v: c.thinkthen_complete_session_observed_row_find_v1) anyerror!SessionObservedRowFind {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.value.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionObservedRowRecognized(a: std.mem.Allocator, v: c.thinkthen_complete_session_observed_row_recognized_v1) anyerror!SessionObservedRowRecognized {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readSessionRecognition(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionRecognition(a: std.mem.Allocator, v: c.thinkthen_complete_session_recognition_v1) anyerror!SessionRecognition {
    return .{
        .entities = blk: {
            const items = try a.alloc(Entity, v.entities.len);
            for (items, 0..) |*out, i| {
                const entry = v.entities.data[i];
                out.* = try readEntity(a, entry[0]);
            }
            break :blk items;
        },
        .mode = try readRecognitionMode(a, v.mode[0]),
        .proposals = switch (v.proposals.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(BoundaryProposal, v.proposals.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.proposals.value.data[i];
                    out.* = try readBoundaryProposal(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .relations = switch (v.relations.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RecognitionEdgeDocument, v.relations.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.relations.value.data[i];
                    out.* = try readRecognitionEdgeDocument(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognitionEdgeDocument(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_edge_v1) anyerror!RecognitionEdgeDocument {
    return .{
        .either = v.either != 0,
        .probability = v.probability,
        .relation = try a.dupe(u8, text(v.relation)),
        .source = try readEntity(a, v.source[0]),
        .target = try readEntity(a, v.target[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognitionMode(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_mode_v1) anyerror!RecognitionMode {
    _ = a;
    return switch (v.kind) {
        1 => .whole,
        2 => .boundary_only,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionObservedRowAnnotated(a: std.mem.Allocator, v: c.thinkthen_complete_session_observed_row_annotated_v1) anyerror!SessionObservedRowAnnotated {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(SessionAnnotation, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readSessionAnnotation(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionAnnotation(a: std.mem.Allocator, v: c.thinkthen_complete_session_annotation_v1) anyerror!SessionAnnotation {
    return .{
        .name = try a.dupe(u8, text(v.name)),
        .value = try readAnnotationValue(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotationValue(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_value_v1) anyerror!AnnotationValue {
    return switch (v.kind) {
        1 => .{ .decision = try readAnnotationValueDecision(a, v.data.decision[0]) },
        2 => .{ .choice = try readAnnotationValueChoice(a, v.data.choice[0]) },
        3 => .{ .score = try readAnnotationValueScore(a, v.data.score[0]) },
        4 => .{ .tags = try readAnnotationValueTags(a, v.data.tags[0]) },
        5 => .{ .failed = try readAnnotationValueFailed(a, v.data.failed[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnnotationValueFailed(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_value_failed_v1) anyerror!AnnotationValueFailed {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readFailure(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFailure(a: std.mem.Allocator, v: c.thinkthen_complete_failure_v1) anyerror!Failure {
    return .{
        .cause = try readFailureCause(a, v.cause[0]),
        .kind = try readFailureFieldKind(a, v.kind[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFailureCause(a: std.mem.Allocator, v: c.thinkthen_complete_failure_cause_v1) anyerror!FailureCause {
    _ = a;
    return switch (v.kind) {
        1 => .missing_answer,
        2 => .wrong_kind,
        3 => .missing_probability,
        4 => .invalid_probability,
        5 => .invalid_distribution,
        6 => .unexpected_probability,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnnotationValueTags(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_value_tags_v1) anyerror!AnnotationValueTags {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc([]const u8, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotationValueScore(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_value_score_v1) anyerror!AnnotationValueScore {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = v.value,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotationValueChoice(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_value_choice_v1) anyerror!AnnotationValueChoice {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.value.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotationValueDecision(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_value_decision_v1) anyerror!AnnotationValueDecision {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.value.value != 0 },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionObservedRowJudgment(a: std.mem.Allocator, v: c.thinkthen_complete_session_observed_row_judgment_v1) anyerror!SessionObservedRowJudgment {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readSessionJudgment(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionJudgment(a: std.mem.Allocator, v: c.thinkthen_complete_session_judgment_v1) anyerror!SessionJudgment {
    return switch (v.kind) {
        1 => .{ .decision = try readSessionJudgmentDecision(a, v.data.decision[0]) },
        2 => .{ .choice = try readSessionJudgmentChoice(a, v.data.choice[0]) },
        3 => .{ .score = try readSessionJudgmentScore(a, v.data.score[0]) },
        4 => .{ .tags = try readSessionJudgmentTags(a, v.data.tags[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionJudgmentTags(a: std.mem.Allocator, v: c.thinkthen_complete_session_judgment_tags_v1) anyerror!SessionJudgmentTags {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc([]const u8, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionJudgmentScore(a: std.mem.Allocator, v: c.thinkthen_complete_session_judgment_score_v1) anyerror!SessionJudgmentScore {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = v.value,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionJudgmentChoice(a: std.mem.Allocator, v: c.thinkthen_complete_session_judgment_choice_v1) anyerror!SessionJudgmentChoice {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.value.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionJudgmentDecision(a: std.mem.Allocator, v: c.thinkthen_complete_session_judgment_decision_v1) anyerror!SessionJudgmentDecision {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.value.value != 0 },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionObservationQuestion(a: std.mem.Allocator, v: c.thinkthen_complete_session_observation_question_v1) anyerror!SessionObservationQuestion {
    return .{
        .detail = try readSessionQuestionDetail(a, v.detail[0]),
        .index = v.index,
        .kind = try a.dupe(u8, text(v.kind)),
        .member = switch (v.member.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.member.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .position = v.position,
        .stage = switch (v.stage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.stage.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionQuestionDetail(a: std.mem.Allocator, v: c.thinkthen_complete_session_question_detail_v1) anyerror!SessionQuestionDetail {
    return .{
        .answer_id = switch (v.answer_id.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readAnswerId(a, v.answer_id.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .cached = v.cached != 0,
        .confidence = switch (v.confidence.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.confidence.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .failed_questions = v.failed_questions,
        .failure = switch (v.failure.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readFailure(a, v.failure.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .failure_id = switch (v.failure_id.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readFailureId(a, v.failure_id.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .input_source = switch (v.input_source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.input_source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .input_sources = blk: {
            const items = try a.alloc(SessionInputSource, v.input_sources.len);
            for (items, 0..) |*out, i| {
                const entry = v.input_sources.data[i];
                out.* = try readSessionInputSource(a, entry[0]);
            }
            break :blk items;
        },
        .inputs = blk: {
            const items = try a.alloc(Value, v.inputs.len);
            for (items, 0..) |*out, i| {
                const entry = v.inputs.data[i];
                out.* = try readValue(a, entry[0]);
            }
            break :blk items;
        },
        .model = try a.dupe(u8, text(v.model)),
        .observations = blk: {
            const items = try a.alloc(Observation, v.observations.len);
            for (items, 0..) |*out, i| {
                const entry = v.observations.data[i];
                out.* = try readObservation(a, entry[0]);
            }
            break :blk items;
        },
        .probabilities = switch (v.probabilities.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readSessionProbabilities(a, v.probabilities.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .question = try readQuestionDocument(a, v.question[0]),
        .question_sha256 = try a.dupe(u8, text(v.question_sha256)),
        .question_sources = blk: {
            const items = try a.alloc(QuestionSource, v.question_sources.len);
            for (items, 0..) |*out, i| {
                const entry = v.question_sources.data[i];
                out.* = try readQuestionSource(a, entry[0]);
            }
            break :blk items;
        },
        .raw_pick = switch (v.raw_pick.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.raw_pick.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .reported_usage = switch (v.reported_usage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readUsage(a, v.reported_usage.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .requests = blk: {
            const items = try a.alloc([]const u8, v.requests.len);
            for (items, 0..) |*out, i| {
                const entry = v.requests.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .requests_sent = v.requests_sent,
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .url = try a.dupe(u8, text(v.url)),
        .usage = switch (v.usage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readTokenUsage(a, v.usage.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readJudgmentValue(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readJudgmentValue(a: std.mem.Allocator, v: c.thinkthen_complete_value_v1) anyerror!JudgmentValue {
    return switch (v.kind) {
        1 => .{ .boolean = v.data.boolean != 0 },
        2 => .null_value,
        3 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        4 => .{ .array = blk: {
            const items = try a.alloc([]const u8, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        } },
        5 => .{ .number = v.data.number },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readTokenUsage(a: std.mem.Allocator, v: c.thinkthen_complete_token_usage_v1) anyerror!TokenUsage {
    return .{
        .input_tokens = v.input_tokens,
        .output_tokens = v.output_tokens,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readThreshold(a: std.mem.Allocator, v: c.thinkthen_complete_threshold_v1) anyerror!Threshold {
    return switch (v.kind) {
        1 => .{ .number = v.data.number },
        2 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readUsage(a: std.mem.Allocator, v: c.thinkthen_complete_usage_v1) anyerror!Usage {
    return .{
        .input_tokens = switch (v.input_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.input_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .output_tokens = switch (v.output_tokens.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.output_tokens.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readQuestionSource(a: std.mem.Allocator, v: c.thinkthen_complete_question_source_v1) anyerror!QuestionSource {
    return .{
        .answered_by = try a.dupe(u8, text(v.answered_by)),
        .batch_size = switch (v.batch_size.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.batch_size.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .origin = try readOrigin(a, v.origin[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readOrigin(a: std.mem.Allocator, v: c.thinkthen_complete_origin_v1) anyerror!Origin {
    _ = a;
    return switch (v.kind) {
        1 => .live,
        2 => .cache,
        3 => .replay,
        4 => .proxy,
        5 => .memory,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readQuestionDocument(a: std.mem.Allocator, v: c.thinkthen_complete_question_document_v1) anyerror!QuestionDocument {
    return switch (v.kind) {
        1 => .{ .decide = try readQuestionDocumentDecide(a, v.data.decide[0]) },
        2 => .{ .choose = try readQuestionDocumentChoose(a, v.data.choose[0]) },
        3 => .{ .tag = try readQuestionDocumentTag(a, v.data.tag[0]) },
        4 => .{ .score = try readQuestionDocumentScore(a, v.data.score[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readQuestionDocumentScore(a: std.mem.Allocator, v: c.thinkthen_complete_question_document_score_v1) anyerror!QuestionDocumentScore {
    return .{
        .batch = switch (v.batch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatch(a, v.batch.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .context_schema = switch (v.context_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.context_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .item_schema = switch (v.item_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.item_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .label_details = switch (v.label_details.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Label, v.label_details.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.label_details.value.data[i];
                    out.* = try readLabel(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = switch (v.name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readQuestionName(a, v.name.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .on = switch (v.on.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.on.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.on.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile = switch (v.profile.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.profile.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .wording_version = switch (v.wording_version.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readWordingVersion(a, v.wording_version.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .levels = blk: {
            const items = try a.alloc([]const u8, v.levels.len);
            for (items, 0..) |*out, i| {
                const entry = v.levels.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .text = switch (v.text.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.text.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .verb = try a.dupe(u8, text(v.verb)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readWordingVersion(a: std.mem.Allocator, v: c.thinkthen_complete_wording_version_v1) anyerror!WordingVersion {
    _ = a;
    return v.value;
}

pub fn readQuestionName(a: std.mem.Allocator, v: c.thinkthen_complete_question_name_v1) anyerror!QuestionName {
    return try a.dupe(u8, text(v.value));
}

pub fn readLabel(a: std.mem.Allocator, v: c.thinkthen_complete_label_v1) anyerror!Label {
    return .{
        .description = switch (v.description.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.description.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = try a.dupe(u8, text(v.name)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readInputDeclaration(a: std.mem.Allocator, v: c.thinkthen_complete_input_declaration_v1) anyerror!InputDeclaration {
    return switch (v.kind) {
        1 => .{ .string_root = try readInputDeclarationStringRoot(a, v.data.string_root[0]) },
        2 => .{ .object_root = try readInputDeclarationObjectRoot(a, v.data.object_root[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readInputDeclarationObjectRoot(a: std.mem.Allocator, v: c.thinkthen_complete_input_declaration_object_root_v1) anyerror!InputDeclarationObjectRoot {
    return .{
        .properties = blk: {
            const entries = try a.alloc(Map(InputPropertyType).Entry, v.properties.len);
            for (entries, 0..) |*out, i| {
                const entry = v.properties.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readInputPropertyType(a, entry.value[0]) };
            }
            break :blk .{ .entries = entries };
        },
        .required = switch (v.required.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.required.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.required.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .type = try readObjectType(a, v.type[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readObjectType(a: std.mem.Allocator, v: c.thinkthen_complete_object_type_v1) anyerror!ObjectType {
    _ = a;
    return switch (v.kind) {
        1 => .object,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readInputPropertyType(a: std.mem.Allocator, v: c.thinkthen_complete_input_property_type_v1) anyerror!InputPropertyType {
    return switch (v.kind) {
        1 => .{ .string = try readInputPropertyTypeString(a, v.data.string[0]) },
        2 => .{ .number = try readInputPropertyTypeNumber(a, v.data.number[0]) },
        3 => .{ .boolean = try readInputPropertyTypeBoolean(a, v.data.boolean[0]) },
        4 => .{ .array = try readInputPropertyTypeArray(a, v.data.array[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readInputPropertyTypeArray(a: std.mem.Allocator, v: c.thinkthen_complete_input_property_type_array_v1) anyerror!InputPropertyTypeArray {
    return .{
        .items = try readStringRoot(a, v.items[0]),
        .type = try a.dupe(u8, text(v.type)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readStringRoot(a: std.mem.Allocator, v: c.thinkthen_complete_string_root_v1) anyerror!StringRoot {
    return .{
        .type = try readStringType(a, v.type[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readStringType(a: std.mem.Allocator, v: c.thinkthen_complete_string_type_v1) anyerror!StringType {
    _ = a;
    return switch (v.kind) {
        1 => .string,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readInputPropertyTypeBoolean(a: std.mem.Allocator, v: c.thinkthen_complete_input_property_type_boolean_v1) anyerror!InputPropertyTypeBoolean {
    return .{
        .type = try a.dupe(u8, text(v.type)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readInputPropertyTypeNumber(a: std.mem.Allocator, v: c.thinkthen_complete_input_property_type_number_v1) anyerror!InputPropertyTypeNumber {
    return .{
        .type = try a.dupe(u8, text(v.type)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readInputPropertyTypeString(a: std.mem.Allocator, v: c.thinkthen_complete_input_property_type_string_v1) anyerror!InputPropertyTypeString {
    return .{
        .type = try a.dupe(u8, text(v.type)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readInputDeclarationStringRoot(a: std.mem.Allocator, v: c.thinkthen_complete_input_declaration_string_root_v1) anyerror!InputDeclarationStringRoot {
    return .{
        .type = try readStringType(a, v.type[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readBatch(a: std.mem.Allocator, v: c.thinkthen_complete_batch_v1) anyerror!Batch {
    return switch (v.kind) {
        1 => .{ .integer = v.data.integer },
        2 => .{ .string = try readBatchString(a, v.data.string[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readQuestionDocumentTag(a: std.mem.Allocator, v: c.thinkthen_complete_question_document_tag_v1) anyerror!QuestionDocumentTag {
    return .{
        .batch = switch (v.batch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatch(a, v.batch.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .context_schema = switch (v.context_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.context_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .item_schema = switch (v.item_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.item_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .label_details = switch (v.label_details.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Label, v.label_details.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.label_details.value.data[i];
                    out.* = try readLabel(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = switch (v.name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readQuestionName(a, v.name.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .on = switch (v.on.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.on.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.on.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile = switch (v.profile.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.profile.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .wording_version = switch (v.wording_version.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readWordingVersion(a, v.wording_version.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .labels = blk: {
            const items = try a.alloc([]const u8, v.labels.len);
            for (items, 0..) |*out, i| {
                const entry = v.labels.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .text = switch (v.text.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.text.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .verb = try a.dupe(u8, text(v.verb)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readQuestionDocumentChoose(a: std.mem.Allocator, v: c.thinkthen_complete_question_document_choose_v1) anyerror!QuestionDocumentChoose {
    return .{
        .batch = switch (v.batch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatch(a, v.batch.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .context_schema = switch (v.context_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.context_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .item_schema = switch (v.item_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.item_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .label_details = switch (v.label_details.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Label, v.label_details.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.label_details.value.data[i];
                    out.* = try readLabel(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = switch (v.name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readQuestionName(a, v.name.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .on = switch (v.on.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.on.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.on.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile = switch (v.profile.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.profile.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .wording_version = switch (v.wording_version.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readWordingVersion(a, v.wording_version.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .options = blk: {
            const items = try a.alloc([]const u8, v.options.len);
            for (items, 0..) |*out, i| {
                const entry = v.options.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .text = switch (v.text.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.text.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .verb = try a.dupe(u8, text(v.verb)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readQuestionDocumentDecide(a: std.mem.Allocator, v: c.thinkthen_complete_question_document_decide_v1) anyerror!QuestionDocumentDecide {
    return .{
        .batch = switch (v.batch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatch(a, v.batch.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .context_schema = switch (v.context_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.context_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .item_schema = switch (v.item_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.item_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .label_details = switch (v.label_details.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Label, v.label_details.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.label_details.value.data[i];
                    out.* = try readLabel(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = switch (v.name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readQuestionName(a, v.name.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .on = switch (v.on.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.on.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.on.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile = switch (v.profile.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.profile.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .wording_version = switch (v.wording_version.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readWordingVersion(a, v.wording_version.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .false = switch (v.false_.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.false_.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .text = switch (v.text.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.text.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .true = switch (v.true_.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.true_.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .verb = try a.dupe(u8, text(v.verb)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionProbabilities(a: std.mem.Allocator, v: c.thinkthen_complete_session_probabilities_v1) anyerror!SessionProbabilities {
    return switch (v.kind) {
        1 => .{ .yes_no = try readSessionProbabilitiesYesNo(a, v.data.yes_no[0]) },
        2 => .{ .named = try readSessionProbabilitiesNamed(a, v.data.named[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionProbabilitiesNamed(a: std.mem.Allocator, v: c.thinkthen_complete_session_probabilities_named_v1) anyerror!SessionProbabilitiesNamed {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(SessionNamedProbability, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readSessionNamedProbability(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionNamedProbability(a: std.mem.Allocator, v: c.thinkthen_complete_session_named_probability_v1) anyerror!SessionNamedProbability {
    return .{
        .name = try a.dupe(u8, text(v.name)),
        .probability = v.probability,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionProbabilitiesYesNo(a: std.mem.Allocator, v: c.thinkthen_complete_session_probabilities_yes_no_v1) anyerror!SessionProbabilitiesYesNo {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .value = v.value,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readObservation(a: std.mem.Allocator, v: c.thinkthen_complete_observation_v1) anyerror!Observation {
    return switch (v.kind) {
        1 => .{ .observation_id = try readObservationObservationId(a, v.data.observation_id[0]) },
        2 => .{ .failure_id = try readObservationFailureId(a, v.data.failure_id[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readObservationFailureId(a: std.mem.Allocator, v: c.thinkthen_complete_observation_failure_id_v1) anyerror!ObservationFailureId {
    return .{
        .failure_id = try readFailureId(a, v.failure_id[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFailureId(a: std.mem.Allocator, v: c.thinkthen_complete_failure_id_v1) anyerror!FailureId {
    return try a.dupe(u8, text(v.value));
}

pub fn readObservationObservationId(a: std.mem.Allocator, v: c.thinkthen_complete_observation_observation_id_v1) anyerror!ObservationObservationId {
    return .{
        .observation_id = try readObservationId(a, v.observation_id[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readObservationId(a: std.mem.Allocator, v: c.thinkthen_complete_observation_id_v1) anyerror!ObservationId {
    return try a.dupe(u8, text(v.value));
}

pub fn readSessionInputSource(a: std.mem.Allocator, v: c.thinkthen_complete_session_input_source_v1) anyerror!SessionInputSource {
    return .{
        .index = v.index,
        .source = try readPhysicalSource(a, v.source[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readPhysicalSource(a: std.mem.Allocator, v: c.thinkthen_complete_physical_source_v1) anyerror!PhysicalSource {
    return .{
        .file = try a.dupe(u8, text(v.file)),
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerId(a: std.mem.Allocator, v: c.thinkthen_complete_answer_id_v1) anyerror!AnswerId {
    return try a.dupe(u8, text(v.value));
}

pub fn readRequestFunction(a: std.mem.Allocator, v: c.thinkthen_complete_request_function_v1) anyerror!RequestFunction {
    _ = a;
    return switch (v.kind) {
        1 => .decide,
        2 => .choose,
        3 => .tag,
        4 => .score,
        5 => .filter,
        6 => .rank,
        7 => .find,
        8 => .annotate,
        9 => .recognize,
        10 => .relate,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionPacketRelateAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_relate_aggregate_v1) anyerror!SessionPacketRelateAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readRelation(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelation(a: std.mem.Allocator, v: c.thinkthen_complete_relation_v1) anyerror!Relation {
    return .{
        .answer = try readAnswers(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .input_sources = switch (v.input_sources.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(SessionInputSource, v.input_sources.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.input_sources.value.data[i];
                    out.* = try readSessionInputSource(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .position = switch (v.position.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPosition(a, v.position.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .question = try readRelateQuestionDocument(a, v.question[0]),
        .schema = try readVersion(a, v.schema[0]),
        .value = blk: {
            const items = try a.alloc(RelatedEntityEdge, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readRelatedEntityEdge(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readVersion(a: std.mem.Allocator, v: c.thinkthen_complete_version_v1) anyerror!Version {
    _ = a;
    return switch (v.kind) {
        1 => .@"thinkthen.result/2",
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRelateQuestionDocument(a: std.mem.Allocator, v: c.thinkthen_complete_relate_question_document_v1) anyerror!RelateQuestionDocument {
    return .{
        .batch = switch (v.batch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatch(a, v.batch.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .context_schema = switch (v.context_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.context_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .fields = switch (v.fields.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readRelateFields(a, v.fields.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .item_schema = switch (v.item_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.item_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .label_details = switch (v.label_details.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Label, v.label_details.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.label_details.value.data[i];
                    out.* = try readLabel(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = switch (v.name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readQuestionName(a, v.name.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .on = switch (v.on.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.on.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.on.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile = switch (v.profile.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.profile.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .relations = blk: {
            const items = try a.alloc(RelationRule, v.relations.len);
            for (items, 0..) |*out, i| {
                const entry = v.relations.data[i];
                out.* = try readRelationRule(a, entry[0]);
            }
            break :blk items;
        },
        .threshold = try readThreshold(a, v.threshold[0]),
        .verb = try a.dupe(u8, text(v.verb)),
        .wording_version = switch (v.wording_version.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readWordingVersion(a, v.wording_version.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelationRule(a: std.mem.Allocator, v: c.thinkthen_complete_relation_rule_v1) anyerror!RelationRule {
    return .{
        .either = v.either != 0,
        .name = try a.dupe(u8, text(v.name)),
        .reads = try a.dupe(u8, text(v.reads)),
        .single = switch (v.single.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.single.value != 0 },
            else => return error.InvalidNativeDiscriminator,
        },
        .source = try a.dupe(u8, text(v.source)),
        .target = try a.dupe(u8, text(v.target)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelateFields(a: std.mem.Allocator, v: c.thinkthen_complete_relate_fields_v1) anyerror!RelateFields {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .name = try a.dupe(u8, text(v.name)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readPosition(a: std.mem.Allocator, v: c.thinkthen_complete_position_v1) anyerror!Position {
    return .{
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first = switch (v.first.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .last = switch (v.last.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readMeta(a: std.mem.Allocator, v: c.thinkthen_complete_meta_v1) anyerror!Meta {
    return .{
        .answered_by = switch (v.answered_by.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.answered_by.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .attempts = switch (v.attempts.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Attempt, v.attempts.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.attempts.value.data[i];
                    out.* = try readAttempt(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .batch_setting = switch (v.batch_setting.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatchSetting(a, v.batch_setting.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .batch_warning = switch (v.batch_warning.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatchWarning(a, v.batch_warning.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .cached = v.cached != 0,
        .context_sha256 = switch (v.context_sha256.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.context_sha256.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .failed_questions = v.failed_questions,
        .model = try a.dupe(u8, text(v.model)),
        .observations = blk: {
            const items = try a.alloc(Observation, v.observations.len);
            for (items, 0..) |*out, i| {
                const entry = v.observations.data[i];
                out.* = try readObservation(a, entry[0]);
            }
            break :blk items;
        },
        .origin = switch (v.origin.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readOrigin(a, v.origin.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile_warning = switch (v.profile_warning.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readProfileWarning(a, v.profile_warning.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .question_sha256 = switch (v.question_sha256.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.question_sha256.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .question_sources = blk: {
            const items = try a.alloc(QuestionSource, v.question_sources.len);
            for (items, 0..) |*out, i| {
                const entry = v.question_sources.data[i];
                out.* = try readQuestionSource(a, entry[0]);
            }
            break :blk items;
        },
        .questions_sha256 = switch (v.questions_sha256.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.questions_sha256.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .requests = blk: {
            const items = try a.alloc([]const u8, v.requests.len);
            for (items, 0..) |*out, i| {
                const entry = v.requests.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .requests_sent = v.requests_sent,
        .tool = try a.dupe(u8, text(v.tool)),
        .url = try a.dupe(u8, text(v.url)),
        .usage = switch (v.usage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readUsage(a, v.usage.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readProfileWarning(a: std.mem.Allocator, v: c.thinkthen_complete_profile_warning_v1) anyerror!ProfileWarning {
    return .{
        .running = try a.dupe(u8, text(v.running)),
        .tuned_for = try a.dupe(u8, text(v.tuned_for)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readBatchWarning(a: std.mem.Allocator, v: c.thinkthen_complete_batch_warning_v1) anyerror!BatchWarning {
    return .{
        .running = try readBatchSetting(a, v.running[0]),
        .tuned_for = try readBatchSetting(a, v.tuned_for[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readBatchSetting(a: std.mem.Allocator, v: c.thinkthen_complete_batch_setting_v1) anyerror!BatchSetting {
    return switch (v.kind) {
        1 => .{ .integer = v.data.integer },
        2 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswers(a: std.mem.Allocator, v: c.thinkthen_complete_answers_v1) anyerror!Answers {
    return .{
        .questions = blk: {
            const items = try a.alloc(RelationMember, v.questions.len);
            for (items, 0..) |*out, i| {
                const entry = v.questions.data[i];
                out.* = try readRelationMember(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelationMember(a: std.mem.Allocator, v: c.thinkthen_complete_relation_member_v1) anyerror!RelationMember {
    return switch (v.kind) {
        1 => .{ .answer_id = try readRelationMemberAnswerId(a, v.data.answer_id[0]) },
        2 => .{ .failure_id = try readRelationMemberFailureId(a, v.data.failure_id[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRelationMemberFailureId(a: std.mem.Allocator, v: c.thinkthen_complete_relation_member_failure_id_v1) anyerror!RelationMemberFailureId {
    return .{
        .direction = try readRelationDirection(a, v.direction[0]),
        .method = try readRelationMethod(a, v.method[0]),
        .observations = blk: {
            const items = try a.alloc(Observation, v.observations.len);
            for (items, 0..) |*out, i| {
                const entry = v.observations.data[i];
                out.* = try readObservation(a, entry[0]);
            }
            break :blk items;
        },
        .question = try readQuestionDocument(a, v.question[0]),
        .question_sources = blk: {
            const items = try a.alloc(QuestionSource, v.question_sources.len);
            for (items, 0..) |*out, i| {
                const entry = v.question_sources.data[i];
                out.* = try readQuestionSource(a, entry[0]);
            }
            break :blk items;
        },
        .reads = try a.dupe(u8, text(v.reads)),
        .relation = try a.dupe(u8, text(v.relation)),
        .request = try a.dupe(u8, text(v.request)),
        .source = try readRelatedEntity(a, v.source[0]),
        .target = switch (v.target.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readRelatedEntity(a, v.target.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = try readThreshold(a, v.threshold[0]),
        .usage = switch (v.usage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readUsage(a, v.usage.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .failure = try readFailure(a, v.failure[0]),
        .failure_id = try readFailureId(a, v.failure_id[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelatedEntity(a: std.mem.Allocator, v: c.thinkthen_complete_related_entity_v1) anyerror!RelatedEntity {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .name = try a.dupe(u8, text(v.name)),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRelationMethod(a: std.mem.Allocator, v: c.thinkthen_complete_relation_method_v1) anyerror!RelationMethod {
    _ = a;
    return switch (v.kind) {
        1 => .yes_no,
        2 => .choice,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRelationDirection(a: std.mem.Allocator, v: c.thinkthen_complete_relation_direction_v1) anyerror!RelationDirection {
    _ = a;
    return switch (v.kind) {
        1 => .source_to_target,
        2 => .either,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRelationMemberAnswerId(a: std.mem.Allocator, v: c.thinkthen_complete_relation_member_answer_id_v1) anyerror!RelationMemberAnswerId {
    return .{
        .direction = try readRelationDirection(a, v.direction[0]),
        .method = try readRelationMethod(a, v.method[0]),
        .observations = blk: {
            const items = try a.alloc(Observation, v.observations.len);
            for (items, 0..) |*out, i| {
                const entry = v.observations.data[i];
                out.* = try readObservation(a, entry[0]);
            }
            break :blk items;
        },
        .question = try readQuestionDocument(a, v.question[0]),
        .question_sources = blk: {
            const items = try a.alloc(QuestionSource, v.question_sources.len);
            for (items, 0..) |*out, i| {
                const entry = v.question_sources.data[i];
                out.* = try readQuestionSource(a, entry[0]);
            }
            break :blk items;
        },
        .reads = try a.dupe(u8, text(v.reads)),
        .relation = try a.dupe(u8, text(v.relation)),
        .request = try a.dupe(u8, text(v.request)),
        .source = try readRelatedEntity(a, v.source[0]),
        .target = switch (v.target.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readRelatedEntity(a, v.target.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = try readThreshold(a, v.threshold[0]),
        .usage = switch (v.usage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readUsage(a, v.usage.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .accepted = v.accepted != 0,
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .probability = v.probability,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswer(a: std.mem.Allocator, v: c.thinkthen_complete_answer_v1) anyerror!Answer {
    return switch (v.kind) {
        1 => .{ .yes_no = try readAnswerYesNo(a, v.data.yes_no[0]) },
        2 => .{ .choice = try readAnswerChoice(a, v.data.choice[0]) },
        3 => .{ .tag = try readAnswerTag(a, v.data.tag[0]) },
        4 => .{ .score = try readAnswerScore(a, v.data.score[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnswerScore(a: std.mem.Allocator, v: c.thinkthen_complete_answer_score_v1) anyerror!AnswerScore {
    return .{
        .confidence = switch (v.confidence.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.confidence.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind = try a.dupe(u8, text(v.kind)),
        .level = try a.dupe(u8, text(v.level)),
        .probabilities = blk: {
            const entries = try a.alloc(Map(f64).Entry, v.probabilities.len);
            for (entries, 0..) |*out, i| {
                const entry = v.probabilities.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = entry.value };
            }
            break :blk .{ .entries = entries };
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerTag(a: std.mem.Allocator, v: c.thinkthen_complete_answer_tag_v1) anyerror!AnswerTag {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .probabilities = blk: {
            const entries = try a.alloc(Map(f64).Entry, v.probabilities.len);
            for (entries, 0..) |*out, i| {
                const entry = v.probabilities.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = entry.value };
            }
            break :blk .{ .entries = entries };
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerChoice(a: std.mem.Allocator, v: c.thinkthen_complete_answer_choice_v1) anyerror!AnswerChoice {
    return .{
        .confidence = switch (v.confidence.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.confidence.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind = try a.dupe(u8, text(v.kind)),
        .pick = try a.dupe(u8, text(v.pick)),
        .probabilities = blk: {
            const entries = try a.alloc(Map(f64).Entry, v.probabilities.len);
            for (entries, 0..) |*out, i| {
                const entry = v.probabilities.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = entry.value };
            }
            break :blk .{ .entries = entries };
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnswerYesNo(a: std.mem.Allocator, v: c.thinkthen_complete_answer_yes_no_v1) anyerror!AnswerYesNo {
    return .{
        .kind = try a.dupe(u8, text(v.kind)),
        .probability = v.probability,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketRecognizeAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_recognize_aggregate_v1) anyerror!SessionPacketRecognizeAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(Recognition, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readRecognition(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognition(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_v1) anyerror!Recognition {
    return .{
        .answer = try readRecognitionOdds(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .position = switch (v.position.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPosition(a, v.position.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .question = try readRecognizeQuestionDocument(a, v.question[0]),
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = try readRecognize(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognize(a: std.mem.Allocator, v: c.thinkthen_complete_recognize_v1) anyerror!Recognize {
    return switch (v.kind) {
        1 => .{ .entities = try readRecognizeEntities(a, v.data.entities[0]) },
        2 => .{ .mode_proposals = try readRecognizeModeProposals(a, v.data.mode_proposals[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRecognizeModeProposals(a: std.mem.Allocator, v: c.thinkthen_complete_recognize_mode_proposals_v1) anyerror!RecognizeModeProposals {
    return .{
        .mode = try readBoundaryMode(a, v.mode[0]),
        .proposals = blk: {
            const items = try a.alloc(BoundaryProposal, v.proposals.len);
            for (items, 0..) |*out, i| {
                const entry = v.proposals.data[i];
                out.* = try readBoundaryProposal(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognizeEntities(a: std.mem.Allocator, v: c.thinkthen_complete_recognize_entities_v1) anyerror!RecognizeEntities {
    return .{
        .entities = blk: {
            const items = try a.alloc(Entity, v.entities.len);
            for (items, 0..) |*out, i| {
                const entry = v.entities.data[i];
                out.* = try readEntity(a, entry[0]);
            }
            break :blk items;
        },
        .relations = switch (v.relations.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(EntityEdge, v.relations.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.relations.value.data[i];
                    out.* = try readEntityEdge(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognizeQuestionDocument(a: std.mem.Allocator, v: c.thinkthen_complete_recognize_question_document_v1) anyerror!RecognizeQuestionDocument {
    return .{
        .batch = switch (v.batch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatch(a, v.batch.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .context_schema = switch (v.context_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.context_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .entity_definition = switch (v.entity_definition.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.entity_definition.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .instructions = switch (v.instructions.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.instructions.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .item_schema = switch (v.item_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.item_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .kinds = blk: {
            const entries = try a.alloc(Map(Value).Entry, v.kinds.len);
            for (entries, 0..) |*out, i| {
                const entry = v.kinds.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readValue(a, entry.value[0]) };
            }
            break :blk .{ .entries = entries };
        },
        .label_details = switch (v.label_details.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Label, v.label_details.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.label_details.value.data[i];
                    out.* = try readLabel(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .mode = switch (v.mode.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readRecognitionMode(a, v.mode.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = switch (v.name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readQuestionName(a, v.name.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .on = switch (v.on.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.on.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.on.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile = switch (v.profile.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.profile.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .relation_threshold = switch (v.relation_threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.relation_threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .relations = switch (v.relations.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RelationRule, v.relations.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.relations.value.data[i];
                    out.* = try readRelationRule(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .snippet_pieces = switch (v.snippet_pieces.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.snippet_pieces.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .stage_context = switch (v.stage_context.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readRecognitionStageContext(a, v.stage_context.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = try readThreshold(a, v.threshold[0]),
        .verb = try readVerb(a, v.verb[0]),
        .wording_version = switch (v.wording_version.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readWordingVersion(a, v.wording_version.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readVerb(a: std.mem.Allocator, v: c.thinkthen_complete_verb_v1) anyerror!Verb {
    _ = a;
    return switch (v.kind) {
        1 => .recognize,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRecognitionStageContext(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_stage_context_v1) anyerror!RecognitionStageContext {
    return .{
        .boundary = switch (v.boundary.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.boundary.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind_edge = switch (v.kind_edge.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.kind_edge.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .relation = switch (v.relation.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.relation.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognitionOdds(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_odds_v1) anyerror!RecognitionOdds {
    return switch (v.kind) {
        1 => .{ .recognize_answer = try readRecognitionOddsRecognizeAnswer(a, v.data.recognize_answer[0]) },
        2 => .{ .boundary_odds = try readRecognitionOddsBoundaryOdds(a, v.data.boundary_odds[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readRecognitionOddsBoundaryOdds(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_odds_boundary_odds_v1) anyerror!RecognitionOddsBoundaryOdds {
    return .{
        .pieces = blk: {
            const items = try a.alloc(PieceOdds, v.pieces.len);
            for (items, 0..) |*out, i| {
                const entry = v.pieces.data[i];
                out.* = try readPieceOdds(a, entry[0]);
            }
            break :blk items;
        },
        .proposals = blk: {
            const items = try a.alloc(BoundaryProposal, v.proposals.len);
            for (items, 0..) |*out, i| {
                const entry = v.proposals.data[i];
                out.* = try readBoundaryProposal(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readPieceOdds(a: std.mem.Allocator, v: c.thinkthen_complete_piece_odds_v1) anyerror!PieceOdds {
    return .{
        .end = v.end,
        .start = v.start,
        .tags = blk: {
            const entries = try a.alloc(Map(f64).Entry, v.tags.len);
            for (entries, 0..) |*out, i| {
                const entry = v.tags.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = entry.value };
            }
            break :blk .{ .entries = entries };
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognitionOddsRecognizeAnswer(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_odds_recognize_answer_v1) anyerror!RecognitionOddsRecognizeAnswer {
    return .{
        .names = blk: {
            const items = try a.alloc(NameOdds, v.names.len);
            for (items, 0..) |*out, i| {
                const entry = v.names.data[i];
                out.* = try readNameOdds(a, entry[0]);
            }
            break :blk items;
        },
        .pairs = blk: {
            const items = try a.alloc(PairOdds, v.pairs.len);
            for (items, 0..) |*out, i| {
                const entry = v.pairs.data[i];
                out.* = try readPairOdds(a, entry[0]);
            }
            break :blk items;
        },
        .pieces = blk: {
            const items = try a.alloc(PieceOdds, v.pieces.len);
            for (items, 0..) |*out, i| {
                const entry = v.pieces.data[i];
                out.* = try readPieceOdds(a, entry[0]);
            }
            break :blk items;
        },
        .proposals = blk: {
            const items = try a.alloc(RecognitionProposal, v.proposals.len);
            for (items, 0..) |*out, i| {
                const entry = v.proposals.data[i];
                out.* = try readRecognitionProposal(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRecognitionProposal(a: std.mem.Allocator, v: c.thinkthen_complete_recognition_proposal_v1) anyerror!RecognitionProposal {
    return .{
        .end = v.end,
        .kept = v.kept != 0,
        .kind = switch (v.kind.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.kind.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .selected = switch (v.selected.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPlace(a, v.selected.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .span_probability = v.span_probability,
        .start = v.start,
        .strength = switch (v.strength.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.strength.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readPlace(a: std.mem.Allocator, v: c.thinkthen_complete_place_v1) anyerror!Place {
    return .{
        .end = v.end,
        .start = v.start,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readPairOdds(a: std.mem.Allocator, v: c.thinkthen_complete_pair_odds_v1) anyerror!PairOdds {
    return .{
        .probability = v.probability,
        .relation = try a.dupe(u8, text(v.relation)),
        .source = try readPlace(a, v.source[0]),
        .target = try readPlace(a, v.target[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readNameOdds(a: std.mem.Allocator, v: c.thinkthen_complete_name_odds_v1) anyerror!NameOdds {
    return .{
        .edges = switch (v.edges.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const entries = try a.alloc(Map(f64).Entry, v.edges.value.len);
                for (entries, 0..) |*out, i| {
                    const entry = v.edges.value.data[i];
                    out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = entry.value };
                }
                break :blk .{ .entries = entries };
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .end = v.end,
        .kinds = switch (v.kinds.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const entries = try a.alloc(Map(f64).Entry, v.kinds.value.len);
                for (entries, 0..) |*out, i| {
                    const entry = v.kinds.value.data[i];
                    out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = entry.value };
                }
                break :blk .{ .entries = entries };
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .start = v.start,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketAnnotateAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_annotate_aggregate_v1) anyerror!SessionPacketAnnotateAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(Annotation, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readAnnotation(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotation(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_v1) anyerror!Annotation {
    return .{
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .answers = blk: {
            const entries = try a.alloc(Map(AnnotationMember).Entry, v.answers.len);
            for (entries, 0..) |*out, i| {
                const entry = v.answers.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readAnnotationMember(a, entry.value[0]) };
            }
            break :blk .{ .entries = entries };
        },
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .position = switch (v.position.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPosition(a, v.position.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = try readAnnotatedRow(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotatedRow(a: std.mem.Allocator, v: c.thinkthen_complete_annotated_row_v1) anyerror!AnnotatedRow {
    return blk: {
        const entries = try a.alloc(Map(AnnotatedField).Entry, v.value.len);
        for (entries, 0..) |*out, i| {
            const entry = v.value.data[i];
            out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = try readAnnotatedField(a, entry.value[0]) };
        }
        break :blk .{ .entries = entries };
    };
}

pub fn readAnnotatedField(a: std.mem.Allocator, v: c.thinkthen_complete_annotated_field_v1) anyerror!AnnotatedField {
    return switch (v.kind) {
        1 => .{ .boolean = v.data.boolean != 0 },
        2 => .null_value,
        3 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        4 => .{ .array = blk: {
            const items = try a.alloc([]const u8, v.data.array.len);
            for (items, 0..) |*out, i| {
                const entry = v.data.array.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        } },
        5 => .{ .number = v.data.number },
        6 => .{ .object = try readFailed(a, v.data.object[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readFailed(a: std.mem.Allocator, v: c.thinkthen_complete_failed_v1) anyerror!Failed {
    return .{
        .failed = try readFailure(a, v.failed[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotationMember(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_member_v1) anyerror!AnnotationMember {
    return switch (v.kind) {
        1 => .{ .answer_id = try readAnnotationMemberAnswerId(a, v.data.answer_id[0]) },
        2 => .{ .failure_id = try readAnnotationMemberFailureId(a, v.data.failure_id[0]) },
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readAnnotationMemberFailureId(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_member_failure_id_v1) anyerror!AnnotationMemberFailureId {
    return .{
        .failure = try readFailure(a, v.failure[0]),
        .failure_id = try readFailureId(a, v.failure_id[0]),
        .observations = blk: {
            const items = try a.alloc(Observation, v.observations.len);
            for (items, 0..) |*out, i| {
                const entry = v.observations.data[i];
                out.* = try readObservation(a, entry[0]);
            }
            break :blk items;
        },
        .question = try readQuestionDocument(a, v.question[0]),
        .question_sources = blk: {
            const items = try a.alloc(QuestionSource, v.question_sources.len);
            for (items, 0..) |*out, i| {
                const entry = v.question_sources.data[i];
                out.* = try readQuestionSource(a, entry[0]);
            }
            break :blk items;
        },
        .request = try a.dupe(u8, text(v.request)),
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .usage = switch (v.usage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readUsage(a, v.usage.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAnnotationMemberAnswerId(a: std.mem.Allocator, v: c.thinkthen_complete_annotation_member_answer_id_v1) anyerror!AnnotationMemberAnswerId {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .observations = blk: {
            const items = try a.alloc(Observation, v.observations.len);
            for (items, 0..) |*out, i| {
                const entry = v.observations.data[i];
                out.* = try readObservation(a, entry[0]);
            }
            break :blk items;
        },
        .question = try readQuestionDocument(a, v.question[0]),
        .question_sources = blk: {
            const items = try a.alloc(QuestionSource, v.question_sources.len);
            for (items, 0..) |*out, i| {
                const entry = v.question_sources.data[i];
                out.* = try readQuestionSource(a, entry[0]);
            }
            break :blk items;
        },
        .request = try a.dupe(u8, text(v.request)),
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .usage = switch (v.usage.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readUsage(a, v.usage.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readJudgmentValue(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketFindAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_find_aggregate_v1) anyerror!SessionPacketFindAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readFind(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFind(a: std.mem.Allocator, v: c.thinkthen_complete_find_v1) anyerror!Find {
    return .{
        .answer = try readFindAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .candidates = switch (v.candidates.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(FindCandidate, v.candidates.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.candidates.value.data[i];
                    out.* = try readFindCandidate(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .file = switch (v.file.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.file.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .first_line = switch (v.first_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.first_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .last_line = switch (v.last_line.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.last_line.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .position = switch (v.position.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPosition(a, v.position.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .question = try readFindQuestionDocument(a, v.question[0]),
        .schema = try readVersion(a, v.schema[0]),
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFindQuestionDocument(a: std.mem.Allocator, v: c.thinkthen_complete_find_question_document_v1) anyerror!FindQuestionDocument {
    return .{
        .batch = switch (v.batch.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readBatch(a, v.batch.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .context_schema = switch (v.context_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.context_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .item_schema = switch (v.item_schema.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readInputDeclaration(a, v.item_schema.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .label_details = switch (v.label_details.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Label, v.label_details.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.label_details.value.data[i];
                    out.* = try readLabel(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .model = switch (v.model.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.model.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .name = switch (v.name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readQuestionName(a, v.name.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .none = v.none != 0,
        .on = switch (v.on.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc([]const u8, v.on.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.on.value.data[i];
                    out.* = try a.dupe(u8, text(entry));
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .profile = switch (v.profile.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.profile.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .text = switch (v.text.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.text.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .verb = try a.dupe(u8, text(v.verb)),
        .wording_version = switch (v.wording_version.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readWordingVersion(a, v.wording_version.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFindCandidate(a: std.mem.Allocator, v: c.thinkthen_complete_find_candidate_v1) anyerror!FindCandidate {
    return .{
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .probability = v.probability,
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFindAnswer(a: std.mem.Allocator, v: c.thinkthen_complete_find_answer_v1) anyerror!FindAnswer {
    return .{
        .confidence = switch (v.confidence.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.confidence.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .kind = try a.dupe(u8, text(v.kind)),
        .pick = try a.dupe(u8, text(v.pick)),
        .probabilities = blk: {
            const entries = try a.alloc(Map(f64).Entry, v.probabilities.len);
            for (entries, 0..) |*out, i| {
                const entry = v.probabilities.data[i];
                out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = entry.value };
            }
            break :blk .{ .entries = entries };
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketRankAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_rank_aggregate_v1) anyerror!SessionPacketRankAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(AtomicNonZeroUsize, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readAtomicNonZeroUsize(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAtomicNonZeroUsize(a: std.mem.Allocator, v: c.thinkthen_complete_atomic_non_zero_usize_v1) anyerror!AtomicNonZeroUsize {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Image, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try readImage(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .members = switch (v.members.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RankMember, v.members.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.members.value.data[i];
                    out.* = try readRankMember(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .question = try readQuestionDocument(a, v.question[0]),
        .question_name = switch (v.question_name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.question_name.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = v.value,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRankMember(a: std.mem.Allocator, v: c.thinkthen_complete_rank_member_v1) anyerror!RankMember {
    return .{
        .name = try a.dupe(u8, text(v.name)),
        .result = try readRankMemberResult(a, v.result[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readRankMemberResult(a: std.mem.Allocator, v: c.thinkthen_complete_rank_member_result_v1) anyerror!RankMemberResult {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Image, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try readImage(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .question = try readQuestionDocument(a, v.question[0]),
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = v.value,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readImage(a: std.mem.Allocator, v: c.thinkthen_complete_image_v1) anyerror!Image {
    return .{
        .base64 = try a.dupe(u8, text(v.base64)),
        .height = v.height,
        .media = try readImageMedia(a, v.media[0]),
        .width = v.width,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readImageMedia(a: std.mem.Allocator, v: c.thinkthen_complete_image_media_v1) anyerror!ImageMedia {
    _ = a;
    return switch (v.kind) {
        1 => .@"image/jpeg",
        2 => .@"image/png",
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readSessionPacketFilterAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_filter_aggregate_v1) anyerror!SessionPacketFilterAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(AtomicBoolean, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readAtomicBoolean(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAtomicBoolean(a: std.mem.Allocator, v: c.thinkthen_complete_atomic_boolean_v1) anyerror!AtomicBoolean {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Image, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try readImage(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .members = switch (v.members.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RankMember, v.members.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.members.value.data[i];
                    out.* = try readRankMember(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .question = try readQuestionDocument(a, v.question[0]),
        .question_name = switch (v.question_name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.question_name.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = v.value != 0,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketScoreAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_score_aggregate_v1) anyerror!SessionPacketScoreAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(AtomicDouble, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readAtomicDouble(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAtomicDouble(a: std.mem.Allocator, v: c.thinkthen_complete_atomic_double_v1) anyerror!AtomicDouble {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Image, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try readImage(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .members = switch (v.members.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RankMember, v.members.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.members.value.data[i];
                    out.* = try readRankMember(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .question = try readQuestionDocument(a, v.question[0]),
        .question_name = switch (v.question_name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.question_name.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = v.value,
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketTagAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_tag_aggregate_v1) anyerror!SessionPacketTagAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(AtomicArrayOfString, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readAtomicArrayOfString(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAtomicArrayOfString(a: std.mem.Allocator, v: c.thinkthen_complete_atomic_array_of_string_v1) anyerror!AtomicArrayOfString {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Image, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try readImage(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .members = switch (v.members.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RankMember, v.members.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.members.value.data[i];
                    out.* = try readRankMember(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .question = try readQuestionDocument(a, v.question[0]),
        .question_name = switch (v.question_name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.question_name.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = blk: {
            const items = try a.alloc([]const u8, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try a.dupe(u8, text(entry));
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketChooseAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_choose_aggregate_v1) anyerror!SessionPacketChooseAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(AtomicNullableString, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readAtomicNullableString(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAtomicNullableString(a: std.mem.Allocator, v: c.thinkthen_complete_atomic_nullable_string_v1) anyerror!AtomicNullableString {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Image, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try readImage(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .members = switch (v.members.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RankMember, v.members.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.members.value.data[i];
                    out.* = try readRankMember(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .question = try readQuestionDocument(a, v.question[0]),
        .question_name = switch (v.question_name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.question_name.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.value.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketDecideAggregate(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_decide_aggregate_v1) anyerror!SessionPacketDecideAggregate {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = blk: {
            const items = try a.alloc(AtomicDecideValue, v.value.len);
            for (items, 0..) |*out, i| {
                const entry = v.value.data[i];
                out.* = try readAtomicDecideValue(a, entry[0]);
            }
            break :blk items;
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readAtomicDecideValue(a: std.mem.Allocator, v: c.thinkthen_complete_atomic_decide_value_v1) anyerror!AtomicDecideValue {
    return .{
        .answer = try readAnswer(a, v.answer[0]),
        .answer_id = try readAnswerId(a, v.answer_id[0]),
        .images = switch (v.images.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(Image, v.images.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.images.value.data[i];
                    out.* = try readImage(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .index = switch (v.index.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = v.index.value },
            else => return error.InvalidNativeDiscriminator,
        },
        .input = switch (v.input.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readValue(a, v.input.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .members = switch (v.members.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = blk: {
                const items = try a.alloc(RankMember, v.members.value.len);
                for (items, 0..) |*out, i| {
                    const entry = v.members.value.data[i];
                    out.* = try readRankMember(a, entry[0]);
                }
                break :blk items;
            } },
            else => return error.InvalidNativeDiscriminator,
        },
        .meta = try readMeta(a, v.meta[0]),
        .question = try readQuestionDocument(a, v.question[0]),
        .question_name = switch (v.question_name.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try a.dupe(u8, text(v.question_name.value)) },
            else => return error.InvalidNativeDiscriminator,
        },
        .schema = try readVersion(a, v.schema[0]),
        .source = switch (v.source.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readPhysicalSource(a, v.source.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .threshold = switch (v.threshold.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readThreshold(a, v.threshold.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .value = switch (v.value.presence) {
            0 => .missing,
            1 => .null_value,
            2 => .{ .value = try readDecideValue(a, v.value.value[0]) },
            else => return error.InvalidNativeDiscriminator,
        },
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readDecideValue(a: std.mem.Allocator, v: c.thinkthen_complete_decide_value_v1) anyerror!DecideValue {
    return try readValue(a, v.value[0]);
}

pub fn readSessionPacketAnnotateRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_annotate_row_v1) anyerror!SessionPacketAnnotateRow {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readAnnotation(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketFilterRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_filter_row_v1) anyerror!SessionPacketFilterRow {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readAtomicBoolean(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketScoreRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_score_row_v1) anyerror!SessionPacketScoreRow {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readAtomicDouble(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketTagRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_tag_row_v1) anyerror!SessionPacketTagRow {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readAtomicArrayOfString(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketChooseRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_choose_row_v1) anyerror!SessionPacketChooseRow {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readAtomicNullableString(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readSessionPacketDecideRow(a: std.mem.Allocator, v: c.thinkthen_complete_session_packet_decide_row_v1) anyerror!SessionPacketDecideRow {
    return .{
        .function = try a.dupe(u8, text(v.function)),
        .kind = try a.dupe(u8, text(v.kind)),
        .value = try readAtomicDecideValue(a, v.value[0]),
        .extensions = try readObject(a, v.extensions),
    };
}

pub fn readFailureFieldKind(a: std.mem.Allocator, v: c.thinkthen_complete_failure_field_kind_v1) anyerror!FailureFieldKind {
    _ = a;
    return switch (v.kind) {
        1 => .backend,
        else => error.InvalidNativeDiscriminator,
    };
}

pub fn readBatchString(a: std.mem.Allocator, v: c.thinkthen_complete_batch_string_v1) anyerror!BatchString {
    _ = a;
    return switch (v.kind) {
        1 => .max,
        else => error.InvalidNativeDiscriminator,
    };
}
pub const Types = struct {
    pub const AnnotatedField = @import("results_generated.zig").AnnotatedField;
    pub const AnnotatedRow = @import("results_generated.zig").AnnotatedRow;
    pub const Annotation = @import("results_generated.zig").Annotation;
    pub const AnnotationMember = @import("results_generated.zig").AnnotationMember;
    pub const AnnotationMemberAnswerId = @import("results_generated.zig").AnnotationMemberAnswerId;
    pub const AnnotationMemberFailureId = @import("results_generated.zig").AnnotationMemberFailureId;
    pub const AnnotationValue = @import("results_generated.zig").AnnotationValue;
    pub const AnnotationValueChoice = @import("results_generated.zig").AnnotationValueChoice;
    pub const AnnotationValueDecision = @import("results_generated.zig").AnnotationValueDecision;
    pub const AnnotationValueFailed = @import("results_generated.zig").AnnotationValueFailed;
    pub const AnnotationValueScore = @import("results_generated.zig").AnnotationValueScore;
    pub const AnnotationValueTags = @import("results_generated.zig").AnnotationValueTags;
    pub const Answer = @import("results_generated.zig").Answer;
    pub const AnswerChoice = @import("results_generated.zig").AnswerChoice;
    pub const AnswerId = @import("results_generated.zig").AnswerId;
    pub const AnswerResolution = @import("results_generated.zig").AnswerResolution;
    pub const AnswerResolutions = @import("results_generated.zig").AnswerResolutions;
    pub const AnswerScore = @import("results_generated.zig").AnswerScore;
    pub const AnswerShape = @import("results_generated.zig").AnswerShape;
    pub const AnswerShape10 = @import("results_generated.zig").AnswerShape10;
    pub const AnswerShape10AnyOf1Items = @import("results_generated.zig").AnswerShape10AnyOf1Items;
    pub const AnswerShape11 = @import("results_generated.zig").AnswerShape11;
    pub const AnswerShape11AnyOf1Items = @import("results_generated.zig").AnswerShape11AnyOf1Items;
    pub const AnswerShape12 = @import("results_generated.zig").AnswerShape12;
    pub const AnswerShape13 = @import("results_generated.zig").AnswerShape13;
    pub const AnswerShape13AnyOf1Items = @import("results_generated.zig").AnswerShape13AnyOf1Items;
    pub const AnswerShape14 = @import("results_generated.zig").AnswerShape14;
    pub const AnswerShape14AnyOf1Items = @import("results_generated.zig").AnswerShape14AnyOf1Items;
    pub const AnswerShape15 = @import("results_generated.zig").AnswerShape15;
    pub const AnswerShape15AnyOf1Items = @import("results_generated.zig").AnswerShape15AnyOf1Items;
    pub const AnswerShape2 = @import("results_generated.zig").AnswerShape2;
    pub const AnswerShape3 = @import("results_generated.zig").AnswerShape3;
    pub const AnswerShape4 = @import("results_generated.zig").AnswerShape4;
    pub const AnswerShape4AnyOf1Items = @import("results_generated.zig").AnswerShape4AnyOf1Items;
    pub const AnswerShape5 = @import("results_generated.zig").AnswerShape5;
    pub const AnswerShape6 = @import("results_generated.zig").AnswerShape6;
    pub const AnswerShape6AnyOf1Items = @import("results_generated.zig").AnswerShape6AnyOf1Items;
    pub const AnswerShape7 = @import("results_generated.zig").AnswerShape7;
    pub const AnswerShape7AnyOf1Items = @import("results_generated.zig").AnswerShape7AnyOf1Items;
    pub const AnswerShape8 = @import("results_generated.zig").AnswerShape8;
    pub const AnswerShape8AnyOf1Items = @import("results_generated.zig").AnswerShape8AnyOf1Items;
    pub const AnswerShape9 = @import("results_generated.zig").AnswerShape9;
    pub const AnswerShape9AnyOf1Items = @import("results_generated.zig").AnswerShape9AnyOf1Items;
    pub const AnswerTag = @import("results_generated.zig").AnswerTag;
    pub const AnswerYesNo = @import("results_generated.zig").AnswerYesNo;
    pub const Answers = @import("results_generated.zig").Answers;
    pub const AtomicArrayOfString = @import("results_generated.zig").AtomicArrayOfString;
    pub const AtomicBoolean = @import("results_generated.zig").AtomicBoolean;
    pub const AtomicDecideValue = @import("results_generated.zig").AtomicDecideValue;
    pub const AtomicDouble = @import("results_generated.zig").AtomicDouble;
    pub const AtomicNonZeroUsize = @import("results_generated.zig").AtomicNonZeroUsize;
    pub const AtomicNullableString = @import("results_generated.zig").AtomicNullableString;
    pub const Attempt = @import("results_generated.zig").Attempt;
    pub const AttemptOutcome = @import("results_generated.zig").AttemptOutcome;
    pub const Batch = @import("results_generated.zig").Batch;
    pub const BatchSetting = @import("results_generated.zig").BatchSetting;
    pub const BatchString = @import("results_generated.zig").BatchString;
    pub const BatchWarning = @import("results_generated.zig").BatchWarning;
    pub const BoundaryMode = @import("results_generated.zig").BoundaryMode;
    pub const BoundaryProposal = @import("results_generated.zig").BoundaryProposal;
    pub const CallError = @import("results_generated.zig").CallError;
    pub const CallId = @import("results_generated.zig").CallId;
    pub const DecideValue = @import("results_generated.zig").DecideValue;
    pub const Entity = @import("results_generated.zig").Entity;
    pub const EntityDocument = @import("results_generated.zig").EntityDocument;
    pub const EntityEdge = @import("results_generated.zig").EntityEdge;
    pub const Error = @import("results_generated.zig").Error;
    pub const EstimatedInputDenial = @import("results_generated.zig").EstimatedInputDenial;
    pub const EstimatedInputDenialAdditionalRequest = @import("results_generated.zig").EstimatedInputDenialAdditionalRequest;
    pub const EstimatedInputDenialInitialRequest = @import("results_generated.zig").EstimatedInputDenialInitialRequest;
    pub const EstimatedInputDenialRetry = @import("results_generated.zig").EstimatedInputDenialRetry;
    pub const Facts = @import("results_generated.zig").Facts;
    pub const Failed = @import("results_generated.zig").Failed;
    pub const Failure = @import("results_generated.zig").Failure;
    pub const FailureCause = @import("results_generated.zig").FailureCause;
    pub const FailureFieldKind = @import("results_generated.zig").FailureFieldKind;
    pub const FailureId = @import("results_generated.zig").FailureId;
    pub const FailureKind = @import("results_generated.zig").FailureKind;
    pub const Find = @import("results_generated.zig").Find;
    pub const FindAnswer = @import("results_generated.zig").FindAnswer;
    pub const FindCandidate = @import("results_generated.zig").FindCandidate;
    pub const FindQuestionDocument = @import("results_generated.zig").FindQuestionDocument;
    pub const Image = @import("results_generated.zig").Image;
    pub const ImageMedia = @import("results_generated.zig").ImageMedia;
    pub const ImmediateFacts = @import("results_generated.zig").ImmediateFacts;
    pub const InputDeclaration = @import("results_generated.zig").InputDeclaration;
    pub const InputDeclarationObjectRoot = @import("results_generated.zig").InputDeclarationObjectRoot;
    pub const InputDeclarationStringRoot = @import("results_generated.zig").InputDeclarationStringRoot;
    pub const InputPropertyType = @import("results_generated.zig").InputPropertyType;
    pub const InputPropertyTypeArray = @import("results_generated.zig").InputPropertyTypeArray;
    pub const InputPropertyTypeBoolean = @import("results_generated.zig").InputPropertyTypeBoolean;
    pub const InputPropertyTypeNumber = @import("results_generated.zig").InputPropertyTypeNumber;
    pub const InputPropertyTypeString = @import("results_generated.zig").InputPropertyTypeString;
    pub const JudgmentValue = @import("results_generated.zig").JudgmentValue;
    pub const Label = @import("results_generated.zig").Label;
    pub const Map = @import("results_generated.zig").Map;
    pub const Meta = @import("results_generated.zig").Meta;
    pub const NameOdds = @import("results_generated.zig").NameOdds;
    pub const Number = @import("results_generated.zig").Number;
    pub const ObjectType = @import("results_generated.zig").ObjectType;
    pub const Observation = @import("results_generated.zig").Observation;
    pub const ObservationFailureId = @import("results_generated.zig").ObservationFailureId;
    pub const ObservationId = @import("results_generated.zig").ObservationId;
    pub const ObservationObservationId = @import("results_generated.zig").ObservationObservationId;
    pub const Origin = @import("results_generated.zig").Origin;
    pub const PairOdds = @import("results_generated.zig").PairOdds;
    pub const PersistenceObservation = @import("results_generated.zig").PersistenceObservation;
    pub const PhysicalSource = @import("results_generated.zig").PhysicalSource;
    pub const PieceOdds = @import("results_generated.zig").PieceOdds;
    pub const Place = @import("results_generated.zig").Place;
    pub const Position = @import("results_generated.zig").Position;
    pub const Presence = @import("results_generated.zig").Presence;
    pub const ProfileWarning = @import("results_generated.zig").ProfileWarning;
    pub const QuestionDocument = @import("results_generated.zig").QuestionDocument;
    pub const QuestionDocumentChoose = @import("results_generated.zig").QuestionDocumentChoose;
    pub const QuestionDocumentDecide = @import("results_generated.zig").QuestionDocumentDecide;
    pub const QuestionDocumentScore = @import("results_generated.zig").QuestionDocumentScore;
    pub const QuestionDocumentTag = @import("results_generated.zig").QuestionDocumentTag;
    pub const QuestionName = @import("results_generated.zig").QuestionName;
    pub const QuestionSource = @import("results_generated.zig").QuestionSource;
    pub const RankMember = @import("results_generated.zig").RankMember;
    pub const RankMemberResult = @import("results_generated.zig").RankMemberResult;
    pub const Recognition = @import("results_generated.zig").Recognition;
    pub const RecognitionEdgeDocument = @import("results_generated.zig").RecognitionEdgeDocument;
    pub const RecognitionMode = @import("results_generated.zig").RecognitionMode;
    pub const RecognitionOdds = @import("results_generated.zig").RecognitionOdds;
    pub const RecognitionOddsBoundaryOdds = @import("results_generated.zig").RecognitionOddsBoundaryOdds;
    pub const RecognitionOddsRecognizeAnswer = @import("results_generated.zig").RecognitionOddsRecognizeAnswer;
    pub const RecognitionProposal = @import("results_generated.zig").RecognitionProposal;
    pub const RecognitionStageContext = @import("results_generated.zig").RecognitionStageContext;
    pub const Recognize = @import("results_generated.zig").Recognize;
    pub const RecognizeEntities = @import("results_generated.zig").RecognizeEntities;
    pub const RecognizeModeProposals = @import("results_generated.zig").RecognizeModeProposals;
    pub const RecognizeQuestionDocument = @import("results_generated.zig").RecognizeQuestionDocument;
    pub const RelateFields = @import("results_generated.zig").RelateFields;
    pub const RelateQuestionDocument = @import("results_generated.zig").RelateQuestionDocument;
    pub const RelatedEntity = @import("results_generated.zig").RelatedEntity;
    pub const RelatedEntityEdge = @import("results_generated.zig").RelatedEntityEdge;
    pub const Relation = @import("results_generated.zig").Relation;
    pub const RelationDirection = @import("results_generated.zig").RelationDirection;
    pub const RelationEndpoint = @import("results_generated.zig").RelationEndpoint;
    pub const RelationEndpointRelatedEntity = @import("results_generated.zig").RelationEndpointRelatedEntity;
    pub const RelationEndpointSource = @import("results_generated.zig").RelationEndpointSource;
    pub const RelationMember = @import("results_generated.zig").RelationMember;
    pub const RelationMemberAnswerId = @import("results_generated.zig").RelationMemberAnswerId;
    pub const RelationMemberFailureId = @import("results_generated.zig").RelationMemberFailureId;
    pub const RelationMethod = @import("results_generated.zig").RelationMethod;
    pub const RelationRule = @import("results_generated.zig").RelationRule;
    pub const RequestFunction = @import("results_generated.zig").RequestFunction;
    pub const SdkRequestId = @import("results_generated.zig").SdkRequestId;
    pub const SendBudgetDenial = @import("results_generated.zig").SendBudgetDenial;
    pub const SendBudgetDenialBeforeAdditionalSend = @import("results_generated.zig").SendBudgetDenialBeforeAdditionalSend;
    pub const SendBudgetDenialBeforeFirstSend = @import("results_generated.zig").SendBudgetDenialBeforeFirstSend;
    pub const SendBudgetDenialBeforeRetry = @import("results_generated.zig").SendBudgetDenialBeforeRetry;
    pub const SessionAnnotation = @import("results_generated.zig").SessionAnnotation;
    pub const SessionInputSource = @import("results_generated.zig").SessionInputSource;
    pub const SessionJudgment = @import("results_generated.zig").SessionJudgment;
    pub const SessionJudgmentChoice = @import("results_generated.zig").SessionJudgmentChoice;
    pub const SessionJudgmentDecision = @import("results_generated.zig").SessionJudgmentDecision;
    pub const SessionJudgmentScore = @import("results_generated.zig").SessionJudgmentScore;
    pub const SessionJudgmentTags = @import("results_generated.zig").SessionJudgmentTags;
    pub const SessionNamedProbability = @import("results_generated.zig").SessionNamedProbability;
    pub const SessionObservation = @import("results_generated.zig").SessionObservation;
    pub const SessionObservationQuestion = @import("results_generated.zig").SessionObservationQuestion;
    pub const SessionObservationRow = @import("results_generated.zig").SessionObservationRow;
    pub const SessionObservedRow = @import("results_generated.zig").SessionObservedRow;
    pub const SessionObservedRowAnnotated = @import("results_generated.zig").SessionObservedRowAnnotated;
    pub const SessionObservedRowFind = @import("results_generated.zig").SessionObservedRowFind;
    pub const SessionObservedRowJudgment = @import("results_generated.zig").SessionObservedRowJudgment;
    pub const SessionObservedRowRecognized = @import("results_generated.zig").SessionObservedRowRecognized;
    pub const SessionObservedRowRelations = @import("results_generated.zig").SessionObservedRowRelations;
    pub const SessionPacket = @import("results_generated.zig").SessionPacket;
    pub const SessionPacketAnnotateAggregate = @import("results_generated.zig").SessionPacketAnnotateAggregate;
    pub const SessionPacketAnnotateRow = @import("results_generated.zig").SessionPacketAnnotateRow;
    pub const SessionPacketChooseAggregate = @import("results_generated.zig").SessionPacketChooseAggregate;
    pub const SessionPacketChooseRow = @import("results_generated.zig").SessionPacketChooseRow;
    pub const SessionPacketDecideAggregate = @import("results_generated.zig").SessionPacketDecideAggregate;
    pub const SessionPacketDecideRow = @import("results_generated.zig").SessionPacketDecideRow;
    pub const SessionPacketFilterAggregate = @import("results_generated.zig").SessionPacketFilterAggregate;
    pub const SessionPacketFilterRow = @import("results_generated.zig").SessionPacketFilterRow;
    pub const SessionPacketFindAggregate = @import("results_generated.zig").SessionPacketFindAggregate;
    pub const SessionPacketObservation = @import("results_generated.zig").SessionPacketObservation;
    pub const SessionPacketRankAggregate = @import("results_generated.zig").SessionPacketRankAggregate;
    pub const SessionPacketRecognizeAggregate = @import("results_generated.zig").SessionPacketRecognizeAggregate;
    pub const SessionPacketRelateAggregate = @import("results_generated.zig").SessionPacketRelateAggregate;
    pub const SessionPacketScoreAggregate = @import("results_generated.zig").SessionPacketScoreAggregate;
    pub const SessionPacketScoreRow = @import("results_generated.zig").SessionPacketScoreRow;
    pub const SessionPacketTagAggregate = @import("results_generated.zig").SessionPacketTagAggregate;
    pub const SessionPacketTagRow = @import("results_generated.zig").SessionPacketTagRow;
    pub const SessionPacketTerminal = @import("results_generated.zig").SessionPacketTerminal;
    pub const SessionProbabilities = @import("results_generated.zig").SessionProbabilities;
    pub const SessionProbabilitiesNamed = @import("results_generated.zig").SessionProbabilitiesNamed;
    pub const SessionProbabilitiesYesNo = @import("results_generated.zig").SessionProbabilitiesYesNo;
    pub const SessionQuestionDetail = @import("results_generated.zig").SessionQuestionDetail;
    pub const SessionRecognition = @import("results_generated.zig").SessionRecognition;
    pub const SessionRelationEdge = @import("results_generated.zig").SessionRelationEdge;
    pub const SettledAnswer = @import("results_generated.zig").SettledAnswer;
    pub const SettledAnswerAnnotate = @import("results_generated.zig").SettledAnswerAnnotate;
    pub const SettledAnswerChoose = @import("results_generated.zig").SettledAnswerChoose;
    pub const SettledAnswerDecide = @import("results_generated.zig").SettledAnswerDecide;
    pub const SettledAnswerFilter = @import("results_generated.zig").SettledAnswerFilter;
    pub const SettledAnswerFind = @import("results_generated.zig").SettledAnswerFind;
    pub const SettledAnswerRank = @import("results_generated.zig").SettledAnswerRank;
    pub const SettledAnswerRecognize = @import("results_generated.zig").SettledAnswerRecognize;
    pub const SettledAnswerRelate = @import("results_generated.zig").SettledAnswerRelate;
    pub const SettledAnswerScore = @import("results_generated.zig").SettledAnswerScore;
    pub const SettledAnswerTag = @import("results_generated.zig").SettledAnswerTag;
    pub const SettledJudgment = @import("results_generated.zig").SettledJudgment;
    pub const SettledOriginal = @import("results_generated.zig").SettledOriginal;
    pub const SettledRecognition = @import("results_generated.zig").SettledRecognition;
    pub const SettledRecognitionEntities = @import("results_generated.zig").SettledRecognitionEntities;
    pub const SettledRecognitionModeProposals = @import("results_generated.zig").SettledRecognitionModeProposals;
    pub const SettledRelations = @import("results_generated.zig").SettledRelations;
    pub const StopCause = @import("results_generated.zig").StopCause;
    pub const Stopped = @import("results_generated.zig").Stopped;
    pub const StringRoot = @import("results_generated.zig").StringRoot;
    pub const StringType = @import("results_generated.zig").StringType;
    pub const Threshold = @import("results_generated.zig").Threshold;
    pub const TokenUsage = @import("results_generated.zig").TokenUsage;
    pub const Usage = @import("results_generated.zig").Usage;
    pub const UsagePersistence = @import("results_generated.zig").UsagePersistence;
    pub const Value = @import("results_generated.zig").Value;
    pub const Verb = @import("results_generated.zig").Verb;
    pub const Version = @import("results_generated.zig").Version;
    pub const WordingVersion = @import("results_generated.zig").WordingVersion;
};

pub const AnswerView = struct {
    packet: *const SessionPacket,
    allocator: std.mem.Allocator,
    fn row(self: @This()) !SessionObservedRow {
        return switch (self.packet.*) {
            .observation => |v| switch (v.value) {
                .row => |r| r.value,
                else => error.WrongAnswer,
            },
            else => error.WrongAnswer,
        };
    }
    fn judgment(self: @This()) !SessionJudgment {
        return switch (try self.row()) {
            .judgment => |v| v.value,
            else => error.WrongAnswer,
        };
    }
    pub fn decide(self: @This()) !?bool {
        return switch (try self.judgment()) {
            .decision => |v| v.value.optional(),
            else => error.WrongAnswer,
        };
    }
    pub fn choose(self: @This()) !?[]const u8 {
        return switch (try self.judgment()) {
            .choice => |v| v.value.optional(),
            else => error.WrongAnswer,
        };
    }
    pub fn score(self: @This()) !f64 {
        return switch (try self.judgment()) {
            .score => |v| v.value,
            else => error.WrongAnswer,
        };
    }
    pub fn tag(self: @This()) ![]const []const u8 {
        return switch (try self.judgment()) {
            .tags => |v| v.value,
            else => error.WrongAnswer,
        };
    }
    pub fn find(self: @This()) !?u64 {
        return switch (try self.row()) {
            .find => |v| v.value.optional(),
            else => error.WrongAnswer,
        };
    }
    pub fn annotate(self: @This()) ![]const SessionAnnotation {
        return switch (try self.row()) {
            .annotated => |v| v.value,
            else => error.WrongAnswer,
        };
    }
    pub fn recognize(self: @This()) !SessionRecognition {
        return switch (try self.row()) {
            .recognized => |v| v.value,
            else => error.WrongAnswer,
        };
    }
    pub fn relate(self: @This()) ![]const SessionRelationEdge {
        return switch (try self.row()) {
            .relations => |v| v.value,
            else => error.WrongAnswer,
        };
    }
    pub fn filter(self: @This()) ![]const Value {
        const receipts: []const AtomicBoolean = switch (self.packet.*) {
            .filter_row => |v| &.{v.value},
            .filter_aggregate => |v| v.value,
            else => return error.WrongAnswer,
        };
        const answer = try self.allocator.alloc(Value, receipts.len);
        for (answer, receipts) |*out, receipt| out.* = switch (receipt.input) {
            .value => |v| v,
            .null_value => .null_value,
            .missing => return error.WrongAnswer,
        };
        return answer;
    }
    pub fn rank(self: @This()) ![]const u64 {
        const receipts: []const AtomicNonZeroUsize = switch (self.packet.*) {
            .rank_aggregate => |v| v.value,
            else => return error.WrongAnswer,
        };
        const answer = try self.allocator.alloc(u64, receipts.len);
        for (answer, receipts) |*out, receipt| out.* = switch (receipt.index) {
            .value => |v| v,
            .null_value => return error.WrongAnswer,
            .missing => return error.WrongAnswer,
        };
        return answer;
    }
};

pub fn callReceiptType(comptime function: anytype) type {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) return AtomicDecideValue;
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) return AtomicNullableString;
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) return AtomicArrayOfString;
    if (comptime std.mem.eql(u8, @tagName(function), "score")) return AtomicDouble;
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) return AtomicBoolean;
    if (comptime std.mem.eql(u8, @tagName(function), "rank")) return AtomicNonZeroUsize;
    if (comptime std.mem.eql(u8, @tagName(function), "find")) return Find;
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) return Annotation;
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) return Recognition;
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) return Relation;
    @compileError("unknown named function");
}
pub fn callReceipts(comptime function: anytype, packet: *const SessionPacket, a: std.mem.Allocator) !?[]const callReceiptType(function) {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) {
        if (packet.* != .decide_aggregate) return null;
        return packet.decide_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) {
        if (packet.* != .choose_aggregate) return null;
        return packet.choose_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) {
        if (packet.* != .tag_aggregate) return null;
        return packet.tag_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "score")) {
        if (packet.* != .score_aggregate) return null;
        return packet.score_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) {
        if (packet.* != .filter_aggregate) return null;
        return packet.filter_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "rank")) {
        if (packet.* != .rank_aggregate) return null;
        return packet.rank_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "find")) {
        if (packet.* != .find_aggregate) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.find_aggregate.value;
        return result;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) {
        if (packet.* != .annotate_aggregate) return null;
        return packet.annotate_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) {
        if (packet.* != .recognize_aggregate) return null;
        return packet.recognize_aggregate.value;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) {
        if (packet.* != .relate_aggregate) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.relate_aggregate.value;
        return result;
    }
    unreachable;
}
pub fn callValueType(comptime function: anytype) type {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) return ?@typeInfo(@TypeOf(@as(@TypeOf(@field(@as(callReceiptType(function), undefined), "value")), undefined).optional())).optional.child;
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) return ?@typeInfo(@TypeOf(@as(@TypeOf(@field(@as(callReceiptType(function), undefined), "value")), undefined).optional())).optional.child;
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) return @TypeOf(@field(@as(callReceiptType(function), undefined), "value"));
    if (comptime std.mem.eql(u8, @tagName(function), "score")) return @TypeOf(@field(@as(callReceiptType(function), undefined), "value"));
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) return @typeInfo(@TypeOf(@as(@TypeOf(@field(@as(callReceiptType(function), undefined), "input")), undefined).optional())).optional.child;
    if (comptime std.mem.eql(u8, @tagName(function), "rank")) return @typeInfo(@TypeOf(@as(@TypeOf(@field(@as(callReceiptType(function), undefined), "index")), undefined).optional())).optional.child;
    if (comptime std.mem.eql(u8, @tagName(function), "find")) return ?@typeInfo(@TypeOf(@as(@TypeOf(@field(@as(callReceiptType(function), undefined), "value")), undefined).optional())).optional.child;
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) return @TypeOf(@field(@as(callReceiptType(function), undefined), "value"));
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) return @TypeOf(@field(@as(callReceiptType(function), undefined), "value"));
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) return @TypeOf(@field(@as(callReceiptType(function), undefined), "value"));
    @compileError("unknown named function");
}
pub fn callValue(comptime function: anytype, receipt: callReceiptType(function)) !callValueType(function) {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) return receipt.value.optional();
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) return receipt.value.optional();
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) return receipt.value;
    if (comptime std.mem.eql(u8, @tagName(function), "score")) return receipt.value;
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) return switch (receipt.input) {
        .value => |v| v,
        .null_value => .null_value,
        .missing => return error.MissingAnswer,
    };
    if (comptime std.mem.eql(u8, @tagName(function), "rank")) return switch (receipt.index) {
        .value => |v| v,
        .null_value => return error.MissingAnswer,
        .missing => return error.MissingAnswer,
    };
    if (comptime std.mem.eql(u8, @tagName(function), "find")) return receipt.value.optional();
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) return receipt.value;
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) return receipt.value;
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) return receipt.value;
    unreachable;
}
pub fn callProbabilityType(comptime function: anytype) type {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "score")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "rank")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "find")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) return ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) return []const ?f64;
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) return []const ?f64;
    @compileError("unknown named function");
}
pub fn callProbability(comptime function: anytype, receipt: callReceiptType(function), ordinal: usize, packets: []const *const SessionPacket, a: std.mem.Allocator) !callProbabilityType(function) {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) {
        const index = receipt.index.optional() orelse ordinal;
        var resolved = false;
        for (packets) |packet| {
            if (packet.* != .observation or packet.observation.value != .row) continue;
            const row = packet.observation.value.row;
            if (row.index != index or row.value != .judgment) continue;
            if (row.value.judgment.value != .decision) continue;
            resolved = row.value.judgment.value.decision.value == .value;
        }
        if (!resolved) return null;
        if (receipt.answer != .yes_no) return error.WrongAnswer;
        return receipt.answer.yes_no.probability;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) {
        const index = receipt.index.optional() orelse ordinal;
        var resolved = false;
        for (packets) |packet| {
            if (packet.* != .observation or packet.observation.value != .row) continue;
            const row = packet.observation.value.row;
            if (row.index != index or row.value != .judgment) continue;
            if (row.value.judgment.value != .choice) continue;
            resolved = row.value.judgment.value.choice.value == .value;
        }
        if (!resolved) return null;
        if (receipt.answer != .choice) return error.WrongAnswer;
        for (receipt.answer.choice.probabilities.entries) |entry| {
            if (std.mem.eql(u8, entry.key, receipt.answer.choice.pick)) return entry.value;
        }
        return null;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) {
        return null;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "score")) {
        return null;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) {
        if (receipt.answer != .yes_no) return error.WrongAnswer;
        return receipt.answer.yes_no.probability;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "rank")) {
        if (receipt.answer != .yes_no) return null;
        return receipt.answer.yes_no.probability;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "find")) {
        if (receipt.index != .value) return null;
        for (receipt.answer.probabilities.entries) |entry| {
            if (std.mem.eql(u8, entry.key, receipt.answer.pick)) return entry.value;
        }
        return null;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) {
        return null;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) {
        switch (receipt.value) {
            .mode_proposals => |value| {
                const result = try a.alloc(?f64, value.proposals.len);
                for (result, value.proposals) |*out, proposal| out.* = proposal.probability;
                return result;
            },
            .entities => |value| {
                if (receipt.answer != .recognize_answer) return error.WrongAnswer;
                const result = try a.alloc(?f64, value.entities.len);
                for (result, value.entities) |*out, entity| {
                    out.* = null;
                    for (receipt.answer.recognize_answer.proposals) |proposal| {
                        if (!proposal.kept) continue;
                        const selected = proposal.selected.optional() orelse continue;
                        const kind = proposal.kind.optional() orelse continue;
                        if (selected.start == entity.start and selected.end == entity.end and std.mem.eql(u8, kind, entity.kind)) {
                            out.* = proposal.span_probability;
                            break;
                        }
                    }
                }
                return result;
            },
        }
    }
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) {
        const result = try a.alloc(?f64, receipt.value.len);
        for (result, receipt.value) |*out, edge| out.* = edge.probability;
        return result;
    }
    unreachable;
}
pub fn callRowReceipts(comptime function: anytype, packet: *const SessionPacket, a: std.mem.Allocator) !?[]const callReceiptType(function) {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) {
        if (packet.* != .decide_row) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.decide_row.value;
        return result;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) {
        if (packet.* != .choose_row) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.choose_row.value;
        return result;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) {
        if (packet.* != .tag_row) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.tag_row.value;
        return result;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "score")) {
        if (packet.* != .score_row) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.score_row.value;
        return result;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) {
        if (packet.* != .filter_row) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.filter_row.value;
        return result;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) {
        if (packet.* != .annotate_row) return null;
        const result = try a.alloc(callReceiptType(function), 1);
        result[0] = packet.annotate_row.value;
        return result;
    }
    return null;
}

pub fn callList(comptime function: anytype, comptime Input: type) bool {
    if (comptime std.mem.eql(u8, @tagName(function), "decide")) return @import("calls_generated.zig").listInput(Input);
    if (comptime std.mem.eql(u8, @tagName(function), "choose")) return @import("calls_generated.zig").listInput(Input);
    if (comptime std.mem.eql(u8, @tagName(function), "tag")) return @import("calls_generated.zig").listInput(Input);
    if (comptime std.mem.eql(u8, @tagName(function), "score")) return @import("calls_generated.zig").listInput(Input);
    if (comptime std.mem.eql(u8, @tagName(function), "filter")) return true;
    if (comptime std.mem.eql(u8, @tagName(function), "rank")) return true;
    if (comptime std.mem.eql(u8, @tagName(function), "find")) return false;
    if (comptime std.mem.eql(u8, @tagName(function), "annotate")) return @import("calls_generated.zig").listInput(Input);
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) return @import("calls_generated.zig").listInput(Input);
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) return false;
    @compileError("unknown named function");
}
pub fn NamedCallResult(comptime function: anytype, comptime Input: type) type {
    const V = callPublicValueType(function);
    const P = callProbabilityType(function);
    return @import("blocking.zig").CallResult(if (callList(function, Input)) []const V else V, if (callList(function, Input)) []const P else P, callReceiptType(function));
}

pub fn callPublicValueType(comptime function: anytype) type {
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) return SessionRecognition;
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) return []const SessionRelationEdge;
    return callValueType(function);
}
pub fn callPublicValue(comptime function: anytype, receipt: callReceiptType(function), ordinal: usize, packets: []const *const SessionPacket) !callPublicValueType(function) {
    if (comptime std.mem.eql(u8, @tagName(function), "recognize")) {
        const index = receipt.index.optional() orelse ordinal;
        for (packets) |packet| {
            if (packet.* != .observation or packet.observation.value != .row) continue;
            const row = packet.observation.value.row;
            if (row.index == index and row.value == .recognized) return row.value.recognized.value;
        }
        return error.MissingAnswer;
    }
    if (comptime std.mem.eql(u8, @tagName(function), "relate")) {
        const index = receipt.index.optional() orelse ordinal;
        for (packets) |packet| {
            if (packet.* != .observation or packet.observation.value != .row) continue;
            const row = packet.observation.value.row;
            if (row.index == index and row.value == .relations) return row.value.relations.value;
        }
        return error.MissingAnswer;
    }
    return callValue(function, receipt);
}
