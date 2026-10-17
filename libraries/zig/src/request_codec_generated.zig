// Generated private request representation; do not export through the host API.
const std = @import("std");
pub fn encode(allocator: std.mem.Allocator, value: anytype) ![]u8 {
    return std.json.Stringify.valueAlloc(allocator, Encoded(@TypeOf(value)){ .value = value }, .{ .emit_null_optional_fields = false });
}
fn Encoded(comptime T: type) type {
    return struct {
        value: T,
        pub fn jsonStringify(self: @This(), writer: anytype) error{WriteFailed}!void {
            try write(self.value, writer);
        }
    };
}
fn write(value: anytype, writer: anytype) error{WriteFailed}!void {
    const T = @TypeOf(value);
    if (T == std.json.Value) return writer.write(value);
    switch (@typeInfo(T)) {
        .optional => if (value) |member| try write(member, writer) else try writer.write(null),
        .@"union" => switch (value) {
            inline else => |member| {
                if (@TypeOf(member) == void) try writer.write(null) else try write(member, writer);
            },
        },
        .@"struct" => {
            try writer.beginObject();
            if (@hasField(T, "entries")) {
                for (value.entries) |entry| {
                    try writer.objectField(entry.key);
                    try write(entry.value, writer);
                }
            } else inline for (std.meta.fields(T)) |field| {
                const member = @field(value, field.name);
                const omit = if (comptime @typeInfo(field.type) == .optional) member == null else false;
                if (!omit) {
                    try writer.objectField(field.name);
                    try write(member, writer);
                }
            }
            try writer.endObject();
        },
        .pointer => |info| {
            if (T == []const u8 or T == []u8) return writer.write(value);
            if (info.size == .one) return write(value.*, writer);
            try writer.beginArray();
            for (value) |member| try write(member, writer);
            try writer.endArray();
        },
        else => try writer.write(value),
    }
}
