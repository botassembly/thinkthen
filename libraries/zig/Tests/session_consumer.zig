const std = @import("std");
const tt = @import("thinkthen");
const request_fixture = @import("request_fixture.zig");
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
    // Fixture envelopes include base64 images and captions; native admission owns limits.
    const input = try std.fs.cwd().readFileAlloc(a, args[1], std.math.maxInt(usize));
    defer a.free(input);
    const fixture = try std.json.parseFromSlice(std.json.Value, a, input, .{});
    defer fixture.deinit();
    const doc = fixture.value.object;
    var settings_arena = std.heap.ArenaAllocator.init(a);
    defer settings_arena.deinit();
    const settings_json = try std.json.parseFromSlice(std.json.Value, a, args[2], .{});
    defer settings_json.deinit();
    const settings = try request_fixture.decode(tt.inputs.EngineSettings, settings_arena.allocator(), settings_json.value);
    var engine = switch (try tt.Engine.initWithSettings(a, settings)) {
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
    var request_arena = std.heap.ArenaAllocator.init(a);
    defer request_arena.deinit();
    const descriptor = try request_fixture.decode(tt.inputs.RequestCall, request_arena.allocator(), call);
    const request = try tt.Request.init(a, @tagName(descriptor.function), descriptor.question, descriptor.input, descriptor.options);
    defer request.deinit();
    if (!doc.get("cancel").?.bool and !doc.get("held_cancel").?.bool and descriptor.input != .feed) {
        inline for (std.meta.fields(tt.inputs.RequestFunction)) |function| {
            if (descriptor.function == @field(tt.inputs.RequestFunction, function.name)) {
                if (@field(tt.calls, function.name)(&engine, .{ .question = descriptor.question, .input = descriptor.input, .options = descriptor.options })) |result| {
                    var answer = result;
                    defer answer.deinit();
                    engine.deinit();
                    if (answer.failure() == null) {
                        var rows: usize = 0;
                        for (answer.packets) |packet| if (packet.value().* == .observation and packet.value().observation.value == .row) {
                            rows += 1;
                        };
                        if (comptime !std.mem.eql(u8, function.name, "rank") and !std.mem.eql(u8, function.name, "filter")) {
                            if (rows != 1) {
                                if (answer.value()) |_| return error.MultipleAnswersAccepted else |err| if (err != error.Cardinality) return err;
                            } else try compareAnswer(function.name, answer, a);
                        } else try compareAnswer(function.name, answer, a);
                    } else {
                        if (answer.value()) |_| return error.FailedAnswerAccepted else |err| if (err != error.CallFailed) return err;
                    }
                    try writePackets(answer.packets, a);
                    return;
                } else |err| {
                    try writeAdmission(err);
                    return;
                }
            }
        }
    }
    var session = request.start(&engine) catch |err| {
        try writeAdmission(err);
        return;
    };
    if (doc.get("cancel").?.bool) session.cancel();
    var packets: std.ArrayList(*tt.session.Packet) = .empty;
    defer {
        for (packets.items) |packet| packet.deinit();
        packets.deinit(a);
    }
    if (doc.get("feed_items")) |items| {
        var arena = std.heap.ArenaAllocator.init(a);
        defer arena.deinit();
        for (items.array.items) |item| {
            const next: tt.inputs.RequestSessionDescriptor = .{ .item = try request_fixture.decode(tt.inputs.RequestItem, arena.allocator(), item) };
            while (try session.push(a, next) == .full) switch (try session.read()) {
                .pending => try session.wait(true),
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
    while (true) switch (try session.next()) {
        .pending => return error.BlockingPending,
        .end => break,
        .packet => |packet| try packets.append(a, packet),
    };
    session.deinit();
    try writePackets(packets.items, a);
}

fn writePackets(packets: []const *tt.session.Packet, a: std.mem.Allocator) !void {
    var projection_arena = std.heap.ArenaAllocator.init(a);
    defer projection_arena.deinit();
    var out = std.fs.File.stdout().writer(&.{});
    try out.interface.writeAll("{\"packets\":[");
    for (packets, 0..) |packet, i| {
        if (i > 0) try out.interface.writeAll(",");
        var writer: std.json.Stringify = .{ .writer = &out.interface };
        try request_fixture.writeNative(try request_fixture.projectAnswers(packet, packets, projection_arena.allocator()), &writer);
    }
    try out.interface.writeAll("]}\n");
}

fn writeAdmission(err: anyerror) !void {
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
}

fn compareAnswer(comptime function: []const u8, answer: anytype, a: std.mem.Allocator) !void {
    const actual = try answer.value();
    if (comptime std.mem.eql(u8, function, "rank") or std.mem.eql(u8, function, "filter")) {
        const Element = @typeInfo(@TypeOf(actual)).pointer.child;
        var expected: std.ArrayList(Element) = .empty;
        defer expected.deinit(a);
        for (answer.packets) |packet| {
            const aggregate = if (comptime std.mem.eql(u8, function, "rank")) packet.value().* == .rank_aggregate else packet.value().* == .filter_aggregate;
            if (aggregate) {
                expected.clearRetainingCapacity();
                const view = packet.answers();
                try expected.appendSlice(a, try @field(@TypeOf(view), function)(view));
                break;
            }
            if (comptime std.mem.eql(u8, function, "filter")) if (packet.value().* == .filter_row) try expected.appendSlice(a, try packet.answers().filter());
        }
        try std.testing.expectEqualDeep(@as(@TypeOf(actual), expected.items), actual);
    } else {
        for (answer.packets) |packet| if (packet.value().* == .observation and packet.value().observation.value == .row) {
            const view = packet.answers();
            try std.testing.expectEqualDeep(try @field(@TypeOf(view), function)(view), actual);
            return;
        };
        return error.MissingAnswer;
    }
}
