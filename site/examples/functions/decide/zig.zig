const std = @import("std");
const tt = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => |failure| {
            tt.releaseFailure(allocator, failure);
            return error.NoEngine;
        },
    };
    defer engine.deinit();

    const question = "Does the customer ask for a refund?";
    const broken =
        "Please refund my order. It arrived broken.";
    var call = try tt.calls.decide(&engine, .{
        .question = .{ .text = .{ .text = question } },
        .input = .{ .text = .{ .text = broken } },
    });
    defer call.deinit();

    const is_refund = (try call.value()) orelse false;
    std.debug.assert(is_refund);
}
