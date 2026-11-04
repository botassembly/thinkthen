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

    const refund = try client.decide(
        init.arena.allocator(),
        "Does the customer request a refund?",
        .{ .text = "I was charged twice for one invoice." },
        .{ .threshold = tt.Threshold.band(0.2, 0.8) },
        &diag,
    );
    switch (refund.outcome) {
        .answered => |answered| try out.print(
            "{}\n",
            .{answered.answer},
        ),
        .unsure => |unsure| try out.print(
            "Not sure ({d}): ask a person.\n",
            .{unsure.probability},
        ),
        .failed => |failure| try out.print(
            "{s}: {s}\n",
            .{ failure.name(), failure.message },
        ),
    }
}
