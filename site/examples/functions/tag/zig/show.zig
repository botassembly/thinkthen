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

    // Every topic that fits, which may be none.
    const topics = try (try client.tag(
        init.arena.allocator(),
        "Which topics appear?",
        tt.choices(&.{ "billing", "shipping" }),
        .{ .text = "I was charged twice and my parcel is late again." },
        .{},
        &diag,
    )).answer(&diag);
    for (topics) |topic| try out.print("{s}\n", .{topic});
}
