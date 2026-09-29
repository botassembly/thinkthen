const std = @import("std");
const tt = @import("thinkthen");
const Allocator = std.mem.Allocator;
const ThreadCase = struct {
    engine: *tt.Engine,
    token: ?*tt.CancelToken = null,
    result: ?tt.Result(tt.CallResult(tt.Answer)) = null,
    err: ?anyerror = null,
    done: std.atomic.Value(bool) = std.atomic.Value(bool).init(false),
    deadline_ms: i64 = -1,
    text: []const u8,
    fn run(self: *ThreadCase) void {
        defer self.done.store(true, .release);
        self.result = self.engine.decide("Is it?", self.text, .{ .deadline_ms = self.deadline_ms, .cancel = if (self.token) |t| t.raw else null }) catch |e| {
            self.err = e;
            return;
        };
    }
};
fn waitFor(path: []const u8) !void {
    for (0..2000) |_| {
        if (std.fs.cwd().access(path, .{})) |_| return else |_| {}
        std.Thread.sleep(5 * std.time.ns_per_ms);
    }
    return error.ArrivalBarrierTimedOut;
}
fn releaseHeld(path: []const u8) !void {
    const file = try std.fs.cwd().createFile(path, .{});
    file.close();
}
fn checked(result: tt.Result(tt.CallResult(tt.Answer)), code: c_int) !void {
    switch (result) {
        .failed => |f| {
            if (f.code != code or f.message.len == 0) return error.BadFailure;
        },
        .ok => return error.ExpectedFailure,
    }
}
fn hold(engine: *tt.Engine, alloc: Allocator, dir: []const u8, text: []const u8) !void {
    var token = try tt.CancelToken.init();
    defer token.deinit();
    var case = ThreadCase{ .engine = engine, .token = &token, .text = text, .deadline_ms = if (std.mem.eql(u8, text, "hold-deadline")) 25 else -1 };
    const arrival = try std.fmt.allocPrint(alloc, "{s}/arrived-{s}", .{ dir, text });
    defer alloc.free(arrival);
    const release = try std.fmt.allocPrint(alloc, "{s}/release-{s}", .{ dir, text });
    defer alloc.free(release);
    const worker = try std.Thread.spawn(.{}, ThreadCase.run, .{&case});
    var joined = false;
    defer {
        if (!joined) {
            token.cancel();
            releaseHeld(release) catch |err| std.debug.print("held release during cleanup: {}\n", .{err});
            worker.join();
        }
        if (case.result) |result| switch (result) {
            .failed => |f| engine.freeFailure(f),
            .ok => |success| success.deinit(alloc),
        };
    }
    try waitFor(arrival); // HTTP request is accepted, before cancellation fires.
    const is_deadline = std.mem.eql(u8, text, "hold-deadline");
    if (!is_deadline) {
        token.cancel();
        token.cancel();
    }
    // The arrival barrier proves in-flight work; allow a cancellation/deadline tick before the held reply.
    std.Thread.sleep(100 * std.time.ns_per_ms);
    std.debug.print("held call completed before release: {} token address: {x}\n", .{ case.done.load(.acquire), @intFromPtr(token.raw) });
    try releaseHeld(release);
    worker.join();
    joined = true;
    if (case.err) |err| return err;
    switch (case.result.?) {
        .ok => |answer| std.debug.print("held call unexpectedly succeeded: {s} {d}\n", .{ @tagName(answer.value.outcome), answer.value.probability }),
        .failed => |f| std.debug.print("held call failed code={d}\n", .{f.code}),
    }
    // Recovery uses distinct evidence so the fixture can count it after each held call.
    var fresh = try tt.CancelToken.init();
    defer fresh.deinit();
    const recovery = if (is_deadline) "recovery-deadline" else "recovery-scalar";
    switch (try engine.decide("Is it?", recovery, .{ .cancel = fresh.raw })) {
        .ok => |answer| {
            defer answer.deinit(alloc);
            if (answer.value.outcome != .yes or answer.value.probability != 0.9) return error.BadRecovery;
        },
        .failed => |f| {
            defer engine.freeFailure(f);
            return error.RecoveryFailed;
        },
    }
    std.debug.print("fresh-token recovery {s} PASS\n", .{recovery});
    if (is_deadline) {
        try checked(case.result.?, tt.c.THINKTHEN_EDEADLINE);
    } else switch (case.result.?) {
        .ok => return error.ScalarCancelledCallSucceeded,
        .failed => |f| try checked(.{ .failed = f }, tt.c.THINKTHEN_ECANCELLED),
    }
}
const BulkCase = struct {
    engine: *tt.Engine,
    token: *tt.CancelToken,
    result: ?tt.Result(tt.CallResult([]tt.Answer)) = null,
    err: ?anyerror = null,
    fn run(self: *BulkCase) void {
        const rows = [_][]const u8{ "hold-bulk-1", "hold-bulk-2", "hold-bulk-3", "hold-bulk-4", "hold-bulk-5", "hold-bulk-6" };
        self.result = self.engine.decideMany("Is it?", &rows, .{ .cancel = self.token.raw }) catch |e| {
            self.err = e;
            return;
        };
    }
};
fn holdBulk(engine: *tt.Engine, alloc: Allocator, dir: []const u8) !void {
    var token = try tt.CancelToken.init();
    defer token.deinit();
    var job = BulkCase{ .engine = engine, .token = &token };
    const arrival = try std.fmt.allocPrint(alloc, "{s}/arrived-hold-bulk-1", .{dir});
    defer alloc.free(arrival);
    var releases: [6][]u8 = undefined;
    var allocated: usize = 0;
    defer for (releases[0..allocated]) |path| alloc.free(path);
    for (&releases, 1..) |*path, i| {
        path.* = try std.fmt.allocPrint(alloc, "{s}/release-hold-bulk-{d}", .{ dir, i });
        allocated += 1;
    }
    const worker = try std.Thread.spawn(.{}, BulkCase.run, .{&job});
    var joined = false;
    defer {
        if (!joined) {
            token.cancel();
            for (releases) |path| releaseHeld(path) catch |err| std.debug.print("bulk release during cleanup: {}\n", .{err});
            worker.join();
        }
        if (job.result) |result| switch (result) {
            .ok => |success| success.deinit(alloc),
            .failed => |f| engine.freeFailure(f),
        };
    }
    try waitFor(arrival);
    token.cancel();
    token.cancel();
    std.Thread.sleep(100 * std.time.ns_per_ms);
    for (releases) |path| try releaseHeld(path);
    worker.join();
    joined = true;
    if (job.err) |e| return e;
    switch (job.result.?) {
        .ok => return error.BulkDeliveredAfterCancel,
        .failed => |f| {
            std.debug.print("held bulk code={d}\n", .{f.code});
            if (f.code != tt.c.THINKTHEN_ECANCELLED) return error.WrongBulkStop;
        },
    }
}
fn factsLifetime(alloc: Allocator, dir: []const u8) !void {
    var engine = switch (try tt.Engine.init(alloc)) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(alloc, failure);
            return error.EngineBuild;
        },
    };
    var closed = false;
    defer if (!closed) engine.deinit();
    var cases = [_]ThreadCase{
        .{ .engine = &engine, .text = "hold-facts-one" },
        .{ .engine = &engine, .text = "hold-facts-no-usage" },
    };
    const arrivals = [_][]const u8{
        try std.fmt.allocPrint(alloc, "{s}/arrived-hold-facts-one", .{dir}),
        try std.fmt.allocPrint(alloc, "{s}/arrived-hold-facts-no-usage", .{dir}),
    };
    defer for (arrivals) |path| alloc.free(path);
    const releases = [_][]const u8{
        try std.fmt.allocPrint(alloc, "{s}/release-hold-facts-one", .{dir}),
        try std.fmt.allocPrint(alloc, "{s}/release-hold-facts-no-usage", .{dir}),
    };
    defer for (releases) |path| alloc.free(path);
    var threads: [2]std.Thread = undefined;
    var started: usize = 0;
    var joined: usize = 0;
    defer {
        if (joined != started) for (releases) |path| releaseHeld(path) catch {};
        for (threads[joined..started]) |thread| thread.join();
    }
    for (&cases, 0..) |*case, i| {
        threads[i] = try std.Thread.spawn(.{}, ThreadCase.run, .{case});
        started += 1;
    }
    try waitFor(arrivals[0]);
    try waitFor(arrivals[1]);
    for (releases) |path| try releaseHeld(path);
    for (threads[0..started]) |thread| {
        thread.join();
        joined += 1;
    }
    defer for (cases) |case| if (case.result) |result| switch (result) {
        .ok => |success| success.deinit(alloc),
        .failed => |failure| engine.freeFailure(failure),
    };
    for (cases) |case| if (case.err) |err| return err;
    const first = switch (cases[0].result.?) {
        .ok => |success| success,
        .failed => return error.FirstFactsFailed,
    };
    const second = switch (cases[1].result.?) {
        .ok => |success| success,
        .failed => return error.SecondFactsFailed,
    };
    try require(first.value.outcome == .yes and first.facts.records == 1 and first.facts.requests_sent == 1 and first.facts.input_tokens != null and first.facts.model != null);
    try require(second.value.outcome == .yes and second.facts.records == 1 and second.facts.requests_sent == 1 and second.facts.input_tokens == null and second.facts.output_tokens == null and second.facts.model != null and std.mem.eql(u8, second.facts.model.?, "jev-1.13.0"));
    const failed = switch (try engine.decide("Is it?", "status-401", .{})) {
        .failed => |failure| failure,
        .ok => |success| {
            defer success.deinit(alloc);
            return error.ExpectedStartedFailure;
        },
    };
    defer tt.releaseFailure(alloc, failed);
    try require(failed.kind == .backend and failed.facts_json != null);
    const failure_facts = try std.json.parseFromSlice(std.json.Value, alloc, failed.facts_json.?, .{});
    defer failure_facts.deinit();
    try require(failure_facts.value.object.get("requests_sent").?.integer == 1);
    const facts_copy = try alloc.dupe(u8, failed.facts_json.?);
    defer alloc.free(facts_copy);
    const later = switch (try engine.decide("Is it?", "recovery-scalar", .{})) {
        .ok => |success| success,
        .failed => |failure| {
            defer engine.freeFailure(failure);
            return error.RecoveryFailed;
        },
    };
    defer later.deinit(alloc);
    engine.deinit();
    closed = true;
    try require(later.value.outcome == .yes and first.facts.input_tokens.? == 1 and second.facts.input_tokens == null and std.mem.eql(u8, failed.facts_json.?, facts_copy));
    std.debug.print("ZIG_FACTS_LIFETIME_PASS\n", .{});
}
fn require(ok: bool) !void {
    if (!ok) return error.BadFacts;
}
pub fn main() !void {
    var gpa = std.heap.DebugAllocator(.{ .thread_safe = true }){};
    defer if (gpa.deinit() != .ok) @panic("allocator leak");
    const alloc = gpa.allocator();
    var engine = switch (try tt.Engine.init(alloc)) {
        .ok => |e| e,
        .failed => |f| {
            defer alloc.free(f.message);
            return error.EngineBuild;
        },
    };
    defer engine.deinit();
    const dir = try std.process.getEnvVarOwned(alloc, "TT_BARRIER_DIR");
    defer alloc.free(dir);
    var args = std.process.args();
    _ = args.next();
    const mode = args.next() orelse return error.ModeRequired;
    if (std.mem.eql(u8, mode, "--facts")) {
        try factsLifetime(alloc, dir);
        return;
    }
    if (std.mem.eql(u8, mode, "--holds-only")) {
        try hold(&engine, alloc, dir, "hold-deadline");
        try holdBulk(&engine, alloc, dir);
        try hold(&engine, alloc, dir, "hold-scalar");
        return;
    }
    if (!std.mem.eql(u8, mode, "--callers-only")) return error.BadMode;
    var cases = [_]ThreadCase{
        .{ .engine = &engine, .text = "failure-one" },
        .{ .engine = &engine, .text = "failure-two" },
        .{ .engine = &engine, .text = "success" },
    };
    var threads: [cases.len]std.Thread = undefined;
    var started: usize = 0;
    var joined: usize = 0;
    defer {
        for (threads[joined..started]) |thread| thread.join();
        for (cases[0..started]) |case| {
            if (case.result) |result| switch (result) {
                .failed => |f| engine.freeFailure(f),
                .ok => |success| success.deinit(alloc),
            };
        }
    }
    for (&cases, 0..) |*case, i| {
        threads[i] = try std.Thread.spawn(.{}, ThreadCase.run, .{case});
        started += 1;
    }
    for (threads[0..started]) |thread| {
        thread.join();
        joined += 1;
    }
    for (&cases, 0..) |*case, i| {
        if (case.err) |err| return err;
        if (i < 2) {
            switch (case.result.?) {
                .failed => |f| {
                    if (f.code != tt.c.THINKTHEN_EBACKEND or std.mem.indexOf(u8, f.message, if (i == 0) "401" else "403") == null) return error.ThreadLocalFailureMismatch;
                },
                .ok => return error.ExpectedThreadFailure,
            }
        } else switch (case.result.?) {
            .ok => |a| {
                if (a.value.outcome != .yes) return error.WrongConcurrentAnswer;
            },
            .failed => return error.ConcurrentSuccessFailed,
        }
    }
    std.debug.print("concurrent: three callers PASS\n", .{});
}
