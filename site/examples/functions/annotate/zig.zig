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
    var call = switch (try tt.calls.annotate(&engine, .{
        .question = .{ .file = .{ .path = form } },
        .input = tt.inputs.RequestInputText{ .text = report },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const triage = call.value;
    var steps = false;
    var area: []const u8 = "";
    var impact: f64 = 0;
    for (triage.entries) |field| {
        if (std.mem.eql(u8, field.key, "steps")) {
            steps = switch (field.value) {
                .boolean => |v| v,
                else => false,
            };
        } else if (std.mem.eql(u8, field.key, "area")) {
            area = switch (field.value) {
                .string => |v| v,
                else => "",
            };
        } else if (std.mem.eql(u8, field.key, "impact")) {
            impact = field.value.number;
        }
    }
    std.debug.assert(steps);
    std.debug.assert(std.mem.eql(u8, area, "login"));
    std.debug.assert(impact == 1.98);
}
