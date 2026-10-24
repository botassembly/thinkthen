const std = @import("std");
const rows = @import("row.zig");
const diagnostic = @import("diagnostic.zig");

pub const Span = struct { text: []const u8, start: usize, end: usize };
pub const Name = struct { text: []const u8, start: usize, end: usize, kind: []const u8, p: f64 };
pub const Endpoint = struct { text: []const u8, start: usize, end: usize, kind: []const u8 };
pub const Relation = struct { source: Endpoint, target: Endpoint, relation: []const u8, p: f64, either: ?bool = null };
pub const Annotation = struct {
    status: rows.Status,
    answer: ?std.json.Value = null,
    detail: ?std.json.Value = null,
    p: ?f64 = null,
    @"error": ?diagnostic.FailureView = null,

    /// Decode a dynamic toolbox entry. The returned owner survives this row.
    pub fn decode(self: Annotation, comptime Answer: type, allocator: std.mem.Allocator) !?std.json.Parsed(Answer) {
        const value = self.answer orelse return null;
        return try std.json.parseFromValue(Answer, allocator, value, .{ .allocate = .alloc_always });
    }
};
pub const Annotations = std.json.ArrayHashMap(Annotation);

/// Owns a typed answer and the original row. Both remain valid after call release.
pub fn TypedRow(comptime Answer: type) type {
    return struct {
        raw: *const rows.Row,
        parsed: ?std.json.Parsed(Answer),
        const Self = @This();

        pub fn answer(self: Self) ?Answer {
            return if (self.parsed) |value| value.value else null;
        }
        pub fn status(self: Self) rows.Status {
            return self.raw.status();
        }
        pub fn probability(self: Self) ?f64 {
            return self.raw.probability();
        }
        pub fn position(self: Self) usize {
            return self.raw.position();
        }
        pub fn kept(self: Self) bool {
            return self.raw.kept();
        }
        pub fn idJson(self: Self) ?[]const u8 {
            return self.raw.idJson();
        }
        pub fn answerJson(self: Self) ?[]const u8 {
            return self.raw.answerJson();
        }
        pub fn detailJson(self: Self) ?[]const u8 {
            return self.raw.detailJson();
        }
        pub fn failure(self: Self) ?diagnostic.FailureView {
            return self.raw.failure();
        }
        pub fn parseDetail(self: Self, allocator: std.mem.Allocator) !?std.json.Parsed(std.json.Value) {
            return self.raw.parseDetail(allocator);
        }
        pub fn deinit(self: Self) void {
            if (self.parsed) |value| value.deinit();
            self.raw.deinit();
        }
    };
}
