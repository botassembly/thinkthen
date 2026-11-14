const std = @import("std");
const tt = @import("thinkthen");
const A = std.mem.Allocator;
const V = std.json.Value;
pub fn toolbox(a: A, value: V, diag: ?*tt.Diagnostic) !tt.Toolbox {
    var fields: std.json.ObjectMap = .empty;
    var iterator = value.object.iterator();
    while (iterator.next()) |entry| if (!std.mem.eql(u8, entry.key_ptr.*, "name")) {
        try fields.put(a, entry.key_ptr.*, entry.value_ptr.*);
    };
    const encoded = try std.json.Stringify.valueAlloc(a, .{ .entries = .{ .fixture = .{ .question = std.json.Value{ .object = fields } } } }, .{});
    return tt.Toolbox.fromJson(a, encoded, diag);
}
pub fn settings(a: A, raw: []const u8) !tt.Settings {
    const value = try std.json.parseFromSliceLeaky(V, a, raw, .{});
    var result: tt.Settings = .{};
    inline for (@typeInfo(tt.Settings).@"struct".field_names) |name| {
        if (comptime !std.mem.eql(u8, name, "api_key") and !std.mem.eql(u8, name, "cancel")) {
            if (value.object.get(name)) |child| @field(result, name) = try std.json.parseFromValueLeaky(@TypeOf(@field(result, name)), a, child, .{ .allocate = .alloc_always });
        }
    }
    return result;
}
