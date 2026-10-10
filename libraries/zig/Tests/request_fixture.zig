//! Decode fixture JSON into generated request types without a second grammar.
const std = @import("std");
pub fn decode(comptime T: type, a: std.mem.Allocator, value: std.json.Value) anyerror!T {
    if (T == std.json.Value) return value;
    switch (@typeInfo(T)) {
        .optional => |info| return if (value == .null) null else try decode(info.child, a, value),
        .bool => return if (value == .bool) value.bool else error.FixtureType,
        .int => return if (value == .integer) std.math.cast(T, value.integer) orelse error.FixtureType else error.FixtureType,
        .float => return switch (value) {
            .integer => @floatFromInt(value.integer),
            .float => @floatCast(value.float),
            else => error.FixtureType,
        },
        .void => return if (value == .null) {} else error.FixtureType,
        .@"enum" => return if (value == .string) std.meta.stringToEnum(T, value.string) orelse error.FixtureType else error.FixtureType,
        .pointer => |info| {
            if (T == []const u8) return if (value == .string) value.string else error.FixtureType;
            if (value != .array or info.size != .slice) return error.FixtureType;
            const out = try a.alloc(info.child, value.array.items.len);
            for (out, value.array.items) |*slot, item| slot.* = try decode(info.child, a, item);
            return out;
        },
        .@"union" => |info| {
            inline for (info.fields) |arm| {
                if (decode(arm.type, a, value)) |answer| return @unionInit(T, arm.name, answer) else |err| if (err == error.OutOfMemory) return err;
            }
            return error.FixtureVariant;
        },
        .@"struct" => |info| {
            if (value != .object) return error.FixtureType;
            if (@hasField(T, "entries")) {
                const Entry = @typeInfo(@TypeOf(@as(T, undefined).entries)).pointer.child;
                const entries = try a.alloc(Entry, value.object.count());
                var it = value.object.iterator();
                var i: usize = 0;
                while (it.next()) |entry| : (i += 1) entries[i] = .{ .key = entry.key_ptr.*, .value = try decode(@TypeOf(@as(Entry, undefined).value), a, entry.value_ptr.*) };
                return .{ .entries = entries };
            }
            var out: T = undefined;
            inline for (info.fields) |member| {
                if (value.object.get(member.name)) |item| @field(out, member.name) = try decode(member.type, a, item) else if (member.defaultValue()) |default| @field(out, member.name) = default else return error.FixtureMissing;
            }
            return out;
        },
        else => return error.FixtureType,
    }
}
