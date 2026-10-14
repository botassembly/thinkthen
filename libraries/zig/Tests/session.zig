const std = @import("std");
const tt = @import("thinkthen");
fn publicTypes(comptime T: type, comptime seen: *[4096]type, comptime count: *usize) void {
    for (seen[0..count.*]) |visited| if (T == visited) return;
    seen[count.*] = T;
    count.* += 1;
    if (std.mem.indexOfPosLinear(u8, @typeName(T), 0, "cimport") != null) @compileError("public native type: " ++ @typeName(T));
    if (T == std.json.Value or T == std.mem.Allocator or T == std.heap.ArenaAllocator) return;
    switch (@typeInfo(T)) {
        .pointer => |v| publicTypes(v.child, seen, count),
        .optional => |v| publicTypes(v.child, seen, count),
        .error_union => |v| publicTypes(v.payload, seen, count),
        .@"fn" => |v| {
            for (v.params) |param| if (param.type) |child| publicTypes(child, seen, count);
            if (v.return_type) |child| publicTypes(child, seen, count);
        },
        .@"struct", .@"union", .@"enum", .@"opaque" => {
            if (std.mem.startsWith(u8, @typeName(T), "std.")) return;
            inline for (std.meta.declarations(T)) |decl| {
                const member = @field(T, decl.name);
                publicTypes(if (@TypeOf(member) == type) member else @TypeOf(member), seen, count);
            }
            if (@typeInfo(T) == .@"struct" or @typeInfo(T) == .@"union") inline for (std.meta.fields(T)) |field| publicTypes(field.type, seen, count);
        },
        else => {},
    }
}
comptime {
    @setEvalBranchQuota(1000000);
    var visited: [4096]type = undefined;
    var count: usize = 0;
    publicTypes(tt, &visited, &count);
    if (@hasDecl(tt.results.SessionPacket, "jsonStringify") or @hasDecl(tt.results.Value, "jsonStringify")) @compileError("result dump methods must stay private");
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
    inline for (.{ "recognize", "relate" }) |kind| {
        var typed = switch (try if (comptime std.mem.eql(u8, kind, "recognize"))
            tt.Question.recognize(&engine, .{ .recognize = .{ .kinds = .{ .entries = &.{.{ .key = "person", .value = .{ .string = "A person" } }} } } })
        else
            tt.Question.relate(&engine, .{ .relate = .{ .relations = &.{.{ .either = false, .name = "knows", .source = "person", .target = "person" }} } })) {
            .ok => |q| q,
            .failed => |failure| {
                defer tt.releaseFailure(a, failure);
                return error.TypedQuestionBuild;
            },
        };
        typed.deinit();
    }
    const request = try tt.Request.init(a, "decide", .{ .text = .{ .text = "Does it pass?" } }, .{ .text = .{ .text = "Evidence." } }, .{ .details = false });
    var plan = try request.plan(&engine);
    request.deinit();
    defer plan.deinit();
    if (plan.value.records != 1 or plan.value.requests != 1 or plan.value.largest_request_bytes == 0 or plan.value.estimated_input_tokens.upper == 0) return error.WrongPlan;
    if (plan.value.estimated_bytes == 0 or plan.value.estimated_input_tokens.lower > plan.value.estimated_input_tokens.upper or plan.value.first_body_utf8 == null or plan.value.largest_request_estimated_input_tokens == 0 or plan.value.token_estimate_method.len == 0 or plan.value.upper_bound) return error.WrongPlanFields;
    const unknown = try tt.Request.init(a, "unknown-function", .{ .text = .{ .text = "Does it pass?" } }, .{ .text = .{ .text = "Evidence." } }, null);
    defer unknown.deinit();
    if (unknown.start(&engine)) |unexpected| {
        var owned = unexpected;
        owned.deinit();
        return error.UnknownFunctionAccepted;
    } else |err| {
        if (err != error.Usage) return err;
        const immediate = try tt.capture(a, &engine, 1);
        defer tt.releaseFailure(a, immediate);
        if (immediate.kind != .usage or immediate.message.len == 0) return error.WrongImmediateFailure;
    }
    const args = try std.process.argsAlloc(a);
    defer std.process.argsFree(a, args);
    if (args.len > 1 and std.mem.eql(u8, args[1], "helpers")) {
        try helperInputs(&engine, a);
        return;
    }
    const failure_case = args.len > 1 and std.mem.eql(u8, args[1], "failure");
    const usage_case = args.len > 1 and std.mem.startsWith(u8, args[1], "usage-");
    if (args.len > 1 and !failure_case and !usage_case) {
        engine.deinit();
        if (plan.value.first_body_utf8 == null) return error.MissingPlanBody;
        std.debug.print("installed Zig plan PASS\n", .{});
        return;
    }
    var session = try tt.session.startNamed(&engine, .decide, .{ .question = .{ .text = .{ .text = "Does it pass?" } }, .input = .{ .feed = .{ .name = "owned" } } });
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
    const closed_request = try tt.Request.init(a, "decide", .{ .text = .{ .text = "Does it pass?" } }, .{ .text = .{ .text = "Evidence." } }, null);
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

fn helperInputs(engine: *tt.Engine, a: std.mem.Allocator) !void {
    const original = try std.json.parseFromSlice(std.json.Value, a, "{\"ticket\":1}", .{});
    defer original.deinit();
    var call = try tt.session.startNamed(engine, .filter, .{ .options = .{ .batch = .{ .integer = 1 } }, .question = .{ .text = .{ .text = "Does it pass?" } }, .input = .{ .records = .{ .items = &.{
        .{ .original = .{ .json = .{ .value = .null } } },
        .{ .original = .{ .json = .{ .value = original.value } } },
    } } } });
    try call.finish();
    var retained: std.ArrayList(*tt.session.Packet) = .empty;
    defer {
        for (retained.items) |packet| packet.deinit();
        retained.deinit(a);
    }
    while (true) switch (try call.next()) {
        .end => break,
        .pending => return error.BlockingPending,
        .packet => |packet| if (packet.value().* == .filter_row) {
            try retained.append(a, packet);
        } else packet.deinit(),
    };
    call.deinit();
    engine.deinit();
    if (retained.items.len != 2) return error.MissingFilter;
    const first = try retained.items[0].answers().filter();
    const second = try retained.items[1].answers().filter();
    if (first.len != 1 or second.len != 1 or first[0] != .null_value or second[0] != .object) return error.WrongOriginal;
    const entries = second[0].object.entries;
    if (entries.len != 1 or !std.mem.eql(u8, entries[0].key, "ticket") or entries[0].value.number.unsigned != 1) return error.WrongOriginal;
    if (retained.items[0].answers().rank()) |_| return error.WrongAccessorAccepted else |err| if (err != error.WrongAnswer) return err;
    std.debug.print("installed Zig helpers PASS\n", .{});
}
