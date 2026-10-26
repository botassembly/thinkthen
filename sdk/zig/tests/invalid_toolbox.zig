const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    var client = try tt.Client.init(.{ .backend = .stub }, null);
    defer client.deinit();
    const toolbox: tt.Toolbox = .{ .entries = &.{} };
    var call = try client.annotate(std.heap.page_allocator, toolbox, .{ .records = &.{} }, .{}, null);
    defer call.deinit();
}
