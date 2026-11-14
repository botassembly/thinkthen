const std = @import("std");
const tt = @import("thinkthen");

const Topic = enum { billing, shipping, other };
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
    const asks_refund: tt.Decide = .{ .question = "Does the customer request a refund?" };
    const asks_topic: tt.Choose(Topic) = .{ .question = "What is the message about?" };
    const asks_urgency: tt.Score(Urgency) = .{ .question = "How urgent is the request?" };
    const questions = tt.annotate(.{
        .refund = asks_refund,
        .topic = asks_topic,
        .urgency = asks_urgency,
    });
    var forms = try client.annotateAll(init.gpa, questions, &messages, &diag);
    defer forms.deinit();
    while (try forms.next(&diag)) |form| {
        _ = try form.errIfFailed(&diag); // Includes every failed entry.
        const entries = try form.answer(&diag);
        const refund = try entries.refund.answer(&diag);
        const topic = try entries.topic.answer(&diag);
        const urgency = try entries.urgency.answer(&diag);
        try stdout.interface.print("{} {t} {d}\n", .{ refund, topic, urgency });
    }
}
