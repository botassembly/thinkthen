const std = @import("std");
const t = @import("thinkthen").complete;
test "prepared requests deep clone ordered records and images" {
    const a = std.testing.allocator;
    var bytes = [_]u8{ 0, 255, 1 };
    const image: t.ImageInput = .{ .media = .png, .bytes = &bytes, .filename = "one.png" };
    const options = [_]t.Choice{ .{ .name = "second", .description = try t.json(a, "{\"detail\":false}"), .weight = 0 }, .{ .name = "first", .description = t.text("description"), .weight = null } };
    const record: t.RecordInput = .{ .original = null, .context = t.text("context"), .options = &options, .images = &.{ image, image } };
    const source = t.InputSource.fromRecords(&.{ record, record });
    const question = t.QuestionInput.asked(t.asked(.choose, t.text("α\r\nquestion")));
    const controls: t.CallControls = .{ .context = null, .batch = null, .batch_max = false, .attempts = true };
    const builders = [_]*const fn (t.QuestionInput, t.InputSource, t.CallControls) t.CompleteRequest{ t.Requests.decide, t.Requests.choose, t.Requests.tag, t.Requests.score, t.Requests.filter, t.Requests.rank, t.Requests.find, t.Requests.annotate, t.Requests.recognize, t.Requests.relate };
    for (builders, 0..) |builder, i| {
        var copy = try t.clone(t.CompleteRequest, a, builder(question, source, controls));
        defer copy.deinit();
        try std.testing.expectEqual(i, @intFromEnum(copy.value.function));
        try std.testing.expectEqualStrings("second", copy.value.source.records.?[0].options[0].name);
        try std.testing.expect(copy.value.source.records.?[0].original == null);
        try std.testing.expectEqualSlices(u8, &.{ 0, 255, 1 }, copy.value.source.records.?[1].images[1].bytes);
    }
    var cloned = try t.clone(t.CompleteRequest, a, t.Requests.choose(question, source, controls));
    defer cloned.deinit();
    bytes[0] = 9;
    try std.testing.expectEqual(@as(u8, 0), cloned.value.source.records.?[0].images[0].bytes[0]);
    const files = t.Requests.find(t.QuestionInput.questionFile("explicit.json"), t.InputSource.fromFiles(.{ .paths = &.{ "a", "a" }, .unit = .window, .window = 2 }), controls);
    try std.testing.expectEqualStrings("explicit.json", files.question.file.?);
    try std.testing.expectEqual(@as(usize, 2), files.source.files.?.paths.len);
}
test "typed field serialization preserves optional empties and exact numbers" {
    const a = std.testing.allocator;
    const span: t.NameSpan = .{ .start = 1, .end = 2, .kinds = null, .edges = &.{} };
    var copy = try t.clone(t.NameSpan, a, span);
    defer copy.deinit();
    try std.testing.expect(copy.value.kinds == null and copy.value.edges.?.len == 0);
    const facts: t.CallFacts = .{ .call_id = try t.CallId.init("a" ** 64), .cache_answers = 0, .estimated_cost_usd = "0.000001", .input_tokens = 0, .model = null, .output_tokens = null, .records = 2, .requests_sent = 0, .seconds = 0, .command_ms = null };
    var exact = try t.clone(t.CallFacts, a, facts);
    defer exact.deinit();
    try std.testing.expectEqualStrings("0.000001", exact.value.estimated_cost_usd.?);
    try std.testing.expect(exact.value.input_tokens.? == 0 and exact.value.output_tokens == null);
    try std.testing.expectError(error.InvalidIdentity, t.AnswerId.init("A" ** 64));
    try std.testing.expectError(error.InvalidIdentity, t.AnswerId.init("a" ** 63));
    try std.testing.expectError(error.InvalidIdentity, t.AnswerId.init("a" ** 65));
    comptime {
        if (t.CallId == t.AnswerId) @compileError("distinct identity types required");
    }
    var maximum = try t.clone(t.TokenUsage, a, .{ .input_tokens = std.math.maxInt(u64), .output_tokens = 0 });
    defer maximum.deinit();
    try std.testing.expectEqual(std.math.maxInt(u64), maximum.value.input_tokens);
}
test "serialized facts require observed identity and preserve absent counters" {
    const a = std.testing.allocator;
    var facts = try t.readFacts(a,
        \\{"call_id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":0,"records":2,"requests_sent":0,"seconds":0}
    );
    defer facts.deinit();
    try std.testing.expect(facts.value.input_tokens.? == 0 and facts.value.output_tokens == null);
    try std.testing.expectEqualStrings("0.000001", facts.value.estimated_cost_usd.?);
    try std.testing.expectError(error.MissingField, t.readFacts(a, "{\"cache_answers\":0}"));
    try std.testing.expectError(error.InvalidCharacter, t.readFacts(a,
        \\{"call_id":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","cache_answers":0,"records":0,"requests_sent":0,"seconds":0}
    ));
}
test "native snapshots preserve existing named initialization" {
    const n = @import("thinkthen").native;
    const c = @import("thinkthen").c;
    var snapshot = n.Snapshot{
        .arena = std.heap.ArenaAllocator.init(std.testing.allocator),
        .summary = std.mem.zeroes(c.thinkthen_summary_v1),
        .rows = &.{},
        .observations = &.{},
        .details = &.{},
        .observation_details = &.{},
        .authors = &.{},
        .observation_authors = &.{},
        .member_authors = &.{},
        .rank_members = &.{},
        .located_recognition = &.{},
        .located_relations = &.{},
    };
    defer snapshot.deinit();
    try std.testing.expectEqual(@as(usize, 0), snapshot.rank_member_details.len);
}
