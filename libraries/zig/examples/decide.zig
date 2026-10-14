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
    var call = try tt.calls.decide(&engine, .{
        .question = .{ .text = .{ .text = "Is this a complaint?" } },
        .input = .{ .text = .{ .text = "I demand a refund today" } },
    });
    defer call.deinit();
    while (true) switch (try call.next()) {
        .pending => return error.Defect,
        .end => break,
        .packet => |value| {
            const packet = value;
            defer packet.deinit();
            if (packet.failure()) |_| return error.DecisionFailed;
            if (packet.value().* == .observation) {
                const answer = packet.answers().decide() catch |err| {
                    if (err == error.WrongAnswer) continue;
                    return err;
                };
                std.debug.print("answer={any}\n", .{answer});
            }
            if (packet.facts()) |facts| std.debug.print("records={d} requests={d}\n", .{ facts.records, facts.requests_sent });
        },
    };
}
