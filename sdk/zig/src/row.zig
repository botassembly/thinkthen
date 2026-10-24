const std = @import("std");
const c = @import("c");
const diagnostic = @import("diagnostic.zig");
const CallError = diagnostic.CallError;
const FailureView = diagnostic.FailureView;
const Diagnostic = diagnostic.Diagnostic;

pub const Status = enum { answered, unsure, failed };
const Columns = struct {
    status: Status,
    p: ?f64 = null,
    id: ?[]const u8 = null,
    answer: ?[]const u8 = null,
    @"error": ?[]const u8 = null,
};
const Storage = struct {
    allocator: std.mem.Allocator,
    native: c.thinkthen_row,
    columns: std.json.Parsed(Columns),
    failure: ?std.json.Parsed(FailureView),
};

/// An immutable row owner. Its accessors borrow until deinit, independently of a call.
pub const Row = opaque {
    fn storage(self: *const Row) *Storage {
        return @ptrCast(@alignCast(@constCast(self)));
    }

    pub fn status(self: *const Row) Status {
        return self.storage().columns.value.status;
    }
    pub fn probability(self: *const Row) ?f64 {
        return self.storage().columns.value.p;
    }
    pub fn position(self: *const Row) usize {
        return self.storage().native.position;
    }
    pub fn kept(self: *const Row) bool {
        return self.storage().native.kept != 0;
    }
    pub fn idJson(self: *const Row) ?[]const u8 {
        return self.storage().columns.value.id;
    }
    pub fn answerJson(self: *const Row) ?[]const u8 {
        return self.storage().columns.value.answer;
    }
    pub fn detailJson(self: *const Row) ?[]const u8 {
        const buffer = self.storage().native.detail;
        return if (buffer.len == 0) null else diagnostic.bytes(buffer);
    }
    pub fn failure(self: *const Row) ?FailureView {
        return if (self.storage().failure) |value| value.value else null;
    }
    pub fn parseDetail(self: *const Row, allocator: std.mem.Allocator) std.json.ParseError(std.json.Scanner)!?std.json.Parsed(std.json.Value) {
        const raw = self.detailJson() orelse return null;
        return try std.json.parseFromSlice(std.json.Value, allocator, raw, .{ .allocate = .alloc_always, .parse_numbers = false });
    }
    pub fn deinit(self: *const Row) void {
        const owner = self.storage();
        if (owner.failure) |value| value.deinit();
        owner.columns.deinit();
        c.thinkthen_row_free(&owner.native);
        owner.allocator.destroy(owner);
    }
};

/// Takes the ABI row on every path, including allocation failure.
pub fn take(allocator: std.mem.Allocator, native: c.thinkthen_row, failure: *Diagnostic) CallError!*const Row {
    var owned = native;
    errdefer c.thinkthen_row_free(&owned);
    var columns: c.thinkthen_buffer = .{};
    var err: c.thinkthen_buffer = .{};
    if (c.thinkthen_row_columns(&owned, &columns, &err) != c.THINKTHEN_OK)
        return failure.capture(err);
    defer c.thinkthen_buffer_free(&columns);
    const parsed = try std.json.parseFromSlice(Columns, allocator, diagnostic.bytes(columns), .{ .allocate = .alloc_always });
    errdefer parsed.deinit();
    const parsed_failure = if (parsed.value.@"error") |raw|
        try std.json.parseFromSlice(FailureView, allocator, raw, .{ .allocate = .alloc_always })
    else
        null;
    errdefer if (parsed_failure) |value| value.deinit();
    const owner = try allocator.create(Storage);
    owner.* = .{ .allocator = allocator, .native = owned, .columns = parsed, .failure = parsed_failure };
    return @ptrCast(owner);
}
