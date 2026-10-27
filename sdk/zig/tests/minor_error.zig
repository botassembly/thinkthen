const std = @import("std");
const tt = @import("thinkthen");
// Feature: call failures from a newer engine preserve unknown raw codes and messages.
pub fn main(init: std.process.Init) !void {
    const args = try init.minimal.args.toSlice(init.arena.allocator());
    var diagnostic = tt.Diagnostic{};
    if (args.len == 3 and std.mem.eql(u8, args[2], "child")) {
        var client = try tt.Client.init(.{ .backend = .stub }, &diagnostic);
        defer client.deinit();
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        defer arena.deinit();
        const questions: tt.AnnotationQuestions = .{ .entries = &.{.{ .name = "future", .question = .{ .decide = .{ .question = "Ready?" } } }} };
        const row = try client.annotateOne(arena.allocator(), questions, .{ .text = "record" }, .{}, &diagnostic);
        switch (row.outcome) {
            .answered => |answer| switch (answer.answer[0].value.decide) {
                .failed => |failure| {
                    std.debug.assert(failure.code == null);
                    std.debug.assert(std.mem.eql(u8, failure.raw_name.?, "future.problem"));
                    std.debug.assert(std.mem.eql(u8, failure.message, args[1]));
                },
                else => return error.FutureChildFailureWasIgnored,
            },
            else => return error.AnnotationWasNotAnswered,
        }
        return;
    }
    if (args.len == 3) {
        var client = try tt.Client.init(.{ .backend = .stub }, &diagnostic);
        defer client.deinit();
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        defer arena.deinit();
        const row = try client.decideOne(arena.allocator(), "Ready?", .missing, .{}, &diagnostic);
        switch (row.outcome) {
            .failed => |failure| {
                std.debug.assert(@backingInt(failure.code.?) == 2147483647);
                std.debug.assert(std.mem.eql(u8, failure.message, args[1]));
            },
            else => return error.FutureRowFailureWasIgnored,
        }
        return;
    }
    const result = tt.Client.init(.{ .backend = .stub }, &diagnostic);
    if (result) |_| return error.FutureErrorWasIgnored else |err| {
        std.debug.assert(err == error.UnknownCode);
        std.debug.assert(std.mem.eql(u8, diagnostic.rawName().?, "future.code"));
        std.debug.assert(@backingInt(diagnostic.code.?) == 2147483647);
        std.debug.assert(std.mem.eql(u8, diagnostic.message(), args[1]));
    }
}
