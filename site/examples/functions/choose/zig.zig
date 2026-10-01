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

    const teams = .{
        .billing = "Invoices, fees, and refunds.",
        .shipping = "Parcels and delivery.",
        .account = "Logins and passwords.",
    };
    const text = "My parcel went to the wrong address.";
    const choose = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .choose = "Which team owns this?",
            .options = teams,
            .evidence = text,
        }, .{})},
        0,
    );
    defer allocator.free(choose);
    const called = try tt.call(choose, .{});
    const reply = switch (called) {
        .ok => |bytes| bytes,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer allocator.free(reply);
    const parsed = try std.json.parseFromSlice(
        struct { value: ?[]const u8 },
        allocator,
        reply,
        .{ .ignore_unknown_fields = true },
    );
    defer parsed.deinit();
    const team = parsed.value.value orelse "";
    std.debug.assert(std.mem.eql(u8, team, "shipping"));
}
