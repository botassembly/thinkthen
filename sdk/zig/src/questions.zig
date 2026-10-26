const std = @import("std");
const t = @import("types.zig");
pub const Function = t.Function;
pub const Choice = t.Choice;
pub const RelationRule = t.RelationRule;
pub const Threshold = t.Threshold;
pub const DecideOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    threshold: ?Threshold = null,
};
pub const Decide = struct {
    question: []const u8,
    options: DecideOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("decide");
        try writer.objectField("question");
        try writer.write(self.question);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const ChooseOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    threshold: ?Threshold = null,
};
pub const Choose = struct {
    question: []const u8,
    choices: []const Choice,
    options: ChooseOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("choose");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("choices");
        try writer.write(self.choices);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const TagOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    threshold: ?f64 = null,
};
pub const Tag = struct {
    question: []const u8,
    labels: []const Choice,
    options: TagOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("tag");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("labels");
        try writer.write(self.labels);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const ScoreOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
};
pub const Score = struct {
    question: []const u8,
    levels: []const Choice,
    options: ScoreOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("score");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("levels");
        try writer.write(self.levels);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const FilterOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    threshold: ?Threshold = null,
};
pub const Filter = struct {
    question: []const u8,
    options: FilterOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("filter");
        try writer.objectField("question");
        try writer.write(self.question);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const RankOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    top: ?u32 = null,
};
pub const Rank = struct {
    question: []const u8,
    levels: []const Choice,
    options: RankOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("rank");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("levels");
        try writer.write(self.levels);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.top) |v| {
            try writer.objectField("top");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const FindOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    threshold: ?f64 = null,
};
pub const Find = struct {
    question: []const u8,
    options: FindOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("find");
        try writer.objectField("question");
        try writer.write(self.question);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const RecognizeOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    threshold: ?f64 = null,
};
pub const Recognize = struct {
    kinds: []const Choice,
    options: RecognizeOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("recognize");
        try writer.objectField("kinds");
        try writer.write(self.kinds);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const RelateOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
    threshold: ?f64 = null,
};
pub const Relate = struct {
    rules: []const RelationRule,
    options: RelateOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("relate");
        try writer.objectField("rules");
        try writer.write(self.rules);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub const AnnotateOptions = struct {
    name: ?[]const u8 = null,
    evidence: ?[]const u8 = null,
};
pub const Annotate = struct {
    questions: AnnotationQuestions,
    options: AnnotateOptions = .{},
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("annotate");
        try writer.objectField("questions");
        try writer.write(self.questions);
        if (self.options.evidence) |v| {
            try writer.objectField("evidence");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
pub fn QuestionType(comptime function: Function) type {
    return switch (function) {
        .decide => Decide,
        .choose => Choose,
        .tag => Tag,
        .score => Score,
        .filter => Filter,
        .rank => Rank,
        .find => Find,
        .recognize => Recognize,
        .relate => Relate,
        .annotate => Annotate,
    };
}
pub const EntryQuestion = union(enum) {
    decide: Decide,
    choose: Choose,
    tag: Tag,
    score: Score,
    filter: Filter,
    rank: Rank,
    find: Find,
    recognize: Recognize,
    relate: Relate,
    pub fn as(self: EntryQuestion, comptime function: Function) ?QuestionType(function) {
        if (comptime !@hasField(EntryQuestion, @tagName(function))) return null;
        return if (std.meta.activeTag(self) == @field(std.meta.Tag(EntryQuestion), @tagName(function))) @field(self, @tagName(function)) else null;
    }
};
pub const AnnotationQuestion = union(enum) { decide: Decide, choose: Choose, tag: Tag, score: Score, filter: Filter, find: Find, recognize: Recognize, relate: Relate };
pub const NamedQuestion = struct { name: []const u8, question: EntryQuestion };
pub const NamedAnnotationQuestion = struct { name: []const u8, question: AnnotationQuestion };
pub const AnnotationQuestions = struct {
    entries: []const NamedAnnotationQuestion,
    pub fn jsonStringify(self: AnnotationQuestions, writer: anytype) !void {
        try writer.beginObject();
        for (self.entries) |entry| {
            try writer.objectField(entry.name);
            switch (entry.question) {
                inline else => |q| try writer.write(q),
            }
        }
        try writer.endObject();
    }
};
