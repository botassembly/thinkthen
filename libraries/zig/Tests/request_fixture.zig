//! Decode fixture JSON into generated request types without a second grammar.
const std = @import("std");
const tt = @import("thinkthen");
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
pub fn projectAnswers(packet: anytype, packets: []const *tt.session.Packet, a: std.mem.Allocator) !@TypeOf(packet.value().*) {
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
        inline .decide_row, .choose_row, .tag_row, .score_row, .annotate_row => |*v| try projectReceipt(&v.value, precedingObservation(packet, packets), a),
        inline .decide_aggregate, .choose_aggregate, .tag_aggregate, .score_aggregate, .annotate_aggregate, .recognize_aggregate => |*v| {
            const receipts = try a.dupe(@typeInfo(@TypeOf(v.value)).pointer.child, v.value);
            for (receipts, 0..) |*receipt, i| try projectReceipt(receipt, observationAt(packets, i), a);
            v.value = receipts;
        },
        inline .find_aggregate, .relate_aggregate => |*v| try projectReceipt(&v.value, observationAt(packets, 0), a),
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

fn observationAt(packets: []const *tt.session.Packet, ordinal: usize) *tt.session.Packet {
    var i: usize = 0;
    for (packets) |packet| if (packet.value().* == .observation and packet.value().observation.value == .row) {
        if (i == ordinal) return packet;
        i += 1;
    };
    @panic("missing answer observation");
}

fn precedingObservation(target: *tt.session.Packet, packets: []const *tt.session.Packet) *tt.session.Packet {
    var previous: ?*tt.session.Packet = null;
    for (packets) |packet| {
        if (packet == target) return previous orelse @panic("missing row observation");
        if (packet.value().* == .observation and packet.value().observation.value == .row) previous = packet;
    }
    @panic("missing row packet");
}

/// Only answer values come from helpers; the receipt retains its native metadata.
fn projectReceipt(receipt: anytype, observation: *tt.session.Packet, a: std.mem.Allocator) !void {
    const T = @typeInfo(@TypeOf(receipt)).pointer.child;
    const answers = observation.answers();
    if (T == tt.results.AtomicDecideValue) {
        receipt.value = if (try answers.decide()) |v| .{ .value = .{ .boolean = v } } else .null_value;
    } else if (T == tt.results.AtomicNullableString) {
        receipt.value = if (try answers.choose()) |v| .{ .value = v } else .null_value;
    } else if (T == tt.results.AtomicDouble) {
        receipt.value = try answers.score();
    } else if (T == tt.results.AtomicArrayOfString) {
        receipt.value = try answers.tag();
    } else if (T == tt.results.Find) {
        receipt.value = if (try answers.find()) |v| .{ .value = .{ .number = .{ .unsigned = v } } } else .null_value;
    } else if (T == tt.results.Annotation) {
        const annotations = try answers.annotate();
        const entries = try a.alloc(@typeInfo(@TypeOf(receipt.value.entries)).pointer.child, annotations.len);
        for (entries, annotations) |*entry, annotation| entry.* = .{ .key = annotation.name, .value = switch (annotation.value) {
            .decision => |v| if (v.value.optional()) |answer| .{ .boolean = answer } else .null_value,
            .choice => |v| if (v.value.optional()) |answer| .{ .string = answer } else .null_value,
            .score => |v| .{ .number = v.value },
            .tags => |v| .{ .array = v.value },
            .failed => |v| .{ .object = .{ .failed = v.value, .extensions = .{ .entries = &.{} } } },
        } };
        receipt.value.entries = entries;
    } else if (T == tt.results.Recognition) {
        const recognized = try answers.recognize();
        if (recognized.mode == .boundary_only) {
            receipt.value = .{ .mode_proposals = .{ .mode = .boundary_only, .proposals = recognized.proposals.optional().?, .extensions = .{ .entries = &.{} } } };
        } else {
            var relations: tt.results.Presence([]const tt.results.EntityEdge) = .missing;
            if (recognized.relations.optional()) |edges| {
                const converted = try a.alloc(tt.results.EntityEdge, edges.len);
                for (converted, edges) |*edge, answer| edge.* = .{ .either = if (answer.either) .{ .value = true } else .missing, .probability = answer.probability, .relation = answer.relation, .source = answer.source, .target = answer.target, .extensions = answer.extensions };
                relations = .{ .value = converted };
            }
            receipt.value = .{ .entities = .{ .entities = recognized.entities, .relations = relations, .extensions = .{ .entries = &.{} } } };
        }
    } else if (T == tt.results.Relation) {
        const edges = try answers.relate();
        if (edges.len != receipt.value.len) return error.RelationCount;
        const converted = try a.dupe(tt.results.RelatedEntityEdge, receipt.value);
        for (converted, edges) |*edge, answer| {
            edge.either = if (answer.either) .{ .value = true } else .missing;
            edge.probability = answer.probability;
            edge.relation = answer.relation;
            switch (edge.source) {
                inline else => |*source| {
                    source.kind = answer.source.kind;
                    source.name = answer.source.name;
                },
            }
            switch (edge.target) {
                inline else => |*target| {
                    target.kind = answer.target.kind;
                    target.name = answer.target.name;
                },
            }
        }
        receipt.value = converted;
    } else @compileError("unsupported answer receipt");
}
