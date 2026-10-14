const std = @import("std");
const tt = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => |failure| {
            tt.releaseFailure(allocator, failure);
            return error.NoEngine;
        },
    };
    defer engine.deinit();

    const form = "form.json";
    const report = "Steps: click Log in. Nobody gets in.";
    var call = try tt.calls.annotate(&engine, .{
        .question = .{ .file = .{ .path = form } },
        .input = .{ .text = .{ .text = report } },
    });
    defer call.deinit();

    const triage = try call.value();
    var steps = false;
    var area: []const u8 = "";
    var impact: f64 = 0;
    for (triage) |field| {
        if (std.mem.eql(u8, field.name, "steps")) {
            steps = field.value.decision.value.optional()
                orelse false;
        } else if (std.mem.eql(u8, field.name, "area")) {
            area = field.value.choice.value.optional()
                orelse "";
        } else if (std.mem.eql(u8, field.name, "impact")) {
            impact = field.value.score.value;
        }
    }
    std.debug.assert(steps);
    std.debug.assert(std.mem.eql(u8, area, "login"));
    std.debug.assert(impact == 1.98);
}
