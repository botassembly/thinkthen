const std = @import("std");
const builtin = @import("builtin");
const tt = @import("thinkthen");
const fixture = @import("fixture.zig");
const A = std.mem.Allocator;
extern "c" fn _write(fd: c_int, bytes: [*]const u8, count: c_uint) c_int;
extern "c" fn _read(fd: c_int, bytes: [*]u8, count: c_uint) c_int;
fn write(bytes: []const u8) void {
    var remaining = bytes;
    while (remaining.len != 0) {
        const count = if (builtin.os.tag == .windows) @as(isize, _write(1, remaining.ptr, @intCast(remaining.len))) else std.c.write(1, remaining.ptr, remaining.len);
        std.debug.assert(count > 0);
        remaining = remaining[@intCast(count)..];
    }
}
fn emit(a: A, value: anytype) !void {
    write(try std.json.Stringify.valueAlloc(a, value, .{ .emit_null_optional_fields = false }));
}
fn fail(a: A, diagnostic: tt.Diagnostic) !void {
    try emit(a, .{ .@"error" = diagnostic.code.?, .http_status = diagnostic.http_status });
}
const Mode = enum { many, cancelled, single };
fn emitRecord(a: A, record: tt.Record) !void {
    switch (record) {
        .text => |text| try emit(a, text),
        .json => |json| write(json),
        .missing => write("null"),
    }
}
fn runNamed(comptime function: tt.Function, client: *const tt.Client, a: A, value: std.json.Value, records: []const tt.Record, mode: Mode) !void {
    var diagnostic: tt.Diagnostic = .{};
    var saved: ?tt.Toolbox = null;
    defer if (saved) |*toolbox| toolbox.deinit();
    var question: tt.QuestionType(function) = if (function == .annotate) .{ .entries = value.object.get("questions").?, .field = if (value.object.get("field")) |field| field.string else null } else blk: {
        saved = fixture.toolbox(a, value, &diagnostic) catch return fail(a, diagnostic);
        break :blk saved.?.get("fixture").?.as(function).?;
    };
    if (value.object.get("name")) |name| question.name = name.string;
    if (mode == .single) {
        if (function == .decide) {
            const item = try client.decide(a, question, records[0], &diagnostic);
            write("[");
            try emit(a, item);
            write("]");
            return;
        } else return error.SingleModeRequiresDecide;
    }
    const method = if (function == .filter or function == .rank) @tagName(function) else @tagName(function) ++ "All";
    var call = @field(tt.Client, method)(client, a, question, records, &diagnostic) catch return fail(a, diagnostic);
    defer call.deinit();
    if (mode == .cancelled) {
        call.cancel();
        _ = call.next(&diagnostic) catch return fail(a, diagnostic);
        return error.CancellationWasIgnored;
    }
    var count: usize = 0;
    while (true) {
        const item = (call.next(&diagnostic) catch |err| {
            if (function == .filter and err == error.Failed) break;
            if (count == 0) return fail(a, diagnostic);
            return err;
        }) orelse break;
        if (count == 0) write(if (function == .filter) "{\"kept\":[" else "[") else write(",");
        if (function == .filter) try emitRecord(a, item) else try emit(a, item);
        count += 1;
    }
    if (count == 0) write(if (function == .filter) "{\"kept\":[" else "[");
    write("]");
    if (function == .filter) {
        write(",\"failed\":[");
        for (call.failed(), 0..) |failed, index| {
            if (index != 0) write(",");
            try emit(a, failed);
        }
        write("]}");
    }
}
fn run(a: A, settings_json: []const u8, request_json: []const u8, mode: Mode) !void {
    const settings = try fixture.settings(a, settings_json);
    var diag: tt.Diagnostic = .{};
    var client = tt.Client.init(settings, &diag) catch {
        return fail(a, diag);
    };
    defer client.deinit();
    const request = try std.json.parseFromSliceLeaky(std.json.Value, a, request_json, .{ .parse_numbers = false });
    const values = request.object.get("records").?.array.items;
    const records = try a.alloc(tt.Record, values.len);
    for (values, records) |value, *record| record.* = switch (value) {
        .string => |v| .{ .text = v },
        .null => .missing,
        else => try tt.Record.fromValue(a, value),
    };
    var question = request.object.get("question").?;
    if (request.object.get("name")) |name| try question.object.put(a, "name", name);
    const name = question.object.get("function").?.string;
    inline for (@typeInfo(tt.Function).@"enum".field_names) |tag| {
        if (std.mem.eql(u8, name, tag)) return runNamed(@field(tt.Function, tag), &client, a, question, records, mode);
    }
    return error.UnknownFixture;
}
fn driveHeld(call: *tt.Call(tt.Decide, tt.Record)) void {
    var diag: tt.Diagnostic = .{};
    _ = call.next(&diag) catch |err| {
        std.debug.assert(err == error.Cancelled and diag.code == .local_cancelled);
        return;
    };
    @panic("held request returned before cancellation");
}
fn held(a: A, settings_json: []const u8, token: bool) !void {
    var settings = try fixture.settings(a, settings_json);
    var stop = try tt.Cancel.init(null);
    defer stop.deinit();
    if (token) settings.cancel = &stop;
    var client = try tt.Client.init(settings, null);
    defer client.deinit();
    var call = try client.decideAll(a, tt.Decide{ .question = "Ready?" }, &[_]tt.Record{.{ .text = "evidence" }}, null);
    defer call.deinit();
    const worker = try std.Thread.spawn(.{}, driveHeld, .{&call});
    var signal: [1]u8 = undefined;
    const count = if (builtin.os.tag == .windows) _read(0, &signal, 1) else std.c.read(0, &signal, 1);
    std.debug.assert(count == 1);
    if (token) stop.cancel() else call.cancel();
    worker.join();
}
pub fn main(init: std.process.Init) !void {
    const a = init.arena.allocator();
    const args = try init.minimal.args.toSlice(a);
    if (std.mem.startsWith(u8, args[1], "stress")) return @import("stress.zig").run(a, args);
    if (std.mem.eql(u8, args[1], "admission-surface")) {
        write("zig\n");
        return;
    }
    if (std.mem.eql(u8, args[1], "load")) {
        var diagnostic: tt.Diagnostic = .{};
        var client = tt.Client.init(.{ .backend = .stub }, &diagnostic) catch |err| {
            write(diagnostic.message());
            return err;
        };
        client.deinit();
        return;
    }
    if (std.mem.eql(u8, args[1], "toolbox")) {
        const json = try @import("toolbox.zig").saved(a, args[2]);
        write(json);
        return;
    }
    if (std.mem.eql(u8, args[1], "toolbox-invalid")) {
        const diagnostic = try @import("toolbox.zig").invalid(a, args[2]);
        write(diagnostic.code.?.name());
        write("\n");
        write(diagnostic.message());
        write("\n");
        return;
    }
    if (std.mem.eql(u8, args[1], "usage")) {
        var client = try tt.Client.init(.{ .backend = .systemone, .url = args[2] }, null);
        _ = try client.decide(a, tt.Decide{ .question = "Ready?" }, "evidence", null);
        const snapshot = try client.usage(null);
        client.deinit();
        return emit(a, snapshot);
    }
    if (std.mem.eql(u8, args[1], "run")) return run(a, args[2], args[3], .many);
    if (std.mem.eql(u8, args[1], "single")) return run(a, args[2], args[3], .single);
    if (std.mem.eql(u8, args[1], "cancelled")) return run(a, args[2], args[3], .cancelled);
    if (std.mem.eql(u8, args[1], "finite-array")) return run(a, args[2], "{\"question\":{\"function\":\"tag\",\"question\":\"What applies?\",\"labels\":[\"billing\",\"refund\",\"shipping\"],\"threshold\":0.7},\"records\":[\"a\",\"b\",\"c\"]}", .many);
    if (std.mem.eql(u8, args[1], "held")) return held(a, args[2], false);
    if (std.mem.eql(u8, args[1], "held-token")) return held(a, args[2], true);
    return error.UnknownTestMode;
}
