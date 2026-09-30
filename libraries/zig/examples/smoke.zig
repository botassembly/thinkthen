// The replay smoke (ticket 0335): one decide through Engine.init, which reads
// the environment, with the question and text sdlc/scripts/smoke names.
const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => return error.EngineBuild,
    };
    defer engine.deinit();
    const question = std.posix.getenv("THINKTHEN_SMOKE_QUESTION") orelse return error.NoQuestion;
    const text = std.posix.getenv("THINKTHEN_SMOKE_TEXT") orelse return error.NoText;
    switch (try engine.decide(question, text, .{})) {
        .ok => |answer| {
            defer answer.deinit(allocator);
            const value = switch (answer.value.outcome) {
                .yes => "true",
                .no => "false",
                .unsure => "null",
            };
            var out = std.fs.File.stdout().writer(&.{});
            try out.interface.print("smoke: {s}\n", .{value});
        },
        .failed => return error.DecisionFailed,
    }
}
