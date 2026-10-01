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

    const reviews = [_][]const u8{
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
        "Does this come in blue?",
        "The strap snapped on day two.",
    };
    const filter = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .filter = "Is this a complaint?",
            .records = reviews,
        }, .{})},
        0,
    );
    defer allocator.free(filter);
    const called = try tt.call(filter, .{});
    const reply = switch (called) {
        .ok => |bytes| bytes,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer allocator.free(reply);
    const filtered = try std.json.parseFromSlice(
        struct { value: []const []const u8 },
        allocator,
        reply,
        .{ .ignore_unknown_fields = true },
    );
    defer filtered.deinit();
    const complaints = filtered.value.value;
    const expected = [_][]const u8{
        reviews[1],
        reviews[3],
    };
    std.debug.assert(complaints.len == expected.len);
    for (complaints, expected) |complaint, want| {
        std.debug.assert(std.mem.eql(u8, complaint, want));
    }
}
