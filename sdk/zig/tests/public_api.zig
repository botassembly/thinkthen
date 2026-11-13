const std = @import("std");
const builtin = @import("builtin");
const tt = @import("thinkthen");
const testing = std.testing;
const Allocator = std.mem.Allocator;

const choices = [_]tt.Choice{ .{ .name = "billing" }, .{ .name = "delivery" } };
const levels = [_]tt.Choice{ .{ .name = "low" }, .{ .name = "high" } };
const records = [_]tt.Record{ .{ .text = "Please refund this order." }, .{ .text = "Where is the parcel?" } };

fn checkRows(comptime function: tt.Function, call: *tt.Call(function)) !void {
    var count: usize = 0;
    while (try (if (function == .filter) call.nextRow(null) else call.next(null))) |item| {
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
                if (function == .filter) try testing.expectEqualStrings(records[count - 1].text, item.record.text);
                if (function == .rank) try testing.expect(answer >= 1 and answer <= records.len);
                if (function == .annotate) {
                    try testing.expectEqual(@as(usize, 1), answer.entries.len);
                    try testing.expect(answer.get(.choose, "route") != null);
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
    try testing.expect(try call.next(null) == null);
}

test "ten named functions expose function indexed rows and required typed lists" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    const input: tt.Input = .{ .records = &records };
    var decide = try client.decideAll(testing.allocator, "Ready?", input, .{ .threshold = tt.Threshold.band(0.2, 0.8) }, null);
    defer decide.deinit();
    try checkRows(.decide, &decide);
    var choose = try client.chooseAll(testing.allocator, "Which department?", &choices, input, .{}, null);
    defer choose.deinit();
    try checkRows(.choose, &choose);
    var tag = try client.tagAll(testing.allocator, "Which departments apply?", &choices, input, .{ .threshold = 0.5 }, null);
    defer tag.deinit();
    try checkRows(.tag, &tag);
    var score = try client.scoreAll(testing.allocator, "How urgent?", &levels, input, .{}, null);
    defer score.deinit();
    try checkRows(.score, &score);
    var filter = try client.filterAll(testing.allocator, "Refund requested?", input, .{}, null);
    defer filter.deinit();
    try checkRows(.filter, &filter);
    var rank = try client.rankAll(testing.allocator, "How urgent?", &levels, input, .{}, null);
    defer rank.deinit();
    // Rank orders by answer, while position remains the original input ordinal.
    var positions = [_]bool{ false, false };
    var rank_count: usize = 0;
    while (try rank.next(null)) |row| {
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
    var find = try client.findAll(testing.allocator, "Which line asks for money?", input, .{ .threshold = 0.4 }, null);
    defer find.deinit();
    try checkRows(.find, &find);
    var recognize = try client.recognizeAll(testing.allocator, &choices, input, .{ .threshold = 0.4 }, null);
    defer recognize.deinit();
    try checkRows(.recognize, &recognize);
    const rules = [_]tt.RelationRule{.{ .name = "routes_to", .source = "billing", .target = "delivery" }};
    var relate = try client.relateAll(testing.allocator, &rules, input, .{ .threshold = 0.4 }, null);
    defer relate.deinit();
    try checkRows(.relate, &relate);
    var annotate = try client.annotateAll(testing.allocator, .{ .entries = &.{.{ .name = "route", .question = .{ .choose = .{ .question = "Which department?", .choices = &choices } } }} }, input, .{}, null);
    defer annotate.deinit();
    try checkRows(.annotate, &annotate);
}

test "toolbox selection retains choices and function types" {
    const toolbox: tt.Toolbox = .{ .entries = &.{.{ .name = "route", .question = .{ .choose = .{ .question = "Which department?", .choices = &choices, .options = .{ .field = "/body" } } } }} };
    try testing.expect(toolbox.get("missing") == null);
    const selected = toolbox.get("route").?;
    try testing.expect(selected.as(.score) == null);
    const question: tt.QuestionType(.choose) = selected.as(.choose).?;
    try testing.expectEqualStrings("billing", question.choices[0].name);
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const row = try client.open(.choose, arena.allocator(), question, .{ .json = "{\"body\":\"Please refund\"}" }, null);
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
    var call = try client.chooseAll(testing.allocator, "Which department?", &choices, .{ .records = &records }, .{}, null);
    client.deinit();
    const row = (try call.nextInto(first.allocator(), null)).?;
    const later = (try call.nextInto(second.allocator(), null)).?;
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

test "filter preserves source originals across reuse and a failed record returns its error" {
    var client = try tt.Client.init(.{ .backend = .stub, .batch = 3 }, null);
    defer client.deinit();
    var source: ReusingSource = .{};
    var call = try client.filterAll(testing.allocator, "Ready?", .{ .source = tt.RecordSource.from(&source) }, .{}, null);
    defer call.deinit();
    var count: usize = 0;
    const expected = [_][]const u8{ "first___", "second__", "third___" };
    while (try call.nextRow(null)) |item| {
        try testing.expectEqualStrings(expected[count], item.record.text);
        try testing.expectEqual(@as(u64, count + 1), item.row.position);
        count += 1;
    }
    try testing.expectEqual(@as(usize, 3), count);
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var diagnostic: tt.Diagnostic = .{};
    try testing.expectError(error.Record, client.filter(arena.allocator(), "Ready?", .missing, .{}, &diagnostic));
    try testing.expectEqual(tt.ErrorCode.record_missing, diagnostic.code.?);
}

test "record ids preserve integer extremes and invalid ids remain failed rows" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const valid = [_][]const u8{ "{\"id\":\"a\\u0000b\",\"body\":\"x\"}", "{\"id\":-9223372036854775808,\"body\":\"x\"}", "{\"id\":18446744073709551615,\"body\":\"x\"}" };
    const string = try client.decide(arena.allocator(), "Ready?", .{ .json = valid[0] }, .{}, null);
    try testing.expectEqualStrings("a\x00b", string.id.?.string);
    const signed = try client.decide(arena.allocator(), "Ready?", .{ .json = valid[1] }, .{}, null);
    try testing.expectEqual(std.math.minInt(i64), signed.id.?.signed);
    const unsigned = try client.decide(arena.allocator(), "Ready?", .{ .json = valid[2] }, .{}, null);
    try testing.expectEqual(std.math.maxInt(u64), unsigned.id.?.unsigned);
    const invalid = [_]tt.Record{ .{ .json = "{\"id\":{},\"body\":\"x\"}" }, .{ .json = "{\"id\":1.5,\"body\":\"x\"}" }, .{ .json = "{\"id\":true,\"body\":\"x\"}" } };
    var call = try client.decideAll(testing.allocator, "Ready?", .{ .records = &invalid }, .{}, null);
    defer call.deinit();
    var count: usize = 0;
    while (try call.next(null)) |row| {
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
        var call = try client.recognizeAll(bookkeeping.allocator(), &choices, .{ .records = &records }, .{}, null);
        if (call.nextInto(row_allocator.allocator(), null)) |row| {
            try testing.expect(row != null);
            successes += 1;
        } else |err| {
            try testing.expectEqual(error.OutOfMemory, err);
            failures += 1;
            try testing.expect(try call.nextInto(arena.allocator(), null) == null);
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
    var call = try client.decideAll(testing.allocator, "Ready?", .{ .source = tt.RecordSource.from(&source) }, .{}, null);
    defer call.deinit();
    var diagnostic: tt.Diagnostic = .{};
    try testing.expectError(error.RecordSource, call.next(&diagnostic));
    try testing.expect(diagnostic.code == null);
    try testing.expect(try call.next(&diagnostic) == null);
}

test "name changes identity and source name while answers stay equal" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const first = try client.decide(arena.allocator(), "Ready?", .{ .text = "evidence" }, .{ .name = "first" }, null);
    const second = try client.decide(arena.allocator(), "Ready?", .{ .text = "evidence" }, .{ .name = "second" }, null);
    try testing.expectEqualStrings("first", first.source.name.?);
    try testing.expectEqualStrings("second", second.source.name.?);
    try testing.expect(!std.mem.eql(u8, first.source.question_identity, second.source.question_identity));
    try testing.expectEqualDeep(first.outcome, second.outcome);
}

test "typed threshold setters preserve nonfinite values for engine admission" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var diagnostic: tt.Diagnostic = .{};
    try testing.expectError(error.Usage, client.decideAll(testing.allocator, "Ready?", .{ .records = &records }, .{ .threshold = .{ .cut = std.math.nan(f64) } }, &diagnostic));
    try testing.expectEqual(tt.ErrorCode.usage_threshold, diagnostic.code.?);
    try testing.expectError(error.Usage, client.findAll(testing.allocator, "Which?", .{ .records = &records }, .{ .threshold = std.math.inf(f64) }, &diagnostic));
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
    try testing.expect((try decoded.answer(null)) == null);
    const annotation: tt.Row(.annotate) = .{ .position = 1, .id = null, .source = row.source, .outcome = .{ .answered = .{ .answer = .{ .entries = &.{.{ .name = "ready", .value = .{ .decide = .{ .answered = .{ .answer = true, .probability = 0.9, .detail = null } } } }} }, .probability = null, .detail = null } } };
    const saved = try std.json.Stringify.valueAlloc(allocator, annotation, .{ .emit_null_optional_fields = false });
    const restored = try std.json.parseFromSliceLeaky(tt.Row(.annotate), allocator, saved, .{});
    try testing.expect(try (try restored.answer(null)).get(.decide, "ready").?.answer(null));
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

// Feature: a projected failed row keeps its code and uses the live engine family.
test "failed row projections classify each engine family" {
    const codes = [_]tt.ErrorCode{ .usage_question, .record_missing, .backend_reply, .local_io, .internal_bug };
    const errors = [_]tt.Error{ error.Usage, error.Record, error.Backend, error.Local, error.Internal };
    for (codes, errors) |code, expected| {
        const row: tt.Row(.decide) = .{
            .position = 1,
            .id = null,
            .source = .{ .question_identity = "fixture", .backend = .stub, .model = "stub" },
            .outcome = .{ .failed = .{ .code = code, .message = "preserve this failure" } },
        };
        var diagnostic: tt.Diagnostic = .{};
        try testing.expectError(expected, row.answer(&diagnostic));
        try testing.expectEqual(code, diagnostic.code.?);
        try testing.expectEqualStrings("preserve this failure", diagnostic.message());
        try testing.expectEqual(@as(u64, 1), diagnostic.position.?);
    }
}

fn projected(comptime function: tt.Function, outcome: tt.Outcome(function)) tt.Row(function) {
    return .{ .position = 3, .id = null, .source = .{ .question_identity = "fixture", .backend = .stub, .model = "stub" }, .outcome = outcome };
}

// Feature: a row is its answer in one step, and a not-sure row used as an answer is an error.
test "answer returns the answer and refuses a not-sure row with error.Unsure" {
    var diagnostic: tt.Diagnostic = .{};
    const yes = projected(.decide, .{ .answered = .{ .answer = true, .probability = 0.97, .detail = null } });
    try testing.expect(try yes.answer(&diagnostic));
    try testing.expectEqual(@as(f64, 0.97), yes.probability().?);
    const printed = try std.fmt.allocPrint(testing.allocator, "{f}", .{yes});
    defer testing.allocator.free(printed);
    try testing.expectEqualStrings("true (0.97)", printed);
    const unsure = projected(.decide, .{ .unsure = .{ .probability = 0.5, .detail = null } });
    try testing.expect(unsure.isUnsure() and !unsure.isFailed());
    const refused: tt.AnswerError!bool = unsure.answer(&diagnostic);
    try testing.expectError(error.Unsure, refused);
    try testing.expectEqual(@as(f64, 0.5), diagnostic.probability.?);
    try testing.expectEqual(@as(u64, 3), diagnostic.position.?);
    try testing.expect(diagnostic.code == null);
    try testing.expectError(error.Unsure, @as(tt.Error!void, error.Unsure));
    const nothing = projected(.find, .{ .unsure = .{ .probability = 0.4, .detail = null } });
    try testing.expectError(error.Unsure, nothing.answer(null));
}

// Feature: one record takes the plain name; many records take All, from texts or JSON Lines, in input order.
test "decide answers one record and decideAll reads texts and JSON Lines into call-owned rows" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var diagnostic: tt.Diagnostic = .{};
    const refund_row: tt.Row(.decide) = try client.decide(arena.allocator(), "Refund requested?", .{ .text = "Please refund my order." }, .{}, &diagnostic);
    _ = try refund_row.answer(&diagnostic);
    const messages = [_][]const u8{ "first", "second", "third" };
    var refund_rows = try client.decideAll(testing.allocator, "Refund requested?", .{ .texts = &messages }, .{}, &diagnostic);
    defer refund_rows.deinit();
    var count: u64 = 0;
    while (try refund_rows.next(&diagnostic)) |row| {
        count += 1;
        try testing.expectEqual(count, row.position);
        try testing.expect(!row.isFailed());
    }
    try testing.expectEqual(@as(u64, messages.len), count);
    const lines = "{\"id\":\"x\",\"text\":\"first\"}\n\n \t\r\n{\"id\":\"y\",\"text\":\"second\"}\r\n";
    var line_rows = try client.decideAll(testing.allocator, "Refund requested?", .{ .json_lines = lines }, .{ .field = "/text" }, &diagnostic);
    defer line_rows.deinit();
    const ids = [_][]const u8{ "x", "y" };
    count = 0;
    while (try line_rows.next(&diagnostic)) |row| {
        try testing.expectEqualStrings(ids[count], row.id.?.string);
        try testing.expect(!row.isFailed());
        count += 1;
    }
    try testing.expectEqual(@as(u64, ids.len), count);
}

// Feature: filter yields only kept records; a failed record returns its error in place and reading continues.
test "filterAll yields kept records and returns a failed record's error in its place" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    const reviews = [_]tt.Record{ .{ .text = "Arrived early." }, .missing, .{ .text = "The zipper broke." }, .{ .text = "The strap snapped." }, .{ .text = "Great colour." }, .{ .text = "Lid cracked." }, .{ .text = "Works well." }, .{ .text = "Never arrived." } };
    var every = try client.filterAll(testing.allocator, "Is this a complaint?", .{ .records = &reviews }, .{}, null);
    defer every.deinit();
    var expected: std.ArrayList([]const u8) = .empty;
    defer expected.deinit(testing.allocator);
    while (try every.nextRow(null)) |item| {
        if (item.row.outcome == .answered and item.row.outcome.answered.answer) try expected.append(testing.allocator, reviews[@intCast(item.row.position - 1)].text);
    }
    try testing.expect(expected.items.len > 0 and expected.items.len < reviews.len - 1);
    var complaints = try client.filterAll(testing.allocator, "Is this a complaint?", .{ .records = &reviews }, .{}, null);
    defer complaints.deinit();
    var diagnostic: tt.Diagnostic = .{};
    var kept: usize = 0;
    var failures: usize = 0;
    while (true) {
        const complaint = complaints.next(&diagnostic) catch |err| {
            try testing.expectEqual(error.Record, err);
            try testing.expectEqual(tt.ErrorCode.record_missing, diagnostic.code.?);
            try testing.expectEqual(@as(u64, 2), diagnostic.position.?);
            failures += 1;
            continue;
        } orelse break;
        try testing.expectEqualStrings(expected.items[kept], complaint.text);
        kept += 1;
    }
    try testing.expectEqual(expected.items.len, kept);
    try testing.expectEqual(@as(usize, 1), failures);
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const first = try client.filter(arena.allocator(), "Is this a complaint?", reviews[0], .{}, null);
    try testing.expectEqual(std.mem.eql(u8, expected.items[0], reviews[0].text), first != null);
}

// Feature: a saved toolbox feeds annotate, and annotate answers by name.
test "annotate takes a saved toolbox and answers each question by name" {
    const saved = "{\"entries\":{\"route\":{\"question\":{\"function\":\"choose\",\"question\":\"Which department?\",\"choices\":[\"billing\",\"delivery\"]}},\"urgent\":{\"question\":{\"function\":\"decide\",\"question\":\"Urgent?\"}}}}";
    var diagnostic: tt.Diagnostic = .{};
    var form = try tt.Toolbox.fromJson(testing.allocator, saved, &diagnostic);
    defer form.deinit();
    const questions = try form.annotation(testing.allocator, &diagnostic);
    defer testing.allocator.free(questions.entries);
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const triage_row = try client.annotate(arena.allocator(), questions, .{ .text = "Please refund my order." }, .{}, &diagnostic);
    const triage = try triage_row.answer(&diagnostic);
    const department = try triage.get(.choose, "route").?.answer(&diagnostic);
    try testing.expect(std.mem.eql(u8, department, "billing") or std.mem.eql(u8, department, "delivery"));
    _ = triage.get(.decide, "urgent").?;
    try testing.expect(triage.get(.decide, "route") == null);
    try testing.expect(triage.get(.choose, "missing") == null);
    const ranked: tt.Toolbox = .{ .entries = &.{.{ .name = "order", .question = .{ .rank = .{ .question = "How urgent?", .levels = &levels } } }} };
    try testing.expectError(error.Usage, ranked.annotation(testing.allocator, &diagnostic));
    try testing.expectEqual(tt.ErrorCode.usage_question, diagnostic.code.?);
}

// Feature: a call with no records yields no rows and makes no request.
test "an empty collection yields no rows without reaching the backend" {
    var client = try tt.Client.init(.{ .backend = .systemone, .url = "http://127.0.0.1:9/v1", .api_key = tt.ApiKey.init("unreachable-test-key"), .retry = .{ .max_retries = 0, .attempt_timeout_ms = 5000 } }, null);
    defer client.deinit();
    var diagnostic: tt.Diagnostic = .{};
    const empties = [_]tt.Input{ .{ .texts = &.{} }, .{ .json_lines = " \n\n" }, .{ .records = &.{} } };
    for (empties) |input| {
        var rows = try client.decideAll(testing.allocator, "Ready?", input, .{}, &diagnostic);
        defer rows.deinit();
        try testing.expect(try rows.next(&diagnostic) == null);
    }
    // The same client fails a real record, so the empty calls above never sent one.
    var reached = try client.decideAll(testing.allocator, "Ready?", .{ .texts = &.{"evidence"} }, .{}, &diagnostic);
    defer reached.deinit();
    const row = (try reached.next(&diagnostic)).?;
    try testing.expectError(error.Backend, row.answer(&diagnostic));
}

// Rust reads the process environment, which Windows keeps apart from the C runtime's copy.
const backend_environment = if (builtin.os.tag == .windows) struct {
    const name = std.unicode.utf8ToUtf16LeStringLiteral("THINKTHEN_BACKEND");
    extern "kernel32" fn SetEnvironmentVariableW([*:0]const u16, ?[*:0]const u16) callconv(.winapi) c_int;
    fn set(comptime value: []const u8) !void {
        try testing.expect(SetEnvironmentVariableW(name, std.unicode.utf8ToUtf16LeStringLiteral(value)) != 0);
    }
    fn unset() void {
        _ = SetEnvironmentVariableW(name, null);
    }
} else struct {
    extern "c" fn setenv(name: [*:0]const u8, value: [*:0]const u8, overwrite: c_int) c_int;
    extern "c" fn unsetenv(name: [*:0]const u8) c_int;
    fn set(comptime value: [:0]const u8) !void {
        try testing.expectEqual(@as(c_int, 0), setenv("THINKTHEN_BACKEND", value, 1));
    }
    fn unset() void {
        _ = unsetenv("THINKTHEN_BACKEND");
    }
};

// Feature: a client with no backend reads THINKTHEN_BACKEND, and an explicit backend wins.
test "an unset backend reads THINKTHEN_BACKEND and the engine admits its name" {
    defer backend_environment.unset();
    try backend_environment.set("stub");
    var client = try tt.Client.init(.{}, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const row = try client.decide(arena.allocator(), "Ready?", .{ .text = "evidence" }, .{}, null);
    try testing.expectEqual(tt.Backend.stub, row.source.backend);
    try backend_environment.set("not-a-backend");
    var diagnostic: tt.Diagnostic = .{};
    try testing.expectError(error.Usage, tt.Client.init(.{}, &diagnostic));
    try testing.expectEqual(tt.ErrorCode.usage_backend, diagnostic.code.?);
    var explicit = try tt.Client.init(.{ .backend = .stub }, null);
    explicit.deinit();
}

// Feature: rank rows carry native-owned original bytes, without retaining the input stream.
test "rank keeps originals through nextInto and call cleanup" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const inputs = [_]tt.Record{ .{ .json = "{\"id\":\"alpha\",\"text\":\"first\"}" }, .{ .text = "second" }, .{ .json = "not JSON" }, .missing };
    var ranks = try client.rankAll(testing.allocator, "Urgent?", &levels, .{ .records = &inputs }, .{ .top = 1 }, null);
    var saved: std.ArrayList(tt.Row(.rank)) = .empty;
    defer saved.deinit(testing.allocator);
    while (try ranks.nextInto(arena.allocator(), null)) |row| try saved.append(testing.allocator, row);
    ranks.deinit();
    client.deinit();
    try testing.expectEqual(@as(usize, 3), saved.items.len);
    for (saved.items) |row| {
        const expected = inputs[@intCast(row.position - 1)];
        try testing.expectEqual(std.meta.activeTag(expected), std.meta.activeTag(row.item));
        switch (expected) {
            .json => |v| try testing.expectEqualStrings(v, row.item.json),
            .text => |v| try testing.expectEqualStrings(v, row.item.text),
            .missing => {},
        }
    }
}

// Feature: host options pass meanings and raw description maps to shared admission, including saved questions.
test "meanings and descriptions survive toolbox roundtrips and unknown keys are refused" {
    var descriptions: std.json.ArrayHashMap([]const u8) = .{};
    defer descriptions.deinit(testing.allocator);
    try descriptions.map.put(testing.allocator, "low", "Can wait");
    const form: tt.Toolbox = .{ .entries = &.{
        .{ .name = "decision", .question = .{ .decide = .{ .question = "Urgent?", .options = .{ .yes_means = "Act now", .no_means = "Can wait" } } } },
        .{ .name = "filter", .question = .{ .filter = .{ .question = "Urgent?", .options = .{ .yes_means = "Keep", .no_means = "Leave" } } } },
        .{ .name = "choose", .question = .{ .choose = .{ .question = "Urgent?", .choices = &levels, .options = .{ .descriptions = descriptions } } } },
        .{ .name = "tag", .question = .{ .tag = .{ .question = "Urgent?", .labels = &levels, .options = .{ .descriptions = descriptions } } } },
        .{ .name = "score", .question = .{ .score = .{ .question = "Urgent?", .levels = &levels, .options = .{ .descriptions = descriptions } } } },
        .{ .name = "rank", .question = .{ .rank = .{ .question = "Urgent?", .levels = &levels, .options = .{ .descriptions = descriptions } } } },
    } };
    const encoded = try form.toJson(testing.allocator, null);
    defer testing.allocator.free(encoded);
    var loaded = try tt.Toolbox.fromJson(testing.allocator, encoded, null);
    defer loaded.deinit();
    try testing.expectEqualStrings("Act now", loaded.get("decision").?.decide.options.yes_means.?);
    try testing.expectEqualStrings("Leave", loaded.get("filter").?.filter.options.no_means.?);
    inline for (.{ "choose", "tag", "score", "rank" }) |function| {
        const question = @field(loaded.get(function).?, function);
        try testing.expectEqualStrings("Can wait", question.options.descriptions.?.map.get("low").?);
    }
    try descriptions.map.put(testing.allocator, "unknown", "Not a label");
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var diag: tt.Diagnostic = .{};
    try testing.expectError(error.Usage, client.rankAll(testing.allocator, "Urgent?", &levels, .{ .texts = &.{"first"} }, .{ .descriptions = descriptions }, &diag));
    try testing.expectEqual(tt.ErrorCode.usage_question, diag.code.?);
}

// Feature: one blank record raises record.empty; blank stream rows and closed clients keep their shared codes.
test "single blank refusal and closed client keep engine codes" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var diag: tt.Diagnostic = .{};
    try testing.expectError(error.Record, client.decide(arena.allocator(), "Ready?", .{ .text = "  " }, .{}, &diag));
    try testing.expectEqual(tt.ErrorCode.record_empty, diag.code.?);
    var rows = try client.decideAll(testing.allocator, "Ready?", .{ .texts = &.{ "  ", "evidence" } }, .{}, &diag);
    defer rows.deinit();
    const blank = (try rows.next(&diag)).?;
    try testing.expectError(error.Record, blank.answer(&diag));
    try testing.expectEqual(tt.ErrorCode.record_empty, diag.code.?);
    try testing.expect(!(try rows.next(&diag)).?.isFailed());
    client.deinit();
    try testing.expectError(error.Usage, client.decideAll(testing.allocator, "Ready?", .{ .texts = &.{"evidence"} }, .{}, &diag));
    try testing.expectEqual(tt.ErrorCode.usage_closed, diag.code.?);
}

const ReusedRankSource = struct {
    buffer: [32 * 1024]u8 = @splat('x'),
    count: usize = 0,
    pub fn next(self: *@This()) error{RecordSource}!?tt.Record {
        if (self.count == 2048) return null;
        self.count += 1;
        const id = std.fmt.bufPrint(self.buffer[0..8], "{d:0>8}", .{self.count}) catch unreachable;
        _ = id;
        return .{ .text = &self.buffer };
    }
};

// Feature: a top-one rank of a large reusable stream fits bounded host memory and owns its winning bytes.
test "rank streaming does not retain every host record" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var source: ReusedRankSource = .{};
    var storage: [128 * 1024]u8 = undefined;
    var bounded = std.heap.FixedBufferAllocator.init(&storage);
    var ranks = try client.rankAll(bounded.allocator(), "Urgent?", &levels, .{ .source = tt.RecordSource.from(&source) }, .{ .top = 1 }, null);
    defer ranks.deinit();
    const winner = (try ranks.next(null)).?;
    try testing.expectEqual(@as(usize, 2048), source.count);
    try testing.expectEqual(@as(usize, 32 * 1024), winner.item.text.len);
    var expected: [8]u8 = undefined;
    _ = try std.fmt.bufPrint(&expected, "{d:0>8}", .{winner.position});
    try testing.expectEqualStrings(&expected, winner.item.text[0..8]);
    @memset(&source.buffer, 'z');
    try testing.expectEqualStrings(&expected, winner.item.text[0..8]);
    try testing.expect(try ranks.next(null) == null);
}

test "usage is an independent value after client deinit" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    const snapshot = try client.usage(testing.allocator, null);
    client.deinit();
    try testing.expectEqual(@as(u64, 0), snapshot.attempts);
    try testing.expectEqual(@as(u64, 0), snapshot.active_calls);
    try testing.expectEqual(@as(?u64, 0), snapshot.input_tokens.reported);
    try testing.expectEqual(@as(u64, 0), snapshot.output_tokens.unknown_attempts);
}

test "request allowance serialization preserves explicit zero" {
    const encoded = try std.json.Stringify.valueAlloc(testing.allocator, tt.Settings{ .backend = .stub, .max_requests = 0 }, .{ .emit_null_optional_fields = false });
    defer testing.allocator.free(encoded);
    try testing.expect(std.mem.indexOf(u8, encoded, "\"max_requests\":0") != null);
}
