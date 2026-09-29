const std = @import("std");
const tt = @import("thinkthen");

fn output(bytes: []const u8) void {
    _ = std.os.linux.write(1, bytes.ptr, bytes.len);
}

pub fn main() !void {
    const alloc = std.heap.page_allocator;
    const args = try std.process.argsAlloc(alloc);
    defer std.process.argsFree(alloc, args);
    if (args.len != 2) return error.OneRequestRequired;
    const request = try alloc.dupeZ(u8, args[1]);
    defer alloc.free(request);
    const constructed = try tt.Engine.init(alloc);
    var engine = switch (constructed) {
        .ok => |value| value,
        .failed => |failure| {
            defer alloc.free(failure.message);
            if (failure.facts_json) |facts| alloc.free(facts);
            return error.NativeConstructorFailed;
        },
    };
    defer engine.deinit();
    const result = try engine.call(request, .{});
    switch (result) {
        .ok => |bytes| {
            defer alloc.free(bytes);
            output(bytes);
            output("\n");
        },
        .failed => |failure| {
            defer engine.freeFailure(failure);
            const json = try std.fmt.allocPrint(alloc, "{{\"error\":\"{s}\"}}\n", .{@tagName(failure.kind)});
            defer alloc.free(json);
            output(json);
        },
    }
}
