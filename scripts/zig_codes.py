"""Generate the Zig error enum from the engine registry into package output."""
import json
from pathlib import Path


def generate(root, destination):
    registry = json.loads((root / 'schemas/contract.json').read_text())
    codes = registry['codes']
    def tag(code):
        return code['name'].replace('.', '_').replace('-', '_')
    lines = ['const std = @import("std");', 'pub const ErrorCode = enum(i32) {']
    lines.extend(f'    {tag(code)} = {code["value"]},' for code in codes)
    lines += ['    _,', '    pub fn name(self: ErrorCode) []const u8 { return switch (self) {']
    lines.extend(f'        .{tag(code)} => "{code["name"]}",' for code in codes)
    lines += ['        _ => "unknown",', '    }; }', '''    pub fn jsonStringify(self: ErrorCode, writer: anytype) !void { try writer.write(self.name()); }
    pub fn jsonParse(allocator: std.mem.Allocator, source: anytype, options: std.json.ParseOptions) std.json.ParseError(@TypeOf(source.*))!ErrorCode {
        const value = try std.json.innerParse([]const u8, allocator, source, options);
        return fromName(value);
    }
    pub fn jsonParseFromValue(allocator: std.mem.Allocator, value: std.json.Value, options: std.json.ParseOptions) std.json.ParseFromValueError!ErrorCode {
        return fromName(try std.json.innerParseFromValue([]const u8, allocator, value, options));
    }
    pub fn fromName(value: []const u8) error{InvalidEnumTag}!ErrorCode {
        inline for (@typeInfo(ErrorCode).@"enum".field_names) |tag| {
            const code = @field(ErrorCode, tag);
            if (std.mem.eql(u8, value, code.name())) return code;
        }
        return error.InvalidEnumTag;
    }
};''']
    destination.write_text('\n'.join(lines) + '\n')
