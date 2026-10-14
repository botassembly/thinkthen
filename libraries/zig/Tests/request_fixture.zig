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

pub fn writeNative(value: anytype, writer: anytype) error{WriteFailed}!void {
    const T = @TypeOf(value);
    switch (@typeInfo(T)) {
        .@"union" => switch (value) {
            inline else => |member| {
                if (@TypeOf(member) == void) try writer.write(null) else try writeNative(member, writer);
            },
        },
        .@"struct" => {
            try writer.beginObject();
            if (@hasField(T, "entries")) {
                for (value.entries) |entry| {
                    try writer.objectField(entry.key);
                    try writeNative(entry.value, writer);
                }
            } else inline for (std.meta.fields(T)) |field| {
                const member = @field(value, field.name);
                if (comptime std.mem.eql(u8, field.name, "extensions")) {
                    for (member.entries) |entry| {
                        try writer.objectField(entry.key);
                        try writeNative(entry.value, writer);
                    }
                } else {
                    const omit = if (comptime @typeInfo(field.type) == .@"union" and @hasField(field.type, "missing")) member == .missing else false;
                    if (!omit) {
                        try writer.objectField(field.name);
                        try writeNative(member, writer);
                    }
                }
            }
            try writer.endObject();
        },
        .pointer => |info| {
            if (T == []const u8) return writer.write(value);
            if (info.size == .one) return writeNative(value.*, writer);
            try writer.beginArray();
            for (value) |member| try writeNative(member, writer);
            try writer.endArray();
        },
        else => try writer.write(value),
    }
}

/// Feed native answer helpers back through the shared outside-in expectations.
pub fn projectAnswers(packet: anytype, a: std.mem.Allocator) !@TypeOf(packet.value().*) {
    var projected = packet.value().*;
    const answers = packet.answers();
    switch (projected) {
        .observation => |*observation| switch (observation.value) {
            .row => |*row| switch (row.value) {
                .judgment => |*judgment| switch (judgment.value) {
                    .decision => |*v| v.value = if (try answers.decide()) |answer| .{ .value = answer } else .null_value,
                    .choice => |*v| v.value = if (try answers.choose()) |answer| .{ .value = answer } else .null_value,
                    .score => |*v| v.value = try answers.score(),
                    .tags => |*v| v.value = try answers.tag(),
                },
                .find => |*v| v.value = if (try answers.find()) |answer| .{ .value = answer } else .null_value,
                .annotated => |*v| v.value = try answers.annotate(),
                .recognized => |*v| v.value = try answers.recognize(),
                .relations => |*v| v.value = try answers.relate(),
            },
            else => {},
        },
        .filter_row => |*v| v.value.input = .{ .value = (try answers.filter())[0] },
        .rank_aggregate => |*v| {
            const indices = try answers.rank();
            const receipts = try a.dupe(@typeInfo(@TypeOf(v.value)).pointer.child, v.value);
            for (receipts, indices) |*receipt, index| receipt.index = .{ .value = index };
            v.value = receipts;
        },
        .filter_aggregate => |*v| {
            const originals = try answers.filter();
            const receipts = try a.dupe(@typeInfo(@TypeOf(v.value)).pointer.child, v.value);
            for (receipts, originals) |*receipt, input| receipt.input = .{ .value = input };
            v.value = receipts;
        },
        else => {},
    }
    return projected;
}
