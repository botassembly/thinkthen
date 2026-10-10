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
    var call = try tt.session.decide(&engine, .{
        .question = .{ .text = .{ .text = "Is this a complaint?" } },
        .input = .{ .text = .{ .text = "I demand a refund today" } },
    });
    defer call.deinit();
    while (true) switch (try call.read()) {
        .pending => std.Thread.yield() catch {},
        .end => break,
        .packet => |value| {
            var packet = value;
            defer packet.deinit();
            if (packet.failure()) |_| return error.DecisionFailed;
            if (packet.facts()) |facts| std.debug.print("records={d} requests={d}\n", .{ facts.records, facts.requests_sent });
        },
    };
}
