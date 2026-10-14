//! Thin owned sessions over the Rust-generated complete packet graph.
const std = @import("std");
const tt = @import("thinkthen.zig");
const c = @import("native.zig").c;
const values = @import("results_generated.zig");
pub const Error = error{ Usage, Backend, Deadline, Local, Cancelled, Defect, ClosedEngine };
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
pub fn bytes(value: anytype) []const u8 {
    return if (value.len == 0) &.{} else value.data[0..value.len];
}
const PacketStorage = struct {
    allocator: std.mem.Allocator,
    arena: std.heap.ArenaAllocator,
    value: values.SessionPacket,
};
/// A packet owns its copied values until explicit release.
pub const Packet = opaque {
    fn storage(self: *const Packet) *PacketStorage {
        return @ptrCast(@alignCast(@constCast(self)));
    }
    pub fn deinit(self: *Packet) void {
        const owned: *PacketStorage = @ptrCast(@alignCast(self));
        const allocator = owned.allocator;
        owned.arena.deinit();
        allocator.destroy(owned);
    }
    pub fn value(self: *const Packet) *const values.SessionPacket {
        return &self.storage().value;
    }
    pub fn answers(self: *const Packet) values.AnswerView {
        return .{ .packet = self.value(), .allocator = self.storage().arena.allocator() };
    }
    pub fn terminal(self: *const Packet) ?*const values.SessionPacketTerminal {
        return switch (self.value().*) {
            .terminal => |*v| v,
            else => null,
        };
    }
    pub fn facts(self: *const Packet) ?*const values.Facts {
        const end = self.terminal() orelse return null;
        return switch (end.facts) {
            .value => |*v| v,
            else => null,
        };
    }
    pub fn failure(self: *const Packet) ?*const values.CallError {
        const end = self.terminal() orelse return null;
        return switch (end.failure) {
            .value => |*v| v,
            else => null,
        };
    }
};
pub const Session = struct {
    handle: *anyopaque,
    allocator: std.mem.Allocator,
    fn native(self: Session) *c.thinkthen_session {
        return @ptrCast(self.handle);
    }
    pub fn deinit(self: *Session) void {
        c.thinkthen_session_free(self.native());
    }
    pub fn cancel(self: Session) void {
        c.thinkthen_session_cancel(self.native());
    }
    pub fn push(self: Session, allocator: std.mem.Allocator, item: tt.authored.inputs.RequestSessionDescriptor) !enum { accepted, full, closed } {
        const json = try std.json.Stringify.valueAlloc(allocator, item, .{ .emit_null_optional_fields = false });
        defer allocator.free(json);
        var status: u32 = undefined;
        try checked(c.thinkthen_session_try_push(self.native(), json.ptr, json.len, &status));
        return switch (status) {
            c.THINKTHEN_SESSION_ACCEPTED_V1 => .accepted,
            c.THINKTHEN_SESSION_FULL_V1 => .full,
            else => .closed,
        };
    }
    pub fn finish(self: Session) Error!void {
        try checked(c.thinkthen_session_finish(self.native(), null, 0));
    }
    pub const Read = union(enum) { pending, end, packet: *Packet };
    pub fn read(self: Session) !Read {
        return self.readNext(false);
    }
    /// Wait until a packet arrives or the cancelled session settles.
    pub fn next(self: Session) !Read {
        return self.readNext(true);
    }
    pub fn wait(self: Session, input: bool) Error!void {
        var ready: u32 = undefined;
        try checked(c.thinkthen_session_wait(self.native(), if (input) c.THINKTHEN_SESSION_WAIT_INPUT_V1 else c.THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, std.math.maxInt(u32), &ready));
    }
    fn readNext(self: Session, blocking: bool) !Read {
        var status: u32 = undefined;
        var raw: ?*c.thinkthen_session_result = null;
        try checked(if (blocking)
            c.thinkthen_session_next_packet(self.native(), c.THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, &status, &raw)
        else
            c.thinkthen_session_try_read(self.native(), &status, &raw));
        switch (status) {
            c.THINKTHEN_SESSION_PENDING_V1 => return .pending,
            c.THINKTHEN_SESSION_END_V1 => return .end,
            else => {
                const owner = raw orelse return error.Defect;
                defer c.thinkthen_session_result_free(owner);
                var view: [*c]const c.thinkthen_complete_session_packet_v1 = null;
                try checked(c.thinkthen_session_result_view(owner, &view));
                const owned = try self.allocator.create(PacketStorage);
                errdefer self.allocator.destroy(owned);
                owned.* = .{ .allocator = self.allocator, .arena = std.heap.ArenaAllocator.init(self.allocator), .value = undefined };
                errdefer owned.arena.deinit();
                owned.value = try values.readSessionPacket(owned.arena.allocator(), view[0]);
                return .{ .packet = @ptrCast(owned) };
            },
        }
    }
};
pub fn startNamed(engine: *tt.Engine, function: tt.inputs.RequestFunction, arguments: struct { question: tt.inputs.RequestQuestion, input: tt.inputs.RequestInput, options: ?tt.inputs.RequestOptions = null }) !Session {
    const request = try tt.Request.init(engine.allocator, @tagName(function), arguments.question, arguments.input, arguments.options);
    defer request.deinit();
    return request.start(engine);
}

pub const CallResult = @import("blocking.zig").CallResult;
pub const completeNamed = @import("blocking.zig").completeNamed;
