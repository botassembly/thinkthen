const std = @import("std");
const builtin = @import("builtin");
extern "c" fn _write(fd: c_int, bytes: [*]const u8, count: c_uint) c_int;
extern "c" fn _read(fd: c_int, bytes: [*]u8, count: c_uint) c_int;
const tt = @import("thinkthen");
const Value = std.json.Value;
const Allocator = std.mem.Allocator;

fn json(allocator: Allocator, value: anytype) ![]u8 {
    return std.json.Stringify.valueAlloc(allocator, value, .{});
}
fn write(bytes: []const u8) void {
    var remaining = bytes;
    while (remaining.len != 0) {
        const count = if (builtin.os.tag == .windows)
            @as(isize, _write(1, remaining.ptr, @intCast(remaining.len)))
        else
            std.c.write(1, remaining.ptr, remaining.len);
        std.debug.assert(count > 0);
        remaining = remaining[@intCast(count)..];
    }
}
fn writeFailure(diagnostic: *tt.Diagnostic, allocator: Allocator) !void {
    const failure = diagnostic.failure().?;
    std.debug.assert(failure.message.len > 0);
    const encoded = try json(allocator, .{ .@"error" = failure.code });
    defer allocator.free(encoded);
    write(encoded);
}
fn writeRow(row: anytype, allocator: Allocator) !void {
    write("{\"status\":");
    const status = try json(allocator, @tagName(row.status()));
    defer allocator.free(status);
    write(status);
    if (row.idJson()) |raw| {
        write(",\"id\":");
        write(raw);
    }
    if (row.answerJson() != null) {
        write(",\"answer\":");
        const typed = try std.json.Stringify.valueAlloc(allocator, row.answer(), .{ .emit_null_optional_fields = false });
        defer allocator.free(typed);
        write(typed);
    }
    if (row.probability()) |probability| {
        const raw = try json(allocator, probability);
        defer allocator.free(raw);
        write(",\"p\":");
        write(raw);
    }
    if (row.detailJson()) |raw| {
        write(",\"detail\":");
        write(raw);
    }
    if (row.failure()) |failure| {
        const raw = try json(allocator, .{ .code = failure.code });
        defer allocator.free(raw);
        write(",\"error\":");
        write(raw);
    }
    write("}");
}
const Function = enum { decide, choose, tag, score, filter, rank, find, annotate, recognize, relate };
fn run(allocator: Allocator, arena: Allocator, settings: []const u8, request_json: []const u8, cancelled: bool) !void {
    var diagnostic = tt.Diagnostic.init(allocator);
    defer diagnostic.deinit();
    const client = tt.Client.init(allocator, settings, &diagnostic) catch |err| {
        if (err != error.EngineFailure) return err;
        return writeFailure(&diagnostic, allocator);
    };
    defer client.deinit();
    const request = try std.json.parseFromSlice(Value, arena, request_json, .{ .parse_numbers = false });
    defer request.deinit();
    const fields = request.value.object;
    var records: std.ArrayList(tt.Record) = .empty;
    for (fields.get("records").?.array.items) |record| {
        try records.append(arena, if (record == .string)
            .{ .text = record.string }
        else
            .{ .json = try json(arena, record) });
    }
    const question = try json(arena, fields.get("question").?);
    const options = if (fields.get("options")) |value| try json(arena, value) else null;
    inline for (@typeInfo(Function).@"enum".field_names) |name| {
        if (std.mem.eql(u8, fields.get("function").?.string, name))
            return runNamed(name, client, allocator, .{ .json = question }, records.items, .{ .json = options }, &diagnostic, cancelled);
    }
    return error.UnknownTestFunction;
}
fn runNamed(comptime name: []const u8, client: *const tt.Client, allocator: Allocator, question: tt.Question, records: []const tt.Record, options: tt.CallOptions, diagnostic: *tt.Diagnostic, cancelled: bool) !void {
    const call = @field(tt.Client, name)(client, allocator, question, records, options, diagnostic) catch |err| {
        if (err != error.EngineFailure) return err;
        return writeFailure(diagnostic, allocator);
    };
    defer call.deinit();
    if (cancelled) call.cancel();
    var rows: std.ArrayList(@TypeOf(call).Row) = .empty;
    defer {
        for (rows.items) |row| row.deinit();
        rows.deinit(allocator);
    }
    var failed = false;
    while (true) {
        const row = call.next(diagnostic) catch |err| {
            if (err != error.EngineFailure) return err;
            failed = true;
            break;
        } orelse break;
        errdefer row.deinit();
        std.debug.assert(row.position() > 0);
        if (std.mem.eql(u8, name, "filter") and row.status() == .answered)
            std.debug.assert(row.kept() == std.mem.eql(u8, row.answerJson().?, "true"));
        try rows.append(allocator, row);
    }
    if (failed) {
        const code = try json(allocator, diagnostic.failure().?.code);
        defer allocator.free(code);
        write("{\"error\":");
        write(code);
        write(",\"rows\":");
    }
    write("[");
    for (rows.items, 0..) |row, index| {
        if (index > 0) write(",");
        try writeRow(row, allocator);
    }
    write("]");
    if (failed) write("}");
}

const LazyInput = struct {
    calls: usize = 0,
    fail: bool = false,
    fn next(context: *anyopaque) anyerror!?tt.Record {
        const self: *LazyInput = @ptrCast(@alignCast(context));
        if (self.fail) return error.SourceFailure;
        defer self.calls += 1;
        return if (self.calls == 0) .{ .text = "alpha" } else null;
    }
};
fn typedAnswers(allocator: Allocator) !void {
    var diagnostic = tt.Diagnostic.init(allocator);
    defer diagnostic.deinit();
    const client = try tt.Client.init(allocator, "{\"adapter\":\"stub\"}", &diagnostic);
    defer client.deinit();
    var input: LazyInput = .{};
    const call = try client.decide(allocator, .{ .text = "Ready?" }, tt.RecordSource{ .context = &input, .next = LazyInput.next }, .{}, &diagnostic);
    defer call.deinit();
    std.debug.assert(input.calls == 0);
    const row = (try call.next(&diagnostic)).?;
    defer row.deinit();
    const verdict: ?bool = row.answer();
    std.debug.assert(verdict != null);
    std.debug.assert(try call.next(&diagnostic) == null);
    std.debug.assert(input.calls == 2);
    var broken: LazyInput = .{ .fail = true };
    const failed = try client.decide(allocator, .{ .text = "Ready?" }, tt.RecordSource{ .context = &broken, .next = LazyInput.next }, .{}, &diagnostic);
    defer failed.deinit();
    if (failed.next(&diagnostic)) |_| return error.ExpectedSourceFailure else |err| std.debug.assert(err == error.SourceFailure);
    std.debug.assert(diagnostic.failure() == null);
    const annotation = try client.annotate(allocator, .{ .json = "{\"type\":\"inline\",\"value\":{\"questions\":{\"ready\":{\"decide\":\"Ready?\"},\"label\":{\"choose\":\"Pick a label\",\"options\":[\"one\",\"two\"]}}}}" }, &.{tt.Record{ .text = "alpha" }}, .{}, &diagnostic);
    const annotated = (try annotation.next(&diagnostic)).?;
    annotation.deinit();
    const scaled = try client.annotate(allocator, .{ .json = "{\"type\":\"inline\",\"value\":{\"questions\":{\"urgency\":{\"score\":\"How urgent?\",\"levels\":[\"low\",\"high\"]}}}}" }, &.{tt.Record{ .text = "alpha" }}, .{}, &diagnostic);
    defer scaled.deinit();
    const scaled_row = (try scaled.next(&diagnostic)).?;
    defer scaled_row.deinit();
    const probabilities = scaled_row.answer().?.map.get("urgency").?.detail.?.array.items;
    std.debug.assert(probabilities.len == 2);
    const entry = annotated.answer().?.map.get("ready").?;
    const decoded = (try entry.decode(bool, allocator)).?;
    const label = (try annotated.answer().?.map.get("label").?.decode([]const u8, allocator)).?;
    const expected_label = try allocator.dupe(u8, annotated.answer().?.map.get("label").?.answer.?.string);
    defer allocator.free(expected_label);
    annotated.deinit();
    defer label.deinit();
    std.debug.assert(std.mem.eql(u8, label.value, expected_label));
    defer decoded.deinit();
    std.debug.assert(decoded.value == verdict.?);
}

fn ownership(allocator: Allocator) !void {
    var diagnostic = tt.Diagnostic.init(allocator);
    defer diagnostic.deinit();
    const client = try tt.Client.init(allocator, "{\"adapter\":\"stub\"}", &diagnostic);
    const records = [_]tt.Record{.{ .json = "{\"id\":{\"a\\u0000b\":18446744073709551615},\"body\":\"evidence\"}" }};
    const call = try client.decide(allocator, .{ .text = "Ready?" }, &records, .{}, &diagnostic);
    client.deinit();
    const row = (try call.next(&diagnostic)).?;
    call.deinit();
    std.debug.assert(row.status() != .failed);
    std.debug.assert(std.mem.eql(u8, row.idJson().?, "{\"a\\u0000b\":18446744073709551615}"));
    const id = try std.json.parseFromSlice(Value, allocator, row.idJson().?, .{ .allocate = .alloc_always, .parse_numbers = false });
    defer id.deinit();
    var failing = std.testing.FailingAllocator.init(allocator, .{ .fail_index = 0 });
    if (row.parseDetail(failing.allocator())) |_| return error.ExpectedAllocationFailure else |err| std.debug.assert(err == error.OutOfMemory);
    std.debug.assert(row.detailJson() != null);
    const detail = (try row.parseDetail(allocator)).?;
    defer detail.deinit();
    row.deinit();
    std.debug.assert(std.mem.eql(u8, id.value.object.get("a\x00b").?.number_string, "18446744073709551615"));
    std.debug.assert(std.mem.eql(u8, detail.value.object.get("source").?.object.get("backend").?.string, "stub"));

    const score_client = try tt.Client.init(allocator, "{\"adapter\":\"stub\"}", &diagnostic);
    defer score_client.deinit();
    const score_call = try score_client.score(allocator, .{ .text = "How urgent?" }, &records, .{ .json = "{\"levels\":[\"low\",\"high\"]}" }, &diagnostic);
    defer score_call.deinit();
    const score_row = (try score_call.next(&diagnostic)).?;
    const score_detail = (try score_row.parseDetail(allocator)).?;
    defer score_detail.deinit();
    score_row.deinit();
    std.debug.assert(score_detail.value.object.get("probabilities").?.array.items[0] == .number_string);

    var failing_diagnostic = tt.Diagnostic.init(failing.allocator());
    defer failing_diagnostic.deinit();
    if (tt.Client.init(allocator, "{\"batch\":0}", &failing_diagnostic)) |_| return error.ExpectedAllocationFailure else |err| std.debug.assert(err == error.OutOfMemory);
    std.debug.assert(failing_diagnostic.failure() == null);
    std.debug.assert(std.mem.indexOf(u8, failing_diagnostic.rawJson().?, "usage.batch") != null);
    failing_diagnostic.clear();
    std.debug.assert(failing_diagnostic.rawJson() == null);

    const stub = try tt.Client.init(allocator, "{\"adapter\":\"stub\"}", &diagnostic);
    defer stub.deinit();
    const empty = try stub.decide(allocator, .{ .text = "Ready?" }, &.{}, .{}, &diagnostic);
    defer empty.deinit();
    std.debug.assert(try empty.next(&diagnostic) == null);
    if (stub.decide(allocator, .{ .text = "" }, &.{}, .{}, &diagnostic)) |_| return error.ExpectedEngineFailure else |err| std.debug.assert(err == error.EngineFailure);
    std.debug.assert(std.mem.eql(u8, diagnostic.failure().?.code, "usage.question"));
    const again = try stub.decide(allocator, .{ .text = "Ready?" }, &.{}, .{}, &diagnostic);
    defer again.deinit();
    std.debug.assert(diagnostic.rawJson() == null);
    var failures: usize = 0;
    var successes: usize = 0;
    for (0..32) |offset| {
        var allocations = std.testing.FailingAllocator.init(allocator, .{});
        const attempt = try stub.decide(allocations.allocator(), .{ .text = "Ready?" }, &records, .{}, &diagnostic);
        allocations.fail_index = allocations.alloc_index + offset;
        const next = attempt.next(&diagnostic);
        if (next) |value| {
            value.?.deinit();
            successes += 1;
        } else |err| {
            std.debug.assert(err == error.OutOfMemory);
            failures += 1;
        }
        attempt.deinit();
        std.debug.assert(allocations.allocated_bytes == allocations.freed_bytes);
    }
    std.debug.assert(failures > 0 and successes > 0);
}

fn driveHeld(call: *tt.Call, allocator: Allocator) void {
    var diagnostic = tt.Diagnostic.init(allocator);
    defer diagnostic.deinit();
    while (true) {
        const row = call.next(&diagnostic) catch |err| {
            std.debug.assert(err == error.EngineFailure);
            std.debug.assert(std.mem.eql(u8, diagnostic.failure().?.code, "local.cancelled"));
            return;
        } orelse @panic("held request completed before cancellation");
        row.deinit();
    }
}
fn held(allocator: Allocator, settings: []const u8) !void {
    var diagnostic = tt.Diagnostic.init(allocator);
    defer diagnostic.deinit();
    const client = try tt.Client.init(allocator, settings, &diagnostic);
    defer client.deinit();
    const records = [_]tt.Record{.{ .text = "evidence" }};
    const call = try client.decide(allocator, .{ .text = "Ready?" }, &records, .{}, &diagnostic);
    defer call.deinit();
    const worker = try std.Thread.spawn(.{}, driveHeld, .{ call.raw, allocator });
    var signal: [1]u8 = undefined;
    const count = if (builtin.os.tag == .windows) _read(0, &signal, 1) else std.c.read(0, &signal, 1);
    std.debug.assert(count == 1);
    call.cancel();
    worker.join();
}

pub fn main(init: std.process.Init) !void {
    var gpa: std.heap.DebugAllocator(.{}) = .init;
    defer std.debug.assert(gpa.deinit() == .ok);
    const allocator = gpa.allocator();
    const args = try init.minimal.args.toSlice(init.arena.allocator());
    if (std.mem.eql(u8, args[1], "run"))
        return run(allocator, init.arena.allocator(), args[2], args[3], args.len > 4);
    if (std.mem.eql(u8, args[1], "held")) return held(allocator, args[2]);
    if (std.mem.eql(u8, args[1], "ownership")) {
        try ownership(allocator);
        return typedAnswers(allocator);
    }
    return error.UnknownTestMode;
}
