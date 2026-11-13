const std = @import("std");
/// The service that answers questions. `stub` needs no key or network and returns synthetic answers.
pub const Backend = enum { systemone, openai, stub };
/// The ten question functions.
pub const Function = enum { decide, choose, tag, score, filter, rank, find, recognize, relate, annotate };
/// When decide and filter answer: one cut, or a band whose middle is not sure.
pub const Threshold = union(enum) {
    /// Answer yes at or above this probability.
    cut: f64,
    /// Not sure between `low` and `high`.
    bounds: struct { low: f64, high: f64 },
    /// Read a parsed JSON value, for `std.json.parseFromValue`.
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Threshold {
        if (value == .object) return .{ .bounds = try std.json.innerParseFromValue(@FieldType(Threshold, "bounds"), allocator, value, options) };
        return .{ .cut = try std.json.innerParseFromValue(f64, allocator, value, options) };
    }
    /// A band: no below `low`, yes above `high`, not sure between.
    pub fn band(low: f64, high: f64) Threshold {
        return .{ .bounds = .{ .low = low, .high = high } };
    }
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: Threshold, writer: anytype) !void {
        switch (self) {
            .cut => |value| try writer.write(value),
            .bounds => |value| try writer.write(value),
        }
    }
};
/// A backend key. It keeps no byte slice, so formatting and JSON print `[redacted]`.
pub const ApiKey = struct {
    pointer: *const anyopaque,
    length: usize,
    /// Borrow `bytes`, which must outlive the Client.init call.
    pub fn init(bytes: []const u8) ApiKey {
        return .{ .pointer = bytes.ptr, .length = bytes.len };
    }
    /// Print `[redacted]` in place of the key.
    pub fn format(_: ApiKey, writer: *std.Io.Writer) std.Io.Writer.Error!void {
        try writer.writeAll("[redacted]");
    }
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(_: ApiKey, writer: anytype) !void {
        try writer.write("[redacted]");
    }
};
/// Retry limits for backend requests. Null keeps the engine default.
pub const Retry = struct { max_retries: ?u32 = null, initial_backoff_ms: ?u64 = null, attempt_timeout_ms: ?u64 = null };
/// Client settings. Null fields keep the engine default.
pub const Settings = struct {
    /// The backend that answers. Null reads `THINKTHEN_BACKEND`, as the command does,
    /// and uses System One when that is unset too.
    backend: ?Backend = null,
    /// The backend model name.
    model: ?[]const u8 = null,
    /// The backend base URL.
    url: ?[]const u8 = null,
    /// The key; null reads the backend's key variable from the environment.
    api_key: ?ApiKey = null,
    /// Records per backend request.
    batch: ?u32 = null,
    /// Client send allowance; null is unlimited and zero forbids sends.
    max_requests: ?u64 = null,
    /// Retry limits.
    retry: ?Retry = null,
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: Settings, writer: anytype) !void {
        try writer.write(.{ .backend = self.backend, .model = self.model, .url = self.url, .batch = self.batch, .max_requests = self.max_requests, .retry = self.retry });
    }
    /// Print `[redacted]` in place of the key.
    pub fn format(_: Settings, writer: *std.Io.Writer) std.Io.Writer.Error!void {
        try writer.writeAll("Settings(api_key=[redacted])");
    }
};
/// One record: text, a JSON value, or a missing record, which fails as `record.missing`.
pub const Record = union(enum) {
    /// Plain text.
    text: []const u8,
    /// One JSON value; the question's `field` pointer selects the text to judge.
    json: []const u8,
    /// No record; it gives a failed row.
    missing: void,
    /// A JSON record from any value `std.json` can write. The caller frees `.json` with allocator.
    pub fn fromValue(allocator: std.mem.Allocator, value: anytype) error{OutOfMemory}!Record {
        return .{ .json = try std.json.Stringify.valueAlloc(allocator, value, .{}) };
    }
    /// A copy whose bytes belong to allocator.
    pub fn copy(self: Record, allocator: std.mem.Allocator) error{OutOfMemory}!Record {
        return switch (self) {
            .text => |v| .{ .text = try allocator.dupe(u8, v) },
            .json => |v| .{ .json = try allocator.dupe(u8, v) },
            .missing => .missing,
        };
    }
    /// Free a copy made with the same allocator.
    pub fn free(self: Record, allocator: std.mem.Allocator) void {
        switch (self) {
            .text, .json => |v| allocator.free(v),
            .missing => {},
        }
    }
};
/// Records pulled one at a time, for input too large to hold at once.
pub const RecordSource = struct {
    context: *anyopaque,
    nextFn: *const fn (*anyopaque) error{RecordSource}!?Record,
    /// Wrap a pointer to any value with `fn next(self) error{RecordSource}!?Record`.
    /// Each record's bytes must stay valid until the next call to its `next`.
    pub fn from(ptr: anytype) RecordSource {
        return .{ .context = ptr, .nextFn = struct {
            fn next(context: *anyopaque) error{RecordSource}!?Record {
                const source: @TypeOf(ptr) = @ptrCast(@alignCast(context));
                return source.next();
            }
        }.next };
    }
};
/// The records of a many-record call. An empty collection yields no rows and makes no request.
pub const Input = union(enum) {
    /// Texts, one record each.
    texts: []const []const u8,
    /// JSON Lines: one JSON record per line. Lines of only whitespace are skipped.
    json_lines: []const u8,
    /// Records of any kind.
    records: []const Record,
    /// Records pulled from a source while rows are read.
    source: RecordSource,
};
/// One label, level or kind.
pub const Choice = struct {
    /// The label.
    name: []const u8,
    /// Read a parsed JSON value, for `std.json.parseFromValue`.
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!Choice {
        return .{ .name = try std.json.innerParseFromValue([]const u8, allocator, if (value == .object) value.object.get("name") orelse return error.MissingField else value, options) };
    }
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: Choice, writer: anytype) !void {
        try writer.write(self.name);
    }
};
/// One relation to find between recognized names. It is written `relation` in saved JSON.
pub const RelationRule = struct {
    /// Read a parsed JSON value, for `std.json.parseFromValue`.
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!RelationRule {
        const Wire = struct { relation: []const u8, source: []const u8, target: []const u8, reads: ?[]const u8 = null, either: ?bool = null };
        const wire = try std.json.innerParseFromValue(Wire, allocator, value, options);
        return .{ .name = wire.relation, .source = wire.source, .target = wire.target, .reads = wire.reads, .either = wire.either };
    }
    /// The relation's name.
    name: []const u8,
    /// The kind of the source name.
    source: []const u8,
    /// The kind of the target name.
    target: []const u8,
    /// How the relation reads in a sentence, such as "pays money to".
    reads: ?[]const u8 = null,
    /// True when the relation holds in either direction.
    either: ?bool = null,
    /// Write as JSON, for `std.json.Stringify`.
    pub fn jsonStringify(self: RelationRule, writer: anytype) !void {
        try writer.write(.{ .relation = self.name, .source = self.source, .target = self.target, .reads = self.reads, .either = self.either });
    }
};

/// Choices from label names, built at compile time: `tt.choices(&.{ "billing", "shipping" })`.
pub fn choices(comptime names: []const []const u8) []const Choice {
    const result = comptime blk: {
        var list: [names.len]Choice = undefined;
        for (names, &list) |name, *choice| choice.* = .{ .name = name };
        break :blk list;
    };
    return &result;
}
