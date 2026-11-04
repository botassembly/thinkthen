const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const d = @import("diagnostic.zig");
const q = @import("questions.zig");
const Allocator = std.mem.Allocator;

/// Saved questions by name. A loaded toolbox owns its questions until deinit;
/// a literal toolbox borrows its entries.
pub const Toolbox = struct {
    /// The questions in saved order.
    entries: []const q.NamedQuestion,
    owned: ?*std.heap.ArenaAllocator = null,

    /// Load a saved toolbox. Free it with deinit.
    pub fn fromJson(allocator: Allocator, json: []const u8, diag: ?*d.Diagnostic) d.Error!Toolbox {
        try ready(diag);
        var handle: ?*c.thinkthen_toolbox = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_toolbox_from_json")(@ptrCast(json.ptr), json.len, &handle, &err), &err, diag);
        defer native.function("thinkthen_toolbox_free")(handle);
        var buffer: c.thinkthen_buffer = .{};
        try d.check(native.function("thinkthen_toolbox_definitions_json")(handle, &buffer, &err), &err, diag);
        defer native.function("thinkthen_buffer_free")(&buffer);
        const arena = try allocator.create(std.heap.ArenaAllocator);
        arena.* = std.heap.ArenaAllocator.init(allocator);
        errdefer {
            arena.deinit();
            allocator.destroy(arena);
        }
        const a = arena.allocator();
        const definitions = std.json.parseFromSliceLeaky(std.json.Value, a, d.bytes(buffer), .{ .allocate = .alloc_always }) catch |failure| return decodeError(failure);
        const entries = try a.alloc(q.NamedQuestion, definitions.object.count());
        var iterator = definitions.object.iterator();
        var index: usize = 0;
        while (iterator.next()) |entry| : (index += 1) {
            entries[index] = .{ .name = entry.key_ptr.*, .question = hydrate(a, entry.value_ptr.*) catch |failure| return decodeError(failure) };
        }
        return .{ .entries = entries, .owned = arena };
    }
    /// Free a loaded toolbox. A literal toolbox needs no deinit.
    pub fn deinit(self: *Toolbox) void {
        if (self.owned) |arena| {
            const allocator = arena.child_allocator;
            arena.deinit();
            allocator.destroy(arena);
        }
        self.* = .{ .entries = &.{} };
    }
    /// The caller owns the returned JSON and frees it with allocator.free.
    pub fn toJson(self: Toolbox, allocator: Allocator, diag: ?*d.Diagnostic) d.Error![]u8 {
        try ready(diag);
        const definitions = try std.json.Stringify.valueAlloc(allocator, Definitions{ .entries = self.entries }, .{ .emit_null_optional_fields = false });
        defer allocator.free(definitions);
        var handle: ?*c.thinkthen_toolbox = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_toolbox_from_definitions_json")(@ptrCast(definitions.ptr), definitions.len, &handle, &err), &err, diag);
        defer native.function("thinkthen_toolbox_free")(handle);
        var buffer: c.thinkthen_buffer = .{};
        try d.check(native.function("thinkthen_toolbox_to_json")(handle, &buffer, &err), &err, diag);
        defer native.function("thinkthen_buffer_free")(&buffer);
        return allocator.dupe(u8, d.bytes(buffer));
    }
    /// The question with this name, or null.
    pub fn get(self: Toolbox, name: []const u8) ?q.EntryQuestion {
        for (self.entries) |entry| if (std.mem.eql(u8, entry.name, name)) {
            return entry.question;
        };
        return null;
    }
    /// Every question as one annotate form, for `client.annotate`. Annotate cannot hold rank,
    /// so a rank entry returns `error.Usage` with code `usage.question`.
    /// The caller frees the result's entries with allocator.free.
    pub fn annotation(self: Toolbox, allocator: Allocator, diag: ?*d.Diagnostic) error{ Usage, OutOfMemory }!q.AnnotationQuestions {
        d.clear(diag);
        const entries = try allocator.alloc(q.NamedAnnotationQuestion, self.entries.len);
        errdefer allocator.free(entries);
        for (self.entries, entries) |entry, *slot| {
            slot.* = .{ .name = entry.name, .question = switch (entry.question) {
                .rank => {
                    if (diag) |target| target.set(.{ .code = .usage_question, .message = "annotate cannot hold rank questions" });
                    return error.Usage;
                },
                inline else => |question, tag| @unionInit(q.AnnotationQuestion, @tagName(tag), question),
            } };
        }
        return .{ .entries = entries };
    }
};

const Definitions = struct {
    entries: []const q.NamedQuestion,
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: Definitions, writer: anytype) !void {
        try writer.beginObject();
        for (self.entries) |entry| {
            try writer.objectField(entry.name);
            switch (entry.question) {
                inline else => |question| try writer.write(question),
            }
        }
        try writer.endObject();
    }
};

fn ready(diag: ?*d.Diagnostic) d.Error!void {
    d.clear(diag);
    native.ready() catch |err| switch (err) {
        error.EngineVersion => return d.incompatible(diag),
        error.LoadFailure => return error.Setup,
    };
}
fn decodeError(failure: anyerror) d.Error {
    return if (failure == error.OutOfMemory) error.OutOfMemory else error.Internal;
}
// The engine has admitted these definitions. This translates their host representation.
fn hydrate(allocator: Allocator, value: std.json.Value) std.json.ParseFromValueError!q.EntryQuestion {
    const function = value.object.get("function").?.string;
    inline for (@typeInfo(q.EntryQuestion).@"union".field_names) |name| {
        if (std.mem.eql(u8, function, name)) {
            const T = @FieldType(q.EntryQuestion, name);
            var question: T = undefined;
            inline for (@typeInfo(T).@"struct".field_names) |field| {
                if (comptime std.mem.eql(u8, field, "options")) {
                    question.options = try std.json.innerParseFromValue(@FieldType(T, "options"), allocator, value, .{ .ignore_unknown_fields = true });
                } else {
                    @field(question, field) = try std.json.innerParseFromValue(@FieldType(T, field), allocator, value.object.get(field).?, .{});
                }
            }
            if (@hasField(@TypeOf(question.options), "yes_means")) {
                question.options.yes_means = if (value.object.get("true")) |meaning| meaning.string else null;
                question.options.no_means = if (value.object.get("false")) |meaning| meaning.string else null;
            }
            if (@hasField(@TypeOf(question.options), "descriptions")) {
                const domain = if (value.object.get("choices")) |v| v else if (value.object.get("labels")) |v| v else value.object.get("levels").?;
                var descriptions: std.json.ArrayHashMap([]const u8) = .{};
                for (domain.array.items) |option| if (option == .object) {
                    try descriptions.map.put(allocator, option.object.get("name").?.string, option.object.get("description").?.string);
                };
                question.options.descriptions = if (descriptions.map.count() == 0) null else descriptions;
            }
            return @unionInit(q.EntryQuestion, name, question);
        }
    }
    return error.UnexpectedToken;
}
