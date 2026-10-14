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

    const question = "Is this urgent?";
    const inbox = [_][]const u8{
        "Newsletter: our autumn catalog is here. " ++
            "No reply needed.",
        "Our checkout page is down and " ++
            "customers cannot pay",
        "Reminder: your invoice is due in 30 days",
        "Please send the signed quote by 5 pm today",
    };
    var records: [4]tt.inputs.RequestItem = undefined;
    for (inbox, &records) |text, *record| {
        record.* = .{
            .original = .{ .text = .{ .text = text } },
        };
    }
    var call = switch (try tt.calls.rank(&engine, tt.calls.Arguments(tt.inputs.RequestInputRecords){
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

    const by_urgency = call.value;
    const expected = [_]u64{ 1, 3, 2, 0 };
    std.debug.assert(
        std.mem.eql(u64, by_urgency, &expected),
    );
}
