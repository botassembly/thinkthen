const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    // answer returns error.Unsure when not sure, and the message's error
    // when it failed.
    const is_refund = try (try client.decide(
        init.arena.allocator(),
        "Does the customer request a refund?",
        .{ .text = "Please refund my order." },
        .{},
        &diag,
    )).answer(&diag);
    try stdout.interface.print("{}\n", .{is_refund});
}
