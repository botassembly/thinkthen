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
        try out.print("{d} {s}\n", .{
            rank,
            messages[@intCast(ranked.position - 1)],
        });
    }
}
