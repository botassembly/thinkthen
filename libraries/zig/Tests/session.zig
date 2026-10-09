const std = @import("std");
const tt = @import("thinkthen");
comptime {
    for (std.meta.declarations(tt.inputs)) |declaration| {
        const value = @field(tt.inputs, declaration.name);
        if (@TypeOf(value) == type) _ = @sizeOf(value);
    }
}
pub fn main() !void {
    var allocator = std.heap.DebugAllocator(.{}){};
    defer std.debug.assert(allocator.deinit() == .ok);
    const a = allocator.allocator();
    var engine = switch (try tt.Engine.initWithSettings(a, "{\"cache\":false,\"max_retries\":0}")) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            return error.EngineBuild;
        },
    };
    var question = switch (try tt.Question.init(&engine, .atomic, .{ .decide = .{ .decide = .{ .string = "Does it pass?" }, .name = "pass", .threshold = .{ .number = 0.7 }, .wording_version = 2 } })) {
        .ok => |value| value,
        .failed => |failure| {
            var owned = failure;
            owned.deinit();
            return error.QuestionBuild;
        },
    };
    defer question.deinit();
    const author = try question.author();
    if (!std.mem.eql(u8, try tt.native.bytes(author.name.value), "pass")) return error.WrongAuthor;
    switch (try tt.Question.init(&engine, .atomic, .{ .decide = .{ .decide = .{ .string = "Does it pass?" }, .threshold = .{ .number = 1.5 } } })) {
        .ok => |unexpected| {
            unexpected.deinit();
            return error.InvalidThresholdAccepted;
        },
        .failed => |failure| {
            var owned = failure;
            defer owned.deinit();
            if (try owned.kind() != .usage) return error.WrongAdmissionFailure;
        },
    }
    const request = try tt.Request.init(a, .{ .decide = .{ .question = .{ .text = .{ .text = "Does it pass?" } }, .input = .{ .text = .{ .text = "Evidence." } }, .options = .{ .details = false } } });
    if (std.mem.indexOf(u8, request.json, "null") != null) return error.OptionalNotOmitted;
    if (std.mem.indexOf(u8, request.json, "\"details\":false") == null) return error.FalseNotRetained;
    var plan = try request.plan(&engine);
    request.deinit();
    defer plan.deinit();
    if (plan.value.records != 1 or plan.value.requests != 1 or plan.value.largest_request_bytes == 0 or plan.value.estimated_input_tokens.upper == 0) return error.WrongPlan;
    const args = try std.process.argsAlloc(a);
    defer std.process.argsFree(a, args);
    if (args.len > 1) {
        engine.deinit();
        if (plan.value.first_body_utf8 == null) return error.MissingPlanBody;
        std.debug.print("installed Zig plan PASS\n", .{});
        return;
    }
    var session = try tt.session.decide(&engine, .{ .question = .{ .text = .{ .text = "Does it pass?" } }, .input = .{ .text = .{ .text = "Evidence." } } });
    engine.deinit();
    var retained: ?tt.session.Packet = null;
    defer if (retained) |*packet| packet.deinit();
    var terminal = false;
    var packets: usize = 0;
    while (true) {
        switch (try session.read()) {
            .pending => std.Thread.yield() catch {},
            .end => break,
            .packet => |value| {
                var packet = value;
                packets += 1;
                if (packet.terminal()) |_| {
                    terminal = true;
                    retained = packet;
                } else packet.deinit();
            },
        }
    }
    session.deinit();
    const packet = retained orelse return error.MissingTerminal;
    if (packet.failure() != null) return error.NativeFailure;
    const facts = packet.facts() orelse return error.MissingFacts;
    const retained_author = try question.author();
    if (!std.mem.eql(u8, try tt.native.bytes(retained_author.name.value), "pass")) return error.QuestionLifetime;
    if (facts.records != 1 or facts.requests_sent != 1) return error.WrongFacts;
    if (!terminal or packets == 0 or plan.value.first_body_utf8 == null) return error.MissingTerminal;
    std.debug.print("installed Zig session PASS\n", .{});
}
