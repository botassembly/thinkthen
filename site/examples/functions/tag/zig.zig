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

    const question = "Which labels fit this message?";
    const labels = [_][]const u8{
        "praise",
        "bug",
        "billing",
    };
    const message = "Love the new dashboard, " ++
        "but export crashes the app,\n" ++
        "and I was charged twice.\n";
    const which_labels: tt.inputs.RequestDefinition = .{
        .tag = .{
            .tag = .{ .string = question },
            .labels = .{ .array = &labels },
        },
    };
    var call = try tt.session.tag(&engine, .{
        .question = .{
            .definition = .{ .value = which_labels },
        },
        .input = .{ .text = .{ .text = message } },
    });
    defer call.deinit();

    const tag_row =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1;
    var fitting_labels: std.ArrayList([]const u8) = .empty;

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
        if (packet.view.kind != tag_row) continue;
        const found = packet.view.data.tag_row.*.value.*;
        const count = found.value.len;
        for (found.value.data[0..count]) |label| {
            const text = tt.session.bytes(label);
            const copy = try allocator.dupe(u8, text);
            try fitting_labels.append(allocator, copy);
        }
    }
    std.debug.assert(fitting_labels.items.len == 3);
    for (labels, fitting_labels.items) |label, fitting| {
        std.debug.assert(std.mem.eql(u8, label, fitting));
    }
}
