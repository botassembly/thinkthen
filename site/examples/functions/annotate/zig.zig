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

    const form = "form.json";
    const report = "Steps: click Log in. Nobody gets in.";
    var call = try tt.session.annotate(&engine, .{
        .question = .{ .file = .{ .path = form } },
        .input = .{ .text = .{ .text = report } },
    });
    defer call.deinit();

    const annotate_row =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1;
    var steps = false;
    var area: []const u8 = "";
    var impact: f64 = 0;

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
        if (packet.view.kind != annotate_row) continue;
        const row = packet.view.data.annotate_row.*.value.*;
        const triage = row.value.*.value;
        for (triage.data[0..triage.len]) |field| {
            const name = tt.session.bytes(field.name);
            const answer = field.value.*.data;
            if (std.mem.eql(u8, name, "steps")) {
                steps = answer.boolean == 1;
            } else if (std.mem.eql(u8, name, "area")) {
                const part =
                    tt.session.bytes(answer.string);
                area = try allocator.dupe(u8, part);
            } else if (std.mem.eql(u8, name, "impact")) {
                impact = answer.number;
            }
        }
    }
    std.debug.assert(steps);
    std.debug.assert(std.mem.eql(u8, area, "login"));
    std.debug.assert(impact == 1.98);
}
