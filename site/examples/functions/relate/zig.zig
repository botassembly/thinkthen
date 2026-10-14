const std = @import("std");
const tt = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => |failure| {
            tt.releaseFailure(allocator, failure);
            return error.NoEngine;
        },
    };
    defer engine.deinit();

    const relations = "makes.json";
    const names = [_][2][]const u8{
        .{ "Apple", "company" },
        .{ "Amazon", "company" },
        .{ "iPhone", "product" },
        .{ "Kindle", "product" },
    };
    var entities: [4]tt.inputs.RequestItem = undefined;
    for (names, &entities) |name, *entity| {
        var fields = std.json.ObjectMap.init(allocator);
        try fields.put("name", .{ .string = name[0] });
        try fields.put("kind", .{ .string = name[1] });
        entity.* = .{ .original = .{ .json = .{
            .value = .{ .object = fields },
        } } };
    }
    var call = switch (try tt.calls.relate(&engine, tt.calls.Arguments(tt.inputs.RequestInputEntities){
        .question = .{ .file = .{ .path = relations } },
        .input = tt.inputs.RequestInputEntities{ .items = &entities },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const edges = call.value;
    var who_makes: std.ArrayList(u8) = .empty;
    defer who_makes.deinit(allocator);
    for (edges) |edge| {
        try who_makes.print(allocator, "{s} makes {s}. ", .{
            edge.source.name, edge.target.name,
        });
    }
    std.debug.assert(std.mem.eql(u8, who_makes.items, "Apple makes iPhone. " ++
        "Amazon makes Kindle. "));
}
