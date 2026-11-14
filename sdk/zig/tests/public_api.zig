const std = @import("std");
const tt = @import("thinkthen");
const testing = std.testing;
const Topic = enum { billing, shipping };
const Urgency = enum(u8) { routine = 9, urgent = 1 };
const Ticket = struct { id: []const u8, text: []const u8, memo: []const u8 = "" };
const asks: tt.Decide = .{ .question = "Ready?", .threshold = .band(0.2, 0.8) };

test "typed named functions preserve caller records and typed detail" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const input: []const u8 = "Morgan works for Example Group.";
    const decision = try client.decide(a, asks, input, null);
    try testing.expectEqual(input.ptr, decision.item.ptr);
    try testing.expect(decision.status != .failed);
    const topic = try client.choose(a, tt.Choose(Topic){ .question = "Which?" }, input, null);
    if (topic.status == .answered) {
        const choice: Topic = try topic.answer(null);
        _ = choice;
        try testing.expect(topic.detail != null);
    }
    const tags = try client.tag(a, tt.Tag(Topic){ .question = "Which?" }, input, null);
    const set: std.EnumSet(Topic) = try tags.answer(null);
    _ = set;
    const score = try client.score(a, tt.Score(Urgency){ .question = "Urgency?" }, input, null);
    try testing.expect(std.math.isFinite(try score.answer(null)));
    try testing.expectEqual(@as(usize, 2), score.detail.?.levels.len);
    const find = try client.find(a, tt.Find{ .question = "Which line?" }, input, null);
    if (find.status == .answered) {
        if (try find.answer(null)) |span| try testing.expectEqualStrings(span.text, input[span.start..span.end]);
    }
    const names = try client.recognize(a, tt.Recognize{ .kinds = &.{ "person", "organization" } }, input, null);
    for (try names.answer(null)) |name| try testing.expectEqualStrings(name.text, input[name.start..name.end]);
    const relations = try client.relate(a, tt.Relate{ .rules = &.{.{ .source = "person", .target = "organization", .relation = "works_for" }} }, input, null);
    for (try relations.answer(null)) |relation| try testing.expectEqualStrings(relation.source.text, input[relation.source.start..relation.source.end]);
    const questions = tt.annotate(.{ .ready = asks, .topic = tt.Choose(Topic){ .question = "Which?" } });
    const form = try client.annotate(a, questions, input, null);
    const entries = try form.answer(null);
    try testing.expectEqual(form.item.ptr, entries.ready.item.ptr);
    try testing.expectEqual(form.position, entries.topic.position);
    _ = try form.errIfFailed(null);
}

test "nextInto retains rows after later reads call and client teardown" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const messages = [_][]const u8{ "first", "second" };
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    var call = try client.decideAll(testing.allocator, asks, &messages, null);
    client.deinit();
    const first = (try call.nextInto(arena.allocator(), null)).?;
    const second = (try call.next(null)).?;
    try testing.expectEqual(@as(u64, 2), second.position);
    call.deinit();
    try testing.expectEqual(@as(u64, 1), first.position);
    try testing.expectEqual(messages[0].ptr, first.item.ptr);
    try testing.expect(first.source.question_identity.len != 0);
    const rendered = try std.fmt.allocPrint(arena.allocator(), "{f}", .{first});
    try testing.expect(rendered.len != 0);
}

test "blank and invalid IDs are failed rows while every later record survives" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var diag: tt.Diagnostic = .{};
    try testing.expectError(error.Failed, client.decide(arena.allocator(), asks, "  ", &diag));
    try testing.expectEqual(tt.ErrorCode.record_empty, diag.code.?);
    const inputs = [_]tt.Record{ .{ .json = "{\"id\":null,\"text\":\"x\"}" }, .{ .json = "{\"id\":{},\"text\":\"x\"}" }, .{ .json = "{\"id\":1.5,\"text\":\"x\"}" }, .{ .text = "valid" } };
    var rows = try client.decideAll(testing.allocator, asks, &inputs, &diag);
    defer rows.deinit();
    var count: usize = 0;
    while (try rows.next(&diag)) |row| : (count += 1) {
        try testing.expectEqual(@as(u64, count + 1), row.position);
        if (count < 3) {
            try testing.expect(row.id == null);
            try testing.expectError(error.Failed, row.answer(&diag));
            try testing.expectEqual(tt.ErrorCode.record_invalid, diag.code.?);
            try testing.expectEqualStrings(inputs[count].json, row.item.json);
        } else try testing.expect(row.status != .failed);
    }
    try testing.expectEqual(inputs.len, count);
    client.deinit();
    try testing.expectError(error.Usage, client.decide(arena.allocator(), asks, "closed", &diag));
    try testing.expectEqual(tt.ErrorCode.usage_closed, diag.code.?);
}

test "integer and string IDs retain exact admitted values" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const strings = try client.decide(a, asks, tt.Record{ .json = "{\"id\":\"a\\u0000b\",\"body\":\"x\"}" }, null);
    try testing.expectEqualStrings("a\x00b", strings.id.?.string);
    const signed = try client.decide(a, asks, tt.Record{ .json = "{\"id\":-9223372036854775808,\"body\":\"x\"}" }, null);
    try testing.expectEqual(std.math.minInt(i64), signed.id.?.signed);
    const unsigned = try client.decide(a, asks, tt.Record{ .json = "{\"id\":18446744073709551615,\"body\":\"x\"}" }, null);
    try testing.expectEqual(std.math.maxInt(u64), unsigned.id.?.unsigned);
}

test "filter gives kept originals then one failure with every failed original" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    const inputs = [_]tt.Record{ .{ .text = "first" }, .missing, .{ .text = "third" }, .{ .json = "invalid JSON" } };
    var call = try client.filter(testing.allocator, tt.Filter{ .question = "Keep?" }, &inputs, null);
    defer call.deinit();
    var diag: tt.Diagnostic = .{};
    while (true) {
        _ = (call.next(&diag) catch |err| {
            try testing.expectEqual(error.Failed, err);
            break;
        }) orelse return error.FilterFailedToReport;
    }
    try testing.expectEqual(@as(usize, 2), call.failed().len);
    try testing.expectEqual(@as(u64, 2), call.failed()[0].position);
    try testing.expectEqual(@as(u64, 4), call.failed()[1].position);
    try testing.expectEqualStrings(inputs[3].json, call.failed()[1].item.json);
    try testing.expect(try call.next(&diag) == null);
}

const ReusingSource = struct {
    index: usize = 0,
    buffer: [8]u8 = undefined,
    pub fn next(self: *@This()) error{RecordSource}!?tt.Record {
        const words = [_][]const u8{ "first___", "second__", "third___" };
        if (self.index == words.len) return null;
        @memcpy(&self.buffer, words[self.index]);
        self.index += 1;
        return .{ .text = &self.buffer };
    }
};
test "untaught sources preserve reused bytes and propagate source failure once" {
    var client = try tt.Client.init(.{ .backend = .stub, .batch = 3 }, null);
    defer client.deinit();
    var source: ReusingSource = .{};
    var rows = try client.decideAll(testing.allocator, asks, tt.Input{ .source = tt.RecordSource.from(&source) }, null);
    defer rows.deinit();
    const words = [_][]const u8{ "first___", "second__", "third___" };
    var count: usize = 0;
    while (try rows.next(null)) |row| : (count += 1) try testing.expectEqualStrings(words[count], row.item.text);
    try testing.expectEqual(words.len, count);
    var broken = struct {
        pub fn next(_: *@This()) error{RecordSource}!?tt.Record {
            return error.RecordSource;
        }
    }{};
    var failed = try client.decideAll(testing.allocator, asks, tt.Input{ .source = tt.RecordSource.from(&broken) }, null);
    defer failed.deinit();
    var diag: tt.Diagnostic = .{};
    try testing.expectError(error.RecordSource, failed.next(&diag));
    try testing.expect(diag.code == null);
    try testing.expect(try failed.next(&diag) == null);
}

test "value equality excludes original records including nested form entries" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var memo = [_]u8{'a'};
    const left: Ticket = .{ .id = "same", .text = "same", .memo = &memo };
    const right: Ticket = .{ .id = "same", .text = "same", .memo = "different" };
    const question: tt.Decide = .{ .question = "Ready?", .field = "/text" };
    const first = try client.decide(arena.allocator(), question, left, null);
    const second = try client.decide(arena.allocator(), question, right, null);
    try testing.expect(first.eql(second));
    memo[0] = 'z';
    try testing.expect(first.eql(second));
    const form_question = tt.annotate(.{ .ready = question });
    const form_left = try client.annotate(arena.allocator(), form_question, left, null);
    const form_right = try client.annotate(arena.allocator(), form_question, right, null);
    try testing.expect(form_left.eql(form_right));
    try testing.expect((try form_left.answer(null)).ready.eql((try form_right.answer(null)).ready));
}

test "rank retains original values including failed rows through cleanup" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    const inputs = [_]tt.Record{ .{ .text = "first" }, .{ .text = "second" }, .missing };
    const question: tt.Rank(Urgency) = .{ .question = "Urgency?", .top = 1 };
    var ranks = try client.rank(testing.allocator, question, &inputs, null);
    var saved: std.ArrayList(tt.Row(tt.Rank(Urgency), tt.Record)) = .empty;
    defer saved.deinit(testing.allocator);
    while (try ranks.nextInto(arena.allocator(), null)) |row| try saved.append(testing.allocator, row);
    ranks.deinit();
    client.deinit();
    try testing.expectEqual(@as(usize, 2), saved.items.len);
    try testing.expectEqualStrings(inputs[@intCast(saved.items[0].position - 1)].text, saved.items[0].item.text);
    try testing.expectEqual(tt.Status.failed, saved.items[1].status);
    try testing.expect(saved.items[1].item == .missing);
}

test "usage is an independent allocation-free snapshot and keys never format" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    const usage = try client.usage(null);
    client.deinit();
    try testing.expectEqual(@as(u64, 0), usage.attempts);
    try testing.expectEqual(@as(?u64, 0), usage.input_tokens.reported);
    const secret = "synthetic-private-key";
    const settings: tt.Settings = .{ .api_key = tt.ApiKey.init(secret), .max_requests = 0 };
    const texts = [_][]u8{ try std.fmt.allocPrint(testing.allocator, "{any}", .{settings}), try std.fmt.allocPrint(testing.allocator, "{f}", .{settings}), try std.json.Stringify.valueAlloc(testing.allocator, settings, .{}) };
    defer for (texts) |text| testing.allocator.free(text);
    for (texts) |text| try testing.expect(std.mem.indexOf(u8, text, secret) == null);
}

test "nonfinite bars reach the shared typed threshold admission" {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var diag: tt.Diagnostic = .{};
    try testing.expectError(error.Usage, client.decide(arena.allocator(), tt.Decide{ .question = "Ready?", .threshold = .cut(std.math.nan(f64)) }, "text", &diag));
    try testing.expectEqual(tt.ErrorCode.usage_threshold, diag.code.?);
    try testing.expectError(error.Usage, client.find(arena.allocator(), tt.Find{ .question = "Which?", .threshold = .cut(std.math.inf(f64)) }, "text", &diag));
    try testing.expectEqual(tt.ErrorCode.usage_threshold, diag.code.?);
}

test "saved rows retain character positions and answered null without an item" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const json = "{\"function\":\"find\",\"position\":1,\"source\":{\"question_identity\":\"saved\",\"backend\":\"stub\",\"model\":\"stub\"},\"status\":\"answered\",\"answer\":{\"text\":\"Morgan\",\"start\":2,\"end\":8},\"probability\":0.9}";
    const row = try std.json.parseFromSliceLeaky(tt.Row(tt.Find, void), a, json, .{});
    try testing.expectEqual(@as(usize, 2), (try row.answer(null)).?.start);
    const output = try std.json.Stringify.valueAlloc(a, row, .{});
    try testing.expect(std.mem.indexOf(u8, output, "\"start\":2") != null);
    const empty = "{\"function\":\"find\",\"position\":1,\"source\":{\"question_identity\":\"saved\",\"backend\":\"stub\",\"model\":\"stub\"},\"status\":\"answered\",\"answer\":null,\"probability\":0.9}";
    const none = try std.json.parseFromSliceLeaky(tt.Row(tt.Find, void), a, empty, .{});
    try testing.expect(try none.answer(null) == null);
    try testing.expectEqualStrings(empty, try std.json.Stringify.valueAlloc(a, none, .{}));
}

test {
    _ = @import("registry.zig");
}

test "clients keep cancellation tokens independently of token teardown" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var stop = try tt.Cancel.init(null);
    var client = try tt.Client.init(.{ .backend = .stub, .cancel = &stop }, null);
    stop.deinit();
    const row = try client.decide(arena.allocator(), asks, "evidence", null);
    try testing.expect(row.status != .failed);
    client.deinit();
    var other = try tt.Cancel.init(null);
    var closed = try tt.Client.init(.{ .backend = .stub, .cancel = &other }, null);
    closed.deinit();
    other.cancel();
    other.deinit();
}

test "loaded enum tag sets and detail maps compare by their values" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const prefix = "{\"function\":\"tag\",\"position\":1,\"source\":{\"question_identity\":\"saved\",\"backend\":\"stub\",\"model\":\"stub\"},\"status\":\"answered\",";
    const left = prefix ++ "\"answer\":[\"billing\",\"shipping\"],\"detail\":{\"labels\":{\"billing\":0.5,\"shipping\":1.0}}}";
    const right = prefix ++ "\"answer\":[\"shipping\",\"billing\"],\"detail\":{\"labels\":{\"shipping\":1,\"billing\":0.50}}}";
    const first = try std.json.parseFromSliceLeaky(tt.Row(tt.Tag(Topic), void), arena.allocator(), left, .{});
    const second = try std.json.parseFromSliceLeaky(tt.Row(tt.Tag(Topic), void), arena.allocator(), right, .{});
    try testing.expect(first.eql(second));
    try testing.expect(second.eql(first));
}

// Feature: a scalar call preserves cancellation as a call error rather than a failed result.
test "scalar cancellation keeps the native cancellation diagnostic" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var stop = try tt.Cancel.init(null);
    defer stop.deinit();
    var client = try tt.Client.init(.{ .backend = .stub, .cancel = &stop }, null);
    defer client.deinit();
    stop.cancel();
    var diag: tt.Diagnostic = .{};
    try testing.expectError(error.Cancelled, client.decide(arena.allocator(), asks, "record", &diag));
    try testing.expectEqual(tt.ErrorCode.local_cancelled, diag.code.?);
}
