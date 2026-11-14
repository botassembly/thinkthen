const std = @import("std");
const t = @import("types.zig");
const d = @import("diagnostic.zig");
const Allocator = std.mem.Allocator;
/// A row's state: answered, not sure, or failed.
pub const Status = enum { answered, unsure, failed };
/// Where an answer came from.
pub const Source = struct {
    /// A digest naming the question and its settings.
    question_identity: []const u8,
    /// The backend that answered.
    backend: t.Backend,
    /// The backend model.
    model: []const u8,
    /// The question's name, when it was given one.
    name: ?[]const u8 = null,
};
/// A record's `id` field: a string or an integer.
pub const RecordId = union(enum) {
    string: []const u8,
    signed: i64,
    unsigned: u64,
    /// Read JSON, for `std.json.parseFromSlice`.
    pub fn jsonParse(allocator: Allocator, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!RecordId {
        var exact_options = options;
        exact_options.parse_numbers = false;
        const value = try std.json.innerParse(std.json.Value, allocator, source, exact_options);
        return fromValue(allocator, value, options);
    }
    /// Read a parsed JSON value, for `std.json.parseFromValue`.
    pub fn jsonParseFromValue(allocator: Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!RecordId {
        return fromValue(allocator, value, options);
    }
    fn fromValue(allocator: Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!RecordId {
        return switch (value) {
            .string => .{ .string = try std.json.innerParseFromValue([]const u8, allocator, value, options) },
            .integer => |v| if (v < 0) .{ .signed = v } else .{ .unsigned = @intCast(v) },
            .number_string => |v| if (std.mem.startsWith(u8, v, "-")) .{ .signed = std.fmt.parseInt(i64, v, 10) catch return error.InvalidNumber } else .{ .unsigned = std.fmt.parseInt(u64, v, 10) catch return error.InvalidNumber },
            else => error.UnexpectedToken,
        };
    }
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: RecordId, writer: anytype) !void {
        switch (self) {
            inline else => |v| try writer.write(v),
        }
    }
};
/// A piece of the record's text. Offsets are UTF-8 byte offsets into the field.
pub const Span = struct { text: []const u8, start: usize, end: usize };
/// A recognized name, with its kind and probability.
pub const Name = struct { kind: []const u8, text: []const u8, start: usize, end: usize, probability: f64 };
/// One end of a relation.
pub const Endpoint = struct { kind: []const u8, text: []const u8, start: usize, end: usize };
/// A relation found between two names.
pub const Relation = struct { source: Endpoint, target: Endpoint, relation: []const u8, probability: f64, either: bool = false };
/// Choose and tag detail: each label's probability.
pub const LabelDetail = struct { labels: std.json.ArrayHashMap(f64) };
/// Score and rank detail: each level's probability, in level order.
pub const LevelDetail = struct { levels: []const f64 };
/// A piece of text with each kind's probability.
pub const Piece = struct { text: []const u8, start: usize, end: usize, tags: std.json.ArrayHashMap(f64) };
/// A candidate name with each kind's probability.
pub const KindDetail = struct { text: []const u8, start: usize, end: usize, kinds: std.json.ArrayHashMap(f64) };
/// Recognize detail.
pub const RecognizeDetail = struct { pieces: []const Piece, names: []const KindDetail };
/// Relate detail: the names found and every pair judged.
pub const RelationDetail = struct { names: []const Name, pairs: []const Relation };
/// The answer type of each function.
pub fn Answer(comptime function: t.Function) type {
    return switch (function) {
        .decide, .filter => bool,
        .choose => []const u8,
        .tag => []const []const u8,
        .score => f64,
        .rank => u64,
        .find => ?Span,
        .recognize => []const Name,
        .relate => []const Relation,
        .annotate => Annotations,
    };
}
/// The detail type of each function; void where there is none.
pub fn Detail(comptime function: t.Function) type {
    return switch (function) {
        .decide, .filter, .find, .annotate => void,
        .choose, .tag => LabelDetail,
        .score, .rank => LevelDetail,
        .recognize => RecognizeDetail,
        .relate => RelationDetail,
    };
}
/// One question's outcome. Read it with `answer`, or `switch` on it to handle every case.
pub fn Outcome(comptime function: t.Function) type {
    return union(Status) {
        answered: struct { answer: Answer(function), probability: ?f64, detail: ?Detail(function) },
        unsure: struct { probability: f64, detail: ?Detail(function) },
        failed: d.Failure,
        const Self = @This();
        /// The answer. A not-sure outcome returns `error.Unsure`, with its probability in diag.
        /// A failed outcome returns its error, with its code and message in diag.
        pub fn answer(self: Self, diag: ?*d.Diagnostic) d.AnswerError!Answer(function) {
            d.clear(diag);
            return switch (self) {
                .answered => |value| value.answer,
                .unsure => |value| {
                    if (diag) |target| target.setUnsure(value.probability);
                    return error.Unsure;
                },
                .failed => |failure| {
                    if (diag) |target| target.set(failure);
                    return error.Failed;
                },
            };
        }
        /// The probability, when the outcome has one.
        pub fn probability(self: Self) ?f64 {
            return switch (self) {
                .answered => |value| value.probability,
                .unsure => |value| value.probability,
                .failed => null,
            };
        }
        /// True when the outcome is not sure.
        pub fn isUnsure(self: Self) bool {
            return self == .unsure;
        }
        /// True when the outcome failed.
        pub fn isFailed(self: Self) bool {
            return self == .failed;
        }
        /// Print the answer and probability, such as `true (0.97)`, `unsure (0.50)` or `failed record.missing: ...`.
        pub fn format(self: Self, writer: *std.Io.Writer) std.Io.Writer.Error!void {
            switch (self) {
                .failed => |failure| {
                    try writer.writeAll("failed: ");
                    try folded(failure.name(), writer);
                    try writer.writeAll(": ");
                    try folded(failure.message, writer);
                },
                .unsure => try writer.writeAll("unsure"),
                .answered => |value| {
                    const item = value.answer;
                    switch (function) {
                        .decide, .filter => try writer.writeAll(if (item) "true" else "false"),
                        .choose => try folded(item, writer),
                        .score, .rank => try writer.print("{d}", .{item}),
                        .tag => for (item, 0..) |word, index| {
                            if (index != 0) try writer.writeByte('\t');
                            try folded(word, writer);
                        },
                        .find => if (item) |span| {
                            try folded(span.text, writer);
                        },
                        .recognize => for (item, 0..) |name, index| {
                            if (index != 0) try writer.writeByte('\t');
                            try folded(name.kind, writer);
                            try writer.writeAll(": ");
                            try folded(name.text, writer);
                            try writer.print(" ({d}..{d})", .{ name.start, name.end });
                        },
                        .relate => for (item, 0..) |relation, index| {
                            if (index != 0) try writer.writeByte('\t');
                            try folded(relation.source.text, writer);
                            try writer.writeAll(" --");
                            try folded(relation.relation, writer);
                            try writer.writeAll("--> ");
                            try folded(relation.target.text, writer);
                        },
                        .annotate => for (item.entries, 0..) |entry, index| {
                            if (index != 0) try writer.writeByte('\t');
                            try folded(entry.name, writer);
                            try writer.writeByte('=');
                            switch (entry.value) {
                                inline else => |child| try child.format(writer),
                            }
                        },
                    }
                },
            }
        }
    };
}
/// One annotate question's outcome, by function.
pub const Annotation = union(enum) { decide: Outcome(.decide), choose: Outcome(.choose), tag: Outcome(.tag), score: Outcome(.score), find: Outcome(.find), recognize: Outcome(.recognize), relate: Outcome(.relate) };
/// One annotate question's name and outcome.
pub const NamedAnnotation = struct { name: []const u8, value: Annotation };
/// Annotate's answer: each question's outcome, read by name with `get`.
pub const Annotations = struct {
    /// The outcomes in question order.
    entries: []const NamedAnnotation,
    /// The outcome of the question with this name and function, or null when there is none.
    pub fn get(self: Annotations, comptime function: t.Function, name: []const u8) ?Outcome(function) {
        if (comptime !@hasField(Annotation, @tagName(function))) @compileError("annotate holds no " ++ @tagName(function) ++ " questions");
        for (self.entries) |entry| {
            if (std.mem.eql(u8, entry.name, name) and std.meta.activeTag(entry.value) == @field(std.meta.Tag(Annotation), @tagName(function))) {
                return @field(entry.value, @tagName(function));
            }
        }
        return null;
    }
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: Annotations, writer: anytype) !void {
        try writer.beginObject();
        for (self.entries) |entry| {
            try writer.objectField(entry.name);
            try writer.beginObject();
            switch (entry.value) {
                inline else => |child, tag| {
                    const f = @field(t.Function, @tagName(tag));
                    try writer.objectField("function");
                    try writer.write(f);
                    try writeOutcome(f, child, writer);
                },
            }
            try writer.endObject();
        }
        try writer.endObject();
    }
};

fn field(comptime T: type, allocator: Allocator, value: std.json.Value, key: []const u8, options: std.json.ParseOptions) std.json.ParseFromValueError!T {
    if (value != .object) return error.UnexpectedToken;
    return std.json.innerParseFromValue(T, allocator, value.object.get(key) orelse return error.MissingField, options);
}
fn optional(comptime T: type, allocator: Allocator, value: std.json.Value, key: []const u8, options: std.json.ParseOptions) std.json.ParseFromValueError!?T {
    if (value != .object) return error.UnexpectedToken;
    const child = value.object.get(key) orelse return null;
    if (child == .null) return null;
    if (T == void) return error.UnexpectedToken;
    return try std.json.innerParseFromValue(T, allocator, child, options);
}
fn answer(comptime function: t.Function, allocator: Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Answer(function) {
    if (function != .annotate) return field(Answer(function), allocator, value, "answer", options);
    const children = value.object.get("answer") orelse return error.MissingField;
    if (children != .object) return error.UnexpectedToken;
    const result = try allocator.alloc(NamedAnnotation, children.object.count());
    var iterator = children.object.iterator();
    var i: usize = 0;
    while (iterator.next()) |entry| : (i += 1) {
        const kind = try field([]const u8, allocator, entry.value_ptr.*, "function", options);
        result[i].name = try allocator.dupe(u8, entry.key_ptr.*);
        var found = false;
        inline for (@typeInfo(Annotation).@"union".field_names) |name| {
            if (std.mem.eql(u8, kind, name)) {
                result[i].value = @unionInit(Annotation, name, try outcome(@field(t.Function, name), allocator, entry.value_ptr.*, options));
                found = true;
            }
        }
        if (!found) return error.InvalidEnumTag;
    }
    std.mem.sort(NamedAnnotation, result, {}, struct {
        fn less(_: void, left: NamedAnnotation, right: NamedAnnotation) bool {
            return std.mem.lessThan(u8, left.name, right.name);
        }
    }.less);
    return .{ .entries = result };
}
pub fn outcome(comptime function: t.Function, allocator: Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Outcome(function) {
    const status = try field(Status, allocator, value, "status", options);
    return switch (status) {
        .answered => .{ .answered = .{ .answer = try answer(function, allocator, value, options), .probability = try optional(f64, allocator, value, "probability", options), .detail = try optional(Detail(function), allocator, value, "detail", options) } },
        .unsure => .{ .unsure = .{ .probability = try field(f64, allocator, value, "probability", options), .detail = try optional(Detail(function), allocator, value, "detail", options) } },
        .failed => .{ .failed = try field(d.Failure, allocator, value, "error", options) },
    };
}
/// One record's result. Its strings and slices live in the memory of the call that returned it.
pub fn Row(comptime function: t.Function) type {
    return struct {
        /// The record's input position, counting from 1.
        position: u64,
        /// The record's `id` field, when it had one.
        id: ?RecordId,
        /// Where the answer came from.
        source: Source,
        /// The outcome. `switch` on it to handle answered, not sure and failed together.
        outcome: Outcome(function),
        /// Rank's original input, copied from the engine into the row's arena. Other functions use void.
        /// Saved canonical row JSON does not include original input bytes.
        item: if (function == .rank) t.Record else void = if (function == .rank) .missing else {},
        const Self = @This();
        /// The row's status.
        pub fn status(self: Self) Status {
            return std.meta.activeTag(self.outcome);
        }
        /// The answer. A not-sure row returns `error.Unsure`, and a failed row returns its error.
        /// diag then holds the row's position, and the probability or the failure's code and message.
        pub fn answer(self: Self, diag: ?*d.Diagnostic) d.AnswerError!Answer(function) {
            const result = self.outcome.answer(diag);
            if (diag) |target| target.position = self.position;
            return result;
        }
        /// The probability, when the row has one.
        pub fn probability(self: Self) ?f64 {
            return self.outcome.probability();
        }
        /// True when the row is not sure.
        pub fn isUnsure(self: Self) bool {
            return self.outcome.isUnsure();
        }
        /// True when the row failed.
        pub fn isFailed(self: Self) bool {
            return self.outcome.isFailed();
        }
        /// Print the outcome, such as `true (0.97)`.
        pub fn format(self: Self, writer: *std.Io.Writer) std.Io.Writer.Error!void {
            return self.outcome.format(writer);
        }
        /// Read JSON, for `std.json.parseFromSlice`.
        pub fn jsonParse(allocator: Allocator, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!Self {
            var exact_options = options;
            exact_options.parse_numbers = false;
            const value = try std.json.innerParse(std.json.Value, allocator, source, exact_options);
            return jsonParseFromValue(allocator, value, options);
        }
        /// Read a parsed JSON value, for `std.json.parseFromValue`.
        pub fn jsonParseFromValue(allocator: Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Self {
            const kind = try field(t.Function, allocator, value, "function", options);
            if (kind != function) return error.UnexpectedToken;
            return .{ .position = try field(u64, allocator, value, "position", options), .id = try optional(RecordId, allocator, value, "id", options), .source = try field(Source, allocator, value, "source", options), .outcome = try outcome(function, allocator, value, options) };
        }
        /// Write as JSON, for `std.json.Stringify`.
        pub fn jsonStringify(self: Self, writer: anytype) !void {
            try writer.beginObject();
            try writer.objectField("function");
            try writer.write(function);
            try writer.objectField("position");
            try writer.write(self.position);
            if (self.id) |id| {
                try writer.objectField("id");
                try writer.write(id);
            }
            try writer.objectField("source");
            try writer.write(self.source);
            try writeOutcome(function, self.outcome, writer);
            try writer.endObject();
        }
    };
}
fn writeOutcome(comptime function: t.Function, value: Outcome(function), writer: anytype) !void {
    try writer.objectField("status");
    try writer.write(std.meta.activeTag(value));
    switch (value) {
        .failed => |failure| {
            try writer.objectField("error");
            try writer.write(failure);
        },
        inline else => |payload, status| {
            if (status == .answered) {
                try writer.objectField("answer");
                try writer.write(payload.answer);
            }
            if (status == .answered) {
                if (payload.probability) |v| {
                    try writer.objectField("probability");
                    try writer.write(v);
                }
            } else {
                try writer.objectField("probability");
                try writer.write(payload.probability);
            }
            if (comptime Detail(function) != void) if (payload.detail) |v| {
                try writer.objectField("detail");
                try writer.write(v);
            };
        },
    }
}
fn folded(text: []const u8, writer: *std.Io.Writer) std.Io.Writer.Error!void {
    for (text) |byte| try writer.writeByte(switch (byte) {
        '\r', '\n', '\t' => ' ',
        else => byte,
    });
}
