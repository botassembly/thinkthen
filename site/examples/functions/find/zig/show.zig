const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    // null when no line answers.
    const action = try (try client.find(
        init.arena.allocator(),
        "Which line states the requested action?",
        .{ .text = "Morgan works for Example Group.\nPlease refund my order." },
        .{},
        &diag,
    )).answer(&diag);
    if (action) |line|
        try stdout.interface.print("{s}\n", .{line.text})
    else
        try stdout.interface.print("no line answers\n", .{});
}
