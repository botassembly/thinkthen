const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var tt = switch (try thinkthen.Engine.init(allocator)) {
        .ok => |engine| engine,
        .failed => return error.NoEngine,
    };
    defer tt.deinit();

    const question = "Does the customer ask for a refund?";
    const broken =
        "Please refund my order. It arrived broken.";
    const asked = try tt.decide(question, broken, .{});
    const is_refund = switch (asked) {
        .ok => |answer| answer,
        .failed => return error.Failed,
    };
    defer is_refund.deinit(allocator);
    std.debug.assert(is_refund.value.outcome == .yes);

    const refund = "{\"decide\": \"" ++ question ++
        "\", \"threshold\": \"0.2:0.8\"}";
    const back = "I want to send this back.";
    const back_call = try tt.decide(refund, back, .{});
    const back_is_refund = switch (back_call) {
        .ok => |answer| answer,
        .failed => return error.Failed,
    };
    defer back_is_refund.deinit(allocator);
    const outcome = back_is_refund.value.outcome;
    std.debug.assert(outcome == .unsure);
}
