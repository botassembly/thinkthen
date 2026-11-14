const std = @import("std");
const tt = @import("thinkthen");

const Topic = enum { billing, shipping, other };

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const asks_topic: tt.Choose(Topic) = .{ .question = "What is the message about?" };
    const topic = try client.choose(arena, asks_topic, "Please refund my order.", &diag);
    try stdout.interface.print("{t}\n", .{try topic.answer(&diag)});
}
