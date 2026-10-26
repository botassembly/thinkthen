const std = @import("std");
const tt = @import("thinkthen");
const A = std.mem.Allocator;
const V = std.json.Value;
fn parse(comptime T: type, a: A, value: V) !T {
    return std.json.parseFromValueLeaky(T, a, value, .{ .allocate = .alloc_always });
}
fn choices(a: A, value: V) ![]const tt.Choice {
    const result = try a.alloc(tt.Choice, value.array.items.len);
    for (value.array.items, result) |entry, *slot| {
        slot.* = .{ .name = entry.string };
    }
    return result;
}
pub fn question(comptime function: tt.Function, a: A, value: V) !tt.QuestionType(function) {
    var result: tt.QuestionType(function) = undefined;
    result.options = .{};
    inline for (@typeInfo(@TypeOf(result.options)).@"struct".field_names) |name| {
        if (value.object.get(name)) |child| {
            if (comptime std.mem.eql(u8, name, "threshold") and (function == .decide or function == .choose or function == .filter)) {
                result.options.threshold = if (child == .object) tt.Threshold.band(try parse(f64, a, child.object.get("low").?), try parse(f64, a, child.object.get("high").?)) else .{ .cut = try parse(f64, a, child) };
            } else @field(result.options, name) = try parse(@TypeOf(@field(result.options, name)), a, child);
        }
    }
    if (@hasField(@TypeOf(result), "question")) result.question = value.object.get("question").?.string;
    inline for (.{ "choices", "labels", "levels", "kinds" }) |name| if (@hasField(@TypeOf(result), name)) {
        @field(result, name) = try choices(a, value.object.get(name).?);
    };
    if (function == .relate) {
        const values = value.object.get("rules").?.array.items;
        const rules = try a.alloc(tt.RelationRule, values.len);
        for (values, rules) |entry, *rule| {
            rule.* = .{ .name = entry.object.get("relation").?.string, .source = entry.object.get("source").?.string, .target = entry.object.get("target").?.string };
            if (entry.object.get("reads")) |v| rule.reads = v.string;
            if (entry.object.get("either")) |v| rule.either = v.bool;
        }
        result.rules = rules;
    }
    if (function == .annotate) {
        const values = value.object.get("questions").?.object;
        const entries = try a.alloc(tt.NamedAnnotationQuestion, values.count());
        var iterator = values.iterator();
        var i: usize = 0;
        while (iterator.next()) |entry| : (i += 1) {
            entries[i].name = entry.key_ptr.*;
            const name = entry.value_ptr.object.get("function").?.string;
            inline for (@typeInfo(tt.AnnotationQuestion).@"union".field_names) |tag| {
                if (std.mem.eql(u8, name, tag)) entries[i].question = @unionInit(tt.AnnotationQuestion, tag, try question(@field(tt.Function, tag), a, entry.value_ptr.*));
            }
        }
        result.questions = .{ .entries = entries };
    }
    return result;
}

pub fn settings(a: A, raw: []const u8) !tt.Settings {
    const value = try std.json.parseFromSliceLeaky(V, a, raw, .{});
    var result: tt.Settings = .{};
    inline for (@typeInfo(tt.Settings).@"struct".field_names) |name| {
        if (comptime !std.mem.eql(u8, name, "api_key")) {
            if (value.object.get(name)) |child| @field(result, name) = try parse(@TypeOf(@field(result, name)), a, child);
        }
    }
    return result;
}
