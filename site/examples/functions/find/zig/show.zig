const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const asks_action: tt.Find = .{ .question = "Which line states the requested action?" };
    const action = try client.find(arena, asks_action,
        "Morgan works for Example Group.\nPlease refund my order.", &diag);
    if (try action.answer(&diag)) |span|
        try stdout.interface.print("{s}\n", .{span.text})
    else
        try stdout.interface.print("no line answers\n", .{});
}
