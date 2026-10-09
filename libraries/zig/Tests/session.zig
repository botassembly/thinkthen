const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    var allocator = std.heap.DebugAllocator(.{}){};
    defer std.debug.assert(allocator.deinit() == .ok);
    var engine = switch (try tt.Engine.initWithSettings(allocator.allocator(), "{\"cache\":false,\"max_retries\":0}")) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(allocator.allocator(), failure);
            return error.EngineBuild;
        },
    };
    var session = try tt.session.decide(&engine, .{ .kind = "text", .text = "Does it pass?" }, .{ .kind = "text", .text = "Evidence." });
    engine.deinit();
    defer session.deinit();
    var terminal = false;
    var packets: usize = 0;
    while (true) {
        switch (try session.read()) {
            .pending => std.Thread.yield() catch {},
            .end => break,
            .packet => |value| {
                var packet = value;
                defer packet.deinit();
                packets += 1;
                if (packet.terminal()) |_| {
                    terminal = true;
                    if (packet.failure() != null) return error.NativeFailure;
                    const facts = packet.facts() orelse return error.MissingFacts;
                    if (facts.records != 1 or facts.requests_sent != 1) return error.WrongFacts;
                }
            },
        }
    }
    if (!terminal or packets == 0) return error.MissingTerminal;
    std.debug.print("installed Zig session PASS\n", .{});
}
