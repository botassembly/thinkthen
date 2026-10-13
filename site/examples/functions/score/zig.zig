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

    const question = "How urgent is this?";
    const levels = [_][]const u8{
        "Routine.",
        "Soon.",
        "Immediate.",
    };
    const outage = "Our checkout page is down and " ++
        "customers cannot pay.\n";
    const how_urgent: tt.inputs.RequestDefinition = .{
        .Authored_score = .{
            .score = .{ .string = question },
            .levels = .{ .array = &levels },
        },
    };
    var call = try tt.session.score(&engine, .{
        .question = .{
            .definition = .{ .value = how_urgent },
        },
        .input = .{ .text = .{ .text = outage } },
    });
    defer call.deinit();

    const score_row =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1;
    var urgency: f64 = -1;

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
        if (packet.view.kind != score_row) continue;
        const row = packet.view.data.score_row.*.value.*;
        urgency = row.value;
    }
    std.debug.assert(urgency == 2.0);
}
