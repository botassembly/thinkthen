const std = @import("std");
const tt = @import("thinkthen");

const Topic = enum { billing, shipping };

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const asks_topics: tt.Tag(Topic) = .{ .question = "Which topics appear?" };
    const topics = try client.tag(arena, asks_topics, "I was charged twice and my parcel is late again.", &diag);
    const fits = try topics.answer(&diag);
    var members = fits.iterator();
    while (members.next()) |topic|
        try stdout.interface.print("{t}\n", .{topic});
}
