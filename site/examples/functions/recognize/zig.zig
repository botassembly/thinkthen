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

    const person = "Part of a person's name.";
    const org = "Part of the name of an organization: " ++
        "a company, band, team, agency, government " ++
        "body, or media outlet.";
    const place = "Part of the name of a place: " ++
        "a country, region, city, or geographic feature.";
    const other = "Part of another named entity: a " ++
        "nationality, an event, a product, or the " ++
        "name of a creative work.";
    const spec = try std.fmt.allocPrintSentinel(
        allocator,
        "{f}",
        .{std.json.fmt(.{
            .version = 1,
            .recognize = .{ .kinds = .{
                .PER = person,
                .ORG = org,
                .LOC = place,
                .MISC = other,
            } },
        }, .{})},
        0,
    );
    defer allocator.free(spec);
    const text = "Maria Chen joined Northwind Freight " ++
        "in Chicago last spring.";
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
        .{ .text = "Maria Chen", .kind = "PER" },
        .{ .text = "Northwind Freight", .kind = "ORG" },
        .{ .text = "Chicago", .kind = "LOC" },
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
