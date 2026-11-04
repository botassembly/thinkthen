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

    // From 0 (routine) to 1 (urgent).
    const urgency = try (try client.score(
        init.arena.allocator(),
        "How urgent is the request?",
        tt.choices(&.{ "routine", "urgent" }),
        .{ .text = "Please refund my order." },
        .{},
        &diag,
    )).answer(&diag);
    try out.print("{d}\n", .{urgency});
}
