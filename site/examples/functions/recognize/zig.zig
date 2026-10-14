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

    const kinds = "kinds.json";
    const text = "Sam Patel says his Kindle " ++
        "arrived broken in Denver.";
    var call = switch (try tt.calls.recognize(&engine, .{
        .question = .{ .file = .{ .path = kinds } },
        .input = tt.inputs.RequestInputText{ .text = text },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const facts = call.value;
    var names: std.ArrayList(u8) = .empty;
    defer names.deinit(allocator);
    for (facts.entities.entities) |one| {
        try names.print(allocator, "{s} ({s}). ", .{
            one.text, one.kind,
        });
    }
    std.debug.assert(std.mem.eql(u8, names.items, "Sam Patel (person). " ++
        "Kindle (product). " ++
        "Denver (place). "));
}
