const std = @import("std");
const thinkthen = @import("thinkthen");

fn decide(
    tt: *thinkthen.Engine,
    question: [:0]const u8,
    text: []const u8,
) !thinkthen.Outcome {
    const asked = try tt.decide(question, text, .{});
    const answer = switch (asked) {
        .ok => |answer| answer,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer answer.deinit(std.heap.page_allocator);
    return answer.value.outcome;
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const settings = [_][:0]const u8{
        "{\"backend\":\"typesafe\"}",
        "{\"backend\":\"liquid\"}",
        "{\"backend\":\"ollama\",\"base_url\":\"http://localhost:11535/v1\"}",
    };
    for (settings) |setting| {
        var tt = switch (try thinkthen.Engine.initWithSettings(allocator, setting)) {
            .ok => |engine| engine,
            .failed => |failure| {
                thinkthen.releaseFailure(allocator, failure);
                return error.NoEngine;
            },
        };
        defer tt.deinit();

        const question = "Does the customer ask for a refund?";
        const broken =
            "Please refund my order. It arrived broken.";
        const thanks = "Thanks for the quick help yesterday!";
        const broken_is_refund =
            try decide(&tt, question, broken);
        const thanks_is_refund =
            try decide(&tt, question, thanks);
        std.debug.assert(broken_is_refund == .yes);
        std.debug.assert(thanks_is_refund == .no);
    }
}
