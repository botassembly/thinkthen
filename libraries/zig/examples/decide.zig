const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    var gpa = std.heap.DebugAllocator(.{}){};
    defer if (gpa.deinit() != .ok) @panic("Zig allocator leak");
    const allocator = gpa.allocator();
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => |failure| {
            defer allocator.free(failure.message);
            std.debug.print("engine failure: {s}\n", .{failure.message});
            return error.EngineBuild;
        },
    };
    defer engine.deinit();
    switch (try engine.decide("Is this a complaint?", "I demand a refund today", .{})) {
        .ok => |answer| {
            defer answer.deinit(allocator);
            // Facts are host JSON: read the members you need and ignore the rest.
            const facts = answer.facts.value.object;
            if (facts.get("records").?.integer != 1 or facts.get("requests_sent").?.integer != 1) return error.WrongFacts;
            std.debug.print("{s} {d:.2}\n", .{ @tagName(answer.value.outcome), answer.value.probability });
        },
        .failed => |failure| {
            defer engine.freeFailure(failure);
            std.debug.print("decision failure {d}: {s}\n", .{ failure.code, failure.message });
            return error.DecisionFailed;
        },
    }
}
