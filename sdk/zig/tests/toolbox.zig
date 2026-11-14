const std = @import("std");
const tt = @import("thinkthen");
pub fn saved(allocator: std.mem.Allocator, json: []const u8) ![]u8 {
    var loaded = try tt.Toolbox.fromJson(allocator, json, null);
    defer loaded.deinit();
    try std.testing.expect(loaded.get("missing") == null);
    try std.testing.expectEqual(@as(usize, 9), loaded.entries.len);
    const topic = try loaded.choose([]const u8, "topic", null);
    try std.testing.expectEqualStrings("billing", topic.choices[0]);
    const names = try loaded.relate("names", null);
    try std.testing.expectEqualStrings("pays", names.rules[0].relation);
    try std.testing.expectEqualStrings("pays money to", names.rules[0].reads.?);
    try std.testing.expect(names.rules[0].either);
    const encoded = try loaded.toJson(allocator, null);
    errdefer allocator.free(encoded);
    var roundtrip = try tt.Toolbox.fromJson(allocator, encoded, null);
    defer roundtrip.deinit();
    const repeated = try roundtrip.choose([]const u8, "topic", null);
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    const record: tt.Record = .{ .json = "{\"body\":\"Bill\"}" };
    const actual = try client.choose(arena.allocator(), topic, record, null);
    const direct = try client.choose(arena.allocator(), repeated, record, null);
    try std.testing.expect(actual.eql(direct));
    return encoded;
}
pub fn invalid(allocator: std.mem.Allocator, json: []const u8) !tt.Diagnostic {
    var diagnostic: tt.Diagnostic = .{};
    try std.testing.expectError(error.Usage, tt.Toolbox.fromJson(allocator, json, &diagnostic));
    try std.testing.expect(!diagnostic.truncated);
    return diagnostic;
}
