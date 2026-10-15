// Generated from the canonical Rust schema; do not edit.
const std = @import("std");
/// Ordered authored maps retain caller member ordering.
pub fn Map(comptime T: type) type {
    return struct {
        entries: []const struct { key: []const u8, value: T },
    };
}

pub const AnnotateDefinition = struct {
    batch: ?std.json.Value = null,

    profile: ?AuthoredProfile = null,

    questions: Map(AnnotateQuestion),

    threshold: ?AuthoredThreshold = null,

    version: i64 = 1,
};

pub const AnnotateQuestion = union(enum) {
    decide: AnnotateQuestionDecide,

    choose: AnnotateQuestionChoose,

    tag: AnnotateQuestionTag,

    score: AnnotateQuestionScore,
};

pub const AnnotateQuestionChoose = struct {
    choose: AuthoredQuestionText,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    options: ?AuthoredOptions = null,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const AnnotateQuestionDecide = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    decide: AuthoredQuestionText,

    false: ?AuthoredCriterion = null,

    item_schema: ?AuthoredInputDeclaration = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    threshold: ?AuthoredThreshold = null,

    true: ?AuthoredCriterion = null,

    wording_version: ?i64 = null,
};

pub const AnnotateQuestionScore = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    levels: ?AuthoredLevels = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    score: AuthoredQuestionText,

    wording_version: ?i64 = null,
};

pub const AnnotateQuestionTag = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    labels: ?AuthoredLabels = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    tag: AuthoredQuestionText,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const AuthoredChoose = struct {
    batch: ?AuthoredQuestionBatch = null,

    choose: AuthoredQuestionText,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    options: ?AuthoredOptions = null,

    profile: ?AuthoredProfile = null,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const AuthoredCriterion = union(enum) {
    string: []const u8,

    object: std.json.Value,

    array: []const std.json.Value,

    null: void,
};

pub const AuthoredCut = union(enum) {
    number: f64,

    string: []const u8,
};

pub const AuthoredDecide = struct {
    batch: ?AuthoredQuestionBatch = null,

    context_schema: ?AuthoredInputDeclaration = null,

    decide: AuthoredQuestionText,

    false: ?AuthoredCriterion = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    threshold: ?AuthoredThreshold = null,

    true: ?AuthoredCriterion = null,

    wording_version: ?i64 = null,
};

pub const AuthoredDescription = AuthoredDescriptionValue;

pub const AuthoredFind = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    find: AuthoredQuestionText,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    wording_version: ?i64 = null,
};

pub const AuthoredInputDeclaration = union(enum) {
    string: AuthoredInputDeclarationString,

    object: AuthoredInputDeclarationObject,
};

pub const AuthoredInputDeclarationObject = struct {
    properties: Map(AuthoredInputProperty),

    required: ?[]const []const u8 = null,

    type: enum { object } = .object,
};

pub const AuthoredInputDeclarationString = struct {
    type: enum { string } = .string,
};

pub const AuthoredInputProperty = union(enum) {
    string: AuthoredInputPropertyString,

    number: AuthoredInputPropertyNumber,

    boolean: AuthoredInputPropertyBoolean,

    array: AuthoredInputPropertyArray,
};

pub const AuthoredInputPropertyArray = struct {
    items: AuthoredInputPropertyArrayItems,

    type: enum { array } = .array,
};

pub const AuthoredInputPropertyBoolean = struct {
    type: enum { boolean } = .boolean,
};

pub const AuthoredInputPropertyNumber = struct {
    type: enum { number } = .number,
};

pub const AuthoredInputPropertyString = struct {
    type: enum { string } = .string,
};

pub const AuthoredLabels = union(enum) {
    array: []const AuthoredName,

    object: Map(AuthoredDescription),
};

pub const AuthoredLevels = union(enum) {
    array: []const AuthoredName,

    object: Map(AuthoredCriterion),
};

pub const AuthoredName = []const u8;

pub const AuthoredOptions = union(enum) {
    array: []const AuthoredName,

    object: Map(AuthoredDescription),
};

pub const AuthoredPointers = union(enum) {
    string: []const u8,

    array: []const []const u8,
};

pub const AuthoredProfile = []const u8;

pub const AuthoredQuestionBatch = union(enum) {
    max: enum { max },

    integer: i64,
};

pub const AuthoredQuestionText = union(enum) {
    string: []const u8,

    object: std.json.Value,

    array: []const std.json.Value,
};

pub const AuthoredRelate = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    profile: ?AuthoredProfile = null,

    relate: RelateQuestion,

    threshold: ?AuthoredCut = null,

    version: i64 = 1,

    wording_version: ?i64 = null,
};

pub const AuthoredRelation = struct {
    either: ?bool = null,

    name: AuthoredName,

    reads: ?AuthoredName = null,

    single: ?bool = null,

    source: ?AuthoredName = null,

    target: ?AuthoredName = null,
};

pub const AuthoredScore = struct {
    batch: ?AuthoredQuestionBatch = null,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    levels: ?AuthoredLevels = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    score: AuthoredQuestionText,

    wording_version: ?i64 = null,
};

pub const AuthoredTag = struct {
    batch: ?AuthoredQuestionBatch = null,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    labels: ?AuthoredLabels = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    tag: AuthoredQuestionText,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const AuthoredThreshold = union(enum) {
    number: f64,

    string: []const u8,
};

pub const CacheDocument = union(enum) {
    string: []const u8,

    disabled_cache: DisabledCache,
};

pub const ContextSchema = union(enum) {
    string: []const u8,

    object: std.json.Value,
};

pub const DisabledCache = bool;

pub const EngineSettings = struct {
    backend: ?[]const u8 = null,

    base_url: ?[]const u8 = null,

    batch: ?RequestBatch = null,

    cache: ?CacheDocument = null,

    max_estimated_input_tokens_total: ??u64 = null,

    max_request_bytes: ?u64 = null,

    max_requests: ??u64 = null,

    max_requests_total: ??u64 = null,

    max_retries: ?u32 = null,

    model: ?[]const u8 = null,

    profile: ?[]const u8 = null,

    proxy: ?std.json.Value = null,

    record: ?[]const u8 = null,

    refresh_cache: ?bool = null,

    replay: ?[]const u8 = null,

    throttle: ?i64 = null,

    timeout: ?u64 = null,

    usd_per_million_input: ?[]const u8 = null,

    usd_per_million_output: ?[]const u8 = null,
};

pub const ImageMedia = enum { @"image/jpeg", @"image/png" };

pub const OptionSchema = struct {
    description: ?std.json.Value = null,

    name: []const u8,
};

pub const ReaderMedia = enum { text, image };

pub const RecognitionExample = union(enum) {
    string: []const u8,

    recognition_example_text: RecognitionExampleText,
};

pub const RecognitionExampleEntity = struct {
    end: u64,

    kind: []const u8,

    start: u64,
};

pub const RecognitionExampleText = struct {
    entities: []const RecognitionExampleEntity,

    kinds: ?[]const []const u8 = null,

    text: []const u8,
};

pub const RecognitionMode = enum { whole, boundary_only };

pub const RecognitionSeedSpan = struct {
    end: u64,

    kind: ?[]const u8 = null,

    start: u64,
};

pub const RecognitionStageContext = struct {
    boundary: ?[]const u8 = null,

    kind_edge: ?[]const u8 = null,

    relation: ?[]const u8 = null,
};

pub const RecognizeDefinition = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    recognize: RecognizeQuestion,

    relation_threshold: ?AuthoredCut = null,

    threshold: ?AuthoredCut = null,

    version: i64 = 1,

    wording_version: ?i64 = null,
};

pub const RecognizeQuestion = struct {
    entity_definition: ?AuthoredQuestionText = null,

    instructions: ?AuthoredQuestionText = null,

    kinds: ?Map(AuthoredDescription) = null,

    mode: ?RecognitionMode = null,

    relations: ?[]const AuthoredRelation = null,

    snippet_pieces: ?u32 = null,

    stage_context: ?RecognitionStageContext = null,
};

pub const RelateFields = struct {
    kind: []const u8,

    name: []const u8,
};

pub const RelateQuestion = struct {
    fields: ?RelateFields = null,

    relations: []const AuthoredRelation,
};

pub const Request = struct {
    call: RequestCall,

    schema: RequestVersion = .@"thinkthen.request/1",
};

pub const RequestBatch = union(enum) {
    integer: u64,

    string: []const u8,
};

pub const RequestCall = struct {
    function: RequestFunction,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestDefinition = union(enum) {
    Authored_decide: RequestDefinitionAuthoredDecide,

    Authored_choose: RequestDefinitionAuthoredChoose,

    Authored_tag: RequestDefinitionAuthoredTag,

    Authored_score: RequestDefinitionAuthoredScore,

    Authored_relate: RequestDefinitionAuthoredRelate,

    Authored_find: RequestDefinitionAuthoredFind,

    RecognizeDefinition: RequestDefinitionRecognizeDefinition,

    AnnotateDefinition: RequestDefinitionAnnotateDefinition,
};

pub const RequestDefinitionAnnotateDefinition = struct {
    batch: ?std.json.Value = null,

    profile: ?AuthoredProfile = null,

    questions: Map(AnnotateQuestion),

    threshold: ?AuthoredThreshold = null,

    version: i64 = 1,
};

pub const RequestDefinitionAuthoredChoose = struct {
    batch: ?AuthoredQuestionBatch = null,

    choose: AuthoredQuestionText,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    options: ?AuthoredOptions = null,

    profile: ?AuthoredProfile = null,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionAuthoredDecide = struct {
    batch: ?AuthoredQuestionBatch = null,

    context_schema: ?AuthoredInputDeclaration = null,

    decide: AuthoredQuestionText,

    false: ?AuthoredCriterion = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    threshold: ?AuthoredThreshold = null,

    true: ?AuthoredCriterion = null,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionAuthoredFind = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    find: AuthoredQuestionText,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionAuthoredRelate = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    profile: ?AuthoredProfile = null,

    relate: RelateQuestion,

    threshold: ?AuthoredCut = null,

    version: i64 = 1,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionAuthoredScore = struct {
    batch: ?AuthoredQuestionBatch = null,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    levels: ?AuthoredLevels = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    score: AuthoredQuestionText,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionAuthoredTag = struct {
    batch: ?AuthoredQuestionBatch = null,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    labels: ?AuthoredLabels = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    tag: AuthoredQuestionText,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionRecognizeDefinition = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    recognize: RecognizeQuestion,

    relation_threshold: ?AuthoredCut = null,

    threshold: ?AuthoredCut = null,

    version: i64 = 1,

    wording_version: ?i64 = null,
};

pub const RequestFraming = enum { document, lines, jsonl, csv, tsv };

pub const RequestFunction = enum { decide, choose, tag, score, filter, rank, find, annotate, recognize, relate };

pub const RequestImage = union(enum) {
    file: RequestImageFile,

    bytes: RequestImageBytes,
};

pub const RequestImageBytes = struct {
    bytes: []const u8,

    kind: enum { bytes } = .bytes,

    media: ImageMedia,
};

pub const RequestImageFile = struct {
    kind: enum { file } = .file,

    media: ?ImageMedia = null,

    path: []const u8,
};

pub const RequestInput = union(enum) {
    text: RequestInputText,

    json: RequestInputJson,

    records: RequestInputRecords,

    units: RequestInputUnits,

    entities: RequestInputEntities,

    source: RequestInputSource,

    feed: RequestInputFeed,
};

pub const RequestInputEntities = struct {
    items: []const RequestItem,

    kind: enum { entities } = .entities,
};

pub const RequestInputFeed = struct {
    framing: ?RequestFraming = null,

    images: ?[]const RequestImage = null,

    kind: enum { feed } = .feed,

    name: []const u8,

    reading: ?RequestReader = null,
};

pub const RequestInputJson = struct {
    images: ?[]const RequestImage = null,

    kind: enum { json } = .json,

    value: std.json.Value,
};

pub const RequestInputRecords = struct {
    items: []const RequestItem,

    kind: enum { records } = .records,
};

pub const RequestInputSource = struct {
    kind: enum { source } = .source,

    source: RequestSource,
};

pub const RequestInputText = struct {
    images: ?[]const RequestImage = null,

    kind: enum { text } = .text,

    text: []const u8,
};

pub const RequestInputUnits = struct {
    items: []const RequestItem,

    kind: enum { units } = .units,
};

pub const RequestItem = struct {
    context: ?ContextSchema = null,

    examples: ?[]const RecognitionExample = null,

    images: ?[]const RequestImage = null,

    options: ?[]const OptionSchema = null,

    original: ?RequestOriginal = null,

    seed_spans: ?[]const RecognitionSeedSpan = null,
};

pub const RequestOptions = struct {
    attempts: ?bool = null,

    batch: ?RequestBatch = null,

    context: ?[]const u8 = null,

    context_field: ?[]const u8 = null,

    deadline_ms: ?i64 = null,

    examples: ?[]const RecognitionExample = null,

    examples_field: ?[]const u8 = null,

    field: ?[]const []const u8 = null,

    files_only: ?bool = null,

    max_requests_total: ?u64 = null,

    mode: ?RecognitionMode = null,

    model: ?[]const u8 = null,

    none: ?bool = null,

    options_field: ?[]const u8 = null,

    relation_threshold: ?RequestThreshold = null,

    seed_spans: ?[]const RecognitionSeedSpan = null,

    seed_spans_field: ?[]const u8 = null,

    snippet_pieces: ?u32 = null,

    stage_context: ?RecognitionStageContext = null,

    threshold: ?RequestThreshold = null,

    top: ?u64 = null,
};

pub const RequestOriginal = union(enum) {
    text: RequestOriginalText,

    json: RequestOriginalJson,
};

pub const RequestOriginalJson = struct {
    kind: enum { json } = .json,

    value: std.json.Value,
};

pub const RequestOriginalText = struct {
    kind: enum { text } = .text,

    text: []const u8,
};

pub const RequestQuestion = union(enum) {
    text: RequestQuestionText,

    definition: RequestQuestionDefinition,

    file: RequestQuestionFile,

    name: RequestQuestionName,

    reference: RequestQuestionReference,
};

pub const RequestQuestionDefinition = struct {
    kind: enum { definition } = .definition,

    value: RequestDefinition,
};

pub const RequestQuestionFile = struct {
    kind: enum { file } = .file,

    path: []const u8,
};

pub const RequestQuestionName = struct {
    kind: enum { name } = .name,

    name: []const u8,
};

pub const RequestQuestionReference = struct {
    kind: enum { reference } = .reference,

    reference: []const u8,
};

pub const RequestQuestionText = struct {
    kind: enum { text } = .text,

    text: []const u8,
};

pub const RequestReader = struct {
    unit: ?SourceUnit = null,

    window: ?u64 = null,
};

pub const RequestSessionDescriptor = struct {
    item: RequestItem,

    location: ?SessionSourceLocation = null,
};

pub const RequestSource = struct {
    framing: ?RequestFraming = null,

    media: ?ReaderMedia = null,

    paths: []const []const u8,

    reading: ?RequestReader = null,
};

pub const RequestThreshold = union(enum) {
    number: f64,

    string: []const u8,
};

pub const RequestVersion = enum { @"thinkthen.request/1" };

pub const SessionSourceLocation = struct {
    file: []const u8,

    first_line: ?u64 = null,

    last_line: ?u64 = null,
};

pub const SourceUnit = enum { line, window, file };

pub const AuthoredDescriptionValue = union(enum) {
    string: []const u8,

    object: std.json.Value,

    array: []const std.json.Value,

    null: void,
};

pub const AuthoredInputPropertyArrayItems = struct {
    type: enum { string } = .string,
};
