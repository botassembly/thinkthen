const std = @import("std");
const tt = @import("thinkthen");
const check = std.testing;
const Topic = enum { billing, shipping, other };
const Urgency = enum(u8) { routine = 9, urgent = 1 };
const Ticket = struct { id: []const u8, text: []const u8, other: []const u8 = "" };
const questions = tt.annotate(.{
    .refund = tt.Decide{ .question = "Does the customer request a refund?", .threshold = .band(0.2, 0.8) },
    .topic = tt.Choose(Topic){ .question = "What is the message about?" },
    .urgency = tt.Score(Urgency){ .question = "How urgent is the request?" },
});
const DeclinedSource = struct {
    allocator: *std.testing.FailingAllocator,
    sent: usize = 0,
    max_live: usize = 0,
    buffer: [4096]u8 = @splat('x'),
    pub fn next(self: *@This()) error{RecordSource}!?tt.Record {
        const live = self.allocator.allocated_bytes - self.allocator.freed_bytes;
        self.max_live = @max(self.max_live, live);
        if (live > 128 * 1024) return error.RecordSource;
        if (self.sent == 256) return null;
        self.sent += 1;
        return .{ .text = &self.buffer };
    }
};
pub fn main(init: std.process.Init) !void {
    const a = init.arena.allocator();
    const args = try init.minimal.args.toSlice(a);
    const mode = args[1];
    var diag: tt.Diagnostic = .{};
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();
    if (std.mem.eql(u8, mode, "scalars")) {
        const all = .{
            tt.Decide{ .question = "Ready?" },
            tt.Choose(Topic){ .question = "Topic?" },
            tt.Tag(Topic){ .question = "Topics?" },
            tt.Score(Urgency){ .question = "Urgency?" },
            tt.Find{ .question = "Which line?" },
            tt.Recognize{ .kinds = &.{ "person", "organization" } },
            tt.Relate{ .rules = &.{.{ .source = "person", .target = "organization", .relation = "works_for" }} },
            questions,
        };
        inline for (all) |question| {
            const method = @tagName(@TypeOf(question).function);
            try check.expectError(error.Failed, @field(tt.Client, method)(&client, a, question, "", &diag));
            try check.expectEqual(tt.ErrorCode.record_empty, diag.code.?);
            try check.expectEqual(@as(u64, 1), diag.position.?);
            try check.expect(diag.message().len != 0);
            try check.expectError(error.Failed, @field(tt.Client, method)(&client, a, question, "", null));
        }
        {
            const records = [_][]const u8{""};
            var call = try client.decideAll(init.gpa, tt.Decide{ .question = "Ready?" }, &records, &diag);
            defer call.deinit();
            const row = (try call.next(&diag)).?;
            try check.expectEqual(tt.Status.failed, row.status);
            try check.expectEqual(records[0].ptr, row.item.ptr);
            try check.expectError(error.Failed, row.answer(&diag));
            try check.expectEqual(tt.ErrorCode.record_empty, diag.code.?);
        }
        const usage = try client.usage(&diag);
        try check.expectEqual(@as(u64, 0), usage.attempts);
        try check.expectEqual(@as(u64, 0), usage.active_calls);
        try check.expectEqual(@as(u64, 0), usage.active_attempts);
        try stdout.interface.print("scalar boundaries okay\n", .{});
    } else if (std.mem.eql(u8, mode, "scalar-outcomes")) {
        const name = args[2];
        const asks: tt.Decide = .{ .question = "Does the customer request a refund?", .threshold = .band(0.2, 0.8), .field = "/text" };
        const record: Ticket = .{ .id = "kept", .text = "Please refund my order." };
        if (client.decide(a, asks, record, &diag)) |row| {
            try check.expect(!std.mem.eql(u8, name, "failed"));
            try check.expect(row.failure == null);
            try check.expectEqual(record.text.ptr, row.item.text.ptr);
            try check.expectEqualStrings(record.id, row.id.?.string);
            if (std.mem.eql(u8, name, "unsure")) {
                try check.expectEqual(tt.Status.unsure, row.status);
                try check.expectEqual(@as(f64, 0.5), row.probability.?);
                try check.expectError(error.Unsure, row.answer(&diag));
            } else {
                try check.expectEqual(std.mem.eql(u8, name, "yes"), try row.answer(&diag));
            }
        } else |err| {
            try check.expectEqualStrings("failed", name);
            try check.expect(err == error.Failed);
            try check.expectEqual(tt.ErrorCode.backend_status, diag.code.?);
            try check.expectEqual(@as(u16, 503), diag.http_status.?);
            try check.expectEqual(@as(u64, 1), diag.position.?);
        }
        const usage = try client.usage(&diag);
        try check.expectEqual(@as(u64, 0), usage.active_calls);
        try check.expectEqual(@as(u64, 0), usage.active_attempts);
        try check.expectError(error.Usage, client.decide(a, tt.Decide{ .question = "Ready?", .threshold = .cut(2) }, "record", &diag));
        try check.expectEqual(tt.ErrorCode.usage_threshold, diag.code.?);
        try stdout.interface.print("scalar outcomes okay\n", .{});
    } else if (std.mem.eql(u8, mode, "forms")) {
        const name = args[2];
        const record: Ticket = .{ .id = "original-id", .text = "Please refund my order." };
        const form = try client.annotate(a, questions, record, &diag);
        try check.expectEqual(tt.Status.answered, form.status);
        const entries = try form.answer(&diag);
        const failed = std.mem.endsWith(u8, name, "failed");
        if (failed) {
            try check.expectError(error.Failed, form.errIfFailed(&diag));
            const first = if (std.mem.eql(u8, name, "topic-failed")) entries.topic.failure.? else if (std.mem.eql(u8, name, "urgency-failed")) entries.urgency.failure.? else entries.refund.failure.?;
            try check.expectEqualStrings(first.name(), diag.code.?.name());
            try check.expectEqualStrings(first.message, diag.message());
            try check.expectEqual(form.position, diag.position.?);
        } else {
            const same = try form.errIfFailed(&diag);
            try check.expect(same.eql(form));
        }
        inline for (.{ "refund", "topic", "urgency" }) |field| {
            const child = @field(entries, field);
            try check.expectEqualStrings(form.id.?.string, child.id.?.string);
            try check.expectEqual(form.position, child.position);
            try check.expectEqualStrings(form.source.question_identity, child.source.question_identity);
            try check.expectEqual(form.item.text.ptr, child.item.text.ptr);
            const saved_child = try std.json.Stringify.valueAlloc(a, child, .{});
            const loaded = try std.json.parseFromSliceLeaky(tt.Row(@TypeOf(@field(questions.entries, field)), void), a, saved_child, .{});
            try check.expectEqual(child.status, loaded.status);
            try check.expectEqual(form.position, loaded.position);
            try check.expectEqualStrings(form.id.?.string, loaded.id.?.string);
            try check.expectEqualStrings(form.source.question_identity, loaded.source.question_identity);
            if (child.status == .failed) try check.expectError(error.Failed, child.answer(&diag));
            if (child.status == .unsure) try check.expectError(error.Unsure, child.answer(&diag));
        }
        if (std.mem.eql(u8, name, "refund-unsure")) try check.expectEqual(tt.Status.unsure, entries.refund.status);
        if (std.mem.eql(u8, name, "topic-unsure")) try check.expectEqual(tt.Status.unsure, entries.topic.status);
        if (std.mem.eql(u8, name, "yes")) try check.expectEqual(@as(f64, 1), try entries.urgency.answer(&diag));
        if (std.mem.eql(u8, name, "low-score")) try check.expectEqual(@as(f64, 0), try entries.urgency.answer(&diag));
        try stdout.interface.print("forms okay {s}\n", .{name});
    } else if (std.mem.eql(u8, mode, "outer-form")) {
        try check.expectError(error.Failed, client.annotate(a, questions, "  ", &diag));
        try check.expectEqual(tt.ErrorCode.record_empty, diag.code.?);
        try stdout.interface.print("outer failure okay\n", .{});
    } else if (std.mem.eql(u8, mode, "unicode")) {
        const asks: tt.Find = .{ .question = "Which line states the requested action?" };
        const ascii = try client.find(a, asks, "ab\nPlease refund my order.", &diag);
        const unicode = try client.find(a, asks, "😀é\nPlease refund my order.", &diag);
        const first = (try ascii.answer(&diag)).?;
        const second = (try unicode.answer(&diag)).?;
        try check.expectEqual(@as(usize, 3), first.start);
        try check.expectEqual(@as(usize, 7), second.start);
        try check.expect(ascii.eql(unicode));
        const saved = try std.json.Stringify.valueAlloc(a, unicode, .{});
        const object = try std.json.parseFromSliceLeaky(std.json.Value, a, saved, .{});
        try check.expectEqual(@as(i64, 3), object.object.get("answer").?.object.get("start").?.integer);
        var form_question = tt.annotate(.{ .action = asks, .override = tt.Find{ .question = asks.question, .field = "/other" } });
        form_question.field = "/text";
        const record: Ticket = .{ .id = "kept", .text = "😀\nPlease refund my order.", .other = "éé\nPlease refund my order." };
        const form = try client.annotate(a, form_question, record, &diag);
        client.deinit();
        const entries = try form.answer(&diag);
        try check.expectEqual(@as(usize, 5), (try entries.action.answer(&diag)).?.start);
        try check.expectEqual(@as(usize, 5), (try entries.override.answer(&diag)).?.start);
        const canonical = try std.json.Stringify.valueAlloc(a, form, .{});
        const values = (try std.json.parseFromSliceLeaky(std.json.Value, a, canonical, .{})).object.get("answer").?.object;
        try check.expectEqual(@as(i64, 2), values.get("action").?.object.get("answer").?.object.get("start").?.integer);
        try check.expectEqual(@as(i64, 3), values.get("override").?.object.get("answer").?.object.get("start").?.integer);
        inline for (.{ "action", "override" }, .{ 2, 3 }) |field, start| {
            const saved_child = try std.json.Stringify.valueAlloc(a, @field(entries, field), .{});
            const loaded = try std.json.parseFromSliceLeaky(tt.Row(tt.Find, void), a, saved_child, .{});
            try check.expectEqual(form.position, loaded.position);
            try check.expectEqualStrings(form.id.?.string, loaded.id.?.string);
            try check.expectEqualStrings(form.source.question_identity, loaded.source.question_identity);
            try check.expectEqual(@as(usize, start), (try loaded.answer(&diag)).?.start);
        }
        try stdout.interface.print("unicode ownership okay\n", .{});
    } else if (std.mem.eql(u8, mode, "order")) {
        const order = tt.annotate(.{ .@"q𐐀" = tt.Decide{ .question = "Missing?", .field = "/missing-astral" }, .@"qＸ" = tt.Decide{ .question = "Missing?", .field = "/missing-bmp" } });
        const row = try client.annotate(a, order, Ticket{ .id = "ordered", .text = "evidence" }, &diag);
        try check.expectError(error.Failed, row.errIfFailed(&diag));
        const first = (try row.answer(null)).@"qＸ";
        try check.expectEqualStrings(first.failure.?.message, diag.message());
        try stdout.interface.print("UTF-8 form order okay\n", .{});
    } else if (std.mem.eql(u8, mode, "print")) {
        const question: tt.Choose([]const u8) = .{ .question = "Which?", .choices = &.{ "billing\nqueue", "shipping" } };
        const row = try client.choose(a, question, "message", &diag);
        try check.expectEqualStrings("billing\nqueue", try row.answer(&diag));
        try check.expectEqualStrings("billing queue", try std.fmt.allocPrint(a, "{f}", .{row}));
        const saved = try std.json.Stringify.valueAlloc(a, row, .{});
        try check.expect(std.mem.indexOf(u8, saved, "billing\\nqueue") != null);
        try stdout.interface.print("printed leaves okay\n", .{});
    } else if (std.mem.eql(u8, mode, "refusals")) {
        const saved = "{\"entries\":{\"topic\":{\"question\":{\"function\":\"choose\",\"question\":\"Which?\",\"choices\":[\"billing\",\"shipping\",\"other\"]}}}}";
        var toolbox = try tt.Toolbox.fromJson(init.gpa, saved, &diag);
        defer toolbox.deinit();
        try check.expectError(error.Usage, toolbox.choose(Topic, "missing", &diag));
        try check.expectEqual(tt.ErrorCode.usage_name, diag.code.?);
        try check.expectEqual(tt.ErrorCode.Family.usage, diag.code.?.family());
        try check.expectError(error.Usage, toolbox.decide("topic", &diag));
        try check.expectEqual(tt.ErrorCode.usage_name, diag.code.?);
        const WrongOrder = enum { shipping, billing, other };
        try check.expectError(error.Usage, toolbox.choose(WrongOrder, "topic", &diag));
        try check.expectEqual(tt.ErrorCode.usage_question, diag.code.?);
        const Missing = enum { billing, shipping };
        try check.expectError(error.Usage, toolbox.choose(Missing, "topic", &diag));
        const band = "{\"entries\":{\"keep\":{\"question\":{\"function\":\"filter\",\"question\":\"Keep?\",\"threshold\":{\"low\":0.2,\"high\":0.8}}}}}";
        try check.expectError(error.Usage, tt.Toolbox.fromJson(init.gpa, band, &diag));
        try check.expectEqual(tt.ErrorCode.usage_threshold, diag.code.?);
        var descriptions: std.json.ArrayHashMap([]const u8) = .{};
        try descriptions.map.put(a, "not-a-choice", "wrong");
        try check.expectError(error.Usage, client.choose(a, tt.Choose([]const u8){ .question = "Which?", .choices = &.{ "billing", "shipping" }, .descriptions = descriptions }, "text", &diag));
        try check.expectEqual(tt.ErrorCode.usage_question, diag.code.?);
        try stdout.interface.print("runtime refusals okay\n", .{});
    } else if (std.mem.eql(u8, mode, "bounded-filter")) {
        var tracker = std.testing.FailingAllocator.init(init.gpa, .{});
        var source: DeclinedSource = .{ .allocator = &tracker };
        var bounded_client = try tt.Client.init(.{ .batch = 4 }, &diag);
        defer bounded_client.deinit();
        var call = try bounded_client.filter(tracker.allocator(), tt.Filter{ .question = "Keep?" }, tt.Input{ .source = tt.RecordSource.from(&source) }, &diag);
        try check.expect(try call.next(&diag) == null);
        try check.expectEqual(@as(usize, 256), source.sent);
        call.deinit();
        try check.expectEqual(tracker.allocated_bytes, tracker.freed_bytes);
        try stdout.interface.print("bounded declined source okay\n", .{});
    } else return error.UnknownMode;
}
