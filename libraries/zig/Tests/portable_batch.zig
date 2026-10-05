const std = @import("std");
const tt = @import("thinkthen");

pub fn main() !void {
    const alloc = std.heap.page_allocator;
    const args = try std.process.argsAlloc(alloc);
    defer std.process.argsFree(alloc, args);
    if (args.len != 7) return error.QuestionAndFiveTextsRequired;
    const question = try alloc.dupeZ(u8, args[1]);
    defer alloc.free(question);
    const texts = try alloc.alloc([]const u8, 5);
    defer alloc.free(texts);
    for (texts, 0..) |*slot, index| slot.* = args[index + 2];
    const settings = std.posix.getenv("TT_PORTABLE_SETTINGS");
    var engine = switch (try if (settings) |value| tt.Engine.initWithSettings(alloc, value) else tt.Engine.init(alloc)) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(alloc, failure);
            return error.NativeConstructorFailed;
        },
    };
    defer engine.deinit();
    switch (try engine.decideMany(question, texts, .{})) {
        .ok => |success| {
            defer success.deinit(alloc);
            const answers = success.value;
            if (answers.len != 5) return error.WrongAnswerCount;
            for (answers) |answer| {
                if (answer.outcome != .yes or answer.probability != 0.9) return error.WrongAnswer;
            }
        },
        .failed => |failure| {
            defer engine.freeFailure(failure);
            return error.NativeDecisionFailed;
        },
    }
    std.debug.print("ZIG_PORTABLE_BATCH_PASS\n", .{});
}
