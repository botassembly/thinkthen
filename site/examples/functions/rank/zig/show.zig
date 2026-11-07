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
    // Every message, most urgent first. The answer is its rank, starting at 1.
    var by_urgency = try client.rankAll(
        init.gpa,
        "How urgent is the request?",
        tt.choices(&.{ "routine", "urgent" }),
        .{ .texts = &messages },
        .{},
        &diag,
    );
    defer by_urgency.deinit();
    while (try by_urgency.next(&diag)) |ranked| {
        const rank = try ranked.answer(&diag);
        try stdout.interface.print("{d} {s}\n", .{
            rank,
            messages[@intCast(ranked.position - 1)],
        });
    }
}
