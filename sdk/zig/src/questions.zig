const std = @import("std");
const t = @import("types.zig");
pub const Decide = struct {
    pub const function: t.Function = .decide;
    question: []const u8,
    threshold: ?t.Threshold = null,
    field: ?[]const u8 = null,
    yes_means: ?[]const u8 = null,
    no_means: ?[]const u8 = null,
    name: ?[]const u8 = null,
    pub fn jsonStringify(self: @This(), w: anytype) !void {
        try write(self, w);
    }
};
pub const Filter = struct {
    pub const function: t.Function = .filter;
    question: []const u8,
    threshold: ?t.Cut = null,
    field: ?[]const u8 = null,
    yes_means: ?[]const u8 = null,
    no_means: ?[]const u8 = null,
    name: ?[]const u8 = null,
    pub fn jsonStringify(self: @This(), w: anytype) !void {
        try write(self, w);
    }
};
fn Descriptions(comptime E: type) type {
    const names = @typeInfo(E).@"enum".field_names;
    const empty: ?[]const u8 = null;
    return @Struct(.auto, null, names, &@splat(?[]const u8), &@splat(.{ .default_value_ptr = @as(?*const anyopaque, @ptrCast(&empty)) }));
}
fn Enumerated(comptime f: t.Function, comptime E: type) type {
    if (E != []const u8 and @typeInfo(E) != .@"enum") @compileError("choices, labels and levels must be an enum or []const u8");
    return struct {
        pub const function = f;
        pub const Choice = E;
        question: []const u8,
        choices: if (E == []const u8) []const []const u8 else void = if (E == []const u8) &.{} else {},
        descriptions: if (E == []const u8) ?std.json.ArrayHashMap([]const u8) else Descriptions(E) = if (E == []const u8) null else .{},
        threshold: if (f == .choose or f == .tag) ?t.Cut else void = if (f == .choose or f == .tag) null else {},
        field: ?[]const u8 = null,
        name: ?[]const u8 = null,
        top: if (f == .rank) ?usize else void = if (f == .rank) null else {},
        pub fn jsonStringify(self: @This(), w: anytype) !void {
            try write(self, w);
        }
    };
}
pub fn Choose(comptime E: type) type {
    return Enumerated(.choose, E);
}
pub fn Tag(comptime E: type) type {
    return Enumerated(.tag, E);
}
pub fn Score(comptime E: type) type {
    return Enumerated(.score, E);
}
pub fn Rank(comptime E: type) type {
    return Enumerated(.rank, E);
}
pub const Find = struct {
    pub const function: t.Function = .find;
    question: []const u8,
    threshold: ?t.Cut = null,
    field: ?[]const u8 = null,
    name: ?[]const u8 = null,
    pub fn jsonStringify(self: @This(), w: anytype) !void {
        try write(self, w);
    }
};
pub const Recognize = struct {
    pub const function: t.Function = .recognize;
    kinds: []const []const u8,
    threshold: ?t.Cut = null,
    field: ?[]const u8 = null,
    name: ?[]const u8 = null,
    pub fn jsonStringify(self: @This(), w: anytype) !void {
        try write(self, w);
    }
};
pub const Relate = struct {
    pub const function: t.Function = .relate;
    rules: []const t.RelationRule,
    threshold: ?t.Cut = null,
    field: ?[]const u8 = null,
    name: ?[]const u8 = null,
    pub fn jsonStringify(self: @This(), w: anytype) !void {
        try write(self, w);
    }
};
pub fn Annotate(comptime Entries: type) type {
    inline for (@typeInfo(Entries).@"struct".field_types) |Q| if (Q.function == .filter or Q.function == .rank or Q.function == .annotate) @compileError("annotate cannot hold filter, rank or annotate questions");
    return struct {
        pub const function: t.Function = .annotate;
        pub const Questions = Entries;
        entries: Entries,
        field: ?[]const u8 = null,
        name: ?[]const u8 = null,
        pub fn jsonStringify(self: @This(), w: anytype) !void {
            try write(self, w);
        }
    };
}
pub fn annotate(entries: anytype) Annotate(@TypeOf(entries)) {
    return .{ .entries = entries };
}
fn write(self: anytype, w: anytype) !void {
    const Q = @TypeOf(self);
    try w.beginObject();
    try w.objectField("function");
    try w.write(Q.function);
    inline for (@typeInfo(Q).@"struct".field_names) |name| {
        if (comptime std.mem.eql(u8, name, "name") or std.mem.eql(u8, name, "choices") or std.mem.eql(u8, name, "descriptions")) continue;
        if (@TypeOf(@field(self, name)) == void) continue;
        if (comptime @typeInfo(@TypeOf(@field(self, name))) == .optional) {
            if (@field(self, name)) |value| {
                try w.objectField(name);
                try w.write(value);
            }
        } else {
            try w.objectField(if (comptime std.mem.eql(u8, name, "entries")) "questions" else name);
            try w.write(@field(self, name));
        }
    }
    if (@hasDecl(Q, "Choice")) {
        const E = Q.Choice;
        try w.objectField(switch (Q.function) {
            .choose => "choices",
            .tag => "labels",
            else => "levels",
        });
        try w.beginArray();
        if (E == []const u8) {
            for (self.choices) |word| {
                const description = if (self.descriptions) |descriptions| descriptions.map.get(word) else null;
                if (description) |text| try w.write(.{ .name = word, .description = text }) else try w.write(word);
            }
        } else inline for (@typeInfo(E).@"enum".field_names) |word| {
            if (@field(self.descriptions, word)) |text| try w.write(.{ .name = word, .description = text }) else try w.write(word);
        }
        try w.endArray();
        // Preserve unknown dynamic-map keys for the engine to refuse at admission.
        if (E == []const u8) if (self.descriptions) |descriptions| {
            var unknown = false;
            for (descriptions.map.keys()) |key| {
                var found = false;
                for (self.choices) |word| if (std.mem.eql(u8, word, key)) {
                    found = true;
                    break;
                };
                if (!found) unknown = true;
            }
            if (unknown) {
                try w.objectField("descriptions");
                try w.write(descriptions);
            }
        };
    }
    try w.endObject();
}

/// A form whose question names are known only at runtime.
pub const DynamicAnnotate = struct {
    pub const function: t.Function = .annotate;
    entries: std.json.Value,
    field: ?[]const u8 = null,
    name: ?[]const u8 = null,
    pub fn jsonStringify(self: @This(), w: anytype) !void {
        try write(self, w);
    }
};
pub fn QuestionType(comptime function: t.Function) type {
    return switch (function) {
        .decide => Decide,
        .choose => Choose([]const u8),
        .tag => Tag([]const u8),
        .score => Score([]const u8),
        .rank => Rank([]const u8),
        .filter => Filter,
        .find => Find,
        .recognize => Recognize,
        .relate => Relate,
        .annotate => DynamicAnnotate,
    };
}
pub const EntryQuestion = union(enum) {
    decide: Decide,
    choose: Choose([]const u8),
    tag: Tag([]const u8),
    score: Score([]const u8),
    filter: Filter,
    rank: Rank([]const u8),
    find: Find,
    recognize: Recognize,
    relate: Relate,
    pub fn as(self: @This(), comptime function: t.Function) ?QuestionType(function) {
        if (comptime !@hasField(@This(), @tagName(function))) return null;
        return if (std.mem.eql(u8, @tagName(std.meta.activeTag(self)), @tagName(function))) @field(self, @tagName(function)) else null;
    }
};
pub const NamedQuestion = struct { name: []const u8, question: EntryQuestion };
