const std = @import("std");
const tt = @import("thinkthen");
const request_fixture = @import("request_fixture.zig");
const view_check = @import("view_check.zig");
const io = @cImport({
    @cInclude("stdio.h");
});
pub fn main() !void {
    var gpa = std.heap.DebugAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);
    const a = gpa.allocator();
    const args = try std.process.argsAlloc(a);
    defer std.process.argsFree(a, args);
    if (args.len != 3) return error.Arguments;
    const input = try std.fs.cwd().readFileAlloc(a, args[1], 32 * 1024 * 1024);
    defer a.free(input);
    const fixture = try std.json.parseFromSlice(std.json.Value, a, input, .{});
    defer fixture.deinit();
    const doc = fixture.value.object;
    var engine = switch (try tt.Engine.initWithSettings(a, args[2])) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            var out = std.fs.File.stdout().writer(&.{});
            try out.interface.print("{{\"admission\":{{\"code\":{d},\"message\":", .{failure.code});
            try std.json.Stringify.value(failure.message, .{}, &out.interface);
            try out.interface.writeAll("}}\n");
            return;
        },
    };
    defer engine.deinit();
    // Parsing the generated request descriptor is fixture glue; admission stays native.
    var call: std.json.Value = .{ .object = std.json.ObjectMap.init(a) };
    defer call.object.deinit();
    try call.object.put("function", doc.get("verb").?);
    inline for (.{ "question", "input", "options" }) |key| try call.object.put(key, doc.get(key).?);
    const verb = doc.get("verb").?.string;
    var session: tt.session.Session = blk: {
        inline for (std.meta.fields(tt.inputs.RequestCall)) |arm| {
            if (std.mem.eql(u8, verb, arm.name)) {
                var arena = std.heap.ArenaAllocator.init(a);
                defer arena.deinit();
                const descriptor = try request_fixture.decode(arm.type, arena.allocator(), call);
                break :blk @field(tt.session, arm.name)(&engine, descriptor) catch |err| {
                    var out = std.fs.File.stdout().writer(&.{});
                    const code: u32 = switch (err) {
                        error.Usage => 1,
                        error.Backend => 2,
                        error.Deadline => 3,
                        error.Local => 4,
                        error.Cancelled => 5,
                        else => 6,
                    };
                    try out.interface.print("{{\"admission\":{{\"code\":{d},\"message\":", .{code});
                    try std.json.Stringify.value(tt.session.message(), .{}, &out.interface);
                    try out.interface.writeAll("}}\n");
                    return;
                };
            }
        }
        return error.UnknownFunction;
    };
    if (doc.get("cancel").?.bool) session.cancel();
    var packets: std.ArrayList(tt.session.Packet) = .empty;
    defer {
        for (packets.items) |*packet| packet.deinit();
        packets.deinit(a);
    }
    if (doc.get("feed_items")) |items| {
        var arena = std.heap.ArenaAllocator.init(a);
        defer arena.deinit();
        for (items.array.items) |item| {
            const next: tt.inputs.RequestSessionDescriptor = .{ .item = try request_fixture.decode(tt.inputs.RequestItem, arena.allocator(), item) };
            while (try session.push(a, next) == .full) switch (try session.read()) {
                .pending => std.Thread.yield() catch {},
                .end => break,
                .packet => |packet| try packets.append(a, packet),
            };
        }
    }
    try session.finish();
    if (doc.get("held_cancel").?.bool) {
        if (io.getchar() != '!') return error.CancelHandshake;
        session.cancel();
        _ = io.puts("cancel-fired");
        _ = io.fflush(io.stdout);
    }
    engine.deinit();
    while (true) switch (try session.read()) {
        .pending => std.Thread.yield() catch {},
        .end => break,
        .packet => |packet| try packets.append(a, packet),
    };
    session.deinit();
    var out = std.fs.File.stdout().writer(&.{});
    try out.interface.writeAll("{\"packets\":[");
    for (packets.items, 0..) |packet, i| {
        var raw: [*c]const u8 = null;
        var len: usize = 0;
        try tt.session.checked(tt.c.thinkthen_session_result_json(packet.raw, &raw, &len));
        const expected = try std.json.parseFromSlice(std.json.Value, a, raw[0..len], .{});
        defer expected.deinit();
        // Every known field is compared through the typed graph after both owners close.
        try view_check.compare(@TypeOf(packet.view), packet.view, expected.value);
        if (i > 0) try out.interface.writeAll(",");
        try out.interface.writeAll(raw[0..len]);
    }
    try out.interface.writeAll("]}\n");
}
