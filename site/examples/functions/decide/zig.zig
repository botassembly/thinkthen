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

    const question = "Does the customer ask for a refund?";
    const broken =
        "Please refund my order. It arrived broken.";
    var call = try tt.session.decide(&engine, .{
        .question = .{ .text = .{ .text = question } },
        .input = .{ .text = .{ .text = broken } },
    });
    defer call.deinit();

    const decide_row =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1;
    var is_refund = false;

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
        if (packet.view.kind != decide_row) continue;
        const row = packet.view.data.decide_row.*.value.*;
        const yes =
            tt.session.optional(row.value) orelse continue;
        is_refund = yes.*.value.*.data.boolean == 1;
    }
    std.debug.assert(is_refund);
}
