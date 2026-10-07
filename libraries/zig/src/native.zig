const std = @import("std");
const tt = @import("thinkthen.zig");
pub const c = tt.c;
pub const Role = enum(u32) { atomic = 1, set, dynamic_choose, recognize, relate, rank, rank_set, find };
pub const Unit = enum(u32) { line = 1, window, file, image_file, jsonl };
pub fn counted(s: []const u8) c.thinkthen_string_v1 {
    return .{ .data = s.ptr, .len = s.len };
}
pub fn text(s: []const u8) c.thinkthen_content_v1 {
    return .{ .kind = 1, .data = counted(s) };
}
pub fn json(s: []const u8) c.thinkthen_content_v1 {
    return .{ .kind = 2, .data = counted(s) };
}
pub fn bytes(s: c.thinkthen_string_v1) ![]const u8 {
    try extent(s.len, 1, s.data);
    return if (s.len == 0) &.{} else s.data[0..s.len];
}
fn extent(n: usize, width: usize, p: anytype) !void {
    if (n > std.math.maxInt(usize) / width or (n != 0 and @intFromPtr(p) == 0)) return error.InvalidNativeExtent;
}
// Copies active unions and counted extents only. Native owner may be freed
// immediately afterwards. The returned snapshot arena owns every nested view.
fn clone(comptime T: type, a: std.mem.Allocator, v: T) anyerror!T {
    switch (@typeInfo(T)) {
        .@"struct" => {
            var out: T = std.mem.zeroes(T);
            if (T == c.thinkthen_string_v1) {
                const data = try bytes(v);
                out.len = data.len;
                out.data = if (data.len == 0) null else (try a.dupe(u8, data)).ptr;
                return out;
            }
            if (T == c.thinkthen_image_view_v1) {
                try extent(v.bytes_len, 1, v.bytes);
                out = v;
                out.bytes = if (v.bytes_len == 0) null else (try a.dupe(u8, v.bytes[0..v.bytes_len])).ptr;
                out.filename = try clone(@TypeOf(v.filename), a, v.filename);
                return out;
            }
            if (@hasField(T, "present") and @hasField(T, "value")) {
                if (v.present != 0 and v.present != 1) return error.InvalidNativePresence;
                if (v.present != 0) {
                    out.present = 1;
                    out.value = try clone(@TypeOf(v.value), a, v.value);
                }
                return out;
            }
            if (@hasField(T, "len") and @hasField(T, "data")) {
                const Child = @typeInfo(@TypeOf(v.data)).pointer.child;
                try extent(v.len, @sizeOf(Child), v.data);
                out.len = v.len;
                if (v.len != 0) {
                    const values = try a.alloc(Child, v.len);
                    for (values, 0..) |*slot, i| slot.* = try clone(Child, a, v.data[i]);
                    out.data = values.ptr;
                }
                return out;
            }
            inline for (@typeInfo(T).@"struct".fields) |field| {
                if (@typeInfo(field.type) == .@"union") {
                    if (T == c.thinkthen_decide_value_v1) {
                        switch (v.kind) {
                            0 => {},
                            1 => out.data.boolean = try clone(@TypeOf(v.data.boolean), a, v.data.boolean),
                            2 => out.data.authored = try clone(@TypeOf(v.data.authored), a, v.data.authored),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_answer_v1) {
                        switch (v.kind) {
                            1 => out.data.probability = try clone(@TypeOf(v.data.probability), a, v.data.probability),
                            2 => out.data.choice = try clone(@TypeOf(v.data.choice), a, v.data.choice),
                            3 => out.data.tag = try clone(@TypeOf(v.data.tag), a, v.data.tag),
                            4 => out.data.score = try clone(@TypeOf(v.data.score), a, v.data.score),
                            5 => out.data.find = try clone(@TypeOf(v.data.find), a, v.data.find),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_member_value_v1) {
                        switch (v.kind) {
                            1 => out.data.decide = try clone(@TypeOf(v.data.decide), a, v.data.decide),
                            2 => out.data.choose = try clone(@TypeOf(v.data.choose), a, v.data.choose),
                            3 => out.data.tag = try clone(@TypeOf(v.data.tag), a, v.data.tag),
                            4 => out.data.score = try clone(@TypeOf(v.data.score), a, v.data.score),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_member_v1) {
                        switch (v.state) {
                            1 => out.data.success = try clone(@TypeOf(v.data.success), a, v.data.success),
                            2 => out.data.failure = try clone(@TypeOf(v.data.failure), a, v.data.failure),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_relation_answer_v1) {
                        switch (v.state) {
                            1 => out.data.success = try clone(@TypeOf(v.data.success), a, v.data.success),
                            2 => out.data.failure = try clone(@TypeOf(v.data.failure), a, v.data.failure),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_observation_identity_v1) {
                        switch (v.kind) {
                            1 => out.data.observation_id = try clone(@TypeOf(v.data.observation_id), a, v.data.observation_id),
                            2 => out.data.failure_id = try clone(@TypeOf(v.data.failure_id), a, v.data.failure_id),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_observed_probabilities_v1) {
                        switch (v.kind) {
                            1 => out.data.yes = try clone(@TypeOf(v.data.yes), a, v.data.yes),
                            2 => out.data.named = try clone(@TypeOf(v.data.named), a, v.data.named),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_question_observation_v1) {
                        switch (v.state) {
                            1 => out.data.success = try clone(@TypeOf(v.data.success), a, v.data.success),
                            2 => out.data.failure = try clone(@TypeOf(v.data.failure), a, v.data.failure),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_row_observation_v1) {
                        switch (v.function) {
                            1 => out.data.decide = try clone(@TypeOf(v.data.decide), a, v.data.decide),
                            2 => out.data.choose = try clone(@TypeOf(v.data.choose), a, v.data.choose),
                            3 => out.data.tag = try clone(@TypeOf(v.data.tag), a, v.data.tag),
                            4 => out.data.score = try clone(@TypeOf(v.data.score), a, v.data.score),
                            5 => out.data.filter = try clone(@TypeOf(v.data.filter), a, v.data.filter),
                            6 => out.data.rank = try clone(@TypeOf(v.data.rank), a, v.data.rank),
                            7 => out.data.find = try clone(@TypeOf(v.data.find), a, v.data.find),
                            8 => out.data.annotate = try clone(@TypeOf(v.data.annotate), a, v.data.annotate),
                            9 => out.data.recognize = try clone(@TypeOf(v.data.recognize), a, v.data.recognize),
                            10 => out.data.relate = try clone(@TypeOf(v.data.relate), a, v.data.relate),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else if (T == c.thinkthen_observation_v1) {
                        switch (v.kind) {
                            1 => out.data.question = try clone(@TypeOf(v.data.question), a, v.data.question),
                            2 => out.data.row = try clone(@TypeOf(v.data.row), a, v.data.row),
                            else => return error.InvalidNativeDiscriminator,
                        }
                    } else @compileError("unhandled native union");
                } else {
                    @field(out, field.name) = try clone(field.type, a, @field(v, field.name));
                }
            }
            return out;
        },
        .pointer => |p| {
            if (v == null) return error.InvalidNativeExtent;
            const out = try a.create(p.child);
            out.* = try clone(p.child, a, v.*);
            return out;
        },
        else => return v,
    }
}
pub const Snapshot = struct {
    arena: std.heap.ArenaAllocator,
    summary: c.thinkthen_summary_v1,
    rows: []const c.thinkthen_row_observation_v1,
    observations: []const c.thinkthen_observation_v1,
    details: []const c.thinkthen_details_v1,
    observation_details: []const c.thinkthen_details_v1,
    authors: []const c.thinkthen_question_author_v1,
    observation_authors: []const c.thinkthen_question_author_v1,
    member_authors: []const []const c.thinkthen_question_author_v1,
    rank_members: []const []const c.thinkthen_rank_view_v1,
    rank_member_details: []const []const c.thinkthen_details_v1,
    located_recognition: []const c.thinkthen_source_recognition_v1,
    located_relations: []const c.thinkthen_source_relations_v1,
    pub fn kind(self: *const Snapshot) !tt.FailureKind {
        if (self.summary.@"error".present != 1) return error.NotAFailure;
        return std.meta.intToEnum(tt.FailureKind, self.summary.@"error".value.code) catch error.InvalidNativeDiscriminator;
    }
    pub fn deinit(self: *Snapshot) void {
        self.arena.deinit();
        self.* = undefined;
    }
    pub fn decide(self: *const Snapshot, i: usize) !c.thinkthen_decide_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 1) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.decide;
    }
    pub fn choose(self: *const Snapshot, i: usize) !c.thinkthen_choose_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 2) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.choose;
    }
    pub fn tag(self: *const Snapshot, i: usize) !c.thinkthen_tag_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 3) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.tag;
    }
    pub fn score(self: *const Snapshot, i: usize) !c.thinkthen_score_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 4) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.score;
    }
    pub fn filter(self: *const Snapshot, i: usize) !c.thinkthen_filter_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 5) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.filter;
    }
    pub fn rank(self: *const Snapshot, i: usize) !c.thinkthen_rank_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 6) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.rank;
    }
    pub fn find(self: *const Snapshot, i: usize) !c.thinkthen_find_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 7) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.find;
    }
    pub fn annotate(self: *const Snapshot, i: usize) !c.thinkthen_annotate_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 8) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.annotate;
    }
    pub fn recognize(self: *const Snapshot, i: usize) !c.thinkthen_recognize_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 9) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.recognize;
    }
    pub fn relate(self: *const Snapshot, i: usize) !c.thinkthen_relate_view_v1 {
        if (i >= self.rows.len or self.rows[i].function != 10) return error.InvalidNativeDiscriminator;
        return self.rows[i].data.relate;
    }
};
pub fn snapshot(allocator: std.mem.Allocator, raw: *c.thinkthen_result) !Snapshot {
    defer c.thinkthen_result_free(raw);
    var arena = std.heap.ArenaAllocator.init(allocator);
    errdefer arena.deinit();
    const a = arena.allocator();
    var s = std.mem.zeroes(c.thinkthen_summary_v1);
    try viewOK(c.thinkthen_result_summary(raw, &s));
    try extent(s.count, @sizeOf(c.thinkthen_row_observation_v1), raw);
    try extent(s.observation_count, @sizeOf(c.thinkthen_observation_v1), raw);
    const rows = try a.alloc(c.thinkthen_row_observation_v1, s.count);
    const details = try a.alloc(c.thinkthen_details_v1, s.count);
    const authors = try a.alloc(c.thinkthen_question_author_v1, s.count);
    const member_authors = try a.alloc([]const c.thinkthen_question_author_v1, s.count);
    const ranks = try a.alloc([]const c.thinkthen_rank_view_v1, s.count);
    const rank_details = try a.alloc([]const c.thinkthen_details_v1, s.count);
    const recs = try a.alloc(c.thinkthen_source_recognition_v1, s.count);
    const rels = try a.alloc(c.thinkthen_source_relations_v1, s.count);
    const events = try a.alloc(c.thinkthen_observation_v1, s.observation_count);
    const event_details = try a.alloc(c.thinkthen_details_v1, s.observation_count);
    const event_authors = try a.alloc(c.thinkthen_question_author_v1, s.observation_count);
    for (events, 0..) |*v, i| {
        var value = std.mem.zeroes(c.thinkthen_observation_v1);
        try viewOK(c.thinkthen_result_observation(raw, i, &value));
        v.* = try clone(@TypeOf(value), a, value);
        var d = std.mem.zeroes(c.thinkthen_details_v1);
        try viewOK(c.thinkthen_result_observation_details(raw, i, &d));
        event_details[i] = try clone(@TypeOf(d), a, d);
        var author = std.mem.zeroes(c.thinkthen_question_author_v1);
        try viewOK(c.thinkthen_result_observation_author(raw, i, &author));
        event_authors[i] = try clone(@TypeOf(author), a, author);
    }
    for (rows, 0..) |*row, i| {
        var value = std.mem.zeroes(c.thinkthen_row_observation_v1);
        try viewOK(c.thinkthen_result_row(raw, i, &value));
        row.* = try clone(@TypeOf(value), a, value);
        var d = std.mem.zeroes(c.thinkthen_details_v1);
        try viewOK(c.thinkthen_result_details(raw, i, &d));
        details[i] = try clone(@TypeOf(d), a, d);
        var author = std.mem.zeroes(c.thinkthen_question_author_v1);
        try viewOK(c.thinkthen_result_question_author(raw, i, &author));
        authors[i] = try clone(@TypeOf(author), a, author);
        member_authors[i] = &.{};
        ranks[i] = &.{};
        rank_details[i] = &.{};
        if (value.function == 8) {
            const members = try a.alloc(c.thinkthen_question_author_v1, value.data.annotate.answers.len);
            for (members, 0..) |*slot, j| {
                try viewOK(c.thinkthen_result_member_author(raw, i, j, &author));
                slot.* = try clone(@TypeOf(author), a, author);
            }
            member_authors[i] = members;
        }
        if (value.function == 6) {
            var n: usize = 0;
            try viewOK(c.thinkthen_result_rank_member_count(raw, i, &n));
            try extent(n, @sizeOf(c.thinkthen_rank_view_v1), raw);
            const members = try a.alloc(c.thinkthen_rank_view_v1, n);
            const mas = try a.alloc(c.thinkthen_question_author_v1, n);
            const mds = try a.alloc(c.thinkthen_details_v1, n);
            for (members, 0..) |*slot, j| {
                var ranked = std.mem.zeroes(c.thinkthen_rank_view_v1);
                try viewOK(c.thinkthen_result_rank_member(raw, i, j, &ranked));
                slot.* = try clone(@TypeOf(ranked), a, ranked);
                try viewOK(c.thinkthen_result_member_author(raw, i, j, &author));
                mas[j] = try clone(@TypeOf(author), a, author);
                try viewOK(c.thinkthen_result_rank_member_details(raw, i, j, &d));
                mds[j] = try clone(@TypeOf(d), a, d);
            }
            ranks[i] = members;
            member_authors[i] = mas;
            rank_details[i] = mds;
        }
        var rec = std.mem.zeroes(c.thinkthen_source_recognition_v1);
        if (value.function == 9) try viewOK(c.thinkthen_result_source_recognition(raw, i, &rec));
        recs[i] = try clone(@TypeOf(rec), a, rec);
        var rel = std.mem.zeroes(c.thinkthen_source_relations_v1);
        if (value.function == 10) try viewOK(c.thinkthen_result_source_relations(raw, i, &rel));
        rels[i] = try clone(@TypeOf(rel), a, rel);
    }
    const summary = try clone(@TypeOf(s), a, s);
    return .{ .arena = arena, .summary = summary, .rows = rows, .observations = events, .details = details, .observation_details = event_details, .authors = authors, .observation_authors = event_authors, .member_authors = member_authors, .rank_members = ranks, .rank_member_details = rank_details, .located_recognition = recs, .located_relations = rels };
}
fn viewOK(code: c_int) !void {
    if (code != 0) return error.NativeAccessorRefused;
}
pub fn Outcome(comptime T: type) type {
    return union(enum) { ok: T, failed: Snapshot };
}
pub fn failure(a: std.mem.Allocator, e: ?*c.thinkthen_engine) !Snapshot {
    var r: ?*c.thinkthen_result = null;
    try viewOK(c.thinkthen_error_complete(e, &r));
    return snapshot(a, r orelse return error.MissingNativeFailure);
}
pub const Question = struct {
    raw: *c.thinkthen_question,
    pub fn deinit(self: Question) void {
        c.thinkthen_question_free(self.raw);
    }
};
pub const Source = struct {
    raw: *c.thinkthen_source,
    pub fn deinit(self: Source) void {
        c.thinkthen_source_free(self.raw);
    }
};
pub const OwnedImageView = struct {
    arena: std.heap.ArenaAllocator,
    value: c.thinkthen_image_view_v1,
    pub fn deinit(self: *OwnedImageView) void {
        self.arena.deinit();
        self.* = undefined;
    }
};
pub const Image = struct {
    raw: *c.thinkthen_image,
    pub fn deinit(self: Image) void {
        c.thinkthen_image_free(self.raw);
    }
    pub fn view(self: Image, allocator: std.mem.Allocator) !OwnedImageView {
        var arena = std.heap.ArenaAllocator.init(allocator);
        errdefer arena.deinit();
        var v = std.mem.zeroes(c.thinkthen_image_view_v1);
        try viewOK(c.thinkthen_image_view(self.raw, &v));
        const value = try clone(@TypeOf(v), arena.allocator(), v);
        return .{ .arena = arena, .value = value };
    }
};
pub fn question(e: *tt.Engine, spec: c.thinkthen_question_spec_v1, author: ?*const c.thinkthen_question_author_v1) !Outcome(Question) {
    var out: ?*c.thinkthen_question = null;
    var s = spec;
    if (c.thinkthen_question_new_authored(e.raw, &s, author, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn parse(e: *tt.Engine, role: Role, grammar: []const u8) !Outcome(Question) {
    var out: ?*c.thinkthen_question = null;
    if (c.thinkthen_question_parse(e.raw, @intFromEnum(role), counted(grammar), &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn load(e: *tt.Engine, path: []const u8) !Outcome(Question) {
    var out: ?*c.thinkthen_question = null;
    if (c.thinkthen_question_load(e.raw, counted(path), &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn named(e: *tt.Engine, role: Role, name: []const u8) !Outcome(Question) {
    var out: ?*c.thinkthen_question = null;
    if (c.thinkthen_question_load_named(e.raw, @intFromEnum(role), counted(name), &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn reference(e: *tt.Engine, role: Role, name: []const u8) !Outcome(Question) {
    var out: ?*c.thinkthen_question = null;
    if (c.thinkthen_question_load_reference(e.raw, @intFromEnum(role), counted(name), &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn records(e: *tt.Engine, values: []const c.thinkthen_record_v1) !Outcome(Source) {
    var out: ?*c.thinkthen_source = null;
    if (c.thinkthen_source_records(e.raw, values.ptr, values.len, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn files(e: *tt.Engine, paths: []const c.thinkthen_string_v1, unit: Unit, window: usize, image_reader: bool) !Outcome(Source) {
    var out: ?*c.thinkthen_source = null;
    var spec = c.thinkthen_source_spec_v1{ .paths = .{ .data = paths.ptr, .len = paths.len }, .unit = @intFromEnum(unit), .window = window };
    const code = if (image_reader) c.thinkthen_source_image_files(e.raw, &spec, &out) else c.thinkthen_source_files(e.raw, &spec, &out);
    if (code != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn image(e: *tt.Engine, data: []const u8, media: u32, filename: ?[]const u8) !Outcome(Image) {
    var out: ?*c.thinkthen_image = null;
    const name = if (filename) |s| c.thinkthen_optional_string_v1{ .present = 1, .value = counted(s) } else std.mem.zeroes(c.thinkthen_optional_string_v1);
    if (c.thinkthen_image_clone(e.raw, data.ptr, data.len, media, name, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = out orelse return error.MissingNativeResult } };
}
pub fn controls() c.thinkthen_controls_v1 {
    var value = std.mem.zeroes(c.thinkthen_controls_v1);
    value.deadline_ms = -1;
    value.surface = counted("zig");
    return value;
}
pub fn decide(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_decide_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn choose(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_choose_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn tag(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_tag_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn score(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_score_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn filter(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_filter_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn rank(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_rank_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn find(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_find_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn annotate(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_annotate_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn recognize(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_recognize_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
pub fn relate(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(Snapshot) {
    var out: ?*c.thinkthen_result = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_relate_complete(e.raw, q.raw, s.raw, &ctl, &out) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = try snapshot(e.allocator, out orelse return error.MissingNativeResult) };
}
/// Engine must stay live through deinit. The creating thread owns start/next/
/// facts/deinit. Returned snapshots own their storage independently.
pub const LazyBatch = struct {
    raw: *c.thinkthen_batch,
    engine: *tt.Engine,
    thread: std.Thread.Id,
    fn creatingThread(self: LazyBatch) !void {
        if (self.thread != std.Thread.getCurrentId()) return error.WrongBatchThread;
    }
    pub fn deinit(self: *LazyBatch) void {
        self.creatingThread() catch @panic("batch belongs to its creating thread");
        c.thinkthen_batch_free(self.raw);
        self.* = undefined;
    }
    pub fn next(self: *LazyBatch) !Outcome(?Snapshot) {
        try self.creatingThread();
        var r: ?*c.thinkthen_result = null;
        if (c.thinkthen_batch_next(self.raw, &r) != 0) return .{ .failed = try failure(self.engine.allocator, self.engine.raw) };
        return .{ .ok = if (r) |raw| try snapshot(self.engine.allocator, raw) else null };
    }
    pub fn facts(self: *LazyBatch) !Outcome(Snapshot) {
        try self.creatingThread();
        var r: ?*c.thinkthen_result = null;
        if (c.thinkthen_batch_facts(self.raw, &r) != 0) return .{ .failed = try failure(self.engine.allocator, self.engine.raw) };
        return .{ .ok = try snapshot(self.engine.allocator, r orelse return error.MissingNativeResult) };
    }
};
pub fn decideBatch(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(LazyBatch) {
    var r: ?*c.thinkthen_batch = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_decide_batch_start(e.raw, q.raw, s.raw, &ctl, &r) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = r orelse return error.MissingNativeResult, .engine = e, .thread = std.Thread.getCurrentId() } };
}
pub fn chooseBatch(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(LazyBatch) {
    var r: ?*c.thinkthen_batch = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_choose_batch_start(e.raw, q.raw, s.raw, &ctl, &r) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = r orelse return error.MissingNativeResult, .engine = e, .thread = std.Thread.getCurrentId() } };
}
pub fn tagBatch(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(LazyBatch) {
    var r: ?*c.thinkthen_batch = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_tag_batch_start(e.raw, q.raw, s.raw, &ctl, &r) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = r orelse return error.MissingNativeResult, .engine = e, .thread = std.Thread.getCurrentId() } };
}
pub fn scoreBatch(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(LazyBatch) {
    var r: ?*c.thinkthen_batch = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_score_batch_start(e.raw, q.raw, s.raw, &ctl, &r) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = r orelse return error.MissingNativeResult, .engine = e, .thread = std.Thread.getCurrentId() } };
}
pub fn filterBatch(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(LazyBatch) {
    var r: ?*c.thinkthen_batch = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_filter_batch_start(e.raw, q.raw, s.raw, &ctl, &r) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = r orelse return error.MissingNativeResult, .engine = e, .thread = std.Thread.getCurrentId() } };
}
pub fn annotateBatch(e: *tt.Engine, q: Question, s: Source, control: c.thinkthen_controls_v1) !Outcome(LazyBatch) {
    var r: ?*c.thinkthen_batch = null;
    var ctl = control;
    ctl.surface = counted("zig");
    if (c.thinkthen_annotate_batch_start(e.raw, q.raw, s.raw, &ctl, &r) != 0) return .{ .failed = try failure(e.allocator, e.raw) };
    return .{ .ok = .{ .raw = r orelse return error.MissingNativeResult, .engine = e, .thread = std.Thread.getCurrentId() } };
}
