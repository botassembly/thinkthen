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

    const form: tt.AnnotationQuestions = .{ .entries = &.{
        .{ .name = "refund", .question = .{ .decide = .{
            .question = "Does the customer request a refund?",
        } } },
        .{ .name = "topic", .question = .{ .choose = .{
            .question = "What is the message about?",
            .choices = tt.choices(&.{ "billing", "shipping", "other" }),
        } } },
        .{ .name = "urgency", .question = .{ .score = .{
            .question = "How urgent is the request?",
            .levels = tt.choices(&.{ "routine", "urgent" }),
        } } },
    } };
    const messages = [_][]const u8{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    var triage = try client.annotateAll(
        init.gpa,
        form,
        .{ .texts = &messages },
        .{},
        &diag,
    );
    defer triage.deinit();
    while (try triage.next(&diag)) |filled| {
        // Each answer can be not sure or failed on its own, and answer says so.
        const answers = try filled.answer(&diag);
        const is_refund = try answers.get(.decide, "refund").?.answer(&diag);
        const topic = try answers.get(.choose, "topic").?.answer(&diag);
        const urgency = try answers.get(.score, "urgency").?.answer(&diag);
        try out.print("{} {s} {d}\n", .{ is_refund, topic, urgency });
    }
}
