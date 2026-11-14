const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const d = @import("diagnostic.zig");
const q = @import("questions.zig");
const A = std.mem.Allocator;
pub const Toolbox = struct {
    owned: *std.heap.ArenaAllocator,
    handle: ?*c.thinkthen_toolbox,
    definitions: std.json.Value,
    entries: []const q.NamedQuestion,
    saved: []const u8,
    pub fn fromJson(allocator: A, json: []const u8, diag: ?*d.Diagnostic) d.Error!Toolbox {
        d.clear(diag);
        try d.ready(diag);
        var handle: ?*c.thinkthen_toolbox = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_toolbox_from_json")(json.ptr, json.len, &handle, &err), &err, diag);
        errdefer native.function("thinkthen_toolbox_free")(handle);
        var buffer: c.thinkthen_buffer = .{};
        try d.check(native.function("thinkthen_toolbox_definitions_json")(handle, &buffer, &err), &err, diag);
        defer native.function("thinkthen_buffer_free")(&buffer);
        const arena = try allocator.create(std.heap.ArenaAllocator);
        arena.* = .init(allocator);
        errdefer {
            arena.deinit();
            allocator.destroy(arena);
        }
        const definitions = std.json.parseFromSliceLeaky(std.json.Value, arena.allocator(), d.bytes(buffer), .{ .allocate = .alloc_always }) catch |failure| return if (failure == error.OutOfMemory) error.OutOfMemory else error.Internal;
        var canonical: c.thinkthen_buffer = .{};
        try d.check(native.function("thinkthen_toolbox_to_json")(handle, &canonical, &err), &err, diag);
        defer native.function("thinkthen_buffer_free")(&canonical);
        var toolbox: Toolbox = .{ .owned = arena, .handle = handle, .definitions = definitions, .entries = &.{}, .saved = try arena.allocator().dupe(u8, d.bytes(canonical)) };
        const entries = try arena.allocator().alloc(q.NamedQuestion, definitions.object.count());
        var iterator = definitions.object.iterator();
        var index: usize = 0;
        while (iterator.next()) |entry| : (index += 1) {
            entries[index].name = entry.key_ptr.*;
            const function = entry.value_ptr.object.get("function").?.string;
            inline for (@typeInfo(q.EntryQuestion).@"union".field_names) |function_name| if (std.mem.eql(u8, function, function_name)) {
                entries[index].question = @unionInit(q.EntryQuestion, function_name, try toolbox.select(q.QuestionType(@field(@import("types.zig").Function, function_name)), entry.key_ptr.*, diag));
            };
        }
        toolbox.entries = entries;
        return toolbox;
    }
    pub fn deinit(self: *Toolbox) void {
        native.function("thinkthen_toolbox_free")(self.handle);
        self.handle = null;
        const allocator = self.owned.child_allocator;
        self.owned.deinit();
        allocator.destroy(self.owned);
    }
    fn select(self: Toolbox, comptime Q: type, name: []const u8, diag: ?*d.Diagnostic) d.Error!Q {
        d.clear(diag);
        const a = self.owned.allocator();
        var selected: ?*c.thinkthen_question = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_toolbox_select_as")(self.handle, name.ptr, name.len, nativeFunction(Q.function), &selected, &err), &err, diag);
        defer native.function("thinkthen_question_free")(selected);
        const value = self.definitions.object.get(name) orelse return error.Internal;
        if (comptime @hasDecl(Q, "Choice")) if (Q.Choice != []const u8) {
            const names = @typeInfo(Q.Choice).@"enum".field_names;
            var words: [names.len][*c]const u8 = undefined;
            inline for (names, 0..) |word, index| words[index] = word.ptr;
            try d.check(native.function("thinkthen_question_expect_choices")(selected, &words, words.len, &err), &err, diag);
        };
        var result: Q = undefined;
        inline for (@typeInfo(Q).@"struct".field_names, @typeInfo(Q).@"struct".field_attrs) |field, attrs| {
            const T = @FieldType(Q, field);
            if (T == void) {
                @field(result, field) = {};
                continue;
            }
            if (comptime std.mem.eql(u8, field, "entries")) {
                @field(result, field) = value.object.get("questions").?;
            } else if (comptime std.mem.eql(u8, field, "descriptions")) {
                const key = switch (Q.function) {
                    .choose => "choices",
                    .tag => "labels",
                    else => "levels",
                };
                const words = value.object.get(key).?.array.items;
                if (Q.Choice == []const u8) {
                    var descriptions: std.json.ArrayHashMap([]const u8) = .{};
                    for (words) |word| if (word == .object) {
                        if (word.object.get("description")) |text| try descriptions.map.put(a, word.object.get("name").?.string, text.string);
                    };
                    result.descriptions = if (descriptions.map.count() == 0) null else descriptions;
                } else {
                    result.descriptions = .{};
                    inline for (@typeInfo(Q.Choice).@"enum".field_names, 0..) |word, index| {
                        if (words[index] == .object) if (words[index].object.get("description")) |text| {
                            @field(result.descriptions, word) = text.string;
                        };
                    }
                }
            } else if (comptime std.mem.eql(u8, field, "choices")) {
                const key = switch (Q.function) {
                    .choose => "choices",
                    .tag => "labels",
                    else => "levels",
                };
                const words = value.object.get(key).?.array.items;
                const labels = try a.alloc([]const u8, words.len);
                for (words, labels) |word, *label| label.* = if (word == .string) word.string else word.object.get("name").?.string;
                @field(result, field) = labels;
            } else if (value.object.get(field)) |child| {
                @field(result, field) = std.json.innerParseFromValue(T, a, child, .{ .ignore_unknown_fields = true, .allocate = .alloc_always }) catch |failure| return if (failure == error.OutOfMemory) error.OutOfMemory else error.Internal;
            } else {
                if (comptime attrs.default_value_ptr) |default| {
                    @field(result, field) = @as(*const T, @ptrCast(@alignCast(default))).*;
                } else return error.Internal;
            }
        }
        return result;
    }
    /// Write the admitted saved toolbox, preserving its non-question entries.
    pub fn toJson(self: Toolbox, allocator: A, diag: ?*d.Diagnostic) d.Error![]u8 {
        d.clear(diag);
        return allocator.dupe(u8, self.saved);
    }
    /// Reference lookup for names discovered at runtime.
    pub fn get(self: Toolbox, name: []const u8) ?q.EntryQuestion {
        for (self.entries) |entry| if (std.mem.eql(u8, entry.name, name)) return entry.question;
        return null;
    }
    /// Build a dynamically named form through the shared question admission.
    pub fn annotation(self: Toolbox, allocator: A, diag: ?*d.Diagnostic) d.Error!q.DynamicAnnotate {
        d.clear(diag);
        const question: q.DynamicAnnotate = .{ .entries = self.definitions };
        const encoded = try std.json.Stringify.valueAlloc(allocator, question, .{ .emit_null_optional_fields = false });
        defer allocator.free(encoded);
        var handle: ?*c.thinkthen_question = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_question_from_json")(encoded.ptr, encoded.len, &handle, &err), &err, diag);
        native.function("thinkthen_question_free")(handle);
        return question;
    }
    pub fn decide(self: Toolbox, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Decide {
        return self.select(q.Decide, name, diag);
    }
    pub fn filter(self: Toolbox, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Filter {
        return self.select(q.Filter, name, diag);
    }
    pub fn find(self: Toolbox, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Find {
        return self.select(q.Find, name, diag);
    }
    pub fn recognize(self: Toolbox, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Recognize {
        return self.select(q.Recognize, name, diag);
    }
    pub fn relate(self: Toolbox, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Relate {
        return self.select(q.Relate, name, diag);
    }
    pub fn choose(self: Toolbox, comptime E: type, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Choose(E) {
        return self.select(q.Choose(E), name, diag);
    }
    pub fn tag(self: Toolbox, comptime E: type, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Tag(E) {
        return self.select(q.Tag(E), name, diag);
    }
    pub fn score(self: Toolbox, comptime E: type, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Score(E) {
        return self.select(q.Score(E), name, diag);
    }
    pub fn rank(self: Toolbox, comptime E: type, name: []const u8, diag: ?*d.Diagnostic) d.Error!q.Rank(E) {
        return self.select(q.Rank(E), name, diag);
    }
};
fn nativeFunction(comptime function: @import("types.zig").Function) c.thinkthen_function {
    const uppercase = comptime blk: {
        const name = @tagName(function);
        var result: [name.len]u8 = undefined;
        for (name, 0..) |byte, index| result[index] = std.ascii.toUpper(byte);
        break :blk result;
    };
    return @field(c, "THINKTHEN_FUNCTION_" ++ uppercase);
}
