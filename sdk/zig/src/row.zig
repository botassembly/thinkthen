const std = @import("std");
const c = @import("c");
const t = @import("types.zig");
const d = @import("diagnostic.zig");
const Allocator = std.mem.Allocator;
pub const Status = enum { answered, unsure, failed };
pub const Source = struct { question_identity: []const u8, backend: t.Backend, model: []const u8, name: ?[]const u8 = null };
pub const RecordId = union(enum) {
    string: []const u8,
    signed: i64,
    unsigned: u64,
    pub fn jsonParse(allocator: Allocator, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!RecordId {
        var exact_options = options;
        exact_options.parse_numbers = false;
        const value = try std.json.innerParse(std.json.Value, allocator, source, exact_options);
        return fromValue(allocator, value, options);
    }
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
    pub fn jsonStringify(self: RecordId, writer: anytype) !void {
        switch (self) {
            inline else => |v| try writer.write(v),
        }
    }
};
pub const Span = struct { text: []const u8, start: u64, end: u64 };
pub const Name = struct { text: []const u8, start: u64, end: u64, kind: []const u8, probability: f64 };
pub const Endpoint = struct { text: []const u8, start: u64, end: u64, kind: []const u8 };
pub const Relation = struct { source: Endpoint, target: Endpoint, relation: []const u8, probability: f64, either: ?bool = null };
pub const LabelDetail = struct { labels: std.json.ArrayHashMap(f64) };
pub const LevelDetail = struct { levels: []const f64 };
pub const Piece = struct { text: []const u8, start: u64, end: u64, tags: std.json.ArrayHashMap(f64) };
pub const KindDetail = struct { start: u64, end: u64, kinds: std.json.ArrayHashMap(f64) };
pub const RecognizeDetail = struct { pieces: []const Piece, names: []const KindDetail };
pub const RelationDetail = struct { names: []const Name, pairs: []const Relation };
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
        .annotate => []const NamedAnnotation,
    };
}
pub fn Detail(comptime function: t.Function) type {
    return switch (function) {
        .decide, .filter, .find, .annotate => void,
        .choose, .tag => LabelDetail,
        .score, .rank => LevelDetail,
        .recognize => RecognizeDetail,
        .relate => RelationDetail,
    };
}
pub fn Outcome(comptime function: t.Function) type {
    return union(Status) {
        answered: struct { answer: Answer(function), probability: ?f64, detail: ?Detail(function) },
        unsure: struct { probability: f64, detail: ?Detail(function) },
        failed: d.Failure,
    };
}
pub const Annotation = union(enum) { decide: Outcome(.decide), choose: Outcome(.choose), tag: Outcome(.tag), score: Outcome(.score), filter: Outcome(.filter), find: Outcome(.find), recognize: Outcome(.recognize), relate: Outcome(.relate) };
pub const NamedAnnotation = struct { name: []const u8, value: Annotation };

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
    return result;
}
fn outcome(comptime function: t.Function, allocator: Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Outcome(function) {
    const status = try field(Status, allocator, value, "status", options);
    return switch (status) {
        .answered => .{ .answered = .{ .answer = try answer(function, allocator, value, options), .probability = try optional(f64, allocator, value, "probability", options), .detail = try optional(Detail(function), allocator, value, "detail", options) } },
        .unsure => .{ .unsure = .{ .probability = try field(f64, allocator, value, "probability", options), .detail = try optional(Detail(function), allocator, value, "detail", options) } },
        .failed => .{ .failed = try field(d.Failure, allocator, value, "error", options) },
    };
}
/// A row borrows all nested values from the arena supplied to next or a One function.
pub fn Row(comptime function: t.Function) type {
    return struct {
        position: u64,
        id: ?RecordId,
        source: Source,
        outcome: Outcome(function),
        const Self = @This();
        pub fn status(self: Self) Status {
            return std.meta.activeTag(self.outcome);
        }
        pub fn intoResult(self: Self, diag: ?*d.Diagnostic) d.Error!?Answer(function) {
            d.clear(diag);
            return switch (self.outcome) {
                .answered => |v| @as(?Answer(function), v.answer),
                .unsure => null,
                .failed => |v| d.promote(v, diag),
            };
        }
        pub fn jsonParse(allocator: Allocator, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!Self {
            var exact_options = options;
            exact_options.parse_numbers = false;
            const value = try std.json.innerParse(std.json.Value, allocator, source, exact_options);
            return jsonParseFromValue(allocator, value, options);
        }
        pub fn jsonParseFromValue(allocator: Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Self {
            const kind = try field(t.Function, allocator, value, "function", options);
            if (kind != function) return error.UnexpectedToken;
            return .{ .position = try field(u64, allocator, value, "position", options), .id = try optional(RecordId, allocator, value, "id", options), .source = try field(Source, allocator, value, "source", options), .outcome = try outcome(function, allocator, value, options) };
        }
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
                if (function == .annotate) {
                    try writer.beginObject();
                    for (payload.answer) |entry| {
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
                } else try writer.write(payload.answer);
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
pub const AnyRow = union(t.Function) { decide: Row(.decide), choose: Row(.choose), tag: Row(.tag), score: Row(.score), filter: Row(.filter), find: Row(.find), recognize: Row(.recognize), relate: Row(.relate), rank: Row(.rank), annotate: Row(.annotate) };
fn parse(comptime T: type, arena: Allocator, buffer: c.thinkthen_buffer) d.Error!T {
    const decoded = std.json.parseFromSliceLeaky(T, arena, d.bytes(buffer), .{ .allocate = .alloc_always, .parse_numbers = false }) catch |err| return if (err == error.OutOfMemory) error.OutOfMemory else error.Internal;
    return decoded;
}
pub fn take(comptime function: t.Function, arena: Allocator, row: c.thinkthen_row) d.Error!Row(function) {
    const detail: ?Detail(function) = if (Detail(function) == void or row.detail.len == 0) null else try parse(?Detail(function), arena, row.detail);
    const result: Outcome(function) = switch (row.status) {
        c.THINKTHEN_STATUS_ANSWERED => .{ .answered = .{ .answer = if (function == .annotate) try annotationAnswer(arena, row.answer) else try parse(Answer(function), arena, row.answer), .probability = if (std.math.isNan(row.probability)) null else row.probability, .detail = detail } },
        c.THINKTHEN_STATUS_UNSURE => .{ .unsure = .{ .probability = row.probability, .detail = detail } },
        c.THINKTHEN_STATUS_FAILED => blk: {
            var failure = try parse(d.Failure, arena, row.@"error");
            failure.code = @as(d.ErrorCode, @fromBackingInt(@intCast(row.error_code)));
            break :blk .{ .failed = failure };
        },
        else => return error.Internal,
    };
    return .{ .position = row.position, .id = if (row.id.len == 0) null else try parse(?RecordId, arena, row.id), .source = try parse(Source, arena, row.source), .outcome = result };
}
fn annotationAnswer(arena: Allocator, buffer: c.thinkthen_buffer) d.Error![]const NamedAnnotation {
    const children = try parse(std.json.Value, arena, buffer);
    var wrapper: std.json.ObjectMap = .empty;
    try wrapper.put(arena, "answer", children);
    return answer(.annotate, arena, .{ .object = wrapper }, .{ .allocate = .alloc_always }) catch |err| return if (err == error.OutOfMemory) error.OutOfMemory else error.Internal;
}
