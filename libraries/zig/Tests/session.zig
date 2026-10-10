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
            tt.releaseFailure(a, failure);
            return error.QuestionBuild;
        },
    };
    defer question.deinit();
    const author = try question.author();
    if (!std.mem.eql(u8, tt.session.bytes(author.name.value), "pass")) return error.WrongAuthor;
    switch (try tt.Question.init(&engine, .atomic, .{ .decide = .{ .decide = .{ .string = "Does it pass?" }, .threshold = .{ .number = 1.5 } } })) {
        .ok => |unexpected| {
            unexpected.deinit();
            return error.InvalidThresholdAccepted;
        },
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            if (failure.kind != .usage) return error.WrongAdmissionFailure;
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
    const invalid = tt.Request{ .allocator = a, .json = "{" };
    if (invalid.start(&engine)) |unexpected| {
        var owned = unexpected;
        owned.deinit();
        return error.InvalidRequestAccepted;
    } else |err| if (err != error.Usage) return err;
    const failure_case = args.len > 1 and std.mem.eql(u8, args[1], "failure");
    const usage_case = args.len > 1 and std.mem.startsWith(u8, args[1], "usage-");
    if (args.len > 1 and !failure_case and !usage_case) {
        engine.deinit();
        if (plan.value.first_body_utf8 == null) return error.MissingPlanBody;
        std.debug.print("installed Zig plan PASS\n", .{});
        return;
    }
    var session = try tt.session.decide(&engine, .{ .question = .{ .text = .{ .text = "Does it pass?" } }, .input = .{ .feed = .{ .name = "owned" } } });
    if (try session.push(a, .{ .item = .{ .original = .{ .text = .{ .text = if (failure_case) "status-401" else if (usage_case) "consumer-zig" else "Evidence." } } }, .location = .{ .file = "owned.txt", .first_line = 1, .last_line = 1 } }) != .accepted) return error.FeedRefused;
    try session.finish();
    var retained_row: ?tt.session.Packet = null;
    defer if (retained_row) |*packet| packet.deinit();
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
                } else if (packet.view.kind == tt.c.THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1) {
                    retained_row = packet;
                } else packet.deinit();
            },
        }
    }
    session.deinit();
    var status: ?tt.UsageStatus = null;
    if (usage_case) {
        var observed = try engine.usagePersistence();
        defer observed.deinit(a);
        status = try engine.finishUsageStatus();
    }
    engine.deinit();
    if (invalid.start(&engine)) |unexpected| {
        var owned = unexpected;
        owned.deinit();
        return error.ClosedStartAccepted;
    } else |err| if (err != error.ClosedEngine) return err;
    if (invalid.plan(&engine)) |unexpected| {
        unexpected.deinit();
        return error.ClosedPlanAccepted;
    } else |err| if (err != error.ClosedEngine) return err;
    defer if (status) |owned| owned.deinit(a);
    if (status) |owned| {
        const wanted: tt.UsagePersistence = if (std.mem.eql(u8, args[1], "usage-written")) .written else if (std.mem.eql(u8, args[1], "usage-failed")) .failed else .disabled;
        if (owned.state != wanted) return error.WrongPersistence;
        if ((owned.advice != null) != (wanted == .failed)) return error.WrongPersistenceAdvice;
        if (engine.finishUsageStatus()) |unexpected| {
            unexpected.deinit(a);
            return error.ClosedUsageAccepted;
        } else |err| if (err != error.ClosedEngine) return err;
    }
    const packet = retained orelse return error.MissingTerminal;
    const facts = packet.facts() orelse return error.MissingFacts;
    const retained_author = try question.author();
    if (!std.mem.eql(u8, tt.session.bytes(retained_author.name.value), "pass")) return error.QuestionLifetime;
    if (facts.requests_sent != 1) return error.WrongFacts;
    if (!terminal or packets == 0 or plan.value.first_body_utf8 == null) return error.MissingTerminal;
    if (failure_case) {
        const failure = packet.failure() orelse return error.MissingFailure;
        const failure_facts = tt.session.optional(failure.facts) orelse return error.MissingFailureFacts;
        if (failure_facts[0].requests_sent != facts.requests_sent) return error.WrongFailureFacts;
        if (failure.@"error"[0].kind[0].kind != tt.c.THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1) return error.WrongFailureKind;
        if (tt.session.bytes(failure.@"error"[0].message).len == 0) return error.MissingFailureMessage;
        std.debug.print("installed Zig failure PASS\n", .{});
        return;
    }
    const row_packet = retained_row orelse return error.MissingRow;
    const row = row_packet.view.data.decide_row[0].value[0];
    if (row.source.presence != tt.c.THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) return error.MissingSource;
    if (!std.mem.eql(u8, tt.session.bytes(row.source.value[0].file), "owned.txt")) return error.SourceLifetime;
    if (row.meta[0].observations.len == 0 or row.meta[0].question_sources.len == 0) return error.MissingMetadata;
    if (row.value.presence != tt.c.THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 or row.value.value[0].value[0].kind != tt.c.THINKTHEN_COMPLETE_JSON_BOOLEAN_V1 or row.value.value[0].value[0].data.boolean != 1) return error.WrongAnswer;
    if (usage_case) {
        var out = std.fs.File.stdout().writer(&.{});
        try out.interface.writeAll("USAGE_PASS\n");
        return;
    }
    if (packet.failure() != null) return error.NativeFailure;
    if (facts.records != 1) return error.WrongRecords;
    std.debug.print("installed Zig session PASS\n", .{});
}
