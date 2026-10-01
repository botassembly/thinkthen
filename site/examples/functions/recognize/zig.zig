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

    const spec = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .version = 1,
            .recognize = .{ .kinds = .{
                .person = null,
                .organization = null,
                .place = null,
            } },
        }, .{})},
        0,
    );
    defer allocator.free(spec);
    const text = "Maria Chen joined Northwind Freight, " ++
        "a company in Chicago.";
    const asked = try tt.recognize(spec, text, .{});
    const recognized = switch (asked) {
        .ok => |success| success,
        .failed => |failure| {
            tt.freeFailure(failure);
            return error.Failed;
        },
    };
    defer recognized.deinit(allocator);
    const Entity = struct {
        text: []const u8,
        kind: []const u8,
    };
    const facts = try std.json.parseFromSlice(
        struct { entities: []const Entity },
        allocator,
        recognized.value,
        .{ .ignore_unknown_fields = true },
    );
    defer facts.deinit();
    const expected = [_]Entity{
        .{ .text = "Maria Chen", .kind = "person" },
        .{
            .text = "Northwind Freight",
            .kind = "organization",
        },
        .{ .text = "Chicago", .kind = "place" },
    };
    const names = facts.value.entities;
    std.debug.assert(names.len == expected.len);
    for (names, expected) |name, want| {
        std.debug.assert(
            std.mem.eql(u8, name.text, want.text),
        );
        std.debug.assert(
            std.mem.eql(u8, name.kind, want.kind),
        );
    }
}
