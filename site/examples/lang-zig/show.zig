const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const asks_refund = try client.decide(
        init.arena.allocator(),
        "Does the customer request a refund?",
        .{ .text = "Please refund my order." },
        .{},
        &diag,
    );
    switch (asks_refund.outcome) {
        .answered => |answered| try stdout.interface.print(
            "answer: {}\n",
            .{answered.answer},
        ),
        .unsure => try stdout.interface.print("not sure\n", .{}),
        .failed => |failure| try stdout.interface.print(
            "{s}: {s}\n",
            .{ failure.name(), failure.message },
        ),
    }
}
