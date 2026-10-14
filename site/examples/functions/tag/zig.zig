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

    const question = "Which labels fit this message?";
    const labels = [_][]const u8{
        "praise",
        "bug",
        "billing",
    };
    const message = "Love the new dashboard, " ++
        "but export crashes the app,\n" ++
        "and I was charged twice.\n";
    const which_labels: tt.inputs.RequestDefinition = .{
        .Authored_tag = .{
            .tag = .{ .string = question },
            .labels = .{ .array = &labels },
        },
    };
    var call = switch (try tt.calls.tag(&engine, tt.calls.Arguments(tt.inputs.RequestInputText){
        .question = .{
            .definition = .{ .value = which_labels },
        },
        .input = tt.inputs.RequestInputText{ .text = message },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const fitting_labels = call.value;
    std.debug.assert(fitting_labels.len == 3);
    for (labels, fitting_labels) |label, fitting| {
        std.debug.assert(std.mem.eql(u8, label, fitting));
    }
}
