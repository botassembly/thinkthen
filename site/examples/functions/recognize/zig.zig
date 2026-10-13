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

    const kinds = "kinds.json";
    const text = "Sam Patel says his Kindle " ++
        "arrived broken in Denver.";
    var call = try tt.session.recognize(&engine, .{
        .question = .{ .file = .{ .path = kinds } },
        .input = .{ .text = .{ .text = text } },
    });
    defer call.deinit();

    const observation =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1;
    var names: std.ArrayList(u8) = .empty;

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
        const found =
            packet.view.data.recognize_aggregate.*;
        if (found.value.len == 0) continue;
        const rows = found.value.data[0..found.value.len];
        for (rows) |row| {
            const recognized = row.*.value.*;
            const facts = recognized.data.entities.*;
            const entities = facts.entities;
            for (entities.data[0..entities.len]) |one| {
                try names.print(allocator, "{s} ({s}). ", .{
                    tt.session.bytes(one.*.text),
                    tt.session.bytes(one.*.kind),
                });
            }
        }
    }
    std.debug.assert(std.mem.eql(u8, names.items,
        "Sam Patel (person). " ++
        "Kindle (product). " ++
        "Denver (place). "));
}
