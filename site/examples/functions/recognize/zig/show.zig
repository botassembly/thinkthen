const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const names = try (try client.recognize(
        init.arena.allocator(),
        tt.choices(&.{ "person", "organization" }),
        .{ .text = "Morgan works for Example Group." },
        .{},
        &diag,
    )).answer(&diag);
    for (names) |name| try stdout.interface.print(
        "{s}: {s}\n",
        .{ name.kind, name.text },
    );
}
