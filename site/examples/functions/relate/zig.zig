const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var tt = switch (try thinkthen.Engine.init(allocator)) {
        .ok => |engine| engine,
        .failed => |failure| {
            thinkthen.releaseFailure(allocator, failure);
            return error.NoEngine;
        },
    };
    defer tt.deinit();

    const sings = .{
        .name = "sings",
        .source = "singer",
        .target = "song",
    };
    const spec = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .version = 1,
            .relate = .{ .relations = .{sings} },
        }, .{})},
        0,
    );
    defer allocator.free(spec);
    const names = [_][]const u8{
        \\{"name": "Paul McCartney", "kind": "singer"}
        ,
        \\{"name": "Ringo Starr", "kind": "singer"}
        ,
        \\{"name": "Yesterday", "kind": "song"}
        ,
        \\{"name": "Octopus's Garden", "kind": "song"}
        ,
    };
    const asked = try tt.relate(spec, &names, .{});
    const related = switch (asked) {
        .ok => |success| success,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer related.deinit(allocator);
    const Named = struct { name: []const u8 };
    const Edge = struct { source: Named, target: Named };
    const who_sings = try std.json.parseFromSlice(
        struct { edges: []const Edge },
        allocator,
        related.value,
        .{ .ignore_unknown_fields = true },
    );
    defer who_sings.deinit();
    const expected = [_][2][]const u8{
        .{ "Paul McCartney", "Yesterday" },
        .{ "Ringo Starr", "Octopus's Garden" },
    };
    const edges = who_sings.value.edges;
    std.debug.assert(edges.len == expected.len);
    for (edges, expected) |edge, want| {
        const singer = edge.source.name;
        const song = edge.target.name;
        std.debug.assert(std.mem.eql(u8, singer, want[0]));
        std.debug.assert(std.mem.eql(u8, song, want[1]));
    }
}
