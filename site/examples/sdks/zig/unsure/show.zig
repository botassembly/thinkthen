const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const asks_refund: tt.Decide = .{
        .question = "Does the customer request a refund?",
        .threshold = .band(0.2, 0.8),
    };
    const refund = try client.decide(arena, asks_refund, "I was charged twice for one invoice.", &diag);
    switch (refund.status) {
        .answered => try stdout.interface.print("{}\n", .{try refund.answer(&diag)}),
        .unsure => try stdout.interface.print("not sure ({d}): ask a person\n", .{refund.probability.?}),
        .failed => {
            _ = try refund.errIfFailed(&diag);
        },
    }
}
