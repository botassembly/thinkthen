const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const works_for = [_]tt.RelationRule{.{
        .name = "works_for",
        .source = "person",
        .target = "organization",
    }};
    const relations = try (try client.relate(
        init.arena.allocator(),
        &works_for,
        .{ .text = "Morgan works for Example Group.\nPlease refund my order." },
        .{},
        &diag,
    )).answer(&diag);
    for (relations) |relation| try stdout.interface.print(
        "{s} --{s}--> {s}\n",
        .{ relation.source.text, relation.relation, relation.target.text },
    );
}
