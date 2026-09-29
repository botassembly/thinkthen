const std = @import("std");
const tt = @import("thinkthen");
fn expectOOM(value: anytype) !void {
    if (value) |_| return error.ExpectedOutOfMemory else |err| if (err != error.OutOfMemory) return err;
}
pub fn main() !void {
    var args = std.process.args();
    _ = args.next();
    if (args.next()) |arg| {
        if (std.mem.eql(u8, arg, "--construction-failure")) {
            var failure = std.testing.FailingAllocator.init(std.heap.page_allocator, .{ .fail_index = 0 });
            try expectOOM(tt.Engine.init(failure.allocator()));
            std.debug.print("construction failure metadata copy OOM PASS\n", .{});
            return;
        }
        if (std.mem.eql(u8, arg, "--facts-allocation")) {
            var gpa = std.heap.DebugAllocator(.{}){};
            defer if (gpa.deinit() != .ok) @panic("facts allocation leak");
            const alloc = gpa.allocator();
            var engine = switch (try tt.Engine.init(alloc)) {
                .ok => |value| value,
                .failed => |failure| {
                    defer tt.releaseFailure(alloc, failure);
                    return error.EngineBuild;
                },
            };
            defer engine.deinit();
            for (4..8) |index| {
                const text = try std.fmt.allocPrint(alloc, "alloc-facts-{d}", .{index});
                defer alloc.free(text);
                const rows = [_][]const u8{ text, text };
                var failing = std.testing.FailingAllocator.init(alloc, .{ .fail_index = index });
                engine.allocator = failing.allocator();
                const result = engine.decideMany("Is it?", &rows, .{});
                engine.allocator = alloc;
                try expectOOM(result);
                std.debug.print("facts allocation index {d}: {d} allocations PASS\n", .{ index, failing.allocations });
            }
            return;
        }
        if (std.mem.eql(u8, arg, "--construction-copy")) {
            var debug = std.heap.DebugAllocator(.{}){};
            defer if (debug.deinit() != .ok) @panic("construction message leak");
            const allocator = debug.allocator();
            switch (try tt.Engine.init(allocator)) {
                .ok => |engine| {
                    var owned = engine;
                    owned.deinit();
                    return error.ExpectedInvalidEnvironment;
                },
                .failed => |f| {
                    defer allocator.free(f.message);
                    if (f.code != tt.c.THINKTHEN_EUSAGE or f.retryable or f.message.len == 0) return error.BadConstructionFailure;
                },
            }
            std.debug.print("construction failure copied and freed PASS\n", .{});
            return;
        }
        return error.BadArgument;
    }
    var gpa = std.heap.DebugAllocator(.{}){};
    defer if (gpa.deinit() != .ok) @panic("Zig leak");
    const alloc = gpa.allocator();
    var engine = switch (try tt.Engine.init(alloc)) {
        .ok => |e| e,
        .failed => |f| {
            defer alloc.free(f.message);
            return error.EngineBuild;
        },
    };
    defer engine.deinit();
    const records = [_][]const u8{ "alloc-first", "alloc-second" };
    for (0..4) |index| {
        var failing = std.testing.FailingAllocator.init(alloc, .{ .fail_index = index });
        engine.allocator = failing.allocator();
        try expectOOM(engine.decideMany("Is it?", &records, .{}));
        engine.allocator = alloc;
        std.debug.print("bulk fail index {d}: Zig allocations {d}\n", .{ index, failing.allocations });
    }
    var failing_message = std.testing.FailingAllocator.init(alloc, .{ .fail_index = 0 });
    engine.allocator = failing_message.allocator();
    try expectOOM(engine.decide("{invalid", "text", .{}));
    engine.allocator = alloc;
    var failing_json = std.testing.FailingAllocator.init(alloc, .{ .fail_index = 0 });
    engine.allocator = failing_json.allocator();
    try expectOOM(engine.call("{\"decide\":\"Is it?\",\"evidence\":\"alloc-json\"}", .{}));
    engine.allocator = alloc;
    std.debug.print("allocation: bulk indexes 0..3, error copy and JSON copy OOM PASS\n", .{});
}
