const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var tt = switch (try thinkthen.Engine.init(allocator)) {
        .ok => |engine| engine,
        .failed => |failure| {
            thinkthen.releaseFailure(allocator, failure);
            return error.NoEngine;
        },
    };
    defer tt.deinit();

    const form = try std.fs.cwd().readFileAlloc(
        allocator,
        "form.json",
        1 << 20,
    );
    defer allocator.free(form);
    const questions = try std.json.parseFromSlice(
        std.json.Value,
        allocator,
        form,
        .{},
    );
    defer questions.deinit();
    const reports = [_][]const u8{
        "Steps: click Log in. Nobody gets in.",
    };
    const annotate = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .annotate = questions.value,
            .records = reports,
        }, .{})},
        0,
    );
    defer allocator.free(annotate);
    const called = try tt.call(annotate, .{});
    const reply = switch (called) {
        .ok => |bytes| bytes,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer allocator.free(reply);
    const annotated = try std.json.parseFromSlice(
        struct { value: []const struct {
            steps: bool,
            area: []const u8,
            impact: f64,
        } },
        allocator,
        reply,
        .{ .ignore_unknown_fields = true },
    );
    defer annotated.deinit();
    const triage = annotated.value.value[0];
    std.debug.assert(triage.steps);
    std.debug.assert(std.mem.eql(u8, triage.area, "login"));
    std.debug.assert(triage.impact == 1.98);
}
