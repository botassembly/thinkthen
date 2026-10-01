const std = @import("std");
const tt = @import("thinkthen");
fn require(ok: bool) !void {
    if (!ok) return error.AssertionFailed;
}
/// Facts are host JSON; a missing member reads as null.
fn fact(facts: tt.Json, name: []const u8) std.json.Value {
    return facts.value.object.get(name) orelse .null;
}
fn count(facts: tt.Json, name: []const u8) i64 {
    return switch (fact(facts, name)) {
        .integer => |value| value,
        else => -1,
    };
}
fn errorCode(engine: *tt.Engine, result: anytype, code: c_int, retryable: bool) !void {
    switch (result) {
        .ok => |value| {
            defer switch (@typeInfo(@TypeOf(value))) {
                .pointer => engine.allocator.free(value),
                .@"struct" => value.deinit(engine.allocator),
                else => unreachable,
            };
            return error.ExpectedFailure;
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            try require(f.code == code and f.retryable == retryable and f.message.len > 0);
        },
    }
}
fn call(engine: *tt.Engine, allocator: std.mem.Allocator, request: [:0]const u8) !std.json.Parsed(std.json.Value) {
    const result = try engine.call(request, .{});
    const bytes = switch (result) {
        .ok => |v| v,
        .failed => |failure| {
            defer engine.freeFailure(failure);
            std.debug.print("request failed: {s} => {s}\n", .{ request, failure.message });
            return error.UnexpectedDomainFailure;
        },
    };
    defer allocator.free(bytes);
    std.debug.print("REP IN DRIFT JSON request={s} result={s}\n", .{ request, bytes });
    return try std.json.parseFromSlice(std.json.Value, allocator, bytes, .{});
}
pub fn main() !void {
    var gpa = std.heap.DebugAllocator(.{}){};
    defer if (gpa.deinit() != .ok) @panic("wrapper allocation leak");
    const alloc = gpa.allocator();
    var engine = switch (try tt.Engine.init(alloc)) {
        .ok => |v| v,
        .failed => |f| {
            defer alloc.free(f.message);
            std.debug.print("engine: {s}\n", .{f.message});
            return error.EngineBuild;
        },
    };
    defer engine.deinit();
    const scalar = [_][]const u8{ "yes", "no", "unsure", "caf\xc3\xa9", "a\x00b" };
    for (scalar, 0..) |text, i| {
        const result = try engine.decide(if (i == 2) "{\"decide\":\"Is it?\",\"threshold\":\"0.4:0.8\"}" else "Is this text?", text, .{});
        switch (result) {
            .ok => |a| {
                defer a.deinit(alloc);
                try require(count(a.facts, "records") == 1 and count(a.facts, "requests_sent") == 1 and count(a.facts, "cache_answers") == 0 and fact(a.facts, "seconds") == .float);
                try require(a.value.outcome == (if (i == 1) tt.Outcome.no else if (i == 2) tt.Outcome.unsure else tt.Outcome.yes) and a.value.probability == (if (i == 1) @as(f64, 0.1) else if (i == 2) @as(f64, 0.5) else @as(f64, 0.9)));
            },
            .failed => |f| {
                defer engine.freeFailure(f);
                return error.ScalarFailed;
            },
        }
    }
    const rows = [_][]const u8{ "first", "second", "third" };
    switch (try engine.decideMany("Is it?", &rows, .{})) {
        .ok => |success| {
            defer success.deinit(alloc);
            const answers = success.value;
            try require(answers.len == 3 and count(success.facts, "records") == 3 and count(success.facts, "requests_sent") == 1 and count(success.facts, "cache_answers") == 0);
            for (answers, 0..) |a, i| {
                if (a.probability != ([_]f64{ 0.9, 0.1, 0.6 })[i]) std.debug.print("REP IN DRIFT bulk index={d} probability={d} expected={d} outcome={d}\n", .{ i, a.probability, ([_]f64{ 0.9, 0.1, 0.6 })[i], @intFromEnum(a.outcome) });
                try require(a.probability == ([_]f64{ 0.9, 0.1, 0.6 })[i]);
            }
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.BulkFailed;
        },
    }
    switch (try engine.decideMany("Is it?", &rows, .{})) {
        .ok => |success| {
            defer success.deinit(alloc);
            try require(success.value.len == 3 and count(success.facts, "records") == 3 and count(success.facts, "requests_sent") == 0 and count(success.facts, "cache_answers") == 3);
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.CachedReplayFailed;
        },
    }
    const repeated = [_][]const u8{ "first", "second", "first", "second" };
    switch (try engine.decideMany("Is it?", &repeated, .{})) {
        .ok => |success| {
            defer success.deinit(alloc);
            const answers = success.value;
            try require(answers.len == 4);
            for (answers, 0..) |answer, i| {
                const yes = i % 2 == 0;
                try require(answer.outcome == (if (yes) tt.Outcome.yes else tt.Outcome.no));
                try require(answer.probability == (if (yes) @as(f64, 0.9) else @as(f64, 0.1)));
            }
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.RepeatedBulkFailed;
        },
    }
    const empty = [_][]const u8{};
    switch (try engine.decideMany("Is it?", &empty, .{})) {
        .ok => |success| {
            defer success.deinit(alloc);
            const answers = success.value;
            try require(answers.len == 0 and count(success.facts, "records") == 0 and fact(success.facts, "model") == .null);
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.EmptyFailed;
        },
    }
    const requests = [_][:0]const u8{
        "{\"decide\":\"Is it?\",\"evidence\":\"json-decide\",\"details\":true}",
        "{\"choose\":\"Which team?\",\"options\":[\"first\",\"second\"],\"evidence\":\"choose\"}",
        "{\"tag\":\"Which labels?\",\"labels\":[\"first\",\"second\"],\"evidence\":\"tag\"}",
        "{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"],\"evidence\":\"score\"}",
        "{\"filter\":\"Is it?\",\"records\":[\"filter-one\",\"filter-two\"]}",
        "{\"rank\":\"Is it?\",\"records\":[\"rank-one\",\"rank-two\"]}",
        "{\"find\":\"Which line?\",\"units\":[\"find-one\",\"find-two\"]}",
        "{\"annotate\":{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\"}}},\"records\":[\"annotate-one\"]}",
        "{\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}},\"version\":1,\"evidence\":\"Maria Chen\"}",
        "{\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]},\"version\":1,\"records\":[{\"name\":\"First\",\"kind\":\"alert\"},{\"name\":\"Second\",\"kind\":\"alert\"}]}",
    };
    for (requests, 0..) |request, i| {
        const parsed = try call(&engine, alloc, request);
        defer parsed.deinit();
        // libraries/c/README.md "Run facts" and result.schema.json $defs/callSuccess:
        // the C JSON door wraps each asking verb as exactly value + facts.
        const envelope = parsed.value.object;
        try require(envelope.count() == 2 and envelope.contains("value") and envelope.contains("facts"));
        const facts = envelope.get("facts").?.object;
        try require(facts.count() == 7 and facts.get("records").?.integer >= 1 and facts.get("requests_sent").?.integer >= 0 and facts.get("cache_answers").?.integer >= 0 and facts.get("seconds").?.float >= 0 and facts.get("input_tokens").?.integer >= 0 and facts.get("output_tokens").?.integer >= 0 and std.mem.eql(u8, facts.get("model").?.string, "jev-1.13.0"));
        const value = envelope.get("value").?;
        switch (i) {
            0 => {
                try require(value == .object and std.mem.eql(u8, value.object.get("schema").?.string, "thinkthen.result/1"));
                try require(value.object.get("value").?.bool and value.object.get("answer").?.object.get("probability").?.float == 0.9 and std.mem.eql(u8, value.object.get("answer").?.object.get("kind").?.string, "yes_no"));
                const meta = value.object.get("meta").?.object;
                try require(meta.get("requests_sent").?.integer == 1 and !meta.get("cached").?.bool);
                try require(meta.get("usage").?.object.get("input_tokens").?.integer == 1);
            },
            1 => try require(value == .string and std.mem.eql(u8, value.string, "first")),
            2 => try require(value == .array and value.array.items.len == 2 and std.mem.eql(u8, value.array.items[0].string, "first") and std.mem.eql(u8, value.array.items[1].string, "second")),
            3 => try require(value == .float and value.float == 0.1),
            4, 5 => {
                try require(value == .array and value.array.items.len == 2);
                const prefix: []const u8 = if (i == 4) "filter" else "rank";
                for (value.array.items, 0..) |item, position| {
                    const expected = try std.fmt.allocPrint(alloc, "{s}-{s}", .{ prefix, if (position == 0) @as([]const u8, "one") else "two" });
                    defer alloc.free(expected);
                    try require(item == .string and std.mem.eql(u8, item.string, expected));
                }
            },
            6 => try require(value == .object and value.object.get("index").?.integer == 0 and std.mem.eql(u8, value.object.get("unit").?.string, "find-one") and value.object.get("probability").?.float == 0.9),
            7 => try require(value == .array and value.array.items.len == 1 and value.array.items[0].object.get("check").?.bool),
            8 => {
                const entities = value.object.get("entities").?.array.items;
                try require(entities.len == 1);
                const entity = entities[0].object;
                try require(std.mem.eql(u8, entity.get("text").?.string, "Maria Chen") and std.mem.eql(u8, entity.get("kind").?.string, "person"));
                try require(entity.get("start").?.integer == 0 and entity.get("end").?.integer == 10 and entity.get("length").?.integer == 10 and entity.get("strength").?.float == 0.81);
            },
            9 => {
                const edges_json = value.object.get("edges").?.array.items;
                try require(edges_json.len == 2);
                const first_edge = edges_json[0].object;
                try require(std.mem.eql(u8, first_edge.get("relation").?.string, "caused_by") and first_edge.get("probability").?.float == 0.9);
                try require(std.mem.eql(u8, first_edge.get("source").?.object.get("name").?.string, "First") and std.mem.eql(u8, first_edge.get("target").?.object.get("name").?.string, "Second"));
            },
            else => unreachable,
        }
    }
    const named = try engine.recognize("{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}", "John Smith", .{});
    switch (named) {
        .ok => |success| {
            defer success.deinit(alloc);
            const bytes = success.value;
            const parsed = try std.json.parseFromSlice(std.json.Value, alloc, bytes, .{});
            defer parsed.deinit();
            try require(parsed.value.object.get("entities") != null and count(success.facts, "records") > 0 and count(success.facts, "requests_sent") == 2);
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.TypedRecognizeFailed;
        },
    }
    const entity_records = [_][]const u8{ "{\"name\":\"Third\",\"kind\":\"alert\"}", "{\"name\":\"Fourth\",\"kind\":\"alert\"}" };
    const edges = try engine.relate("{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}", &entity_records, .{});
    switch (edges) {
        .ok => |success| {
            defer success.deinit(alloc);
            const bytes = success.value;
            const parsed = try std.json.parseFromSlice(std.json.Value, alloc, bytes, .{});
            defer parsed.deinit();
            try require(parsed.value.object.get("edges") != null and count(success.facts, "records") > 0 and count(success.facts, "requests_sent") == 1);
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.TypedRelateFailed;
        },
    }
    const usage = try call(&engine, alloc, "{\"usage\":true}");
    defer usage.deinit();
    std.debug.print("recognition-era usage {d} {d} {d}\n", .{ usage.value.object.get("requests_sent").?.integer, usage.value.object.get("input_tokens").?.integer, usage.value.object.get("output_tokens").?.integer });
    // 20, not 21: the repeated first/second bulk call reads the question cache (ADR 0111).
    try require(usage.value.object.get("requests_sent").?.integer == 20 and usage.value.object.get("input_tokens").?.integer == 20 and usage.value.object.get("output_tokens").?.integer == 20);
    try std.testing.expectError(error.EmbeddedNul, engine.call("{\"usage\":true}\x00suffix", .{}));
    try std.testing.expectError(error.EmbeddedNul, engine.decide("Is it?\x00suffix", "x", .{}));
    try std.testing.expectError(error.EmbeddedNul, engine.decideMany("Is it?\x00suffix", &repeated, .{}));
    try std.testing.expectError(error.EmbeddedNul, engine.recognize("{}\x00suffix", "x", .{}));
    try std.testing.expectError(error.EmbeddedNul, engine.relate("{}\x00suffix", &empty, .{}));
    const fail = switch (try engine.decide("{invalid", "text", .{})) {
        .ok => return error.ExpectedFailure,
        .failed => |f| f,
    };
    defer engine.freeFailure(fail);
    const copy = try alloc.dupe(u8, fail.message);
    defer alloc.free(copy);
    var other = switch (try tt.Engine.init(alloc)) {
        .ok => |value| value,
        .failed => |failure| {
            defer alloc.free(failure.message);
            return error.SecondEngineBuild;
        },
    };
    const other_fail = try other.decide("Is it?", "", .{});
    try errorCode(&other, other_fail, tt.c.THINKTHEN_EUSAGE, false);
    try require(tt.c.thinkthen_error_code(engine.raw) == tt.c.THINKTHEN_EUSAGE);
    other.deinit();
    try require(std.mem.eql(u8, fail.message, copy));
    const expired = try engine.decide("Is it?", "text", .{ .deadline_ms = 0 });
    switch (expired) {
        .ok => return error.ExpectedDeadline,
        .failed => |f| {
            defer engine.freeFailure(f);
            try require(f.code == tt.c.THINKTHEN_EDEADLINE);
        },
    }
    try require(fail.code == tt.c.THINKTHEN_EUSAGE and std.mem.eql(u8, fail.message, copy));
    var token = try tt.CancelToken.init();
    defer token.deinit();
    token.cancel();
    token.cancel();
    const cancelled = try engine.decide("Is it?", "text", .{ .cancel = token.raw });
    switch (cancelled) {
        .ok => return error.ExpectedCancelled,
        .failed => |f| {
            defer engine.freeFailure(f);
            try require(f.code == tt.c.THINKTHEN_ECANCELLED);
        },
    }
    try errorCode(&engine, try engine.decide("Is it?", "", .{}), tt.c.THINKTHEN_EUSAGE, false);
    try errorCode(&engine, try engine.decide("Is it?", &[_]u8{0xff}, .{}), tt.c.THINKTHEN_EUSAGE, false);
    try errorCode(&engine, try engine.decide("Is it?", "zero", .{ .deadline_ms = 0 }), tt.c.THINKTHEN_EDEADLINE, false);
    try errorCode(&engine, try engine.decide("Is it?", "negative", .{ .deadline_ms = -2 }), tt.c.THINKTHEN_EUSAGE, false);
    try errorCode(&engine, try engine.decide("Is it?", "too-large", .{ .deadline_ms = 4294967295001 }), tt.c.THINKTHEN_EUSAGE, false);
    switch (try engine.decide("Is it?", "max-boundary", .{ .deadline_ms = 4294967295000 })) {
        .ok => |a| {
            defer a.deinit(alloc);
            try require(a.value.outcome == .yes and a.value.probability == 0.9);
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.MaxDeadlineFailed;
        },
    }
    switch (try engine.decide("Is it?", "status-401", .{})) {
        .ok => return error.ExpectedBackendFailure,
        .failed => |f| {
            defer engine.freeFailure(f);
            try require(f.kind == .backend and !f.retryable and f.facts_json != null);
            var facts = try std.json.parseFromSlice(std.json.Value, alloc, f.facts_json.?, .{});
            defer facts.deinit();
            try require(facts.value.object.get("requests_sent").?.integer > 0);
        },
    }
    try errorCode(&engine, try engine.decide("Is it?", "retry-status", .{ .deadline_ms = 500 }), tt.c.THINKTHEN_EBACKEND, true);
    try errorCode(&engine, try engine.decide("Is it?", "transport-close", .{}), tt.c.THINKTHEN_EBACKEND, false);
    try errorCode(&engine, try engine.decide("Is it?", "malformed-backend", .{}), tt.c.THINKTHEN_EBACKEND, false);
    try errorCode(&engine, try engine.call("{not-json", .{}), tt.c.THINKTHEN_EUSAGE, false);
    const failed_bulk = [_][]const u8{ "bulk-first-good", "bulk-middle-bad", "bulk-last-good" };
    try errorCode(&engine, try engine.decideMany("Is it?", &failed_bulk, .{}), tt.c.THINKTHEN_EBACKEND, false);
    switch (try engine.decide("Is it?", "after-errors", .{})) {
        .ok => |a| {
            defer a.deinit(alloc);
            try require(a.value.outcome == .yes);
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.RecoveryAfterErrorsFailed;
        },
    }
    var disposable = switch (try tt.Engine.init(alloc)) {
        .ok => |value| value,
        .failed => |f| {
            defer alloc.free(f.message);
            return error.DisposableEngineBuild;
        },
    };
    const teardown_failure = switch (try disposable.decide("{invalid", "text", .{})) {
        .failed => |f| f,
        .ok => return error.ExpectedTeardownFailure,
    };
    defer alloc.free(teardown_failure.message);
    const snapshot = try alloc.dupe(u8, teardown_failure.message);
    defer alloc.free(snapshot);
    disposable.deinit();
    try require(teardown_failure.code == tt.c.THINKTHEN_EUSAGE and
        std.mem.eql(u8, teardown_failure.message, snapshot));
    std.debug.print("matrix: ten verbs, usage, typed, byte inputs, failure copies and guards PASS\n", .{});
}
