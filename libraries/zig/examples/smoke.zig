const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    const a = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(a)) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            return error.EngineBuild;
        },
    };
    defer engine.deinit();
    const question = std.posix.getenv("THINKTHEN_TEST_SMOKE_QUESTION") orelse return error.NoQuestion;
    const text = std.posix.getenv("THINKTHEN_TEST_SMOKE_TEXT") orelse return error.NoText;
    var call = try tt.session.decide(&engine, .{ .question = .{ .text = .{ .text = question } }, .input = .{ .text = .{ .text = text } } });
    defer call.deinit();
    while (true) switch (try call.read()) {
        .pending => std.Thread.yield() catch {},
        .end => break,
        .packet => |value| {
            var packet = value;
            defer packet.deinit();
            if (packet.failure()) |_| return error.DecisionFailed;
            if (packet.view.kind == tt.c.THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1) {
                const result = packet.view.data.decide_row[0].value[0].value;
                const node = (tt.session.optional(result) orelse return error.MissingAnswer)[0].value[0];
                const answer = if (node.kind == tt.c.THINKTHEN_COMPLETE_JSON_NULL_V1) "null" else if (node.kind == tt.c.THINKTHEN_COMPLETE_JSON_BOOLEAN_V1) (if (node.data.boolean == 1) "true" else "false") else return error.WrongAnswer;
                var out = std.fs.File.stdout().writer(&.{});
                try out.interface.print("smoke: {s}\n", .{answer});
            }
        },
    };
}
