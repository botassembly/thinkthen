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
    var call = switch (try tt.calls.decide(&engine, .{
        .question = .{ .text = .{ .text = question } },
        .input = tt.inputs.RequestInputText{ .text = broken },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const is_refund = if (call.value) |v| v.boolean else false;
    std.debug.assert(is_refund);
}
