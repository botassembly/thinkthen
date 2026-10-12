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

    const question = "Is this a complaint?";
    const reviews = [_][]const u8{
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
        "Does this come in blue?",
        "The strap snapped on day two.",
    };
    var records: [4]tt.inputs.RequestItem = undefined;
    for (reviews, &records) |text, *record| {
        record.* = .{
            .original = .{ .text = .{ .text = text } },
        };
    }
    var call = try tt.session.filter(&engine, .{
        .question = .{ .text = .{ .text = question } },
        .input = .{ .records = .{ .items = &records } },
    });
    defer call.deinit();

    const filter_row =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1;
    var is_complaint = [_]bool{false} ** reviews.len;

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
        if (packet.view.kind != filter_row) continue;
        const row = packet.view.data.filter_row.*.value.*;
        is_complaint[row.index.value] = row.value == 1;
    }
    const expected = [_]bool{ false, true, false, true };
    std.debug.assert(
        std.mem.eql(bool, &is_complaint, &expected),
    );
}
