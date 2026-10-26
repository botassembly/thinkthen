# ThinkThen in Zig

Methods ending in `One` return one row; named methods return pull Calls. A JSON array inside one Record remains one record. Deinit calls and clients explicitly; returned row allocations belong to the allocator you supply.

Check `thinkthen/README.md` inside the archive for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Use Zig 0.17.0. Extract the supplied platform-specific Zig 0.2.0 package. Keep its layout intact; the module needs its matching C headers and native engine.

Set `THINKTHEN_API_KEY` in the environment before creating a normal client. Keep keys out of code and reports. `tt.Client.init(.{}, &diagnostic)` selects the default backend. The stub needs no key or network and returns synthetic installation-check answers.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    const allocator = std.heap.page_allocator;
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    const bytes = try std.Io.Dir.cwd().readFileAlloc(init.io, "records.jsonl", allocator, .limited(16 * 1024 * 1024));
    defer allocator.free(bytes);
    var records: std.ArrayList(tt.Record) = .empty;
    defer records.deinit(allocator);
    var lines = std.mem.splitScalar(u8, bytes, '\n');
    while (lines.next()) |line| {
        const json = std.mem.trim(u8, line, " \t\r");
        if (json.len != 0) try records.append(allocator, .{ .json = json });
    }
    var diagnostic: tt.Diagnostic = .{};
    var client = try tt.Client.init(.{ .backend = .stub }, &diagnostic);
    defer client.deinit();
    const array = try client.decideOne(arena.allocator(), "Does the customer request a refund?",
        .{ .json = "[\"first\",\"second\"]" }, .{}, &diagnostic);
    switch (array.outcome) {
        .answered => std.debug.print("array answered\n", .{}),
        .unsure => std.debug.print("array unsure\n", .{}),
        .failed => |failure| std.debug.print("array failed {s}\n", .{failure.message}),
    }
    _ = arena.reset(.free_all);
    var rows = try client.decide(allocator, "Does the customer request a refund?",
        .{ .records = records.items }, .{ .evidence = "/text" }, &diagnostic);
    defer rows.deinit();
    while (try rows.next(arena.allocator(), &diagnostic)) |row| {
        switch (row.outcome) {
            .answered => |result| std.debug.print("{d} answered {}\n", .{row.position, result.answer}),
            .unsure => std.debug.print("{d} unsure\n", .{row.position}),
            .failed => |failure| std.debug.print("{d} failed {s}\n", .{row.position, failure.message}),
        }
        _ = arena.reset(.free_all);
    }
}
```

Run `zig init` in a new application directory. Add `.thinkthen = .{ .path = "../thinkthen" }` under dependencies in its generated build.zig.zon, with the relative path to the extracted package directory containing build.zig. Save the example above as src/main.zig. Replace build.zig with:

```zig
const std = @import("std");
const thinkthen = @import("thinkthen");
pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const dependency = b.dependency("thinkthen", .{ .target = target, .optimize = optimize });
    const executable = b.addExecutable(.{
        .name = "example",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .link_libc = true,
        }),
    });
    executable.root_module.addImport("thinkthen", dependency.module("thinkthen"));
    const installed = b.addInstallArtifact(executable, .{});
    thinkthen.installNative(b, dependency, installed);
    b.getInstallStep().dependOn(&installed.step);
}
```

Run `zig build`, then `./zig-out/bin/example`. installNative places the matching engine beside the executable.

## Records and the ten functions

Record is a tagged union: `.text` holds UTF-8 bytes, `.json` holds encoded JSON bytes and `.missing` marks a missing record. `Record.fromValue(allocator, value)` encodes a host value. Keep borrowed input bytes alive while the call uses them. Structured records can carry string, signed or unsigned integer IDs. Options.evidence takes a JSON Pointer such as `"/text"`; escape `/` as `~1` and `~` as `~0` inside keys.

Each named function returns a pull Call. It takes allocator, required question/domain arguments, Input, function-specific options and an optional Diagnostic pointer:

| Function | Required arguments before Input | Typed answer |
| --- | --- | --- |
| decide, filter | Wording | bool |
| choose | Wording, `[]const Choice` choices | UTF-8 string |
| tag | Wording, Choice labels | Slice of labels |
| score | Wording, Choice levels | f64 from 0 to 1 |
| rank | Wording, Choice levels | u64 rank |
| find | Wording | ?Span |
| recognize | Choice kinds | Slice of Name |
| relate | RelationRule rules | Slice of Relation |
| annotate | AnnotationQuestions | Slice of NamedAnnotation |

For example `client.choose(allocator, "Which topic?", &.{.{ .name = "billing" }, .{ .name = "shipping" }}, input, .{}, &diagnostic)`. Recognize and relate have no invented question prompt. RelationRule uses `name`, `source`, `target` and optional `reads` and `either`.

All ten methods have `One` forms such as `decideOne` taking one Record instead of Input. Typed calls use `client.open(.decide, allocator, question, input, &diagnostic)` or `client.one(...)`. Typed questions retain lists and options.

Decide, choose and filter accept `.threshold = .{ .cut = 0.8 }` or `tt.Threshold.band(0.2, 0.8)`. Tag, find, recognize and relate take f64 cuts only. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winning choice to reach the upper cut. Score and rank levels go lowest to highest. Rank puts highest scores first; options.top limits successful ranks and retains failures.

A rank row's answer is its rank; `position` is its input order. Filter returns FilterRow with original `record`, typed `row` and `kept()`. Every filter outcome appears.

## Pull rows, questions and files

Input is `.records = records` for a finite slice or `.source = tt.RecordSource.from(&source)` for a source implementing `next() error{RecordSource}!?Record`. Prefer one call over the collection so batching works. Settings.batch controls packing, with byte and backend limits also applied. One record may need several backend questions.

```zig
var call = try client.decide(allocator, "Does the customer request a refund?",
    .{ .records = &records }, .{}, &diagnostic);
defer call.deinit();
while (try call.next(arena.allocator(), &diagnostic)) |row| {
    switch (row.outcome) {
        .failed => return error.FailedRecord,
        else => {},
    }
    // Save or copy needed row data before resetting its arena.
    _ = arena.reset(.free_all);
}
```

A row's allocated strings and slices belong to the allocator passed to next. Arena reset invalidates them. Filter original records are copied into that row allocation. Rows retain exact RecordId, input position, source and outcome. Source identifies the question, backend and served model. `.answered` carries answer, optional probability and typed detail; `.unsure` carries probability and detail; `.failed` carries Failure. Detail holds labels, levels, recognition pieces or relation pairs. Find can answer null. Spans use UTF-8 byte offsets from zero with an exclusive end. Find selects a whole nonblank line, so preserve line breaks.

Toolbox holds NamedQuestion entries. Use `toolbox.get(name)` and then `entry.as(.decide)` for a typed question. AnnotationQuestions holds NamedAnnotationQuestion entries and excludes rank. `Toolbox.fromJson` loads saved question entries and `toJson` saves them. Shell runs remain a command feature. Inspect each named annotation's tagged function outcome; an answered outer annotation can contain failed children.

Use Zig's file and serialization APIs to read records and save results. Rows implement jsonStringify for contract-row export, including typed answers and details. Preserve exact IDs and input positions. The SDK has no spreadsheet reader or writer.

Call `relate` or `relateOne` alone when you need names and relations from the same evidence. The answered outcome's `detail.names` contains all admitted names, including names with no returned relation; `detail.pairs` contains scored relation pairs. Read accepted relations from its answer. A preceding recognize call is unnecessary.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```zig
const toolbox_json = try std.Io.Dir.cwd().readFileAlloc(
    init.io, "toolbox.json", allocator, .limited(16 * 1024 * 1024));
defer allocator.free(toolbox_json);
var box = try tt.Toolbox.fromJson(allocator, toolbox_json, &diagnostic);
defer box.deinit();
const encoded = try box.toJson(allocator, &diagnostic);
defer allocator.free(encoded);
const entry = box.get("topic") orelse return error.MissingQuestion;
const topic = entry.as(.choose) orelse return error.WrongQuestionKind;
var choices = try client.open(.choose, allocator, topic, .{ .records = records.items }, &diagnostic);
defer choices.deinit();
while (try choices.next(arena.allocator(), &diagnostic)) |row| {
    switch (row.outcome) {
        .answered => |value| std.debug.print("{s}\n", .{value.answer}),
        .unsure => std.debug.print("unsure\n", .{}),
        .failed => return error.FailedRecord,
    }
    _ = arena.reset(.free_all);
}
```

This code belongs inside the first example's main function with its allocator, client, records and diagnostic. Save `encoded` with your application's file IO. Loaded entries borrow the Toolbox owner; keep it alive until all selected calls finish, then call `deinit()`. The JSON returned by `toJson` belongs to the supplied allocator. `box.get("ready")` followed by `entry.as(.filter)` selects a loaded filter question for `client.open(.filter, ...)`.

## Failures and cancellation

Record and backend failures return `.failed` rows, including One calls. They do not become Zig errors automatically. Stop and deinit the call when the workload requires stopping after failure, and inspect annotation children too. Call-level errors use tt.Error; Diagnostic carries the stable code and message. Diagnostic and returned row allocations have separate ownership.

`call.cancel()` requests cancellation; the next read returns a cancellation error. Always `defer call.deinit()` and `defer client.deinit()`. Make each question unambiguous: define the evidence criterion, distinguish absence from explicit negatives and use distinct choices. The stub checks installation, not question quality.
