//! Thin owned sessions over the Rust-generated complete packet graph.
const std = @import("std");
const tt = @import("thinkthen.zig");
const c = tt.c;
pub const Function = enum { decide, choose, tag, score, filter, rank, find, annotate, recognize, relate };
pub const Error = error{ Usage, Backend, Deadline, Local, Cancelled, Defect };
pub fn checked(code: c_int) Error!void {
    switch (code) {
        c.THINKTHEN_OK => {},
        c.THINKTHEN_EUSAGE => return error.Usage,
        c.THINKTHEN_EBACKEND => return error.Backend,
        c.THINKTHEN_EDEADLINE => return error.Deadline,
        c.THINKTHEN_ELOCAL => return error.Local,
        c.THINKTHEN_ECANCELLED => return error.Cancelled,
        else => return error.Defect,
    }
}
pub fn message() []const u8 {
    return std.mem.span(c.thinkthen_session_error_message());
}
pub fn bytes(value: c.thinkthen_complete_utf8_v1) []const u8 {
    return if (value.len == 0) &.{} else value.data[0..value.len];
}
/// Distinguish absent, present null and present value without a JSON reader.
pub fn Presence(comptime T: type) type {
    return union(enum) { missing, null_value, value: T };
}
pub fn presence(field: anytype) Presence(@TypeOf(field.value)) {
    return switch (field.presence) {
        c.THINKTHEN_COMPLETE_PRESENCE_MISSING_V1 => .missing,
        c.THINKTHEN_COMPLETE_PRESENCE_NULL_V1 => .null_value,
        else => .{ .value = field.value },
    };
}
/// For fields whose native schema allows absence but does not allow null.
pub fn optional(field: anytype) ?@TypeOf(field.value) {
    return if (field.presence == c.THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) field.value else null;
}
pub const Packet = struct {
    raw: *c.thinkthen_session_result,
    view: *const c.thinkthen_complete_session_packet_v1,
    pub fn deinit(self: *Packet) void {
        c.thinkthen_session_result_free(self.raw);
    }
    pub fn terminal(self: Packet) ?*const c.thinkthen_complete_session_packet_terminal_v1 {
        return if (self.view.kind == c.THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1) self.view.data.terminal else null;
    }
    pub fn facts(self: Packet) ?*const c.thinkthen_complete_facts_v1 {
        const value = self.terminal() orelse return null;
        return optional(value.facts) orelse return null;
    }
    pub fn failure(self: Packet) ?*const c.thinkthen_complete_call_error_v1 {
        const value = self.terminal() orelse return null;
        return optional(value.failure) orelse return null;
    }
};
pub const Session = struct {
    raw: *c.thinkthen_session,
    pub fn deinit(self: *Session) void {
        c.thinkthen_session_free(self.raw);
    }
    pub fn cancel(self: Session) void {
        c.thinkthen_session_cancel(self.raw);
    }
    pub fn push(self: Session, allocator: std.mem.Allocator, item: tt.authored.inputs.RequestSessionDescriptor) !enum { accepted, full, closed } {
        const json = try std.json.Stringify.valueAlloc(allocator, item, .{ .emit_null_optional_fields = false });
        defer allocator.free(json);
        var status: u32 = undefined;
        try checked(c.thinkthen_session_try_push(self.raw, json.ptr, json.len, &status));
        return switch (status) {
            c.THINKTHEN_SESSION_ACCEPTED_V1 => .accepted,
            c.THINKTHEN_SESSION_FULL_V1 => .full,
            else => .closed,
        };
    }
    pub fn finish(self: Session) Error!void {
        try checked(c.thinkthen_session_finish(self.raw, null, 0));
    }
    pub const Read = union(enum) { pending, end, packet: Packet };
    pub fn read(self: Session) Error!Read {
        var status: u32 = undefined;
        var raw: ?*c.thinkthen_session_result = null;
        try checked(c.thinkthen_session_try_read(self.raw, &status, &raw));
        switch (status) {
            c.THINKTHEN_SESSION_PENDING_V1 => return .pending,
            c.THINKTHEN_SESSION_END_V1 => return .end,
            else => {
                const owner = raw orelse return error.Defect;
                errdefer c.thinkthen_session_result_free(owner);
                var view: [*c]const c.thinkthen_complete_session_packet_v1 = null;
                try checked(c.thinkthen_session_result_view(owner, &view));
                return .{ .packet = .{ .raw = owner, .view = view } };
            },
        }
    }
};
pub fn decide(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallDecide) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .decide = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn choose(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallChoose) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .choose = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn tag(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallTag) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .tag = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn score(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallScore) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .score = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn filter(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallFilter) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .filter = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn rank(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallRank) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .rank = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn find(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallFind) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .find = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn annotate(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallAnnotate) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .annotate = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn recognize(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallRecognize) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .recognize = arguments });
    defer request.deinit();
    return request.start(engine);
}
pub fn relate(engine: *tt.Engine, arguments: tt.authored.inputs.RequestCallRelate) !Session {
    const request = try tt.authored.Request.init(engine.allocator, .{ .relate = arguments });
    defer request.deinit();
    return request.start(engine);
}
