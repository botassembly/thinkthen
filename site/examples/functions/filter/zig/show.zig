const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const messages = [_][]const u8{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    // next yields only the kept messages.
    // A failed message returns its error in its place.
    var refunds = try client.filterAll(
        init.gpa,
        "Does the customer request a refund?",
        .{ .texts = &messages },
        .{},
        &diag,
    );
    defer refunds.deinit();
    while (try refunds.next(&diag)) |refund|
        try stdout.interface.print("{s}\n", .{refund.text});
}
