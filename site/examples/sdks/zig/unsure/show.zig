const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const refund = try client.decide(
        init.arena.allocator(),
        "Does the customer request a refund?",
        .{ .text = "I was charged twice for one invoice." },
        .{ .threshold = tt.Threshold.band(0.2, 0.8) },
        &diag,
    );
    switch (refund.outcome) {
        .answered => |answered| try stdout.interface.print(
            "{}\n",
            .{answered.answer},
        ),
        .unsure => |unsure| try stdout.interface.print(
            "Not sure ({d}): ask a person.\n",
            .{unsure.probability},
        ),
        .failed => |failure| try stdout.interface.print(
            "{s}: {s}\n",
            .{ failure.name(), failure.message },
        ),
    }
}
