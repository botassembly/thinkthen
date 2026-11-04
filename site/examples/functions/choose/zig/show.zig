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

    const topic = try (try client.choose(
        init.arena.allocator(),
        "What is the message about?",
        tt.choices(&.{ "billing", "shipping", "other" }),
        .{ .text = "Please refund my order." },
        .{},
        &diag,
    )).answer(&diag);
    try out.print("{s}\n", .{topic});
}
