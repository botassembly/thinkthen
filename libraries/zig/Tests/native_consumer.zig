const std = @import("std");
const tt = @import("thinkthen");
const n = tt.native;
const c = tt.c;
const io = @cImport({
    @cInclude("stdio.h");
});
extern fn native_print_summary(*const anyopaque) void;
extern fn native_print_details(*const anyopaque) void;
extern fn native_print_source_recognition(*const anyopaque) void;
extern fn native_print_source_relations(*const anyopaque) void;
extern fn native_print_question_author(*const anyopaque) void;
extern fn native_print_observation(*const anyopaque) void;
extern fn native_print_row_observation(*const anyopaque) void;
fn key(s: [:0]const u8) void {
    _ = io.printf(",\"%s\":[", s.ptr);
}
fn emit(s: *const n.Snapshot) !void {
    _ = io.fputs("{\"summary\":", io.stdout);
    native_print_summary(&s.summary);
    key("rows");
    for (s.rows, 0..) |row, i| {
        if (i != 0) _ = io.putchar(44);
        switch (row.function) {
            1 => _ = try s.decide(i),
            2 => _ = try s.choose(i),
            3 => _ = try s.tag(i),
            4 => _ = try s.score(i),
            5 => _ = try s.filter(i),
            6 => _ = try s.rank(i),
            7 => _ = try s.find(i),
            8 => _ = try s.annotate(i),
            9 => _ = try s.recognize(i),
            10 => _ = try s.relate(i),
            else => return error.InvalidFixtureFunction,
        }
        native_print_row_observation(&row);
    }
    _ = io.putchar(93);
    inline for (.{ "observations", "details", "observation_details", "authors", "observation_authors", "located_recognition", "located_relations" }) |field| {
        key(field);
        for (@field(s, field), 0..) |v, i| {
            if (i != 0) _ = io.putchar(44);
            if (comptime std.mem.eql(u8, field, "observations")) native_print_observation(&v) else if (comptime std.mem.eql(u8, field, "details") or std.mem.eql(u8, field, "observation_details")) native_print_details(&v) else if (comptime std.mem.eql(u8, field, "authors") or std.mem.eql(u8, field, "observation_authors")) native_print_question_author(&v) else if (comptime std.mem.eql(u8, field, "located_recognition")) native_print_source_recognition(&v) else native_print_source_relations(&v);
        }
        _ = io.putchar(93);
    }
    key("member_authors");
    for (s.member_authors, 0..) |members, i| {
        if (i != 0) _ = io.putchar(44);
        _ = io.putchar(91);
        for (members, 0..) |v, j| {
            if (j != 0) _ = io.putchar(44);
            native_print_question_author(&v);
        }
        _ = io.putchar(93);
    }
    _ = io.putchar(93);
    key("rank_member_details");
    for (s.rank_member_details, 0..) |members, i| {
        if (i != 0) _ = io.putchar(44);
        _ = io.putchar(91);
        for (members, 0..) |v, j| {
            if (j != 0) _ = io.putchar(44);
            native_print_details(&v);
        }
        _ = io.putchar(93);
    }
    _ = io.putchar(93);
    key("rank_members");
    for (s.rank_members, 0..) |members, i| {
        if (i != 0) _ = io.putchar(44);
        _ = io.putchar(91);
        for (members, 0..) |v, j| {
            if (j != 0) _ = io.putchar(44);
            var row = std.mem.zeroes(c.thinkthen_row_observation_v1);
            row.function = 6;
            row.data.rank = v;
            native_print_row_observation(&row);
        }
        _ = io.putchar(93);
    }
    _ = io.putchar(93);
    _ = io.puts("}");
}
fn take(comptime T: type, value: n.Outcome(T)) !T {
    return switch (value) {
        .ok => |v| v,
        .failed => |v| blk: {
            var failure = v;
            defer failure.deinit();
            _ = try failure.kind();
            try emit(&failure);
            break :blk error.EmittedNativeFailure;
        },
    };
}
fn get(v: std.json.Value, key_name: []const u8) std.json.Value {
    return if (v == .object) v.object.get(key_name) orelse .null else .null;
}
fn string(v: std.json.Value) ![]const u8 {
    return if (v == .string) v.string else error.MissingFixtureString;
}
fn number(v: std.json.Value) !u32 {
    return if (v == .integer) std.math.cast(u32, v.integer) orelse error.InvalidFixtureInteger else error.InvalidFixtureInteger;
}
fn flag(v: std.json.Value, key_name: []const u8) bool {
    const value = get(v, key_name);
    return value == .bool and value.bool;
}
fn cancelOnInput(token: *tt.CancelToken) void {
    if (io.getchar() != 33) @panic("missing fixture cancellation");
    token.cancel();
    _ = io.puts("cancel-fired");
    _ = io.fflush(io.stdout);
}
fn ownedFailure(allocator: std.mem.Allocator, engine: *tt.Engine) !void {
    const original = engine.allocator;
    engine.allocator = allocator;
    defer engine.allocator = original;
    switch (try n.parse(engine, .atomic, "{")) {
        .ok => |question| {
            question.deinit();
            return error.ExpectedConstructorFailure;
        },
        .failed => |value| {
            var failure = value;
            defer failure.deinit();
            try std.testing.expectEqual(tt.FailureKind.usage, try failure.kind());
            try std.testing.expect((try n.bytes(failure.summary.@"error".value.message)).len > 0);
            try std.testing.expectEqual(@as(usize, 0), failure.rows.len);
        },
    }
}
fn ownedImage(allocator: std.mem.Allocator, engine: *tt.Engine, data: []const u8, media: u32) !void {
    var copied = blk: {
        const image = try take(n.Image, try n.image(engine, data, media, "copied.png"));
        defer image.deinit();
        break :blk try image.view(allocator);
    };
    defer copied.deinit();
    try std.testing.expectEqualSlices(u8, data, copied.value.bytes[0..copied.value.bytes_len]);
    try std.testing.expectEqualStrings("copied.png", try n.bytes(copied.value.filename.value));
}
fn run() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const args = try std.process.argsAlloc(a);
    if (args.len != 3) return error.FixtureArgumentsRequired;
    const data = try std.fs.cwd().readFileAlloc(a, args[1], std.math.maxInt(usize));
    const parsed = try std.json.parseFromSlice(std.json.Value, a, data, .{});
    const v = parsed.value;
    const settings = try std.json.parseFromSlice(std.json.Value, a, args[2], .{});
    if (!std.mem.startsWith(u8, try string(get(settings.value, "base_url")), "http://127.0.0.1:")) return error.OwnedLoopbackRequired;
    const settings_z = try a.dupeZ(u8, args[2]);
    var engine = switch (try tt.Engine.initWithSettings(a, settings_z)) {
        .ok => |e| e,
        .failed => |f| {
            tt.releaseFailure(a, f);
            var failure = try n.failure(a, null);
            defer failure.deinit();
            _ = try failure.kind();
            try emit(&failure);
            return;
        },
    };
    defer engine.deinit();
    var memory = std.heap.DebugAllocator(.{}){};
    defer if (memory.deinit() != .ok) @panic("native owner leaked arena storage");
    try std.testing.checkAllAllocationFailures(memory.allocator(), ownedFailure, .{&engine});
    const role = std.meta.intToEnum(n.Role, try number(get(v, "role"))) catch return error.InvalidFixtureRole;
    const verb = try string(get(v, "verb"));
    const q: n.Question = blk: {
        if (flag(v, "find_none")) {
            var spec = std.mem.zeroes(c.thinkthen_question_spec_v1);
            spec.kind = 7;
            spec.none = 1;
            const t = try string(get(v, "find_text"));
            spec.text = if (flag(v, "find_text_literal")) n.text(t) else n.json(t);
            var author = std.mem.zeroes(c.thinkthen_question_author_v1);
            const metadata = get(v, "metadata");
            if (metadata == .object) {
                author.name = .{ .present = 1, .value = n.counted(try string(get(metadata, "name"))) };
                author.wording_version = .{ .present = 1, .value = try number(get(metadata, "wording_version")) };
            }
            break :blk try take(n.Question, try n.question(&engine, spec, &author));
        }
        const loader = get(v, "loader");
        if (loader == .string) {
            const ref = try string(get(v, "reference"));
            if (std.mem.eql(u8, loader.string, "load") or std.mem.eql(u8, loader.string, "file")) break :blk try take(n.Question, try n.load(&engine, ref));
            if (std.mem.eql(u8, loader.string, "load_named") or std.mem.eql(u8, loader.string, "named")) break :blk try take(n.Question, try n.named(&engine, role, ref));
            break :blk try take(n.Question, try n.reference(&engine, role, ref));
        }
        const form = get(v, "question_form");
        if (form == .string and std.mem.eql(u8, form.string, "file")) break :blk try take(n.Question, try n.load(&engine, "fixture-question.json"));
        break :blk try take(n.Question, try n.parse(&engine, role, try string(get(v, "question_json"))));
    };
    defer q.deinit();
    var token = try tt.CancelToken.init();
    defer token.deinit();
    var controls = n.controls();
    controls.attempts = 1;
    const op = get(get(v, "operation"), "injection");
    if (op == .string) {
        if (std.mem.eql(u8, op.string, "cancel_token")) {
            token.cancel();
            controls.cancel = token.raw;
        }
        if (std.mem.eql(u8, op.string, "expired_deadline")) controls.deadline_ms = 0;
    }
    const image_data = get(v, "image_data").array.items;
    const owners = try a.alloc(n.Image, image_data.len);
    var image_count: usize = 0;
    defer for (owners[0..image_count]) |image| image.deinit();
    const images = try a.alloc(?*const c.thinkthen_image, image_data.len);
    for (image_data, 0..) |hex, i| {
        const s = try string(hex);
        const bytes = try a.alloc(u8, s.len / 2);
        for (bytes, 0..) |*b, j| b.* = try std.fmt.parseInt(u8, s[j * 2 .. j * 2 + 2], 16);
        owners[i] = try take(n.Image, try n.image(&engine, bytes, try number(get(v, "media_code")), null));
        image_count += 1;
        try std.testing.checkAllAllocationFailures(memory.allocator(), ownedImage, .{ &engine, bytes, try number(get(v, "media_code")) });
        var copied = try owners[i].view(a);
        defer copied.deinit();
        if (!std.mem.eql(u8, bytes, copied.value.bytes[0..copied.value.bytes_len])) return error.ImageCopyChanged;
        images[i] = owners[i].raw;
    }
    const shared = get(v, "shared_context_bytes");
    if (shared == .string) controls.context = .{ .present = 1, .value = .{ .kind = try number(get(v, "shared_context_kind")), .data = n.counted(shared.string) } };
    const source: n.Source = blk: {
        const paths = get(v, "paths");
        if (paths == .array) {
            const values = try a.alloc(c.thinkthen_string_v1, paths.array.items.len);
            for (values, 0..) |*slot, i| slot.* = n.counted(try string(paths.array.items[i]));
            const unit = std.meta.intToEnum(n.Unit, try number(get(v, "source_unit"))) catch return error.InvalidFixtureUnit;
            const window = get(v, "window");
            break :blk try take(n.Source, try n.files(&engine, values, unit, if (window == .integer) @intCast(window.integer) else 0, flag(v, "image_reader")));
        }
        const originals = get(v, "items_bytes").array.items;
        const kinds = get(v, "items_kind").array.items;
        const values = try a.alloc(c.thinkthen_record_v1, originals.len);
        for (values, 0..) |*row, i| {
            row.* = std.mem.zeroes(c.thinkthen_record_v1);
            if (!flag(v, "image_only")) {
                var bytes = try string(originals[i]);
                if (flag(v, "caption_files")) bytes = try std.fs.cwd().readFileAlloc(a, try std.fmt.allocPrint(a, "caption-{d}.txt", .{i}), std.math.maxInt(usize));
                row.original = .{ .present = 1, .value = .{ .kind = try number(kinds[i]), .data = n.counted(bytes) } };
            }
            if (flag(v, "context_present")) row.context = .{ .present = 1, .value = .{ .kind = try number(get(v, "context_kind")), .data = n.counted(try string(get(v, "context_bytes"))) } };
            const orders = get(v, "candidate_orders");
            if (orders == .array) {
                const order = orders.array.items[i].array.items;
                const options = try a.alloc(c.thinkthen_choice_v1, order.len);
                for (options, 0..) |*option, j| {
                    option.* = std.mem.zeroes(c.thinkthen_choice_v1);
                    option.name = n.counted(try string(order[j]));
                }
                row.options = .{ .data = options.ptr, .len = options.len };
            }
            row.images = .{ .data = images.ptr, .len = images.len };
        }
        break :blk try take(n.Source, try n.records(&engine, values));
    };
    defer source.deinit();
    var cancellation: ?std.Thread = null;
    if (flag(v, "held_cancel")) {
        controls.cancel = token.raw;
        cancellation = try std.Thread.spawn(.{}, cancelOnInput, .{&token});
    }
    defer if (cancellation) |thread| thread.join();
    if (flag(v, "incremental")) {
        var batch = blk: {
            if (std.mem.eql(u8, verb, "decide")) break :blk try take(n.LazyBatch, try n.decideBatch(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "choose")) break :blk try take(n.LazyBatch, try n.chooseBatch(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "tag")) break :blk try take(n.LazyBatch, try n.tagBatch(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "score")) break :blk try take(n.LazyBatch, try n.scoreBatch(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "filter")) break :blk try take(n.LazyBatch, try n.filterBatch(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "annotate")) break :blk try take(n.LazyBatch, try n.annotateBatch(&engine, q, source, controls));
            return error.InvalidFixtureBatch;
        };
        defer batch.deinit();
        while (try take(?n.Snapshot, try batch.next())) |value| {
            var row = value;
            defer row.deinit();
            try emit(&row);
        }
        var facts = try take(n.Snapshot, try batch.facts());
        defer facts.deinit();
        try emit(&facts);
    } else {
        var result = blk: {
            if (std.mem.eql(u8, verb, "decide")) break :blk try take(n.Snapshot, try n.decide(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "choose")) break :blk try take(n.Snapshot, try n.choose(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "tag")) break :blk try take(n.Snapshot, try n.tag(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "score")) break :blk try take(n.Snapshot, try n.score(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "filter")) break :blk try take(n.Snapshot, try n.filter(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "rank")) break :blk try take(n.Snapshot, try n.rank(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "find")) break :blk try take(n.Snapshot, try n.find(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "annotate")) break :blk try take(n.Snapshot, try n.annotate(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "recognize")) break :blk try take(n.Snapshot, try n.recognize(&engine, q, source, controls));
            if (std.mem.eql(u8, verb, "relate")) break :blk try take(n.Snapshot, try n.relate(&engine, q, source, controls));
            return error.InvalidFixtureFunction;
        };
        defer result.deinit();
        try emit(&result);
    }
}
pub fn main() void {
    run() catch |err| {
        if (err == error.EmittedNativeFailure) return;
        std.debug.print("native fixture failed: {s}\n", .{@errorName(err)});
        std.process.exit(1);
    };
}
