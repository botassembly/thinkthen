const std = @import("std");
const tt = @import("thinkthen");

// "type-case REQUEST" prints one JSON-door reply. "type-case fields REQUEST"
// reads each annotate row member through readField. "type-case plan VERB
// QUESTION SETTINGS TEXT..." prints the plan object. "type-case limits" checks
// ticket 0291's zero budgets and zero cap; "type-case helper" checks
// readField's edge table.
fn output(bytes: []const u8) void {
    _ = std.os.linux.write(1, bytes.ptr, bytes.len);
}
fn require(ok: bool) !void {
    if (!ok) return error.AssertionFailed;
}
fn printFailure(alloc: std.mem.Allocator, failure: tt.Failure) !void {
    const json = try std.fmt.allocPrint(alloc, "{{\"failed\":{{\"kind\":\"{s}\",\"code\":{d}}}}}\n", .{ @tagName(failure.kind), @intFromEnum(failure.kind) });
    defer alloc.free(json);
    output(json);
}
fn state(alloc: std.mem.Allocator, member: std.json.Value) ![]u8 {
    return switch (try tt.readField(member)) {
        .unresolved => alloc.dupe(u8, "unresolved"),
        .answered => alloc.dupe(u8, "answered"),
        .failed => |failure| std.fmt.allocPrint(alloc, "failed {s} {s}", .{ @tagName(failure.kind), failure.cause }),
    };
}
/// ADR 0112 section 4: null is unresolved, {"failed": ...} is a failure whose
/// unknown extra member reads without error, and any other value is answered.
fn helper(alloc: std.mem.Allocator) !void {
    const cases = [_][2][]const u8{
        .{ "null", "unresolved" },
        .{ "true", "answered" },
        .{ "\"billing\"", "answered" },
        .{ "[\"billing\",\"urgent\"]", "answered" },
        .{ "1.2", "answered" },
        .{ "{\"failed\":{\"kind\":\"backend\",\"cause\":\"missing_probability\",\"later\":1}}", "failed backend missing_probability" },
    };
    for (cases) |case| {
        const parsed = try std.json.parseFromSlice(std.json.Value, alloc, case[0], .{});
        defer parsed.deinit();
        const read = try state(alloc, parsed.value);
        defer alloc.free(read);
        try require(std.mem.eql(u8, read, case[1]));
    }
    for ([_][]const u8{ "{\"failed\":null}", "{\"team\":\"billing\"}", "{\"failed\":{\"kind\":\"later\",\"cause\":\"x\"}}" }) |text| {
        const parsed = try std.json.parseFromSlice(std.json.Value, alloc, text, .{});
        defer parsed.deinit();
        try std.testing.expectError(error.NotAField, tt.readField(parsed.value));
    }
    output("{\"helper\":\"pass\"}\n");
}
fn refusedAs(engine: *tt.Engine, result: anytype, kind: tt.FailureKind) !void {
    switch (result) {
        .ok => |value| {
            defer switch (@typeInfo(@TypeOf(value))) {
                .pointer => engine.allocator.free(value),
                else => value.deinit(engine.allocator),
            };
            return error.ExpectedRefusal;
        },
        .failed => |failure| {
            defer engine.freeFailure(failure);
            try require(failure.kind == kind and failure.code == @intFromEnum(kind));
            if (kind == .usage) try require(std.mem.indexOf(u8, failure.message, "process send budget") != null);
        },
    }
}
/// Ticket 0291: a zero cap and a zero budget each refuse before sending.
/// Relate gets two entities, since one entity has no pair to ask.
fn limits(alloc: std.mem.Allocator, engine: *tt.Engine) !void {
    var capped = switch (try tt.Engine.initWithSettings(alloc, "{\"max_requests_total\":0,\"cache\":false}")) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(alloc, failure);
            return error.CappedEngine;
        },
    };
    defer capped.deinit();
    try refusedAs(&capped, try capped.decide("Is it?", "capped", .{}), .usage);
    const zero: tt.Options = .{ .deadline_ms = 0 };
    try refusedAs(engine, try engine.call("{\"decide\":\"Is it?\",\"evidence\":\"zero-call\"}", zero), .deadline);
    try refusedAs(engine, try engine.recognize("{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}", "zero-recognize", zero), .deadline);
    const pair = [_][]const u8{ "{\"name\":\"A\",\"kind\":\"alert\"}", "{\"name\":\"B\",\"kind\":\"alert\"}" };
    try refusedAs(engine, try engine.relate("{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}", &pair, zero), .deadline);
    output("{\"limits\":\"pass\"}\n");
}
fn fields(alloc: std.mem.Allocator, reply: []const u8) !void {
    const parsed = try std.json.parseFromSlice(std.json.Value, alloc, reply, .{});
    defer parsed.deinit();
    var out: std.Io.Writer.Allocating = .init(alloc);
    defer out.deinit();
    try out.writer.writeByte('[');
    for (parsed.value.object.get("value").?.array.items, 0..) |row, i| {
        try out.writer.writeAll(if (i == 0) "{" else ",{");
        var members = row.object.iterator();
        var first = true;
        while (members.next()) |entry| : (first = false) {
            const read = try state(alloc, entry.value_ptr.*);
            defer alloc.free(read);
            try out.writer.print("{s}\"{s}\":\"{s}\"", .{ if (first) "" else ",", entry.key_ptr.*, read });
        }
        try out.writer.writeByte('}');
    }
    try out.writer.writeAll("]\n");
    output(out.written());
}

pub fn main() !void {
    const alloc = std.heap.page_allocator;
    const args = try std.process.argsAlloc(alloc);
    defer std.process.argsFree(alloc, args);
    if (args.len < 2) return error.OneRequestRequired;
    const constructed = try tt.Engine.init(alloc);
    var engine = switch (constructed) {
        .ok => |value| value,
        .failed => |failure| {
            defer alloc.free(failure.message);
            if (failure.facts_json) |facts| alloc.free(facts);
            return error.NativeConstructorFailed;
        },
    };
    defer engine.deinit();
    if (args.len == 2 and std.mem.eql(u8, args[1], "helper")) return helper(alloc);
    if (args.len == 2 and std.mem.eql(u8, args[1], "limits")) return limits(alloc, &engine);
    if (args.len >= 5 and std.mem.eql(u8, args[1], "plan")) {
        const texts = try alloc.alloc([]const u8, args.len - 5);
        defer alloc.free(texts);
        for (args[5..], texts) |arg, *text| text.* = arg;
        switch (try engine.plan(args[2], args[3], texts, args[4])) {
            .ok => |plan| {
                defer plan.deinit();
                const json = try std.json.Stringify.valueAlloc(alloc, plan.value, .{});
                defer alloc.free(json);
                output(json);
                output("\n");
            },
            .failed => |failure| {
                defer engine.freeFailure(failure);
                try printFailure(alloc, failure);
            },
        }
        return;
    }
    const fields_mode = args.len == 3 and std.mem.eql(u8, args[1], "fields");
    if (args.len != 2 and !fields_mode) return error.OneRequestRequired;
    const request = try alloc.dupeZ(u8, args[args.len - 1]);
    defer alloc.free(request);
    const result = try engine.call(request, .{});
    switch (result) {
        .ok => |bytes| {
            defer alloc.free(bytes);
            if (fields_mode) return fields(alloc, bytes);
            output(bytes);
            output("\n");
        },
        .failed => |failure| {
            defer engine.freeFailure(failure);
            try printFailure(alloc, failure);
        },
    }
}
