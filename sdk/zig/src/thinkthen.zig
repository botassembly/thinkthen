const std = @import("std");
const c = @import("c");
const errors = @import("diagnostic.zig");
const rows = @import("row.zig");
pub const answers = @import("answers.zig");
pub const TypedRow = answers.TypedRow;
pub const RecordSource = struct {
    context: *anyopaque,
    next: *const fn (*anyopaque) anyerror!?Record,
};
const Input = union(enum) { records: []const Record, source: RecordSource };
pub const Diagnostic = errors.Diagnostic;
pub const FailureView = errors.FailureView;
pub const CallError = errors.CallError;
pub const Row = rows.Row;
pub const Status = rows.Status;
pub const Record = union(enum) { text: []const u8, json: []const u8 };
pub const Question = union(enum) { text: []const u8, json: []const u8 };
pub const CallOptions = struct {
    json: ?[]const u8 = null,
};

const ClientStorage = struct { allocator: std.mem.Allocator, native: *c.thinkthen_client };
pub const Client = opaque {
    fn storage(self: *const Client) *ClientStorage {
        return @ptrCast(@alignCast(@constCast(self)));
    }
    pub fn init(allocator: std.mem.Allocator, settings: []const u8, diagnostic: *Diagnostic) CallError!*Client {
        diagnostic.clear();
        var native: ?*c.thinkthen_client = null;
        var err: c.thinkthen_buffer = .{};
        if (c.thinkthen_client_open(settings.ptr, settings.len, &native, &err) != c.THINKTHEN_OK)
            return diagnostic.capture(err);
        errdefer c.thinkthen_client_free(native);
        const owner = try allocator.create(ClientStorage);
        owner.* = .{ .allocator = allocator, .native = native.? };
        return @ptrCast(owner);
    }
    pub fn deinit(self: *const Client) void {
        const owner = self.storage();
        c.thinkthen_client_free(owner.native);
        owner.allocator.destroy(owner);
    }
    pub fn decide(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall(bool) {
        return .{ .raw = try self.open(allocator, "decide", question, records, options, diagnostic) };
    }
    pub fn choose(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall([]const u8) {
        return .{ .raw = try self.open(allocator, "choose", question, records, options, diagnostic) };
    }
    pub fn tag(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall([]const []const u8) {
        return .{ .raw = try self.open(allocator, "tag", question, records, options, diagnostic) };
    }
    pub fn score(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall(f64) {
        return .{ .raw = try self.open(allocator, "score", question, records, options, diagnostic) };
    }
    pub fn filter(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall(bool) {
        return .{ .raw = try self.open(allocator, "filter", question, records, options, diagnostic) };
    }
    pub fn rank(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall(usize) {
        return .{ .raw = try self.open(allocator, "rank", question, records, options, diagnostic) };
    }
    pub fn find(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall(answers.Span) {
        return .{ .raw = try self.open(allocator, "find", question, records, options, diagnostic) };
    }
    pub fn annotate(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall(answers.Annotations) {
        return .{ .raw = try self.open(allocator, "annotate", question, records, options, diagnostic) };
    }
    pub fn recognize(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall([]const answers.Name) {
        return .{ .raw = try self.open(allocator, "recognize", question, records, options, diagnostic) };
    }
    pub fn relate(self: *const Client, allocator: std.mem.Allocator, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!TypedCall([]const answers.Relation) {
        return .{ .raw = try self.open(allocator, "relate", question, records, options, diagnostic) };
    }
    fn open(self: *const Client, allocator: std.mem.Allocator, function: []const u8, question: Question, records: anytype, options: CallOptions, diagnostic: *Diagnostic) CallError!*Call {
        diagnostic.clear();
        const request = try encode(allocator, function, question, options.json);
        defer allocator.free(request);
        var native: ?*c.thinkthen_call = null;
        var err: c.thinkthen_buffer = .{};
        if (c.thinkthen_call_open(self.storage().native, request.ptr, request.len, c.THINKTHEN_UTF8, &native, &err) != c.THINKTHEN_OK)
            return diagnostic.capture(err);
        errdefer c.thinkthen_free(native);
        const owner = try allocator.create(CallStorage);
        owner.* = .{ .allocator = allocator, .native = native.?, .input = if (@TypeOf(records) == RecordSource) .{ .source = records } else .{ .records = records } };
        return @ptrCast(owner);
    }
};

const CallStorage = struct {
    allocator: std.mem.Allocator,
    native: *c.thinkthen_call,
    input: Input,
    position: usize = 0,
};
pub const Call = opaque {
    fn storage(self: *Call) *CallStorage {
        return @ptrCast(@alignCast(self));
    }
    pub fn next(self: *Call, diagnostic: *Diagnostic) anyerror!?*const Row {
        diagnostic.clear();
        const owner = self.storage();
        errdefer c.thinkthen_cancel(owner.native);
        while (true) {
            var row: c.thinkthen_row = .{};
            var err: c.thinkthen_buffer = .{};
            switch (c.thinkthen_next(owner.native, &row, &err)) {
                c.THINKTHEN_END => return null,
                c.THINKTHEN_ERROR => return diagnostic.capture(err),
                c.THINKTHEN_NEEDS_INPUT => {
                    const record: ?Record = switch (owner.input) {
                        .records => |records| if (owner.position == records.len) null else records[owner.position],
                        .source => |source| try source.next(source.context),
                    };
                    if (record) |value| {
                        const result = switch (value) {
                            .text => |text| c.thinkthen_send_text(owner.native, text.ptr, text.len, &err),
                            .json => |json| c.thinkthen_send_json(owner.native, json.ptr, json.len, &err),
                        };
                        if (result != c.THINKTHEN_OK) return diagnostic.capture(err);
                        owner.position += 1;
                    } else {
                        c.thinkthen_end_input(owner.native);
                    }
                },
                else => return try rows.take(owner.allocator, row, diagnostic),
            }
        }
    }
    pub fn cancel(self: *Call) void {
        c.thinkthen_cancel(self.storage().native);
    }
    pub fn deinit(self: *Call) void {
        const owner = self.storage();
        c.thinkthen_free(owner.native);
        owner.allocator.destroy(owner);
    }
};

/// Named calls retain the raw pull operation for dynamic callers.
pub fn TypedCall(comptime Answer: type) type {
    return struct {
        raw: *Call,
        pub const Row = TypedRow(Answer);
        const Self = @This();
        pub fn next(self: Self, diagnostic: *Diagnostic) anyerror!?TypedRow(Answer) {
            const raw = try self.raw.next(diagnostic) orelse return null;
            errdefer {
                raw.deinit();
                self.raw.cancel();
            }
            const bytes = raw.answerJson();
            const parsed = if (bytes) |value|
                if (std.mem.eql(u8, value, "null")) null else try std.json.parseFromSlice(Answer, self.raw.storage().allocator, value, .{ .allocate = .alloc_always })
            else
                null;
            return .{ .raw = raw, .parsed = parsed };
        }
        pub fn cancel(self: Self) void {
            self.raw.cancel();
        }
        pub fn deinit(self: Self) void {
            self.raw.deinit();
        }
    };
}

fn encode(allocator: std.mem.Allocator, function: []const u8, question: Question, options: ?[]const u8) error{OutOfMemory}![]u8 {
    var request: std.ArrayList(u8) = .empty;
    errdefer request.deinit(allocator);
    try request.appendSlice(allocator, "{\"function\":\"");
    try request.appendSlice(allocator, function);
    try request.appendSlice(allocator, "\",\"question\":");
    switch (question) {
        .json => |raw| try request.appendSlice(allocator, raw),
        .text => |text| {
            const tagged = try std.json.Stringify.valueAlloc(allocator, .{ .type = "inline", .value = text }, .{});
            defer allocator.free(tagged);
            try request.appendSlice(allocator, tagged);
        },
    }
    if (options) |raw| {
        try request.appendSlice(allocator, ",\"options\":");
        try request.appendSlice(allocator, raw);
    }
    try request.append(allocator, '}');
    return request.toOwnedSlice(allocator);
}
