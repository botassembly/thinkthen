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
    try emit(a, .{ .@"error" = diagnostic.code.? });
}
fn runNamed(comptime function: tt.Function, client: *const tt.Client, a: A, value: std.json.Value, records: []const tt.Record) !void {
    var diagnostic: tt.Diagnostic = .{};
    var q = try fixture.question(function, a, value);
    var call = client.open(function, a, q, .{ .records = records }, &diagnostic) catch {
        return fail(a, diagnostic);
    };
    defer call.deinit();
    q = undefined;
    write("[");
    var count: usize = 0;
    while (try call.next(a, &diagnostic)) |item| {
        if (count > 0) write(",");
        const row = if (function == .filter) item.row else item;
        try emit(a, row);
        count += 1;
    }
    write("]");
}
fn run(a: A, settings_json: []const u8, request_json: []const u8) !void {
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
        if (std.mem.eql(u8, name, tag)) return runNamed(@field(tt.Function, tag), &client, a, question, records);
    }
    return error.UnknownFixture;
}
fn driveHeld(call: *tt.Call(.decide), a: A) void {
    var diag: tt.Diagnostic = .{};
    _ = call.next(a, &diag) catch |err| {
        std.debug.assert(err == error.Cancelled and diag.code == .local_cancelled);
        return;
    };
    @panic("held request returned before cancellation");
}
fn held(a: A, settings_json: []const u8) !void {
    const settings = try fixture.settings(a, settings_json);
    var client = try tt.Client.init(settings, null);
    defer client.deinit();
    var call = try client.decide(a, "Ready?", .{ .records = &.{.{ .text = "evidence" }} }, .{}, null);
    defer call.deinit();
    const worker = try std.Thread.spawn(.{}, driveHeld, .{ &call, a });
    var signal: [1]u8 = undefined;
    const count = if (builtin.os.tag == .windows) _read(0, &signal, 1) else std.c.read(0, &signal, 1);
    std.debug.assert(count == 1);
    call.cancel();
    worker.join();
}
pub fn main(init: std.process.Init) !void {
    const a = init.arena.allocator();
    const args = try init.minimal.args.toSlice(a);
    if (builtin.os.tag != .windows and std.mem.startsWith(u8, args[1], "stress")) return @import("stress.zig").run(a, args);
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
    if (std.mem.eql(u8, args[1], "run")) return run(a, args[2], args[3]);
    if (std.mem.eql(u8, args[1], "ownership")) return;
    if (std.mem.eql(u8, args[1], "finite-array")) return run(a, args[2], "{\"question\":{\"function\":\"tag\",\"question\":\"What applies?\",\"labels\":[\"billing\",\"refund\",\"shipping\"],\"threshold\":0.7},\"records\":[\"a\",\"b\",\"c\"]}");
    if (std.mem.eql(u8, args[1], "held")) return held(a, args[2]);
    return error.UnknownTestMode;
}
