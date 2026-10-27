const std = @import("std");
const tt = @import("thinkthen");
const testing = std.testing;
const Allocator = std.mem.Allocator;

const choices = [_]tt.Choice{ .{ .name = "billing" }, .{ .name = "delivery" } };
const levels = [_]tt.Choice{ .{ .name = "low" }, .{ .name = "high" } };
const records = [_]tt.Record{ .{ .text = "Please refund this order." }, .{ .text = "Where is the parcel?" } };

fn checkRows(comptime function: tt.Function, call: *tt.Call(function), allocator: Allocator) !void {
    var count: usize = 0;
    while (try call.next(allocator, null)) |item| {
        const row: tt.Row(function) = if (function == .filter) item.row else item;
        count += 1;
        try testing.expectEqual(@as(u64, count), row.position);
        try testing.expectEqual(tt.Backend.stub, row.source.backend);
        try testing.expect(row.source.question_identity.len != 0);
        switch (row.outcome) {
            .failed => return error.UnexpectedFailedStubRow,
            .unsure => |result| {
                const probability: f64 = result.probability;
                try testing.expect(std.math.isFinite(probability));
                if (function == .filter) try testing.expect(!item.kept());
            },
            .answered => |result| {
                const answer: tt.Answer(function) = result.answer;
                if (function == .choose) {
                    try testing.expect(std.mem.eql(u8, answer, "billing") or std.mem.eql(u8, answer, "delivery"));
                    const detail: ?tt.Detail(.choose) = result.detail;
                    try testing.expect(detail != null);
                }
                if (function == .score) {
                    const detail: tt.Detail(.score) = result.detail.?;
                    try testing.expectEqual(@as(usize, 2), detail.levels.len);
                    try testing.expect(std.math.isFinite(answer));
                }
                if (function == .filter) {
                    try testing.expectEqual(answer, item.kept());
                    try testing.expectEqualStrings(records[count - 1].text, item.record.text);
                }
                if (function == .rank) try testing.expect(answer >= 1 and answer <= records.len);
                if (function == .annotate) {
                    try testing.expectEqual(@as(usize, 1), answer.len);
                    try testing.expectEqualStrings("route", answer[0].name);
                }
                if (function == .recognize) for (answer) |name| {
                    const probability: f64 = name.probability;
                    try testing.expect(std.math.isFinite(probability));
                };
                if (function == .relate) for (answer) |relation| {
                    const probability: f64 = relation.probability;
                    try testing.expect(std.math.isFinite(probability));
                };
            },
        }
    }
    try testing.expectEqual(records.len, count);
    try testing.expect(try call.next(allocator, null) == null);
}

test "ten named functions expose function indexed rows and required typed lists" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    const input: tt.Input = .{ .records = &records };
    var decide = try client.decide(testing.allocator, "Ready?", input, .{ .threshold = tt.Threshold.band(0.2, 0.8) }, null);
    defer decide.deinit();
    try checkRows(.decide, &decide, arena.allocator());
    var choose = try client.choose(testing.allocator, "Which department?", &choices, input, .{}, null);
    defer choose.deinit();
    try checkRows(.choose, &choose, arena.allocator());
    var tag = try client.tag(testing.allocator, "Which departments apply?", &choices, input, .{ .threshold = 0.5 }, null);
    defer tag.deinit();
    try checkRows(.tag, &tag, arena.allocator());
    var score = try client.score(testing.allocator, "How urgent?", &levels, input, .{}, null);
    defer score.deinit();
    try checkRows(.score, &score, arena.allocator());
    var filter = try client.filter(testing.allocator, "Refund requested?", input, .{}, null);
    defer filter.deinit();
    try checkRows(.filter, &filter, arena.allocator());
    var rank = try client.rank(testing.allocator, "How urgent?", &levels, input, .{}, null);
    defer rank.deinit();
    // Rank orders by answer, while position remains the original input ordinal.
    var positions = [_]bool{ false, false };
    var rank_count: usize = 0;
    while (try rank.next(arena.allocator(), null)) |row| {
        try testing.expect(row.position >= 1 and row.position <= records.len);
        try testing.expect(!positions[@intCast(row.position - 1)]);
        positions[@intCast(row.position - 1)] = true;
        rank_count += 1;
        switch (row.outcome) {
            .answered => |result| try testing.expectEqual(@as(u64, rank_count), result.answer),
            else => return error.UnexpectedRankStatus,
        }
    }
    try testing.expectEqual(records.len, rank_count);
    var find = try client.find(testing.allocator, "Which line asks for money?", input, .{ .threshold = 0.4 }, null);
    defer find.deinit();
    try checkRows(.find, &find, arena.allocator());
    var recognize = try client.recognize(testing.allocator, &choices, input, .{ .threshold = 0.4 }, null);
    defer recognize.deinit();
    try checkRows(.recognize, &recognize, arena.allocator());
    const rules = [_]tt.RelationRule{.{ .name = "routes_to", .source = "billing", .target = "delivery" }};
    var relate = try client.relate(testing.allocator, &rules, input, .{ .threshold = 0.4 }, null);
    defer relate.deinit();
    try checkRows(.relate, &relate, arena.allocator());
    var annotate = try client.annotate(testing.allocator, .{ .entries = &.{.{ .name = "route", .question = .{ .choose = .{ .question = "Which department?", .choices = &choices } } }} }, input, .{}, null);
    defer annotate.deinit();
    try checkRows(.annotate, &annotate, arena.allocator());
}

test "toolbox selection retains choices and function types" {
    const toolbox: tt.Toolbox = .{ .entries = &.{.{ .name = "route", .question = .{ .choose = .{ .question = "Which department?", .choices = &choices, .options = .{ .evidence = "/body" } } } }} };
    try testing.expect(toolbox.get("missing") == null);
    const selected = toolbox.get("route").?;
    try testing.expect(selected.as(.score) == null);
    const question: tt.QuestionType(.choose) = selected.as(.choose).?;
    try testing.expectEqualStrings("billing", question.choices[0].name);
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const row = try client.one(.choose, arena.allocator(), question, .{ .json = "{\"body\":\"Please refund\"}" }, null);
    try testing.expectEqual(tt.Backend.stub, row.source.backend);
    switch (row.outcome) {
        .answered => |result| try testing.expect(result.answer.len != 0),
        else => return error.ExpectedChosenAnswer,
    }
}

test "separate row arenas survive later reads and call and client teardown" {
    var first = std.heap.ArenaAllocator.init(testing.allocator);
    defer first.deinit();
    var second = std.heap.ArenaAllocator.init(testing.allocator);
    defer second.deinit();
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    var call = try client.choose(testing.allocator, "Which department?", &choices, .{ .records = &records }, .{}, null);
    client.deinit();
    const row = (try call.next(first.allocator(), null)).?;
    const later = (try call.next(second.allocator(), null)).?;
    try testing.expectEqual(@as(u64, 2), later.position);
    _ = second.reset(.free_all);
    call.deinit();
    try testing.expectEqual(@as(u64, 1), row.position);
    try testing.expect(row.source.question_identity.len != 0);
    switch (row.outcome) {
        .answered => |result| try testing.expect(result.answer.len != 0),
        else => return error.ExpectedChosenAnswer,
    }
}

const ReusingSource = struct {
    index: usize = 0,
    buffer: [8]u8 = undefined,
    pub fn next(self: *@This()) error{RecordSource}!?tt.Record {
        if (self.index == 3) return null;
        const values = [_][]const u8{ "first___", "second__", "third___" };
        @memcpy(&self.buffer, values[self.index]);
        self.index += 1;
        return .{ .text = &self.buffer };
    }
};

test "filter preserves source originals across reuse and retains missing rows" {
    var client = try tt.Client.init(.{ .backend = .stub, .batch = 3 }, null);
    defer client.deinit();
    var source: ReusingSource = .{};
    var call = try client.filter(testing.allocator, "Ready?", .{ .source = tt.RecordSource.from(&source) }, .{}, null);
    defer call.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var count: usize = 0;
    const expected = [_][]const u8{ "first___", "second__", "third___" };
    while (try call.next(arena.allocator(), null)) |item| {
        try testing.expectEqualStrings(expected[count], item.record.text);
        try testing.expectEqual(@as(u64, count + 1), item.row.position);
        count += 1;
    }
    try testing.expectEqual(@as(usize, 3), count);
    const missing = try client.filterOne(arena.allocator(), "Ready?", .missing, .{}, null);
    try testing.expect(!missing.kept());
    switch (missing.row.outcome) {
        .failed => |failure| try testing.expectEqual(tt.ErrorCode.record_missing, failure.code.?),
        else => return error.ExpectedMissingFailure,
    }
}

test "record ids preserve integer extremes and invalid ids remain failed rows" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const valid = [_][]const u8{ "{\"id\":\"a\\u0000b\",\"body\":\"x\"}", "{\"id\":-9223372036854775808,\"body\":\"x\"}", "{\"id\":18446744073709551615,\"body\":\"x\"}" };
    const string = try client.decideOne(arena.allocator(), "Ready?", .{ .json = valid[0] }, .{}, null);
    try testing.expectEqualStrings("a\x00b", string.id.?.string);
    const signed = try client.decideOne(arena.allocator(), "Ready?", .{ .json = valid[1] }, .{}, null);
    try testing.expectEqual(std.math.minInt(i64), signed.id.?.signed);
    const unsigned = try client.decideOne(arena.allocator(), "Ready?", .{ .json = valid[2] }, .{}, null);
    try testing.expectEqual(std.math.maxInt(u64), unsigned.id.?.unsigned);
    const invalid = [_]tt.Record{ .{ .json = "{\"id\":{},\"body\":\"x\"}" }, .{ .json = "{\"id\":1.5,\"body\":\"x\"}" }, .{ .json = "{\"id\":true,\"body\":\"x\"}" } };
    var call = try client.decide(testing.allocator, "Ready?", .{ .records = &invalid }, .{}, null);
    defer call.deinit();
    var count: usize = 0;
    while (try call.next(arena.allocator(), null)) |row| {
        count += 1;
        try testing.expectEqual(@as(u64, count), row.position);
        switch (row.outcome) {
            .failed => |failure| try testing.expectEqual(tt.ErrorCode.record_invalid, failure.code.?),
            else => return error.ExpectedInvalidIdFailure,
        }
    }
    try testing.expectEqual(invalid.len, count);
}

test "row allocation failure permits arena reclamation and frees call bookkeeping" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var failures: usize = 0;
    var successes: usize = 0;
    // Sweep backing-arena allocations, including failure after a partial row.
    for (0..256) |offset| {
        var bookkeeping = testing.FailingAllocator.init(testing.allocator, .{});
        var backing = testing.FailingAllocator.init(testing.allocator, .{});
        var arena = std.heap.ArenaAllocator.init(backing.allocator());
        var row_allocator = testing.FailingAllocator.init(arena.allocator(), .{ .fail_index = offset });
        var call = try client.recognize(bookkeeping.allocator(), &choices, .{ .records = &records }, .{}, null);
        if (call.next(row_allocator.allocator(), null)) |row| {
            try testing.expect(row != null);
            successes += 1;
        } else |err| {
            try testing.expectEqual(error.OutOfMemory, err);
            failures += 1;
            try testing.expect(try call.next(arena.allocator(), null) == null);
        }
        call.deinit();
        arena.deinit();
        try testing.expectEqual(backing.allocated_bytes, backing.freed_bytes);
        try testing.expectEqual(bookkeeping.allocated_bytes, bookkeeping.freed_bytes);
        if (successes != 0) break;
    }
    try testing.expect(failures > 0 and successes > 0);
}

test "generic formatting custom formatting and JSON never reveal the API key" {
    const secret = "sk-test-0123456789";
    const key = tt.ApiKey.init(secret);
    const settings: tt.Settings = .{ .backend = .openai, .api_key = key };
    const rendered = [_][]u8{
        try std.fmt.allocPrint(testing.allocator, "{any}", .{key}),
        try std.fmt.allocPrint(testing.allocator, "{f}", .{key}),
        try std.fmt.allocPrint(testing.allocator, "{any}", .{settings}),
        try std.fmt.allocPrint(testing.allocator, "{f}", .{settings}),
        try std.json.Stringify.valueAlloc(testing.allocator, key, .{}),
        try std.json.Stringify.valueAlloc(testing.allocator, settings, .{}),
    };
    defer for (rendered) |bytes| testing.allocator.free(bytes);
    for (rendered) |bytes| try testing.expect(std.mem.indexOf(u8, bytes, secret) == null);
    var diagnostic: tt.Diagnostic = .{};
    try testing.expectError(error.Usage, tt.Client.init(.{ .backend = .openai, .api_key = key, .batch = 0 }, &diagnostic));
    try testing.expect(std.mem.indexOf(u8, diagnostic.message(), secret) == null);
}

test "JSON row decoding preserves full probability and typed detail" {
    const encoded = "{\"function\":\"choose\",\"position\":7,\"id\":18446744073709551615,\"source\":{\"question_identity\":\"fixture-question\",\"backend\":\"openai\",\"model\":\"fixture-model\"},\"status\":\"answered\",\"answer\":\"billing\",\"probability\":0.8731942678901234,\"detail\":{\"labels\":{\"billing\":0.8731942678901234,\"delivery\":0.1268057321098766}}}";
    const decoded = try std.json.parseFromSlice(tt.Row(.choose), testing.allocator, encoded, .{ .allocate = .alloc_always });
    defer decoded.deinit();
    try testing.expectEqual(@as(u64, 7), decoded.value.position);
    try testing.expectEqual(std.math.maxInt(u64), decoded.value.id.?.unsigned);
    try testing.expectEqual(tt.Backend.openai, decoded.value.source.backend);
    switch (decoded.value.outcome) {
        .answered => |result| {
            const detail: tt.Detail(.choose) = result.detail.?;
            _ = detail.labels;
            try testing.expectEqualStrings("billing", result.answer);
            try testing.expectEqual(@as(f64, 0.8731942678901234), result.probability.?);
        },
        else => return error.ExpectedAnsweredRow,
    }
}

const BrokenSource = struct {
    pub fn next(_: *@This()) error{RecordSource}!?tt.Record {
        return error.RecordSource;
    }
};

test "record source error is returned once without an invented backend code" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var source: BrokenSource = .{};
    var call = try client.decide(testing.allocator, "Ready?", .{ .source = tt.RecordSource.from(&source) }, .{}, null);
    defer call.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var diagnostic: tt.Diagnostic = .{};
    try testing.expectError(error.RecordSource, call.next(arena.allocator(), &diagnostic));
    try testing.expect(diagnostic.code == null);
    try testing.expect(try call.next(arena.allocator(), &diagnostic) == null);
}

test "name changes identity and source name while answers stay equal" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const first = try client.decideOne(arena.allocator(), "Ready?", .{ .text = "evidence" }, .{ .name = "first" }, null);
    const second = try client.decideOne(arena.allocator(), "Ready?", .{ .text = "evidence" }, .{ .name = "second" }, null);
    try testing.expectEqualStrings("first", first.source.name.?);
    try testing.expectEqualStrings("second", second.source.name.?);
    try testing.expect(!std.mem.eql(u8, first.source.question_identity, second.source.question_identity));
    try testing.expectEqualDeep(first.outcome, second.outcome);
}

test "typed threshold setters preserve nonfinite values for engine admission" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var diagnostic: tt.Diagnostic = .{};
    try testing.expectError(error.Usage, client.decide(testing.allocator, "Ready?", .{ .records = &records }, .{ .threshold = .{ .cut = std.math.nan(f64) } }, &diagnostic));
    try testing.expectEqual(tt.ErrorCode.usage_threshold, diagnostic.code.?);
    try testing.expectError(error.Usage, client.find(testing.allocator, "Which?", .{ .records = &records }, .{ .threshold = std.math.inf(f64) }, &diagnostic));
    try testing.expectEqual(tt.ErrorCode.usage_threshold, diagnostic.code.?);
}

test "saved typed rows round trip answered null and typed annotation children" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    const row: tt.Row(.find) = .{ .position = 1, .id = null, .source = .{ .question_identity = "saved", .backend = .stub, .model = "stub" }, .outcome = .{ .answered = .{ .answer = null, .probability = 0.9, .detail = null } } };
    const encoded = try std.json.Stringify.valueAlloc(allocator, row, .{ .emit_null_optional_fields = false });
    try testing.expect(std.mem.indexOf(u8, encoded, "\"answer\":null") != null);
    const decoded = try std.json.parseFromSliceLeaky(tt.Row(.find), allocator, encoded, .{});
    try testing.expect(decoded.outcome.answered.answer == null);
    try testing.expect((try decoded.intoResult(null)) != null);
    const annotation: tt.Row(.annotate) = .{ .position = 1, .id = null, .source = row.source, .outcome = .{ .answered = .{ .answer = &.{.{ .name = "ready", .value = .{ .decide = .{ .answered = .{ .answer = true, .probability = 0.9, .detail = null } } } }}, .probability = null, .detail = null } } };
    const saved = try std.json.Stringify.valueAlloc(allocator, annotation, .{ .emit_null_optional_fields = false });
    const restored = try std.json.parseFromSliceLeaky(tt.Row(.annotate), allocator, saved, .{});
    try testing.expect(restored.outcome.answered.answer[0].value.decide.answered.answer);
}

test {
    _ = @import("registry.zig");
}

// Feature: choice labels use the shared string grammar.
test "choice exposes only its label and encodes as a string" {
    try testing.expect(!@hasField(tt.Choice, "description"));
    const encoded = try std.json.Stringify.valueAlloc(testing.allocator, choices[0], .{});
    defer testing.allocator.free(encoded);
    try testing.expectEqualStrings("\"billing\"", encoded);
}
