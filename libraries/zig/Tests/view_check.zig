//! Compare the complete typed C graph with native canonical packets in a caller.
const std = @import("std");
const tt = @import("thinkthen");
const labels = @import("view_labels.zig");
const c = tt.c;
fn holds(ok: bool) !void {
    if (!ok) return error.TypedViewMismatch;
}
fn child(value: std.json.Value, name: []const u8) !std.json.Value {
    try holds(value == .object);
    return value.object.get(name) orelse error.MissingTypedField;
}
pub fn compare(comptime T: type, value: T, expected: std.json.Value) anyerror!void {
    switch (@typeInfo(T)) {
        .pointer => {
            try holds(@intFromPtr(value) != 0);
            return compare(@typeInfo(T).pointer.child, (if (@typeInfo(T).pointer.size == .one) value.* else value[0]), expected);
        },
        .int => {
            if (expected == .bool) return holds(value == @as(T, if (expected.bool) 1 else 0));
            const number: T = switch (expected) {
                .integer => std.math.cast(T, expected.integer) orelse return error.WrongTypedNumber,
                .number_string => try std.fmt.parseInt(T, expected.number_string, 10),
                else => return error.WrongTypedNumber,
            };
            return holds(value == number);
        },
        .float => {
            if (expected == .bool) return holds(value == @as(T, if (expected.bool) 1 else 0));
            const number: f64 = switch (expected) {
                .integer => @floatFromInt(expected.integer),
                .float => expected.float,
                .number_string => try std.fmt.parseFloat(f64, expected.number_string),
                else => return error.WrongTypedNumber,
            };
            return holds(value == number);
        },
        .@"struct" => {
            if (T == c.thinkthen_complete_utf8_v1) {
                try holds(expected == .string);
                return holds(std.mem.eql(u8, tt.session.bytes(value), expected.string));
            }
            if (T == c.thinkthen_complete_json_v1) {
                switch (value.kind) {
                    c.THINKTHEN_COMPLETE_JSON_NULL_V1 => return holds(expected == .null),
                    c.THINKTHEN_COMPLETE_JSON_BOOLEAN_V1 => return compare(u32, value.data.boolean, expected),
                    c.THINKTHEN_COMPLETE_JSON_NUMBER_V1 => {
                        const parsed = try std.fmt.parseFloat(f64, tt.session.bytes(value.data.number));
                        return compare(f64, parsed, expected);
                    },
                    c.THINKTHEN_COMPLETE_JSON_STRING_V1 => return compare(c.thinkthen_complete_utf8_v1, value.data.string, expected),
                    c.THINKTHEN_COMPLETE_JSON_ARRAY_V1 => return compare(c.thinkthen_complete_json_array_v1, value.data.array, expected),
                    c.THINKTHEN_COMPLETE_JSON_OBJECT_V1 => return compare(c.thinkthen_complete_json_object_v1, value.data.object, expected),
                    else => return error.InvalidNativeDiscriminator,
                }
            }
            if (@hasField(T, "presence")) {
                if (value.presence == c.THINKTHEN_COMPLETE_PRESENCE_NULL_V1) return holds(expected == .null);
                try holds(value.presence == c.THINKTHEN_COMPLETE_PRESENCE_VALUE_V1);
                return compare(@TypeOf(value.value), value.value, expected);
            }
            if (@hasField(T, "len") and @hasField(T, "data")) {
                const Entry = @typeInfo(@TypeOf(value.data)).pointer.child;
                const map = @typeInfo(Entry) == .@"struct" and @hasField(Entry, "name") and @hasField(Entry, "value") and std.meta.fields(Entry).len == 2;
                if (map) {
                    try holds(expected == .object and expected.object.count() == value.len);
                    for (0..value.len) |i| try compare(@TypeOf(value.data[i].value), value.data[i].value, try child(expected, tt.session.bytes(value.data[i].name)));
                } else {
                    try holds(expected == .array and expected.array.items.len == value.len);
                    for (0..value.len) |i| try compare(Entry, value.data[i], expected.array.items[i]);
                }
                return;
            }
            if (@hasField(T, "kind") and @hasField(T, "data")) {
                inline for (std.meta.fields(@TypeOf(value.data)), 1..) |arm, tag| {
                    if (value.kind == tag) {
                        if (comptime std.mem.eql(u8, arm.name, "null")) return holds(expected == .null);
                        return compare(arm.type, @field(value.data, arm.name), expected);
                    }
                }
                return error.InvalidNativeDiscriminator;
            }
            if (@hasField(T, "kind") and std.meta.fields(T).len == 1) {
                const label = labels.enumLabel(T, value.kind) orelse return error.InvalidNativeEnum;
                try holds(expected == .string);
                return holds(std.mem.eql(u8, label, expected.string));
            }
            if (@hasField(T, "value") and std.meta.fields(T).len == 1) return compare(@TypeOf(value.value), value.value, expected);
            inline for (std.meta.fields(T)) |field| {
                const member = @field(value, field.name);
                if (comptime std.mem.eql(u8, field.name, "extensions")) {
                    for (0..member.len) |i| {
                        const entry = member.data[i];
                        const decoded = try std.json.parseFromSlice(std.json.Value, std.heap.page_allocator, tt.session.bytes(entry.json), .{});
                        defer decoded.deinit();
                        const wanted = try child(expected, tt.session.bytes(entry.name));
                        const actual_json = try std.json.Stringify.valueAlloc(std.heap.page_allocator, decoded.value, .{});
                        defer std.heap.page_allocator.free(actual_json);
                        const wanted_json = try std.json.Stringify.valueAlloc(std.heap.page_allocator, wanted, .{});
                        defer std.heap.page_allocator.free(wanted_json);
                        try holds(std.mem.eql(u8, actual_json, wanted_json));
                    }
                } else {
                    const name = if (comptime std.mem.eql(u8, field.name, "true_")) "true" else if (comptime std.mem.eql(u8, field.name, "false_")) "false" else field.name;
                    const wanted = if (expected == .object) expected.object.get(name) else null;
                    if (@typeInfo(field.type) == .@"struct" and @hasField(field.type, "presence")) {
                        if (wanted == null) try holds(member.presence == c.THINKTHEN_COMPLETE_PRESENCE_MISSING_V1) else try compare(field.type, member, wanted.?);
                    } else try compare(field.type, member, wanted orelse return error.MissingTypedField);
                }
            }
        },
        else => return error.UnsupportedTypedView,
    }
}
