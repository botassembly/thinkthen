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

    const question = "Is this urgent?";
    const inbox = [_][]const u8{
        "Newsletter: our autumn catalog is here. " ++
            "No reply needed.",
        "Our checkout page is down and " ++
            "customers cannot pay",
        "Reminder: your invoice is due in 30 days",
        "Please send the signed quote by 5 pm today",
    };
    var records: [4]tt.inputs.RequestItem = undefined;
    for (inbox, &records) |text, *record| {
        record.* = .{
            .original = .{ .text = .{ .text = text } },
        };
    }
    var call = try tt.session.rank(&engine, .{
        .question = .{ .text = .{ .text = question } },
        .input = .{ .records = .{ .items = &records } },
    });
    defer call.deinit();

    const observation =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1;
    var by_urgency = [_]u64{0} ** inbox.len;

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
        const ranked = packet.view.data.rank_aggregate.*;
        const count = ranked.value.len;
        for (ranked.value.data[0..count]) |one| {
            by_urgency[one.*.value - 1] = one.*.index.value;
        }
    }
    const expected = [_]u64{ 1, 3, 2, 0 };
    std.debug.assert(
        std.mem.eql(u64, &by_urgency, &expected),
    );
}
