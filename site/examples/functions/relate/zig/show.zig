const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const finds_relations: tt.Relate = .{ .rules = &.{
        .{ .source = "person", .target = "organization", .relation = "works_for" },
    } };
    const relations = try client.relate(arena, finds_relations,
        "Morgan works for Example Group.\nPlease refund my order.", &diag);
    for (try relations.answer(&diag)) |relation|
        try stdout.interface.print("{s} --{s}--> {s}\n",
            .{ relation.source.text, relation.relation, relation.target.text });
}
