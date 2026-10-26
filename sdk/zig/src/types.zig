const std = @import("std");
pub const Backend = enum { systemone, openai, stub };
pub const Function = enum { decide, choose, tag, score, filter, rank, find, recognize, relate, annotate };
pub const Threshold = union(enum) {
    cut: f64,
    bounds: struct { low: f64, high: f64 },
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Threshold {
        if (value == .object) return .{ .bounds = try std.json.innerParseFromValue(@FieldType(Threshold, "bounds"), allocator, value, options) };
        return .{ .cut = try std.json.innerParseFromValue(f64, allocator, value, options) };
    }
    pub fn band(low: f64, high: f64) Threshold {
        return .{ .bounds = .{ .low = low, .high = high } };
    }
    pub fn jsonStringify(self: Threshold, writer: anytype) !void {
        switch (self) {
            .cut => |value| try writer.write(value),
            .bounds => |value| try writer.write(value),
        }
    }
};
pub const ApiKey = struct {
    pointer: *const anyopaque,
    length: usize,
    pub fn init(bytes: []const u8) ApiKey {
        return .{ .pointer = bytes.ptr, .length = bytes.len };
    }
    pub fn format(_: ApiKey, writer: *std.Io.Writer) std.Io.Writer.Error!void {
        try writer.writeAll("[redacted]");
    }
    pub fn jsonStringify(_: ApiKey, writer: anytype) !void {
        try writer.write("[redacted]");
    }
};
pub const Retry = struct { max_retries: ?u32 = null, initial_backoff_ms: ?u64 = null, attempt_timeout_ms: ?u64 = null };
pub const Settings = struct {
    backend: Backend = .systemone,
    model: ?[]const u8 = null,
    url: ?[]const u8 = null,
    api_key: ?ApiKey = null,
    batch: ?u32 = null,
    retry: ?Retry = null,
    pub fn jsonStringify(self: Settings, writer: anytype) !void {
        try writer.write(.{ .backend = self.backend, .model = self.model, .url = self.url, .batch = self.batch, .retry = self.retry });
    }
    pub fn format(_: Settings, writer: *std.Io.Writer) std.Io.Writer.Error!void {
        try writer.writeAll("Settings(api_key=[redacted])");
    }
};
pub const Record = union(enum) {
    text: []const u8,
    json: []const u8,
    missing: void,
    pub fn fromValue(allocator: std.mem.Allocator, value: anytype) error{OutOfMemory}!Record {
        return .{ .json = try std.json.Stringify.valueAlloc(allocator, value, .{}) };
    }
    pub fn copy(self: Record, allocator: std.mem.Allocator) error{OutOfMemory}!Record {
        return switch (self) {
            .text => |v| .{ .text = try allocator.dupe(u8, v) },
            .json => |v| .{ .json = try allocator.dupe(u8, v) },
            .missing => .missing,
        };
    }
    pub fn free(self: Record, allocator: std.mem.Allocator) void {
        switch (self) {
            .text, .json => |v| allocator.free(v),
            .missing => {},
        }
    }
};
pub const RecordSource = struct {
    context: *anyopaque,
    nextFn: *const fn (*anyopaque) error{RecordSource}!?Record,
    pub fn from(ptr: anytype) RecordSource {
        return .{ .context = ptr, .nextFn = struct {
            fn next(context: *anyopaque) error{RecordSource}!?Record {
                const source: @TypeOf(ptr) = @ptrCast(@alignCast(context));
                return source.next();
            }
        }.next };
    }
};
pub const Input = union(enum) { records: []const Record, source: RecordSource };
pub const Choice = struct {
    name: []const u8,
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Choice {
        return .{ .name = try std.json.innerParseFromValue([]const u8, allocator, value, options) };
    }
    pub fn jsonStringify(self: Choice, writer: anytype) !void {
        try writer.write(self.name);
    }
};
pub const RelationRule = struct {
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!RelationRule {
        const Wire = struct { relation: []const u8, source: []const u8, target: []const u8, reads: ?[]const u8 = null, either: ?bool = null };
        const wire = try std.json.innerParseFromValue(Wire, allocator, value, options);
        return .{ .name = wire.relation, .source = wire.source, .target = wire.target, .reads = wire.reads, .either = wire.either };
    }
    name: []const u8,
    source: []const u8,
    target: []const u8,
    reads: ?[]const u8 = null,
    either: ?bool = null,
    pub fn jsonStringify(self: RelationRule, writer: anytype) !void {
        try writer.write(.{ .relation = self.name, .source = self.source, .target = self.target, .reads = self.reads, .either = self.either });
    }
};
