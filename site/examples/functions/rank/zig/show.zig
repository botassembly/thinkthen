const std = @import("std");
const tt = @import("thinkthen");

const Urgency = enum { routine, urgent };

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
    const asks_urgency: tt.Rank(Urgency) = .{ .question = "How urgent is the request?" };
    var urgency_ranks = try client.rank(init.gpa, asks_urgency, &messages, &diag);
    defer urgency_ranks.deinit();
    while (try urgency_ranks.next(&diag)) |ranked|
        try stdout.interface.print("{d} {s}\n", .{ try ranked.answer(&diag), ranked.item });
}
