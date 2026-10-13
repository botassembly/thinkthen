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
    while (true) switch (try call.next()) {
        .pending => return error.BlockingPending,
        .end => break,
        .packet => |value| {
            var packet = value;
            defer packet.deinit();
            if (packet.failure()) |_| return error.DecisionFailed;
            if (packet.value().* == .decide_row) {
                const node = switch (packet.value().decide_row.value.value) {
                    .value => |v| v,
                    else => return error.MissingAnswer,
                };
                const answer = if (node == .null_value) "null" else if (node == .boolean) (if (node.boolean) "true" else "false") else return error.WrongAnswer;
                var out = std.fs.File.stdout().writerStreaming(&.{});
                try out.interface.print("smoke: {s}\n", .{answer});
            }
        },
    };
}
