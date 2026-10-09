// Generated from the canonical Rust schema; do not edit.
const std = @import("std");
/// Ordered authored maps preserve member ordering during transport.
pub fn Map(comptime T: type) type {
    return struct {
        entries: []const struct { key: []const u8, value: T },
        pub fn jsonStringify(self: @This(), writer: anytype) !void {
            try writer.beginObject();
            for (self.entries) |entry| {
                try writer.objectField(entry.key);
                try writer.write(entry.value);
            }
            try writer.endObject();
        }
    };
}

pub const Plan = struct {
    estimated_bytes: u64,

    estimated_input_tokens: TokenBand,

    first_body_utf8: ?[]const u8,

    largest_request_bytes: u64,

    largest_request_estimated_input_tokens: u64,

    records: u64,

    requests: u64,

    token_estimate_method: []const u8,

    upper_bound: bool,
};

pub const TokenBand = struct {
    lower: u64,

    upper: u64,
};
