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

    const question =
        "Which line gives the refund deadline?";
    const policy = [_][]const u8{
        "Returns need the original receipt.",
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
        "Gift cards cannot be exchanged for cash.",
    };
    var records: [4]tt.inputs.RequestItem = undefined;
    for (policy, &records) |text, *record| {
        record.* = .{
            .original = .{ .text = .{ .text = text } },
        };
    }
    var call = switch (try tt.calls.find(&engine, tt.calls.Arguments(tt.inputs.RequestInputUnits){
        .question = .{ .text = .{ .text = question } },
        .input = tt.inputs.RequestInputUnits{ .items = &records },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const found = call.value orelse return error.NotSure;
    const refund_deadline = found.string;
    std.debug.assert(
        std.mem.eql(u8, refund_deadline, policy[1]),
    );
}
