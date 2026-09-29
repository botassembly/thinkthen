const std = @import("std");
const tt = @import("thinkthen");

fn opened(result: tt.Result(tt.Engine), alloc: std.mem.Allocator) !tt.Engine {
    return switch (result) {
        .ok => |engine| engine,
        .failed => |failure| {
            defer tt.releaseFailure(alloc, failure);
            return error.NativeConstructorFailed;
        },
    };
}

fn yes(engine: *tt.Engine, text: [:0]const u8) !void {
    switch (try engine.decide("Is it?", text, .{})) {
        .ok => |answer| {
            defer answer.deinit(engine.allocator);
            if (answer.value.outcome != .yes) return error.WrongAnswer;
        },
        .failed => |failure| {
            defer engine.freeFailure(failure);
            return error.DecisionFailed;
        },
    }
}

pub fn main() !void {
    const alloc = std.heap.page_allocator;
    const args = try std.process.argsAlloc(alloc);
    defer std.process.argsFree(alloc, args);
    if (args.len != 3) return error.ConfiguredUrlAndCacheRequired;
    const settings = try std.fmt.allocPrint(alloc, "{{\"base_url\":\"{s}\",\"cache\":\"{s}\"}}", .{ args[1], args[2] });
    defer alloc.free(settings);
    const terminated = try alloc.dupeZ(u8, settings);
    defer alloc.free(terminated);
    var configured = try opened(try tt.Engine.initWithSettings(alloc, terminated), alloc);
    defer configured.deinit();
    var environment = try opened(try tt.Engine.init(alloc), alloc);
    defer environment.deinit();
    var empty = try opened(try tt.Engine.initWithSettings(alloc, "{}"), alloc);
    defer empty.deinit();
    for ([_][:0]const u8{ "{\"nope\":1}", "{\"timeout\":\"30\"}" }) |invalid| {
        switch (try tt.Engine.initWithSettings(alloc, invalid)) {
            .ok => |engine| {
                var unexpected = engine;
                unexpected.deinit();
                return error.InvalidSettingsAccepted;
            },
            .failed => |failure| {
                defer tt.releaseFailure(alloc, failure);
                if (failure.kind != .usage or failure.retryable or failure.facts_json != null) return error.WrongSettingsFailure;
            },
        }
    }
    try yes(&configured, "zig-settings");
    try yes(&environment, "zig-env");
    try yes(&empty, "zig-empty");
    std.debug.print("ZIG_SETTINGS_PASS\n", .{});
}
