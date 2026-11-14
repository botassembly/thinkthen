const std = @import("std");
const tt = @import("thinkthen");
// Feature: call failures from a newer engine preserve unknown raw codes and messages.
pub fn main(init: std.process.Init) !void {
    const args = try init.minimal.args.toSlice(init.arena.allocator());
    var diagnostic = tt.Diagnostic{};
    if (args.len == 3 and std.mem.eql(u8, args[2], "malformed")) {
        var client = try tt.Client.init(.{ .backend = .stub }, &diagnostic);
        defer client.deinit();
        const result = client.decide(init.arena.allocator(), tt.Decide{ .question = "Ready?" }, "record", &diagnostic);
        if (result) |_| return error.MalformedRowWasIgnored else |err| {
            std.debug.assert(err == error.Setup);
            std.debug.assert(diagnostic.code == .usage_abi);
            std.debug.assert(diagnostic.message().len != 0);
        }
        return;
    }
    if (args.len == 3 and std.mem.eql(u8, args[2], "child")) {
        var client = try tt.Client.init(.{ .backend = .stub }, &diagnostic);
        defer client.deinit();
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        defer arena.deinit();
        const questions = tt.annotate(.{ .future = tt.Decide{ .question = "Ready?" } });
        const row = try client.annotate(arena.allocator(), questions, "record", &diagnostic);
        const child = (try row.answer(&diagnostic)).future;
        std.debug.assert(child.status == .failed);
        const failure = child.failure.?;
        std.debug.assert(failure.code == null);
        std.debug.assert(std.mem.eql(u8, failure.raw_name.?, "future.problem"));
        std.debug.assert(std.mem.eql(u8, failure.message, args[1]));
        return;
    }
    if (args.len == 3) {
        var client = try tt.Client.init(.{ .backend = .stub }, &diagnostic);
        defer client.deinit();
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        defer arena.deinit();
        const asks: tt.Decide = .{ .question = "Ready?" };
        var call = try client.decideAll(std.heap.page_allocator, asks, &[_]tt.Record{.{ .missing = {} }}, &diagnostic);
        defer call.deinit();
        const row = (try call.nextInto(arena.allocator(), &diagnostic)).?;
        std.debug.assert(row.status == .failed);
        const failure = row.failure.?;
        std.debug.assert(failure.code == null);
        std.debug.assert(std.mem.eql(u8, failure.raw_name.?, "future.code"));
        std.debug.assert(std.mem.eql(u8, failure.message, args[1]));
        const result = client.decide(arena.allocator(), asks, tt.Record{ .missing = {} }, &diagnostic);
        if (result) |_| return error.FailedScalarWasReturned else |err| {
            std.debug.assert(err == error.Failed);
            std.debug.assert(diagnostic.code == null);
            std.debug.assert(std.mem.eql(u8, diagnostic.rawName().?, "future.code"));
            std.debug.assert(std.mem.eql(u8, diagnostic.message(), args[1]));
        }
        return;
    }
    const result = tt.Client.init(.{ .backend = .stub }, &diagnostic);
    if (result) |_| return error.FutureErrorWasIgnored else |err| {
        std.debug.assert(err == error.Internal);
        std.debug.assert(std.mem.eql(u8, diagnostic.rawName().?, "future.code"));
        std.debug.assert(@backingInt(diagnostic.code.?) == 2147483647);
        std.debug.assert(std.mem.eql(u8, diagnostic.message(), args[1]));
    }
}
