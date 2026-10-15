// Generated from the canonical Rust schema; do not edit.
const std = @import("std");
/// Ordered authored maps retain caller member ordering.
pub fn Map(comptime T: type) type {
    return struct {
        entries: []const struct { key: []const u8, value: T },
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
