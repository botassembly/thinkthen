const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const d = @import("diagnostic.zig");
const t = @import("types.zig");
const r = @import("result.zig");
const A = std.mem.Allocator;
pub fn RecordType(comptime Collection: type) type {
    if (Collection == t.Input) return t.Record;
    const info = @typeInfo(Collection);
    return switch (info) {
        .pointer => |p| if (p.size == .slice) p.child else switch (@typeInfo(p.child)) {
            .array => |a| a.child,
            else => @compileError("All calls take a slice or an array of records"),
        },
        .array => |a| a.child,
        else => @compileError("All calls take a slice or an array of records"),
    };
}
pub fn singleType(comptime T: type) type {
    if (T == t.Record) return T;
    switch (@typeInfo(T)) {
        .array => @compileError("one-record calls refuse arrays; use the All form"),
        .pointer => |p| switch (p.size) {
            .slice => if (p.child == u8) return []const u8 else @compileError("one-record calls refuse slices of records; use the All form"),
            .one => switch (@typeInfo(p.child)) {
                .array => |a| if (a.child == u8) return []const u8 else @compileError("one-record calls refuse arrays; use the All form"),
                else => {},
            },
            else => {},
        },
        else => {},
    }
    return T;
}
fn descriptor(arena: A, value: anytype) d.Error!c.thinkthen_record_input {
    const T = @TypeOf(value);
    const record: t.Record = if (T == t.Record) value else if (comptime singleType(T) == []const u8) .{ .text = value } else .{ .json = try std.json.Stringify.valueAlloc(arena, value, .{}) };
    return switch (record) {
        .text => |v| .{ .struct_size = @sizeOf(c.thinkthen_record_input), .kind = c.THINKTHEN_RECORD_TEXT, .data = @ptrCast(v.ptr), .len = v.len },
        .json => |v| .{ .struct_size = @sizeOf(c.thinkthen_record_input), .kind = c.THINKTHEN_RECORD_JSON, .data = @ptrCast(v.ptr), .len = v.len },
        .missing => .{ .struct_size = @sizeOf(c.thinkthen_record_input), .kind = c.THINKTHEN_RECORD_MISSING },
    };
}
pub fn open(client: ?*c.thinkthen_client, allocator: A, question: anytype, records: anytype, diag: ?*d.Diagnostic) d.Error!Call(@TypeOf(question), RecordType(@TypeOf(records))) {
    d.clear(diag);
    if (client == null) return d.promote(.{ .code = .usage_closed, .message = "the client is closed" }, diag);
    const Q = @TypeOf(question);
    const T = RecordType(@TypeOf(records));
    var scratch = std.heap.ArenaAllocator.init(allocator);
    defer scratch.deinit();
    const a = scratch.allocator();
    const builder = try admit(a, question, diag);
    defer native.function("thinkthen_question_free")(builder);
    var err = d.emptyError();
    var call: Call(Q, T) = .{ .allocator = allocator, .records = &.{}, .rows = .init(allocator), .retained = .init(allocator), .owned = .init(allocator) };
    errdefer call.deinit();
    if (comptime @TypeOf(records) == t.Input) {
        switch (records) {
            .source => |source| call.source = source,
            .records => |items| call.records = items,
            .texts => |texts| {
                const items = try call.owned.allocator().alloc(T, texts.len);
                for (texts, items) |text, *item| item.* = .{ .text = text };
                call.records = items;
            },
            .json_lines => |text| {
                var items: std.ArrayList(T) = .empty;
                var lines = std.mem.splitScalar(u8, text, '\n');
                while (lines.next()) |line| if (std.mem.trim(u8, line, " \t\r").len != 0) {
                    try items.append(call.owned.allocator(), .{ .json = line });
                };
                call.records = try items.toOwnedSlice(call.owned.allocator());
            },
        }
    } else call.records = records;
    const inputs = try a.alloc(c.thinkthen_record_input, call.records.len);
    for (call.records, inputs) |record, *slot| slot.* = try descriptor(a, record);
    var name: c.thinkthen_text = undefined;
    const name_ptr: ?*const c.thinkthen_text = if (question.name) |text| blk: {
        name = .{ .data = @ptrCast(text.ptr), .len = text.len };
        break :blk &name;
    } else null;
    if (call.source != null) {
        try d.check(native.function("thinkthen_call_open_question")(client, builder, name_ptr, c.THINKTHEN_OFFSET_UTF8, &call.handle, &err), &err, diag);
    } else try d.check(native.function("thinkthen_call_open_records_question")(client, builder, name_ptr, inputs.ptr, inputs.len, @sizeOf(c.thinkthen_record_input), c.THINKTHEN_OFFSET_UTF8, &call.handle, &err), &err, diag);
    return call;
}
fn admit(a: A, question: anytype, diag: ?*d.Diagnostic) d.Error!*c.thinkthen_question {
    const Q = @TypeOf(question);
    var handle: ?*c.thinkthen_question = null;
    var err = d.emptyError();
    if (Q.function == .annotate and @hasDecl(Q, "Questions")) {
        try d.check(native.function("thinkthen_question_new")(c.THINKTHEN_FUNCTION_ANNOTATE, null, 0, &handle, &err), &err, diag);
    } else {
        var base = question;
        if (@hasField(Q, "threshold") and @FieldType(Q, "threshold") != void) base.threshold = null;
        const encoded = try std.json.Stringify.valueAlloc(a, base, .{ .emit_null_optional_fields = false });
        try d.check(native.function("thinkthen_question_from_json")(encoded.ptr, encoded.len, &handle, &err), &err, diag);
    }
    errdefer native.function("thinkthen_question_free")(handle);
    if (@hasField(Q, "threshold") and @FieldType(Q, "threshold") != void) if (question.threshold) |threshold| {
        if (Q.function == .decide) switch (threshold) {
            .value => |value| try d.check(native.function("thinkthen_question_set_cut")(handle, value, &err), &err, diag),
            .bounds => |bounds| try d.check(native.function("thinkthen_question_set_band")(handle, bounds.low, bounds.high, &err), &err, diag),
        } else try d.check(native.function("thinkthen_question_set_cut")(handle, threshold.value, &err), &err, diag);
    };
    if (Q.function == .annotate and @hasDecl(Q, "Questions")) {
        if (question.field) |field| try d.check(native.function("thinkthen_question_set_field")(handle, field.ptr, field.len, &err), &err, diag);
        inline for (@typeInfo(Q.Questions).@"struct".field_names) |name| {
            const child = try admit(a, @field(question.entries, name), diag);
            defer native.function("thinkthen_question_free")(child);
            try d.check(native.function("thinkthen_question_add_entry")(handle, name.ptr, name.len, child, &err), &err, diag);
        }
    }
    return handle.?;
}
pub fn Call(comptime Q: type, comptime T: type) type {
    return struct {
        allocator: A,
        handle: ?*c.thinkthen_call = null,
        records: []const T,
        rows: std.heap.ArenaAllocator,
        owned: std.heap.ArenaAllocator,
        source: ?t.RecordSource = null,
        source_ended: bool = false,
        sent: u64 = 0,
        pending: std.AutoHashMapUnmanaged(u64, t.Record) = .empty,
        retained: std.heap.ArenaAllocator,
        failures: std.ArrayList(r.Row(Q, T)) = .empty,
        done: bool = false,
        const Self = @This();
        const Item = if (Q.function == .filter) T else r.Row(Q, T);
        pub fn cancel(self: *const Self) void {
            if (self.handle) |handle| native.function("thinkthen_call_cancel")(handle);
        }
        pub fn deinit(self: *Self) void {
            if (self.handle) |handle| native.function("thinkthen_call_free")(handle);
            self.handle = null;
            self.done = true;
            self.failures.deinit(self.allocator);
            self.rows.deinit();
            self.retained.deinit();
            self.owned.deinit();
            var items = self.pending.valueIterator();
            while (items.next()) |item| item.free(self.allocator);
            self.pending.deinit(self.allocator);
        }
        pub fn failed(self: *const Self) []const r.Row(Q, T) {
            if (Q.function != .filter) @compileError("failed reads filter failures");
            return self.failures.items;
        }
        pub fn next(self: *Self, diag: ?*d.Diagnostic) d.Error!?Item {
            _ = self.rows.reset(.retain_capacity);
            return self.nextInto(self.rows.allocator(), diag);
        }
        pub fn nextInto(self: *Self, arena: A, diag: ?*d.Diagnostic) d.Error!?Item {
            d.clear(diag);
            if (self.done) return null;
            errdefer {
                self.cancel();
                self.done = true;
            }
            while (true) {
                var row: c.thinkthen_row = .{ .struct_size = @sizeOf(c.thinkthen_row) };
                var err = d.emptyError();
                const status = native.function("thinkthen_call_next")(self.handle, &row, &err);
                if (status == c.THINKTHEN_END) {
                    self.done = true;
                    if (Q.function == .filter and self.failures.items.len != 0) {
                        if (diag) |target| {
                            target.set(self.failures.items[0].failure.?);
                            target.position = self.failures.items[0].position;
                        }
                        return error.Failed;
                    }
                    return null;
                }
                if (status == c.THINKTHEN_NEEDS_INPUT) {
                    if (self.source_ended) return error.Internal;
                    const source = self.source orelse return error.Internal;
                    if (try source.nextFn(source.context)) |record| {
                        self.sent += 1;
                        const copy = try record.copy(self.allocator);
                        self.pending.put(self.allocator, self.sent, copy) catch |failure| {
                            copy.free(self.allocator);
                            return failure;
                        };
                        const rc = switch (record) {
                            .text => |text| native.function("thinkthen_call_send_text")(self.handle, text.ptr, text.len, &err),
                            .json => |text| native.function("thinkthen_call_send_json")(self.handle, text.ptr, text.len, &err),
                            .missing => native.function("thinkthen_call_send_missing")(self.handle, &err),
                        };
                        try d.check(rc, &err, diag);
                    } else {
                        native.function("thinkthen_call_end_input")(self.handle);
                        self.source_ended = true;
                    }
                    continue;
                }
                try d.check(status, &err, diag);
                defer native.function("thinkthen_row_free")(&row);
                const source_original = if (self.source != null) self.pending.fetchRemove(row.position) orelse return error.Internal else null;
                defer if (source_original) |entry| entry.value.free(self.allocator);
                if (Q.function == .filter and row.status != c.THINKTHEN_STATUS_FAILED) {
                    var values: ?*c.thinkthen_value = null;
                    try d.check(native.function("thinkthen_row_values")(&row, &values, &err), &err, diag);
                    defer native.function("thinkthen_value_free")(values);
                    var keep: u8 = 0;
                    try d.check(native.function("thinkthen_value_bool")(values, &keep, &err), &err, diag);
                    if (keep == 0) continue;
                }
                const original: T = if (comptime T == t.Record) if (source_original) |entry| try entry.value.copy(if (Q.function == .filter and row.status == c.THINKTHEN_STATUS_FAILED) self.retained.allocator() else arena) else blk: {
                    if (row.position == 0 or row.position > self.records.len) return error.Internal;
                    break :blk self.records[@intCast(row.position - 1)];
                } else blk: {
                    if (row.position == 0 or row.position > self.records.len) return error.Internal;
                    break :blk self.records[@intCast(row.position - 1)];
                };
                if (Q.function == .filter) {
                    if (row.status == c.THINKTHEN_STATUS_FAILED) {
                        const decoded = try decode(self.retained.allocator(), row, original, diag);
                        try self.failures.append(self.allocator, decoded);
                        continue;
                    }
                    return original;
                } else return try decode(arena, row, original, diag);
            }
        }
        fn decode(arena: A, row: c.thinkthen_row, original: T, diag: ?*d.Diagnostic) d.Error!r.Row(Q, T) {
            return r.take(Q, T, arena, row, original) catch |failure| {
                if (failure == error.OutOfMemory) return error.OutOfMemory;
                return d.incompatible(diag);
            };
        }
    };
}
