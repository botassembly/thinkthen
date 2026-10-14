const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    const a = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(a)) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            return error.EngineBuild;
        },
    };
    defer engine.deinit();
    var call = switch (try tt.calls.decide(&engine, .{
        .question = .{ .text = .{ .text = "Is this a complaint?" } },
        .input = tt.inputs.RequestInputText{ .text = "I demand a refund today" },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();
    const answer = call.value;
    std.debug.print("answer={any}\n", .{answer});
    if (call.facts) |facts| std.debug.print("records={d} requests={d}\n", .{ facts.records, facts.requests_sent });
}
