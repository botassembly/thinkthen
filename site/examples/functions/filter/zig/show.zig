const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const messages = [_][]const u8{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    const asks_refund: tt.Filter = .{ .question = "Does the customer request a refund?" };
    var refunds = try client.filter(init.gpa, asks_refund, &messages, &diag);
    defer refunds.deinit();
    // All kept originals arrive before an aggregate failure is reported.
    while (try refunds.next(&diag)) |refund|
        try stdout.interface.print("{s}\n", .{refund});
}
