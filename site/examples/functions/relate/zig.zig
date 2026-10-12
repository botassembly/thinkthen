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

    const relations = "sings.json";
    const names = [_][2][]const u8{
        .{ "Paul McCartney", "singer" },
        .{ "Ringo Starr", "singer" },
        .{ "Yesterday", "song" },
        .{ "Octopus's Garden", "song" },
    };
    var entities: [4]tt.inputs.RequestItem = undefined;
    for (names, &entities) |name, *entity| {
        var fields = std.json.ObjectMap.init(allocator);
        try fields.put("name", .{ .string = name[0] });
        try fields.put("kind", .{ .string = name[1] });
        entity.* = .{ .original = .{ .json = .{
            .value = .{ .object = fields },
        } } };
    }
    var call = try tt.session.relate(&engine, .{
        .question = .{ .file = .{ .path = relations } },
        .input = .{ .entities = .{ .items = &entities } },
    });
    defer call.deinit();

    const observation =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1;
    var who_sings: std.ArrayList(u8) = .empty;

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
        const found = packet.view.data.relate_aggregate.*;
        const edges = found.value.*.value;
        for (edges.data[0..edges.len]) |edge| {
            const source = edge.*.source.*.data;
            const target = edge.*.target.*.data;
            const singer = source.fields_kind_name.*.name;
            const song = target.fields_kind_name.*.name;
            try who_sings.print(
                allocator,
                "{s} sings {s}. ",
                .{
                    tt.session.bytes(singer),
                    tt.session.bytes(song),
                },
            );
        }
    }
    std.debug.assert(std.mem.eql(u8, who_sings.items,
        "Paul McCartney sings Yesterday. " ++
        "Ringo Starr sings Octopus's Garden. "));
}
