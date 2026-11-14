const std = @import("std");
const tt = @import("thinkthen");

const Urgency = enum { routine, urgent };

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const asks_urgency: tt.Score(Urgency) = .{ .question = "How urgent is the request?" };
    const urgency = try client.score(arena, asks_urgency, "Please refund my order.", &diag);
    try stdout.interface.print("{d}\n", .{try urgency.answer(&diag)});
}
