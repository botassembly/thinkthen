const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    // From 0 (routine) to 1 (urgent).
    const urgency = try (try client.score(
        init.arena.allocator(),
        "How urgent is the request?",
        tt.choices(&.{ "routine", "urgent" }),
        .{ .text = "Please refund my order." },
        .{},
        &diag,
    )).answer(&diag);
    try stdout.interface.print("{d}\n", .{urgency});
}
