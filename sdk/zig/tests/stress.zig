// Feature: Zig callers share a client and own each call's allocator and rows.
const std = @import("std");
const tt = @import("thinkthen");
const fixture = @import("fixture.zig");
const builtin = @import("builtin");
extern "c" fn getenv(name: [*:0]const u8) ?[*:0]u8;
extern "c" fn system(command: [*:0]const u8) c_int;
extern "c" fn _read(fd: c_int, bytes: [*]u8, count: c_uint) c_int;
const Source = struct {
    offset: usize,
    count: usize,
    sent: usize = 0,
    buffer: [160]u8 = undefined,
    pub fn next(self: *@This()) error{RecordSource}!?tt.Record {
        if (self.sent == self.count) return null;
        const index = self.offset + self.sent;
        const json = std.fmt.bufPrint(&self.buffer, "{{\"id\":\"record-{d}\",\"body\":\"record {d}\"}}", .{ index, index }) catch return error.RecordSource;
        self.sent += 1;
        return .{ .json = json };
    }
};
const Work = struct {
    source: Source,
    call: tt.Call(.decide),
    count: usize,
    probability: f64,
    code: []const u8,
    cancel: bool,
    failed: usize = 0,
    identifier_width_mask: u64 = 0,
    offset: usize,
};
fn drive(work: *Work) void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    var received: usize = 0;
    var diag: tt.Diagnostic = .{};
    while (true) {
        const row = work.call.next(arena.allocator(), &diag) catch |err| {
            std.debug.assert(work.cancel and err == error.Cancelled and diag.code == .local_cancelled and received == 0);
            return;
        } orelse break;
        received += 1;
        std.debug.assert(row.position == received);
        work.identifier_width_mask |= @as(u64, 1) << @intCast(row.id.?.string.len);
        const id = std.fmt.allocPrint(arena.allocator(), "record-{d}", .{work.offset + received - 1}) catch @panic("allocation failed");
        std.debug.assert(std.mem.eql(u8, row.id.?.string, id));
        switch (row.outcome) {
            .answered => |answer| std.debug.assert(answer.answer and answer.probability.? == work.probability),
            .failed => |failure| {
                work.failed += 1;
                std.debug.assert(std.mem.eql(u8, failure.name(), work.code));
            },
            .unsure => @panic("unexpected unsure row"),
        }
        _ = arena.reset(.retain_capacity);
    }
    std.debug.assert(!work.cancel and received == work.count);
}
pub fn run(a: std.mem.Allocator, args: []const [:0]const u8) !void {
    const cancel = std.mem.eql(u8, args[1], "stress-cancel");
    const count = try std.fmt.parseInt(usize, args[3], 10);
    const workers = if (cancel) 1 else try std.fmt.parseInt(usize, args[7], 10);
    var config = try fixture.settings(a, args[2]);
    const raw = try std.json.parseFromSliceLeaky(std.json.Value, a, args[2], .{});
    config.api_key = tt.ApiKey.init(raw.object.get("api_key").?.string);
    var client = try tt.Client.init(config, null);
    defer client.deinit();

    const work = try a.alloc(Work, workers);
    const threads = try a.alloc(std.Thread, workers);
    var failed: usize = 0;
    var widths: u64 = 0;
    const rounds = try std.fmt.parseInt(usize, args[8], 10);
    for (0..rounds) |round| {
        for (work, threads, 0..) |*item, *thread, worker| {
            const offset = try std.fmt.parseInt(usize, args[9], 10) + (round * workers + worker) * count;
            item.* = .{ .call = undefined, .source = .{ .offset = offset, .count = count }, .count = count, .probability = try std.fmt.parseFloat(f64, args[6]), .code = args[5], .cancel = cancel, .offset = offset };
            item.call = try client.decide(std.heap.page_allocator, "Ready?", .{ .source = tt.RecordSource.from(&item.source) }, .{ .evidence = "/body" }, null);
            thread.* = try std.Thread.spawn(.{}, drive, .{item});
        }
        if (cancel) {
            var byte: [1]u8 = undefined;
            const n = if (builtin.os.tag == .windows) _read(0, &byte, 1) else std.c.read(0, &byte, 1);
            std.debug.assert(n == 1 and byte[0] == 'c');
            work[0].call.cancel();
        }
        for (work, threads) |*item, thread| {
            thread.join();
            failed += item.failed;
            widths |= item.identifier_width_mask;
            item.call.deinit();
        }
        if (getenv("THINKTHEN_STRESS_CHECKPOINT_COMMAND")) |hook| {
            std.debug.assert(system(hook) == 0);
        }
    }
    std.debug.assert(cancel or if (std.mem.eql(u8, args[4], "failed")) failed > 0 else failed == 0);
    const summary = try std.json.Stringify.valueAlloc(a, .{ .identifier_width_mask = widths, .calls = workers * rounds, .rows = if (cancel) 0 else workers * count * rounds, .failed = failed, .cancelled = cancel }, .{});
    _ = std.c.write(1, summary.ptr, summary.len);
    _ = std.c.write(1, "\n", 1);
}
