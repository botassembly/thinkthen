const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
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
    for (topics) |topic| try stdout.interface.print("{s}\n", .{topic});
}
