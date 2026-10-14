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

    const question = "Is this a complaint?";
    const reviews = [_][]const u8{
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
        "Does this come in blue?",
        "The strap snapped on day two.",
    };
    var records: [4]tt.inputs.RequestItem = undefined;
    for (reviews, &records) |text, *record| {
        record.* = .{
            .original = .{ .text = .{ .text = text } },
        };
    }
    var call = switch (try tt.calls.filter(&engine, tt.calls.Arguments(tt.inputs.RequestInputRecords){
        .question = .{ .text = .{ .text = question } },
        .input = tt.inputs.RequestInputRecords{ .items = &records },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const complaints = call.value;
    std.debug.assert(complaints.len == 2);
    std.debug.assert(
        std.mem.eql(u8, complaints[0].string, reviews[1]),
    );
    std.debug.assert(
        std.mem.eql(u8, complaints[1].string, reviews[3]),
    );
}
