const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const topic = try (try client.choose(
        init.arena.allocator(),
        "What is the message about?",
        tt.choices(&.{ "billing", "shipping", "other" }),
        .{ .text = "Please refund my order." },
        .{},
        &diag,
    )).answer(&diag);
    try stdout.interface.print("{s}\n", .{topic});
}
