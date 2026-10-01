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

    const policy = [_][]const u8{
        "Returns need the original receipt.",
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
        "Gift cards cannot be exchanged for cash.",
    };
    const find = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .find = "Which line gives the refund deadline?",
            .units = policy,
        }, .{})},
        0,
    );
    defer allocator.free(find);
    const called = try tt.call(find, .{});
    const reply = switch (called) {
        .ok => |bytes| bytes,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer allocator.free(reply);
    const located = try std.json.parseFromSlice(
        struct { value: struct { unit: []const u8 } },
        allocator,
        reply,
        .{ .ignore_unknown_fields = true },
    );
    defer located.deinit();
    const refund_deadline = located.value.value.unit;
    std.debug.assert(
        std.mem.eql(u8, refund_deadline, policy[1]),
    );
}
