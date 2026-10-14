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
    var call = try tt.session.startNamed(&engine, .decide, .{ .question = .{ .text = .{ .text = question } }, .input = .{ .text = .{ .text = text } } });
    defer call.deinit();
    while (true) switch (try call.next()) {
        .pending => return error.BlockingPending,
        .end => break,
        .packet => |value| {
            var packet = value;
            defer packet.deinit();
            if (packet.failure()) |_| return error.DecisionFailed;
            if (packet.value().* == .observation) {
                const value_answer = packet.answers().decide() catch |err| {
                    if (err == error.WrongAnswer) continue;
                    return err;
                };
                const answer = if (value_answer) |value_bool| (if (value_bool) "true" else "false") else "null";
                var out = std.fs.File.stdout().writerStreaming(&.{});
                try out.interface.print("smoke: {s}\n", .{answer});
            }
        },
    };
}
