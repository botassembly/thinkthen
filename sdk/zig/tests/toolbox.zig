const std = @import("std");
const tt = @import("thinkthen");

// Feature: saved Zig toolboxes own hydrated domains and preserve native rows.
pub fn saved(allocator: std.mem.Allocator, json: []const u8) ![]u8 {
    var loaded = try tt.Toolbox.fromJson(allocator, json, null);
    defer loaded.deinit();
    try std.testing.expect(loaded.get("missing") == null);
    try std.testing.expectEqual(@as(usize, 9), loaded.entries.len);
    const topic = loaded.get("topic").?.as(.choose).?;
    try std.testing.expectEqual(@as(usize, 2), topic.choices.len);
    try std.testing.expectEqualStrings("billing", topic.choices[0].name);
    const names = loaded.get("names").?.as(.relate).?;
    try std.testing.expectEqualStrings("pays", names.rules[0].name);
    try std.testing.expectEqualStrings("pays money to", names.rules[0].reads.?);
    try std.testing.expect(names.rules[0].either.?);
    const constructed: tt.Toolbox = .{ .entries = loaded.entries };
    const encoded = try constructed.toJson(allocator, null);
    errdefer allocator.free(encoded);
    var roundtrip = try tt.Toolbox.fromJson(allocator, encoded, null);
    defer roundtrip.deinit();
    try std.testing.expectEqualDeep(loaded.entries, roundtrip.entries);
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const record: tt.Record = .{ .json = "{\"body\":\"Bill\"}" };
    const actual = try client.chooseOne(a, topic.question, topic.choices, record, topic.options, null);
    const choices = [_]tt.Choice{ .{ .name = "billing" }, .{ .name = "shipping" } };
    const direct = try client.chooseOne(a, "Which topic?", &choices, record, .{ .evidence = "/body", .threshold = .{ .cut = 0.7 } }, null);
    const direct_json = try std.json.Stringify.valueAlloc(a, direct, .{ .emit_null_optional_fields = false });
    const actual_json = try std.json.Stringify.valueAlloc(a, actual, .{ .emit_null_optional_fields = false });
    try std.testing.expectEqualStrings(direct_json, actual_json);
    return encoded;
}
pub fn invalid(allocator: std.mem.Allocator, json: []const u8) !tt.Diagnostic {
    var diagnostic: tt.Diagnostic = .{};
    try std.testing.expectError(error.Usage, tt.Toolbox.fromJson(allocator, json, &diagnostic));
    try std.testing.expect(!diagnostic.truncated);
    return diagnostic;
}
