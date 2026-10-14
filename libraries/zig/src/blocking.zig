//! One owned answer and its native observations, receipts and terminal facts.
const std = @import("std");
const tt = @import("thinkthen.zig");

pub fn Answer(comptime T: type) type {
    return struct {
        arena: std.heap.ArenaAllocator,
        packets: []const *tt.session.Packet,
        outcome: anyerror!T,
        pub fn value(self: @This()) !T {
            return self.outcome;
        }
        pub fn facts(self: @This()) ?*const tt.results.Facts {
            for (self.packets) |packet| if (packet.facts()) |v| return v;
            return null;
        }
        pub fn failure(self: @This()) ?*const tt.results.CallError {
            for (self.packets) |packet| if (packet.failure()) |v| return v;
            return null;
        }
        pub fn deinit(self: *@This()) void {
            for (self.packets) |packet| packet.deinit();
            self.arena.deinit();
        }
    };
}

/// Rust admits the named call and waits for packets; Zig only retains ownership.
pub fn completeNamed(engine: *tt.Engine, comptime function: tt.inputs.RequestFunction, arguments: tt.calls.Arguments, comptime T: type) !Answer(T) {
    var session = try tt.session.startNamed(engine, function, arguments);
    defer session.deinit();
    try session.finish();
    var arena = std.heap.ArenaAllocator.init(engine.allocator);
    errdefer arena.deinit();
    const a = arena.allocator();
    var packets: std.ArrayList(*tt.session.Packet) = .empty;
    errdefer for (packets.items) |packet| packet.deinit();
    while (true) switch (try session.next()) {
        .pending => return error.Defect,
        .end => break,
        .packet => |packet| {
            packets.append(a, packet) catch |err| {
                packet.deinit();
                return err;
            };
        },
    };
    var failed = false;
    for (packets.items) |packet| if (packet.failure() != null) {
        failed = true;
    };
    const outcome: anyerror!T = if (failed) error.CallFailed else project(function, T, packets.items, a);
    return .{ .arena = arena, .packets = packets.items, .outcome = outcome };
}

fn project(comptime function: tt.inputs.RequestFunction, comptime T: type, packets: []const *tt.session.Packet, a: std.mem.Allocator) !T {
    if (comptime function == .rank or function == .filter) {
        const Element = @typeInfo(T).pointer.child;
        var items: std.ArrayList(Element) = .empty;
        var aggregate = false;
        for (packets) |packet| {
            const matches = if (comptime function == .rank) packet.value().* == .rank_aggregate else packet.value().* == .filter_aggregate;
            if (matches) {
                if (aggregate) return error.Cardinality;
                aggregate = true;
                items.clearRetainingCapacity();
                try items.appendSlice(a, try @field(@TypeOf(packet.answers()), @tagName(function))(packet.answers()));
            } else if (comptime function == .filter) {
                if (!aggregate and packet.value().* == .filter_row) try items.appendSlice(a, try packet.answers().filter());
            }
        }
        if (comptime function == .rank) if (!aggregate) return error.MissingAnswer;
        return items.items;
    }
    var count: usize = 0;
    var value: T = undefined;
    for (packets) |packet| {
        if (packet.value().* != .observation or packet.value().observation.value != .row) continue;
        const view = packet.answers();
        value = try @field(@TypeOf(view), @tagName(function))(view);
        count += 1;
    }
    if (count != 1) return error.Cardinality;
    return value;
}
