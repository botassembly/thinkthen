const std = @import("std");
const tt = @import("thinkthen");
comptime {
    if (@hasDecl(tt, "c") or @hasDecl(tt.results, "readSessionPacket")) @compileError("native readers must stay private");
    const capture_engine = @typeInfo(@TypeOf(tt.capture)).@"fn".params[1].type.?;
    if (capture_engine != ?*tt.Engine) @compileError("capture must accept the Zig engine: " ++ @typeName(capture_engine));
    for (std.meta.declarations(tt.inputs)) |declaration| {
        const value = @field(tt.inputs, declaration.name);
        if (@TypeOf(value) == type) _ = @sizeOf(value);
    }
}
pub fn main() !void {
    var allocator = std.heap.DebugAllocator(.{}){};
    defer std.debug.assert(allocator.deinit() == .ok);
    const a = allocator.allocator();
    var engine = switch (try tt.Engine.initWithSettings(a, .{ .cache = .{ .disabled_cache = false }, .max_retries = 0 })) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            return error.EngineBuild;
        },
    };
    var question = switch (try tt.Question.init(&engine, .atomic, .{ .Authored_decide = .{ .decide = .{ .string = "Does it pass?" }, .name = "pass", .threshold = .{ .number = 0.7 }, .wording_version = 2 } })) {
        .ok => |value| value,
        .failed => |failure| {
            tt.releaseFailure(a, failure);
            return error.QuestionBuild;
        },
    };
    defer question.deinit();
    const author = (try question.name()).?;
    if (!std.mem.eql(u8, author, "pass")) return error.WrongAuthor;
    switch (try tt.Question.init(&engine, .atomic, .{ .Authored_decide = .{ .decide = .{ .string = "Does it pass?" }, .threshold = .{ .number = 1.5 } } })) {
        .ok => |unexpected| {
            unexpected.deinit();
            return error.InvalidThresholdAccepted;
        },
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            if (failure.kind != .usage) return error.WrongAdmissionFailure;
        },
    }
    const request = try tt.Request.init(a, .{ .function = .decide, .question = .{ .text = .{ .text = "Does it pass?" } }, .input = .{ .text = .{ .text = "Evidence." } }, .options = .{ .details = false } });
    var plan = try request.plan(&engine);
    request.deinit();
    defer plan.deinit();
    if (plan.value.records != 1 or plan.value.requests != 1 or plan.value.largest_request_bytes == 0 or plan.value.estimated_input_tokens.upper == 0) return error.WrongPlan;
    const args = try std.process.argsAlloc(a);
    defer std.process.argsFree(a, args);
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
    var retained_row: ?*tt.session.Packet = null;
    defer if (retained_row) |packet| packet.deinit();
    var retained: ?*tt.session.Packet = null;
    defer if (retained) |packet| packet.deinit();
    var terminal = false;
    var packets: usize = 0;
    while (true) {
        switch (try session.next()) {
            .pending => return error.BlockingPending,
            .end => break,
            .packet => |value| {
                var packet = value;
                packets += 1;
                if (packet.terminal()) |_| {
                    terminal = true;
                    retained = packet;
                } else if (packet.value().* == .decide_row) {
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
    const closed_request = try tt.Request.init(a, .{ .function = .decide, .question = .{ .text = .{ .text = "Does it pass?" } }, .input = .{ .text = .{ .text = "Evidence." } } });
    defer closed_request.deinit();
    engine.deinit();
    if (closed_request.start(&engine)) |unexpected| {
        var owned = unexpected;
        owned.deinit();
        return error.ClosedStartAccepted;
    } else |err| if (err != error.ClosedEngine) return err;
    if (closed_request.plan(&engine)) |unexpected| {
        var owned = unexpected;
        owned.deinit();
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
    const retained_author = (try question.name()).?;
    if (!std.mem.eql(u8, retained_author, "pass")) return error.QuestionLifetime;
    if (facts.requests_sent != 1) return error.WrongFacts;
    if (!terminal or packets == 0 or plan.value.first_body_utf8 == null) return error.MissingTerminal;
    if (failure_case) {
        const failure = packet.failure() orelse return error.MissingFailure;
        const failure_facts = switch (failure.facts) {
            .value => |*v| v,
            else => null,
        } orelse return error.MissingFailureFacts;
        if (failure_facts.requests_sent != facts.requests_sent) return error.WrongFailureFacts;
        if (failure.@"error".kind != .backend) return error.WrongFailureKind;
        if (failure.@"error".message.len == 0) return error.MissingFailureMessage;
        std.debug.print("installed Zig failure PASS\n", .{});
        return;
    }
    const row_packet = retained_row orelse return error.MissingRow;
    const row = row_packet.value().decide_row.value;
    const source = switch (row.source) {
        .value => |v| v,
        else => return error.MissingSource,
    };
    if (!std.mem.eql(u8, source.file, "owned.txt")) return error.SourceLifetime;
    if (row.meta.observations.len == 0 or row.meta.question_sources.len == 0) return error.MissingMetadata;
    const answer = switch (row.value) {
        .value => |v| v,
        else => return error.WrongAnswer,
    };
    if (answer != .boolean or !answer.boolean) return error.WrongAnswer;
    if (usage_case) {
        var out = std.fs.File.stdout().writer(&.{});
        try out.interface.writeAll("USAGE_PASS\n");
        return;
    }
    if (packet.failure() != null) return error.NativeFailure;
    if (facts.records != 1) return error.WrongRecords;
    std.debug.print("installed Zig session PASS\n", .{});
}
