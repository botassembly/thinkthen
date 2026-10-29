const std = @import("std");
const c = @import("c");
const native = @import("native.zig");
const t = @import("types.zig");
const q = @import("questions.zig");
const r = @import("row.zig");
const d = @import("diagnostic.zig");
const Allocator = std.mem.Allocator;
pub const FilterRow = struct {
    record: t.Record,
    row: r.Row(.filter),
    pub fn kept(self: FilterRow) bool {
        return switch (self.row.outcome) {
            .answered => |v| v.answer,
            else => false,
        };
    }
};
pub fn Item(comptime function: t.Function) type {
    return if (function == .filter) FilterRow else r.Row(function);
}
fn descriptor(record: t.Record) c.thinkthen_record_input {
    return switch (record) {
        .text => |v| .{ .struct_size = @sizeOf(c.thinkthen_record_input), .kind = c.THINKTHEN_RECORD_TEXT, .data = v.ptr, .len = v.len },
        .json => |v| .{ .struct_size = @sizeOf(c.thinkthen_record_input), .kind = c.THINKTHEN_RECORD_JSON, .data = v.ptr, .len = v.len },
        .missing => .{ .struct_size = @sizeOf(c.thinkthen_record_input), .kind = c.THINKTHEN_RECORD_MISSING, .data = null, .len = 0 },
    };
}
pub fn open(comptime function: t.Function, client: ?*c.thinkthen_client, allocator: Allocator, question: q.QuestionType(function), input: t.Input, diag: ?*d.Diagnostic) d.Error!Call(function) {
    d.clear(diag);
    if (client == null) return error.Internal;
    const builder = try admit(function, allocator, question, diag);
    defer native.function("thinkthen_question_free")(builder);
    var err = d.emptyError();
    var result: Call(function) = .{ .allocator = allocator, .input = input };
    errdefer result.deinit();
    var name: c.thinkthen_text = undefined;
    const name_ptr: ?*const c.thinkthen_text = if (question.options.name) |v| named: {
        name = .{ .data = v.ptr, .len = v.len };
        break :named &name;
    } else null;
    switch (input) {
        .records => |records| {
            const inputs = try allocator.alloc(c.thinkthen_record_input, records.len);
            defer allocator.free(inputs);
            for (records, inputs) |record, *slot| slot.* = descriptor(record);
            if (function == .filter) for (records, 1..) |record, position| {
                const copy = try record.copy(allocator);
                errdefer copy.free(allocator);
                try result.originals.put(allocator, position, copy);
            };
            try d.check(native.function("thinkthen_call_open_records_question")(client, builder, name_ptr, inputs.ptr, inputs.len, @sizeOf(c.thinkthen_record_input), c.THINKTHEN_OFFSET_UTF8, &result.handle, &err), &err, diag);
        },
        .source => try d.check(native.function("thinkthen_call_open_question")(client, builder, name_ptr, c.THINKTHEN_OFFSET_UTF8, &result.handle, &err), &err, diag),
    }
    return result;
}
pub fn Call(comptime function: t.Function) type {
    return struct {
        handle: ?*c.thinkthen_call = null,
        allocator: Allocator,
        input: t.Input,
        done: bool = false,
        input_ended: bool = false,
        originals: std.AutoHashMapUnmanaged(u64, t.Record) = .empty,
        next_position: u64 = 1,
        const Self = @This();
        pub fn cancel(self: *const Self) void {
            if (self.handle) |handle| native.function("thinkthen_call_cancel")(handle);
        }
        pub fn deinit(self: *Self) void {
            if (self.handle) |handle| native.function("thinkthen_call_free")(handle);
            self.handle = null;
            var originals = self.originals.valueIterator();
            while (originals.next()) |record| record.free(self.allocator);
            self.originals.deinit(self.allocator);
            self.originals = .empty;
            self.done = true;
        }
        /// Returned rows and retained filter records live in arena until it is reset or freed.
        pub fn next(self: *Self, arena: Allocator, diag: ?*d.Diagnostic) d.Error!?Item(function) {
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
                switch (status) {
                    c.THINKTHEN_END => {
                        self.done = true;
                        return null;
                    },
                    c.THINKTHEN_NEEDS_INPUT => {
                        if (self.input_ended) return error.Internal;
                        const record = switch (self.input) {
                            .source => |source| try source.nextFn(source.context),
                            .records => null,
                        };
                        if (record) |value| {
                            if (function == .filter) {
                                const copy = try value.copy(self.allocator);
                                errdefer copy.free(self.allocator);
                                try self.originals.put(self.allocator, self.next_position, copy);
                                self.next_position += 1;
                            }
                            const result = switch (value) {
                                .text => |v| native.function("thinkthen_call_send_text")(self.handle, v.ptr, v.len, &err),
                                .json => |v| native.function("thinkthen_call_send_json")(self.handle, v.ptr, v.len, &err),
                                .missing => native.function("thinkthen_call_send_missing")(self.handle, &err),
                            };
                            try d.check(result, &err, diag);
                        } else {
                            native.function("thinkthen_call_end_input")(self.handle);
                            self.input_ended = true;
                        }
                    },
                    c.THINKTHEN_OK => {
                        defer native.function("thinkthen_row_free")(&row);
                        const decoded = r.take(function, arena, row) catch |failure| {
                            if (failure == error.OutOfMemory) return error.OutOfMemory;
                            return d.incompatible(diag);
                        };
                        if (function == .filter) {
                            const original = self.originals.fetchRemove(row.position) orelse return error.Internal;
                            defer original.value.free(self.allocator);
                            const copy = try original.value.copy(arena);
                            return .{ .row = decoded, .record = copy };
                        } else return decoded;
                    },
                    else => {
                        try d.check(status, &err, diag);
                        return error.Internal;
                    },
                }
            }
        }
    };
}

fn admit(comptime function: t.Function, allocator: Allocator, question: q.QuestionType(function), diag: ?*d.Diagnostic) d.Error!*c.thinkthen_question {
    var builder: ?*c.thinkthen_question = null;
    var err = d.emptyError();
    if (function == .annotate) {
        try d.check(native.function("thinkthen_question_new")(c.THINKTHEN_FUNCTION_ANNOTATE, null, 0, &builder, &err), &err, diag);
    } else {
        var base = question;
        if (@hasField(@TypeOf(base.options), "threshold")) base.options.threshold = null;
        const encoded = try std.json.Stringify.valueAlloc(allocator, base, .{ .emit_null_optional_fields = false });
        defer allocator.free(encoded);
        try d.check(native.function("thinkthen_question_from_json")(encoded.ptr, encoded.len, &builder, &err), &err, diag);
    }
    errdefer native.function("thinkthen_question_free")(builder);
    if (@hasField(@TypeOf(question.options), "threshold")) {
        if (question.options.threshold) |threshold| {
            if (function == .decide or function == .choose or function == .filter) {
                switch (threshold) {
                    .cut => |v| try d.check(native.function("thinkthen_question_set_cut")(builder, v, &err), &err, diag),
                    .bounds => |v| try d.check(native.function("thinkthen_question_set_band")(builder, v.low, v.high, &err), &err, diag),
                }
            } else try d.check(native.function("thinkthen_question_set_cut")(builder, threshold, &err), &err, diag);
        }
    }
    if (function == .annotate) {
        if (question.options.evidence) |v| try d.check(native.function("thinkthen_question_set_evidence")(builder, v.ptr, v.len, &err), &err, diag);
        for (question.questions.entries) |entry| {
            switch (entry.question) {
                inline else => |child, tag| {
                    const handle = try admit(@field(t.Function, @tagName(tag)), allocator, child, diag);
                    defer native.function("thinkthen_question_free")(handle);
                    try d.check(native.function("thinkthen_question_add_entry")(builder, entry.name.ptr, entry.name.len, handle, &err), &err, diag);
                },
            }
        }
    }
    return builder.?;
}
