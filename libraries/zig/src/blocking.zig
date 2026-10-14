//! One owned call result and its native observations, receipts and terminal facts.
const std = @import("std");
const tt = @import("thinkthen.zig");
const values = @import("results_generated.zig");

pub fn CallResult(comptime V: type, comptime P: type, comptime R: type) type {
    return struct {
        value: V,
        probability: P,
        results: []const R,
        facts: ?*const tt.results.Facts,
        arena: std.heap.ArenaAllocator,
        packets: []const *tt.session.Packet,
        pub fn deinit(self: *@This()) void {
            for (self.packets) |packet| packet.deinit();
            self.arena.deinit();
        }
    };
}

/// Native admission and event waits retain the existing owned failure carrier.
pub fn completeNamed(engine: *tt.Engine, comptime function: tt.inputs.RequestFunction, arguments: anytype) !tt.Result(values.NamedCallResult(function, @TypeOf(arguments.input))) {
    const Input = @TypeOf(arguments.input);
    const arguments_typed: tt.calls.Arguments(Input) = arguments;
    var session = tt.session.startNamed(engine, function, .{ .question = arguments_typed.question, .input = tt.calls.nativeInput(arguments_typed.input), .options = arguments_typed.options }) catch |err| {
        return .{ .failed = try tt.captureSession(engine.allocator, err) };
    };
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
    var facts: ?*const tt.results.Facts = null;
    for (packets.items) |packet| {
        if (packet.facts()) |v| facts = v;
        if (packet.failure()) |failure| {
            var immediate: ?tt.results.ImmediateFacts = null;
            if (failure.facts.optional()) |v| {
                var copied: tt.results.ImmediateFacts = undefined;
                inline for (std.meta.fields(tt.results.ImmediateFacts)) |field| {
                    @field(copied, field.name) = if (comptime std.mem.eql(u8, field.name, "call_id")) .{ .value = v.call_id } else @field(v, field.name);
                }
                immediate = copied;
            }
            const kind = std.meta.stringToEnum(tt.FailureKind, @tagName(failure.@"error".kind)) orelse return error.Defect;
            return .{ .failed = .{ .kind = kind, .code = @intFromEnum(kind), .retryable = failure.@"error".retryable, .message = try engine.allocator.dupe(u8, failure.@"error".message), .facts = immediate, .arena = arena, .packets = packets.items, .terminal = failure } };
        }
    }
    const R = values.callReceiptType(function);
    var receipts: std.ArrayList(R) = .empty;
    const native = try a.alloc(*const values.SessionPacket, packets.items.len);
    var aggregate = false;
    for (packets.items, native) |packet, *out| {
        out.* = packet.value();
        if (try values.callReceipts(function, packet.value(), a)) |rows| {
            if (aggregate) return error.Cardinality;
            aggregate = true;
            receipts.clearRetainingCapacity();
            try receipts.appendSlice(a, rows);
        } else if (!aggregate) {
            if (try values.callRowReceipts(function, packet.value(), a)) |rows| try receipts.appendSlice(a, rows);
        }
    }
    const V = values.callPublicValueType(function);
    const P = values.callProbabilityType(function);
    const list = comptime values.callList(function, Input);
    if (!list and receipts.items.len != 1) return error.Cardinality;
    const result_values = try a.alloc(V, receipts.items.len);
    const probabilities = try a.alloc(P, receipts.items.len);
    for (receipts.items, result_values, probabilities, 0..) |receipt, *value, *probability, i| {
        value.* = try values.callPublicValue(function, receipt, i, native);
        probability.* = try values.callProbability(function, receipt, i, native, a);
    }
    return .{ .ok = .{ .value = if (list) result_values else result_values[0], .probability = if (list) probabilities else probabilities[0], .results = receipts.items, .facts = facts, .arena = arena, .packets = packets.items } };
}
