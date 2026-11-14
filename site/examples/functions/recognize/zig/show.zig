const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const finds_names: tt.Recognize = .{ .kinds = &.{ "person", "organization" } };
    const names = try client.recognize(arena, finds_names, "Morgan works for Example Group.", &diag);
    for (try names.answer(&diag)) |name|
        try stdout.interface.print("{s}: {s}\n", .{ name.kind, name.text });
}
