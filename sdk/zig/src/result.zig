const std = @import("std");
const r = @import("row.zig");
const q = @import("questions.zig");
const d = @import("diagnostic.zig");
const t = @import("types.zig");
const c = @import("c");
const native = @import("native.zig");
const A = std.mem.Allocator;
pub fn Answer(comptime Q: type, comptime Record: type) type {
    return switch (Q.function) {
        .choose => Q.Choice,
        .tag => if (Q.Choice == []const u8) []const []const u8 else std.EnumSet(Q.Choice),
        .annotate => if (Q == q.DynamicAnnotate) DynamicAnnotations(Record) else blk: {
            const info = @typeInfo(Q.Questions).@"struct";
            var fields: [info.field_names.len]type = undefined;
            for (info.field_types, 0..) |Child, i| fields[i] = Row(Child, Record);
            break :blk @Struct(.auto, null, info.field_names, &fields, &@splat(.{}));
        },
        else => r.Answer(Q.function),
    };
}
pub fn Row(comptime Q: type, comptime Record: type) type {
    return struct {
        status: r.Status,
        probability: ?f64,
        failure: ?d.Failure,
        detail: ?r.Detail(Q.function),
        position: u64,
        id: ?r.RecordId,
        source: r.Source,
        item: Record,
        payload: union(r.Status) { answered: Answer(Q, Record), unsure: void, failed: void },
        saved: []const u8,
        identity: std.json.Value,
        display: []const u8,
        const Self = @This();
        pub fn answer(self: Self, diag: ?*d.Diagnostic) d.AnswerError!Answer(Q, Record) {
            d.clear(diag);
            if (diag) |target| target.position = self.position;
            return switch (self.payload) {
                .answered => |value| value,
                .unsure => {
                    if (diag) |target| {
                        target.setUnsure(self.probability.?);
                        target.position = self.position;
                    }
                    return error.Unsure;
                },
                .failed => {
                    if (diag) |target| {
                        target.set(self.failure.?);
                        target.position = self.position;
                    }
                    return error.Failed;
                },
            };
        }
        pub fn errIfFailed(self: Self, diag: ?*d.Diagnostic) error{Failed}!Self {
            d.clear(diag);
            if (self.status == .failed) {
                if (diag) |target| {
                    target.set(self.failure.?);
                    target.position = self.position;
                }
                return error.Failed;
            }
            if (comptime Q.function == .annotate) if (self.status == .answered) {
                if (Q == q.DynamicAnnotate) {
                    for (self.payload.answered.entries) |entry| switch (entry.value) {
                        inline else => |child| {
                            _ = try child.errIfFailed(diag);
                        },
                    };
                } else {
                    const fields = comptime sortedNames(Q.Questions);
                    inline for (fields) |name| _ = try @field(self.payload.answered, name).errIfFailed(diag);
                }
            };
            return self;
        }
        /// Saved rows have canonical character offsets and no original record.
        pub fn jsonParse(allocator: A, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!Self {
            if (Record != void) @compileError("saved rows contain no item; parse Row(Question, void)");
            const raw = try r.Row(Q.function).jsonParse(allocator, source, options);
            const saved = std.json.Stringify.valueAlloc(allocator, raw, .{ .emit_null_optional_fields = false }) catch return error.OutOfMemory;
            return convert(Q, Record, allocator, raw, saved, {}) catch |failure| return if (failure == error.OutOfMemory) error.OutOfMemory else error.UnexpectedToken;
        }
        pub fn eql(self: Self, other: Self) bool {
            if (self.status != other.status or self.probability != other.probability or self.position != other.position or !equalId(self.id, other.id) or !equalSource(self.source, other.source) or !equalFailure(self.failure, other.failure)) return false;
            if (!equalOptionalValue(self.identity.object.get("detail"), other.identity.object.get("detail"))) return false;
            if (self.status != .answered) return true;
            if (Q.function == .annotate) {
                if (Q == q.DynamicAnnotate) {
                    const left = self.payload.answered.entries;
                    const right = other.payload.answered.entries;
                    if (left.len != right.len) return false;
                    for (left, right) |a, b| {
                        if (!std.mem.eql(u8, a.name, b.name) or std.meta.activeTag(a.value) != std.meta.activeTag(b.value)) return false;
                        switch (a.value) {
                            inline else => |child, tag| if (!child.eql(@field(b.value, @tagName(tag)))) return false,
                        }
                    }
                } else inline for (@typeInfo(Q.Questions).@"struct".field_names) |name| {
                    if (!@field(self.payload.answered, name).eql(@field(other.payload.answered, name))) return false;
                }
                return true;
            }
            if (Q.function == .tag and Q.Choice != []const u8) return std.meta.eql(self.payload.answered, other.payload.answered);
            return equalOptionalValue(self.identity.object.get("answer"), other.identity.object.get("answer"));
        }
        pub fn format(self: Self, writer: *std.Io.Writer) std.Io.Writer.Error!void {
            try writer.writeAll(self.display);
        }
        pub fn jsonStringify(self: Self, writer: anytype) !void {
            try writer.print("{s}", .{self.saved});
        }
    };
}
fn sortedNames(comptime T: type) [@typeInfo(T).@"struct".field_names.len][:0]const u8 {
    var names = @typeInfo(T).@"struct".field_names.*;
    std.mem.sort([:0]const u8, &names, {}, struct {
        fn less(_: void, left: [:0]const u8, right: [:0]const u8) bool {
            return std.mem.lessThan(u8, left, right);
        }
    }.less);
    return names;
}
fn convert(comptime Q: type, comptime Record: type, arena: A, host: r.Row(Q.function), saved: []const u8, item: Record) d.Error!Row(Q, Record) {
    const state = host.status();
    const identity = std.json.parseFromSliceLeaky(std.json.Value, arena, saved, .{ .allocate = .alloc_always, .parse_numbers = false }) catch |failure| return if (failure == error.OutOfMemory) error.OutOfMemory else error.Internal;
    const canonical: r.Row(Q.function) = .{ .position = host.position, .id = host.id, .source = host.source, .outcome = r.outcome(Q.function, arena, identity, .{ .allocate = .alloc_if_needed }) catch |failure| return if (failure == error.OutOfMemory) error.OutOfMemory else error.Internal };
    const display = try std.fmt.allocPrint(arena, "{f}", .{canonical});
    var result: Row(Q, Record) = .{ .status = state, .probability = host.probability(), .failure = if (state == .failed) host.outcome.failed else null, .detail = switch (host.outcome) {
        .answered => |v| v.detail,
        .unsure => |v| v.detail,
        .failed => null,
    }, .position = host.position, .id = host.id, .source = host.source, .item = item, .payload = undefined, .saved = saved, .identity = identity, .display = display };
    result.payload = switch (state) {
        .failed => .failed,
        .unsure => .unsure,
        .answered => .{ .answered = blk: {
            const value = host.outcome.answered.answer;
            if (Q.function == .choose) {
                if (Q.Choice == []const u8) break :blk value;
                break :blk std.meta.stringToEnum(Q.Choice, value) orelse return error.Internal;
            }
            if (Q.function == .tag) {
                if (Q.Choice == []const u8) break :blk value;
                var set: std.EnumSet(Q.Choice) = .{};
                for (value) |word| set.insert(std.meta.stringToEnum(Q.Choice, word) orelse return error.Internal);
                break :blk set;
            }
            if (Q.function == .annotate) {
                if (Q == q.DynamicAnnotate) {
                    const entries = try arena.alloc(NamedRow(Record), value.entries.len);
                    for (value.entries, entries) |entry, *slot| {
                        slot.name = entry.name;
                        switch (entry.value) {
                            inline else => |_, tag| {
                                slot.value = @unionInit(AnyRow(Record), @tagName(tag), try projectChild(q.QuestionType(@field(t.Function, @tagName(tag))), Record, arena, host, identity, entry.name, item));
                            },
                        }
                    }
                    break :blk .{ .entries = entries };
                }
                var entries: Answer(Q, Record) = undefined;
                inline for (@typeInfo(Q.Questions).@"struct".field_names, @typeInfo(Q.Questions).@"struct".field_types) |name, Child| {
                    @field(entries, name) = try projectChild(Child, Record, arena, host, identity, name, item);
                }
                break :blk entries;
            }
            break :blk value;
        } },
    };
    return result;
}
pub fn take(comptime Q: type, comptime Record: type, arena: A, row: c.thinkthen_row, item: Record) d.Error!Row(Q, Record) {
    var host_buffer: c.thinkthen_buffer = .{};
    var saved_buffer: c.thinkthen_buffer = .{};
    var err = d.emptyError();
    try d.check(native.function("thinkthen_row_host_json")(&row, &host_buffer, &err), &err, null);
    defer native.function("thinkthen_buffer_free")(&host_buffer);
    try d.check(native.function("thinkthen_row_to_json")(&row, &saved_buffer, &err), &err, null);
    defer native.function("thinkthen_buffer_free")(&saved_buffer);
    const saved = try arena.dupe(u8, d.bytes(saved_buffer));
    const options: std.json.ParseOptions = .{ .allocate = .alloc_always, .parse_numbers = false };
    const host = std.json.parseFromSliceLeaky(r.Row(Q.function), arena, d.bytes(host_buffer), options) catch |failure| return if (failure == error.OutOfMemory) error.OutOfMemory else error.Internal;
    return convert(Q, Record, arena, host, saved, item);
}

pub fn AnyRow(comptime Record: type) type {
    return union(enum) { decide: Row(q.Decide, Record), choose: Row(q.Choose([]const u8), Record), tag: Row(q.Tag([]const u8), Record), score: Row(q.Score([]const u8), Record), find: Row(q.Find, Record), recognize: Row(q.Recognize, Record), relate: Row(q.Relate, Record) };
}
pub fn NamedRow(comptime Record: type) type {
    return struct { name: []const u8, value: AnyRow(Record) };
}
pub fn DynamicAnnotations(comptime Record: type) type {
    return struct {
        entries: []const NamedRow(Record),
        pub fn get(self: @This(), comptime function: t.Function, name: []const u8, diag: ?*d.Diagnostic) error{Usage}!Row(q.QuestionType(function), Record) {
            if (comptime !@hasField(AnyRow(Record), @tagName(function))) @compileError("annotate cannot hold this function");
            d.clear(diag);
            for (self.entries) |entry| if (std.mem.eql(u8, entry.name, name)) {
                if (std.mem.eql(u8, @tagName(std.meta.activeTag(entry.value)), @tagName(function))) return @field(entry.value, @tagName(function));
                break;
            };
            if (diag) |target| target.set(.{ .code = .usage_question, .message = "the form has no entry of that name and function" });
            return error.Usage;
        }
    };
}

fn projectChild(comptime Q: type, comptime Record: type, arena: A, parent: r.Row(.annotate), canonical: std.json.Value, name: []const u8, item: Record) d.Error!Row(Q, Record) {
    const host: r.Row(Q.function) = .{ .position = parent.position, .id = parent.id, .source = parent.source, .outcome = parent.outcome.answered.answer.get(Q.function, name) orelse return error.Internal };
    const entry = canonical.object.get("answer").?.object.get(name) orelse return error.Internal;
    var standalone: std.json.ObjectMap = .{};
    var fields = entry.object.iterator();
    while (fields.next()) |field| try standalone.put(arena, field.key_ptr.*, field.value_ptr.*);
    inline for (.{ "position", "id", "source" }) |field| if (canonical.object.get(field)) |value| try standalone.put(arena, field, value);
    const saved = try std.json.Stringify.valueAlloc(arena, std.json.Value{ .object = standalone }, .{});
    return convert(Q, Record, arena, host, saved, item);
}

fn equalValue(left: std.json.Value, right: std.json.Value) bool {
    if (std.meta.activeTag(left) != std.meta.activeTag(right)) return false;
    return switch (left) {
        .null => true,
        .bool => |value| value == right.bool,
        .integer => |value| value == right.integer,
        .float => |value| value == right.float,
        .string => |value| std.mem.eql(u8, value, right.string),
        .number_string => |value| blk: {
            const a_integer: ?i128 = std.fmt.parseInt(i128, value, 10) catch null;
            const b_integer: ?i128 = std.fmt.parseInt(i128, right.number_string, 10) catch null;
            if (a_integer != null and b_integer != null) break :blk a_integer.? == b_integer.?;
            const a = std.fmt.parseFloat(f64, value) catch break :blk false;
            const b = std.fmt.parseFloat(f64, right.number_string) catch break :blk false;
            break :blk a == b;
        },
        .array => |array| blk: {
            if (array.items.len != right.array.items.len) break :blk false;
            for (array.items, right.array.items) |a, b| if (!equalValue(a, b)) break :blk false;
            break :blk true;
        },
        .object => |object| blk: {
            if (object.count() != right.object.count()) break :blk false;
            var iterator = object.iterator();
            while (iterator.next()) |entry| {
                const other = right.object.get(entry.key_ptr.*) orelse break :blk false;
                if (!equalValue(entry.value_ptr.*, other)) break :blk false;
            }
            break :blk true;
        },
    };
}

fn equalText(left: ?[]const u8, right: ?[]const u8) bool {
    return if (left) |a| if (right) |b| std.mem.eql(u8, a, b) else false else right == null;
}
fn equalSource(left: r.Source, right: r.Source) bool {
    return left.backend == right.backend and std.mem.eql(u8, left.question_identity, right.question_identity) and std.mem.eql(u8, left.model, right.model) and equalText(left.name, right.name);
}
fn equalId(left: ?r.RecordId, right: ?r.RecordId) bool {
    if (left) |a| {
        const b = right orelse return false;
        if (std.meta.activeTag(a) != std.meta.activeTag(b)) return false;
        return switch (a) {
            .string => |value| std.mem.eql(u8, value, b.string),
            .signed => |value| value == b.signed,
            .unsigned => |value| value == b.unsigned,
        };
    }
    return right == null;
}

fn equalOptionalValue(left: ?std.json.Value, right: ?std.json.Value) bool {
    return if (left) |a| if (right) |b| equalValue(a, b) else false else right == null;
}
fn equalFailure(left: ?d.Failure, right: ?d.Failure) bool {
    if (left) |a| {
        const b = right orelse return false;
        return std.mem.eql(u8, a.name(), b.name()) and std.mem.eql(u8, a.message, b.message) and a.http_status == b.http_status;
    }
    return right == null;
}
