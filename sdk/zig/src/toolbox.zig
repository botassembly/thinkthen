const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const d = @import("diagnostic.zig");
const q = @import("questions.zig");
const Allocator = std.mem.Allocator;

pub const Toolbox = struct {
    entries: []const q.NamedQuestion,
    owned: ?*std.heap.ArenaAllocator = null,

    /// Loaded questions borrow this owner until deinit. Literal toolboxes borrow their entries.
    pub fn fromJson(allocator: Allocator, json: []const u8, diag: ?*d.Diagnostic) d.Error!Toolbox {
        try ready(diag);
        var handle: ?*c.thinkthen_toolbox = null;
        var err = d.emptyError();
        try d.check(native.function("thinkthen_toolbox_from_json")(json.ptr, json.len, &handle, &err), &err, diag);
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
        try d.check(native.function("thinkthen_toolbox_from_definitions_json")(definitions.ptr, definitions.len, &handle, &err), &err, diag);
        defer native.function("thinkthen_toolbox_free")(handle);
        var buffer: c.thinkthen_buffer = .{};
        try d.check(native.function("thinkthen_toolbox_to_json")(handle, &buffer, &err), &err, diag);
        defer native.function("thinkthen_buffer_free")(&buffer);
        return allocator.dupe(u8, d.bytes(buffer));
    }
    pub fn get(self: Toolbox, name: []const u8) ?q.EntryQuestion {
        for (self.entries) |entry| if (std.mem.eql(u8, entry.name, name)) {
            return entry.question;
        };
        return null;
    }
};

const Definitions = struct {
    entries: []const q.NamedQuestion,
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
            return @unionInit(q.EntryQuestion, name, question);
        }
    }
    return error.UnexpectedToken;
}
