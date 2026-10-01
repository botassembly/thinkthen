const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var tt = switch (try thinkthen.Engine.init(allocator)) {
        .ok => |engine| engine,
        .failed => |failure| {
            thinkthen.releaseFailure(allocator, failure);
            return error.NoEngine;
        },
    };
    defer tt.deinit();

    const question = "Does the customer ask for a refund?";
    const text =
        "Please refund my order. It arrived broken.";
    const asked = try tt.decide(question, text, .{});
    const is_refund = switch (asked) {
        .ok => |answer| answer,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer is_refund.deinit(allocator);
    std.debug.assert(is_refund.value.outcome == .yes);
}
