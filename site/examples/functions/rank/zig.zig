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

    const inbox = [_][]const u8{
        "Newsletter: our autumn catalog is here. " ++
            "No reply needed.",
        "Our checkout page is down and customers " ++
            "cannot pay",
        "Reminder: your invoice is due in 30 days",
        "Please send the signed quote by 5 pm today",
    };
    const rank = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .rank = "Is this urgent?",
            .records = inbox,
        }, .{})},
        0,
    );
    defer allocator.free(rank);
    const called = try tt.call(rank, .{});
    const reply = switch (called) {
        .ok => |bytes| bytes,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer allocator.free(reply);
    const ranking = try std.json.parseFromSlice(
        struct { value: []const struct { index: usize } },
        allocator,
        reply,
        .{ .ignore_unknown_fields = true },
    );
    defer ranking.deinit();
    const by_urgency = ranking.value.value;
    const expected = [_]usize{ 1, 3, 2, 0 };
    std.debug.assert(by_urgency.len == expected.len);
    for (by_urgency, expected) |one, want| {
        std.debug.assert(one.index == want);
    }
}
