const std = @import("std");
const tt = @import("thinkthen");
const c = tt.c;

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
    var call = try tt.session.find(&engine, .{
        .question = .{ .text = .{ .text = question } },
        .input = .{ .records = .{ .items = &records } },
    });
    defer call.deinit();

    const observation =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1;
    var refund_deadline: []const u8 = "";

    while (true) {
        var packet = switch (try call.read()) {
            .pending => {
                var ready: u32 = undefined;
                _ = c.thinkthen_session_wait(
                    call.raw,
                    c.THINKTHEN_SESSION_WAIT_RESULT_V1,
                    50,
                    &ready,
                );
                continue;
            },
            .end => break,
            .packet => |value| value,
        };
        defer packet.deinit();
        if (packet.terminal() != null) continue;
        if (packet.view.kind == observation) continue;
        const found = packet.view.data.find_aggregate.*;
        const answer = found.value.*.value;
        const line =
            tt.session.optional(answer) orelse continue;
        const text = tt.session.bytes(line.*.data.string);
        refund_deadline = try allocator.dupe(u8, text);
    }
    std.debug.assert(
        std.mem.eql(u8, refund_deadline, policy[1]),
    );
}
