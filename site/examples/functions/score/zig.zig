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

    const question = "How urgent is this?";
    const levels = [_][]const u8{
        "Routine.",
        "Soon.",
        "Immediate.",
    };
    const outage = "Our checkout page is down and " ++
        "customers cannot pay.\n";
    const how_urgent: tt.inputs.RequestDefinition = .{
        .Authored_score = .{
            .score = .{ .string = question },
            .levels = .{ .array = &levels },
        },
    };
    var call = try tt.calls.score(&engine, .{
        .question = .{
            .definition = .{ .value = how_urgent },
        },
        .input = .{ .text = .{ .text = outage } },
    });
    defer call.deinit();

    const urgency = try call.value();
    std.debug.assert(urgency == 2.0);
}
