const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var buffer: [4096]u8 = undefined;
    var stdout = std.Io.File.stdout().writer(init.io, &buffer);
    const out = &stdout.interface;
    defer out.flush() catch {};
    var diag: tt.Diagnostic = .{};
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
        try out.print("{s}\n", .{line.text})
    else
        try out.print("no line answers\n", .{});
}
