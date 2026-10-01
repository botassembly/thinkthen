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

    const levels = [_][]const u8{
        "Routine.",
        "Soon.",
        "Immediate.",
    };
    const score = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .score = "How urgent is this?",
            .levels = levels,
            .evidence = "Our checkout page is down " ++
                "and customers cannot pay.\n",
        }, .{})},
        0,
    );
    defer allocator.free(score);
    const called = try tt.call(score, .{});
    const reply = switch (called) {
        .ok => |bytes| bytes,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer allocator.free(reply);
    const scored = try std.json.parseFromSlice(
        struct { value: f64 },
        allocator,
        reply,
        .{ .ignore_unknown_fields = true },
    );
    defer scored.deinit();
    const urgency = scored.value.value;
    std.debug.assert(urgency == 2.0);
}
