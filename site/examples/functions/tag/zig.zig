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

    const labels = [_][]const u8{
        "praise",
        "bug",
        "billing",
    };
    const tag = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .tag = "Which labels fit this message?",
            .labels = labels,
            .evidence = "Love the new dashboard, " ++
                "but export crashes the app,\n" ++
                "and I was charged twice.\n",
        }, .{})},
        0,
    );
    defer allocator.free(tag);
    const called = try tt.call(tag, .{});
    const reply = switch (called) {
        .ok => |bytes| bytes,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer allocator.free(reply);
    const tagged = try std.json.parseFromSlice(
        struct { value: []const []const u8 },
        allocator,
        reply,
        .{ .ignore_unknown_fields = true },
    );
    defer tagged.deinit();
    const fitting_labels = tagged.value.value;
    std.debug.assert(fitting_labels.len == labels.len);
    for (fitting_labels, labels) |label, want| {
        std.debug.assert(std.mem.eql(u8, label, want));
    }
}
