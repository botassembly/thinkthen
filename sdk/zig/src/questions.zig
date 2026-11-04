const std = @import("std");
const t = @import("types.zig");
pub const Function = t.Function;
pub const Choice = t.Choice;
pub const RelationRule = t.RelationRule;
pub const Threshold = t.Threshold;
/// Options for decide.
pub const DecideOptions = struct {
    /// What yes means for this question; null keeps the question's own wording.
    yes_means: ?[]const u8 = null,
    /// What no means for this question; null keeps the question's own wording.
    no_means: ?[]const u8 = null,
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// The answer threshold: a cut, or a band whose middle is not sure.
    threshold: ?Threshold = null,
};
/// A decide question.
pub const Decide = struct {
    /// The question text.
    question: []const u8,
    /// The options.
    options: DecideOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("decide");
        try writer.objectField("question");
        try writer.write(self.question);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        if (self.options.yes_means) |v| {
            try writer.objectField("true");
            try writer.write(v);
        }
        if (self.options.no_means) |v| {
            try writer.objectField("false");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for choose.
pub const ChooseOptions = struct {
    /// Descriptions by label or level. The engine refuses keys outside the question's choices.
    descriptions: ?std.json.ArrayHashMap([]const u8) = null,
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// The answer threshold: a cut, or a band whose middle is not sure.
    threshold: ?Threshold = null,
};
/// A choose question.
pub const Choose = struct {
    /// The question text.
    question: []const u8,
    /// The labels, one of which is chosen.
    choices: []const Choice,
    /// The options.
    options: ChooseOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("choose");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("choices");
        try writeOptions(writer, self.choices, self.options.descriptions);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for tag.
pub const TagOptions = struct {
    /// Descriptions by label or level. The engine refuses keys outside the question's choices.
    descriptions: ?std.json.ArrayHashMap([]const u8) = null,
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// The lowest probability that counts.
    threshold: ?f64 = null,
};
/// A tag question.
pub const Tag = struct {
    /// The question text.
    question: []const u8,
    /// The labels, any of which may fit.
    labels: []const Choice,
    /// The options.
    options: TagOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("tag");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("labels");
        try writeOptions(writer, self.labels, self.options.descriptions);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for score.
pub const ScoreOptions = struct {
    /// Descriptions by label or level. The engine refuses keys outside the question's choices.
    descriptions: ?std.json.ArrayHashMap([]const u8) = null,
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
};
/// A score question.
pub const Score = struct {
    /// The question text.
    question: []const u8,
    /// The ordered levels, lowest first.
    levels: []const Choice,
    /// The options.
    options: ScoreOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("score");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("levels");
        try writeOptions(writer, self.levels, self.options.descriptions);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for filter.
pub const FilterOptions = struct {
    /// What yes means for this question; null keeps the question's own wording.
    yes_means: ?[]const u8 = null,
    /// What no means for this question; null keeps the question's own wording.
    no_means: ?[]const u8 = null,
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// The answer threshold: a cut, or a band whose middle is not sure.
    threshold: ?Threshold = null,
};
/// A filter question.
pub const Filter = struct {
    /// The question text.
    question: []const u8,
    /// The options.
    options: FilterOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("filter");
        try writer.objectField("question");
        try writer.write(self.question);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        if (self.options.yes_means) |v| {
            try writer.objectField("true");
            try writer.write(v);
        }
        if (self.options.no_means) |v| {
            try writer.objectField("false");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for rank.
pub const RankOptions = struct {
    /// Descriptions by label or level. The engine refuses keys outside the question's choices.
    descriptions: ?std.json.ArrayHashMap([]const u8) = null,
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// Keep only this many of the top-ranked records.
    top: ?u32 = null,
};
/// A rank question.
pub const Rank = struct {
    /// The question text.
    question: []const u8,
    /// The ordered levels, lowest first.
    levels: []const Choice,
    /// The options.
    options: RankOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("rank");
        try writer.objectField("question");
        try writer.write(self.question);
        try writer.objectField("levels");
        try writeOptions(writer, self.levels, self.options.descriptions);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.top) |v| {
            try writer.objectField("top");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for find.
pub const FindOptions = struct {
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// The lowest probability that counts.
    threshold: ?f64 = null,
};
/// A find question.
pub const Find = struct {
    /// The question text.
    question: []const u8,
    /// The options.
    options: FindOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("find");
        try writer.objectField("question");
        try writer.write(self.question);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for recognize.
pub const RecognizeOptions = struct {
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// The lowest probability that counts.
    threshold: ?f64 = null,
};
/// A recognize question.
pub const Recognize = struct {
    /// The kinds of names to recognize.
    kinds: []const Choice,
    /// The options.
    options: RecognizeOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("recognize");
        try writer.objectField("kinds");
        try writer.write(self.kinds);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for relate.
pub const RelateOptions = struct {
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
    /// The lowest probability that counts.
    threshold: ?f64 = null,
};
/// A relate question.
pub const Relate = struct {
    /// The relations to look for.
    rules: []const RelationRule,
    /// The options.
    options: RelateOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("relate");
        try writer.objectField("rules");
        try writer.write(self.rules);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        if (self.options.threshold) |v| {
            try writer.objectField("threshold");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// Options for annotate.
pub const AnnotateOptions = struct {
    /// A name for the question; it changes the question's identity and appears in each row's source.
    name: ?[]const u8 = null,
    /// A JSON pointer, such as `/text`, that selects what to judge in JSON records.
    field: ?[]const u8 = null,
};
/// A annotate question.
pub const Annotate = struct {
    /// The named questions.
    questions: AnnotationQuestions,
    /// The options.
    options: AnnotateOptions = .{},
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: @This(), writer: anytype) !void {
        try writer.beginObject();
        try writer.objectField("function");
        try writer.write("annotate");
        try writer.objectField("questions");
        try writer.write(self.questions);
        if (self.options.field) |v| {
            try writer.objectField("field");
            try writer.write(v);
        }
        try writer.endObject();
    }
};
/// The question type of each function.
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
/// A saved question of any function.
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
    /// This question as the given function's type, or null when it is another function.
    pub fn as(self: EntryQuestion, comptime function: Function) ?QuestionType(function) {
        if (comptime !@hasField(EntryQuestion, @tagName(function))) return null;
        return if (std.meta.activeTag(self) == @field(std.meta.Tag(EntryQuestion), @tagName(function))) @field(self, @tagName(function)) else null;
    }
};
/// An annotate question: any function but rank and annotate.
pub const AnnotationQuestion = union(enum) { decide: Decide, choose: Choose, tag: Tag, score: Score, filter: Filter, find: Find, recognize: Recognize, relate: Relate };
/// A saved question and its name.
pub const NamedQuestion = struct { name: []const u8, question: EntryQuestion };
/// An annotate question and its name.
pub const NamedAnnotationQuestion = struct { name: []const u8, question: AnnotationQuestion };
/// Annotate's named questions. Build one, or get one from `Toolbox.annotation`.
pub const AnnotationQuestions = struct {
    /// The named questions, in order.
    entries: []const NamedAnnotationQuestion,
    /// Write as JSON, for `std.json.Stringify`.
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

// Adapt host description maps into the shared OptionWire form. Keep unmatched keys
// on the wire so shared admission, rather than this adapter, diagnoses them.
fn writeOptions(writer: anytype, options: []const Choice, descriptions: ?std.json.ArrayHashMap([]const u8)) !void {
    try writer.beginArray();
    var unknown = false;
    if (descriptions) |values| for (values.map.keys()) |key| {
        var found = false;
        for (options) |option| if (std.mem.eql(u8, option.name, key)) {
            found = true;
            break;
        };
        if (!found) unknown = true;
    };
    for (options) |option| {
        if (if (descriptions) |values| values.map.get(option.name) else null) |description| {
            try writer.write(.{ .name = option.name, .description = description });
        } else try writer.write(option.name);
    }
    try writer.endArray();
    if (descriptions) |values| if (unknown) {
        try writer.objectField("descriptions");
        try writer.write(values);
    };
}
