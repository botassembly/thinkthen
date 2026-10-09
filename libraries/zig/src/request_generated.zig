// Generated from the canonical Rust schema; do not edit.
const std = @import("std");
/// Ordered authored maps preserve member ordering during transport.
pub fn Map(comptime T: type) type {
    return struct {
        entries: []const struct { key: []const u8, value: T },
        pub fn jsonStringify(self: @This(), writer: anytype) !void {
            try writer.beginObject();
            for (self.entries) |entry| {
                try writer.objectField(entry.key);
                try writer.write(entry.value);
            }
            try writer.endObject();
        }
    };
}

pub const AuthoredChoose = struct {
    batch: ?AuthoredChooseBatch = null,

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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .object => |value| try writer.write(value),

            .array => |value| try writer.write(value),

            .null => try writer.write(null),
        }
    }
};

pub const AuthoredCut = union(enum) {
    number: f64,

    string: []const u8,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .number => |value| try writer.write(value),

            .string => |value| try writer.write(value),
        }
    }
};

pub const AuthoredDecide = struct {
    batch: ?AuthoredDecideBatch = null,

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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .object => |value| try writer.write(value),
        }
    }
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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .number => |value| try writer.write(value),

            .boolean => |value| try writer.write(value),

            .array => |value| try writer.write(value),
        }
    }
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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .array => |value| try writer.write(value),

            .object => |value| try writer.write(value),
        }
    }
};

pub const AuthoredLevels = union(enum) {
    array: []const AuthoredName,

    object: Map(AuthoredCriterion),

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .array => |value| try writer.write(value),

            .object => |value| try writer.write(value),
        }
    }
};

pub const AuthoredName = []const u8;

pub const AuthoredOptions = union(enum) {
    array: []const AuthoredName,

    object: Map(AuthoredDescription),

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .array => |value| try writer.write(value),

            .object => |value| try writer.write(value),
        }
    }
};

pub const AuthoredPointers = union(enum) {
    string: []const u8,

    array: []const []const u8,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .array => |value| try writer.write(value),
        }
    }
};

pub const AuthoredProfile = []const u8;

pub const AuthoredQuestionText = union(enum) {
    string: []const u8,

    object: std.json.Value,

    array: []const std.json.Value,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .object => |value| try writer.write(value),

            .array => |value| try writer.write(value),
        }
    }
};

pub const AuthoredRelate = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    profile: ?AuthoredProfile = null,

    relate: AuthoredRelateRelate,

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
    batch: ?AuthoredScoreBatch = null,

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
    batch: ?AuthoredTagBatch = null,

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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .number => |value| try writer.write(value),

            .string => |value| try writer.write(value),
        }
    }
};

pub const ContextSchema = union(enum) {
    string: []const u8,

    object: std.json.Value,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .object => |value| try writer.write(value),
        }
    }
};

pub const ImageMedia = enum { @"image/jpeg", @"image/png" };

pub const OptionSchema = struct {
    description: ?std.json.Value = null,

    name: []const u8,
};

pub const ReaderMedia = enum { text, image };

pub const RecognitionExample = union(enum) {
    string: []const u8,

    alternative1: RecognitionExampleText,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .alternative1 => |value| try writer.write(value),
        }
    }
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

pub const Request = struct {
    call: RequestCall,

    schema: RequestVersion = .@"thinkthen.request/1",
};

pub const RequestBatch = union(enum) {
    integer: u64,

    string: []const u8,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .integer => |value| try writer.write(value),

            .string => |value| try writer.write(value),
        }
    }
};

pub const RequestCall = union(enum) {
    decide: RequestCallDecide,

    choose: RequestCallChoose,

    tag: RequestCallTag,

    score: RequestCallScore,

    filter: RequestCallFilter,

    rank: RequestCallRank,

    find: RequestCallFind,

    annotate: RequestCallAnnotate,

    recognize: RequestCallRecognize,

    relate: RequestCallRelate,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .decide => |value| try writer.write(value),

            .choose => |value| try writer.write(value),

            .tag => |value| try writer.write(value),

            .score => |value| try writer.write(value),

            .filter => |value| try writer.write(value),

            .rank => |value| try writer.write(value),

            .find => |value| try writer.write(value),

            .annotate => |value| try writer.write(value),

            .recognize => |value| try writer.write(value),

            .relate => |value| try writer.write(value),
        }
    }
};

pub const RequestCallAnnotate = struct {
    function: enum { annotate } = .annotate,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallChoose = struct {
    function: enum { choose } = .choose,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallDecide = struct {
    function: enum { decide } = .decide,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallFilter = struct {
    function: enum { filter } = .filter,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallFind = struct {
    function: enum { find } = .find,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallRank = struct {
    function: enum { rank } = .rank,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallRecognize = struct {
    function: enum { recognize } = .recognize,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallRelate = struct {
    function: enum { relate } = .relate,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallScore = struct {
    function: enum { score } = .score,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestCallTag = struct {
    function: enum { tag } = .tag,

    input: RequestInput,

    options: ?RequestOptions = null,

    question: RequestQuestion,
};

pub const RequestDefinition = union(enum) {
    decide: RequestDefinitionFieldsDecide,

    choose: RequestDefinitionFieldsChoose,

    tag: RequestDefinitionFieldsTag,

    score: RequestDefinitionFieldsScore,

    relate_version: RequestDefinitionFieldsRelateVersion,

    find: RequestDefinitionFieldsFind,

    recognize_version: RequestDefinitionFieldsRecognizeVersion,

    questions_version: RequestDefinitionFieldsQuestionsVersion,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .decide => |value| try writer.write(value),

            .choose => |value| try writer.write(value),

            .tag => |value| try writer.write(value),

            .score => |value| try writer.write(value),

            .relate_version => |value| try writer.write(value),

            .find => |value| try writer.write(value),

            .recognize_version => |value| try writer.write(value),

            .questions_version => |value| try writer.write(value),
        }
    }
};

pub const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties = union(enum) {
    decide: RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide,

    choose: RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose,

    tag: RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag,

    score: RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .decide => |value| try writer.write(value),

            .choose => |value| try writer.write(value),

            .tag => |value| try writer.write(value),

            .score => |value| try writer.write(value),
        }
    }
};

pub const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose = struct {
    choose: AuthoredQuestionText,

    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    options: ?AuthoredOptions = null,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide = struct {
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

pub const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    levels: ?AuthoredLevels = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    score: AuthoredQuestionText,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    labels: ?AuthoredLabels = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    tag: AuthoredQuestionText,

    threshold: ?AuthoredCut = null,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionFieldsChoose = struct {
    batch: ?RequestDefinitionFieldsChooseBatch = null,

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

pub const RequestDefinitionFieldsDecide = struct {
    batch: ?RequestDefinitionFieldsDecideBatch = null,

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

pub const RequestDefinitionFieldsFind = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    find: AuthoredQuestionText,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionFieldsQuestionsVersion = struct {
    batch: ?std.json.Value = null,

    profile: ?AuthoredProfile = null,

    questions: Map(RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties),

    threshold: ?AuthoredThreshold = null,

    version: i64 = 1,
};

pub const RequestDefinitionFieldsRecognizeVersion = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    on: ?AuthoredPointers = null,

    profile: ?AuthoredProfile = null,

    recognize: RequestDefinitionFieldsRecognizeVersionRecognize,

    relation_threshold: ?AuthoredCut = null,

    threshold: ?AuthoredCut = null,

    version: i64 = 1,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionFieldsRelateVersion = struct {
    context_schema: ?AuthoredInputDeclaration = null,

    item_schema: ?AuthoredInputDeclaration = null,

    model: ?AuthoredName = null,

    name: ?[]const u8 = null,

    profile: ?AuthoredProfile = null,

    relate: RequestDefinitionFieldsRelateVersionRelate,

    threshold: ?AuthoredCut = null,

    version: i64 = 1,

    wording_version: ?i64 = null,
};

pub const RequestDefinitionFieldsScore = struct {
    batch: ?RequestDefinitionFieldsScoreBatch = null,

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

pub const RequestDefinitionFieldsTag = struct {
    batch: ?RequestDefinitionFieldsTagBatch = null,

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

pub const RequestFraming = enum { document, lines, jsonl, csv, tsv };

pub const RequestImage = union(enum) {
    file: RequestImageFile,

    bytes: RequestImageBytes,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .file => |value| try writer.write(value),

            .bytes => |value| try writer.write(value),
        }
    }
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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .text => |value| try writer.write(value),

            .json => |value| try writer.write(value),

            .records => |value| try writer.write(value),

            .units => |value| try writer.write(value),

            .entities => |value| try writer.write(value),

            .source => |value| try writer.write(value),

            .feed => |value| try writer.write(value),
        }
    }
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

    details: ?bool = null,

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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .text => |value| try writer.write(value),

            .json => |value| try writer.write(value),
        }
    }
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

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .text => |value| try writer.write(value),

            .definition => |value| try writer.write(value),

            .file => |value| try writer.write(value),

            .name => |value| try writer.write(value),

            .reference => |value| try writer.write(value),
        }
    }
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
    media: ?ReaderMedia = null,

    paths: []const []const u8,

    reading: ?RequestReader = null,
};

pub const RequestThreshold = union(enum) {
    number: f64,

    string: []const u8,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .number => |value| try writer.write(value),

            .string => |value| try writer.write(value),
        }
    }
};

pub const RequestVersion = enum { @"thinkthen.request/1" };

pub const SessionSourceLocation = struct {
    file: []const u8,

    first_line: ?u64 = null,

    last_line: ?u64 = null,
};

pub const SourceUnit = enum { line, window, file };

pub const AuthoredChooseBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const AuthoredDecideBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const AuthoredDescriptionValue = union(enum) {
    string: []const u8,

    object: std.json.Value,

    array: []const std.json.Value,

    null: void,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .string => |value| try writer.write(value),

            .object => |value| try writer.write(value),

            .array => |value| try writer.write(value),

            .null => try writer.write(null),
        }
    }
};

pub const AuthoredInputPropertyArrayItems = struct {
    type: enum { string } = .string,
};

pub const AuthoredRelateRelate = struct {
    fields: ?AuthoredRelateRelateFields = null,

    relations: []const AuthoredRelation,
};

pub const AuthoredScoreBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const AuthoredTagBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const RequestDefinitionFieldsChooseBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const RequestDefinitionFieldsDecideBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const RequestDefinitionFieldsRecognizeVersionRecognize = struct {
    entity_definition: ?AuthoredQuestionText = null,

    instructions: ?AuthoredQuestionText = null,

    kinds: ?Map(AuthoredDescription) = null,

    mode: ?RecognitionMode = null,

    relations: ?[]const AuthoredRelation = null,

    snippet_pieces: ?u32 = null,

    stage_context: ?RecognitionStageContext = null,
};

pub const RequestDefinitionFieldsRelateVersionRelate = struct {
    fields: ?RequestDefinitionFieldsRelateVersionRelateFields = null,

    relations: []const AuthoredRelation,
};

pub const RequestDefinitionFieldsScoreBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const RequestDefinitionFieldsTagBatch = union(enum) {
    max: enum { max },

    integer: i64,

    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        switch (self) {
            .max => |value| try writer.write(value),

            .integer => |value| try writer.write(value),
        }
    }
};

pub const AuthoredRelateRelateFields = struct {
    kind: []const u8,

    name: []const u8,
};

pub const RequestDefinitionFieldsRelateVersionRelateFields = struct {
    kind: []const u8,

    name: []const u8,
};
