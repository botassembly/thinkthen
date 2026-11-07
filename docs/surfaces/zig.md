# Zig functions that answer questions about text

ThinkThen is a classifier you call like a function: it reads text, answers one bounded question, and returns a typed Zig value. Install it with `zig fetch --save=thinkthen https://github.com/botassembly/thinkthen/releases/download/v0.2.0/thinkthen-zig-0.2.0-linux-x86_64.tar.gz` (or the `linux-arm64` or `macos-arm64` archive), and set `THINKTHEN_API_KEY` in the environment ([services and keys](service.md)).

## Read a support inbox

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();
    const question = "Does the customer request a refund?";

    // A JSON array is one record, so this asks about the whole thread at once.
    const thread = try client.decide(init.arena.allocator(), question,
        .{ .json = "[\"Where is my parcel?\",\"Please refund my order.\"]" }, .{}, &diag);
    try stdout.interface.print("array {t}\n", .{thread.status()});

    // In JSON Lines, each line is its own record and gets its own row, in input order.
    const inbox = try std.Io.Dir.cwd().readFileAlloc(init.io, "records.jsonl", init.gpa, .limited(16 << 20));
    defer init.gpa.free(inbox);
    var refunds = try client.decideAll(init.gpa, question, .{ .json_lines = inbox }, .{ .field = "/text" }, &diag);
    defer refunds.deinit();
    while (try refunds.next(&diag)) |refund| {
        switch (refund.outcome) {
            .answered => |decision| try stdout.interface.print("{d} answered {}\n", .{ refund.position, decision.answer }),
            .unsure => try stdout.interface.print("{d} unsure: send it to a person\n", .{refund.position}),
            .failed => |failure| try stdout.interface.print("{d} failed {s}: {s}\n", .{ refund.position, failure.name(), failure.message }),
        }
    }
}
```

Save the inbox beside it as `records.jsonl`:

```json
{"id":"a1","text":"Please refund my order."}
{"id":"a2","text":"Where is my parcel?"}
null
{"id":"a4"}
[{"text":"first"},{"text":"second"}]
{"id":"a6","text":null}
```

<!-- Output: fill from the real-backend capture run. -->

Every line gets a row, in order. Lines 3 to 5 hold no text at `/text`, so each fails in its place and reading goes on. A no is `answered false`, never a missing row.

## Add it to your build

Each ThinkThen release supports exactly the Zig version in its `build.zig.zon`, which is Zig 0.17.0 for ThinkThen 0.2.0.

`zig fetch --save` adds the dependency to `build.zig.zon`. Import the module and install the engine beside your executable in `build.zig`:

```zig
const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn build(b: *std.Build) !void {
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
    try thinkthen.installNative(b, dependency, installed);
    b.getInstallStep().dependOn(&installed.step);
}
```

Run `zig build`, then `./zig-out/bin/example`.

## Get one answer

Each function has a one-record form named for the function, such as `decide`, and a many-record form ending in `All`, such as `decideAll`. A one-record call returns a row, and `answer` turns the row into its answer in one step:

```zig
const is_refund = try (try client.decide(init.arena.allocator(), "Does the customer request a refund?",
    .{ .text = "Please refund my order." }, .{}, &diag)).answer(&diag);
try stdout.interface.print("{}\n", .{is_refund});
```

The examples from here on go inside a `main` like the first one, with its `stdout`, `diag` and `client`. A one-record call's row lives in the allocator you pass, which must be an arena; `init.arena` lasts until the program exits.

## Send not sure to a person

`answer` returns `error.Unsure` when the model is not sure, and the record's error when it failed: `error.Record`, `error.Backend` and the rest of `tt.AnswerError`. It never turns either into an answer. Catch them to handle each case:

```zig
const messages = [_][]const u8{ "Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice." };
var refunds = try client.decideAll(init.gpa, "Does the customer request a refund?", .{ .texts = &messages }, .{}, &diag);
defer refunds.deinit();
while (try refunds.next(&diag)) |refund| {
    const message = messages[@intCast(refund.position - 1)];
    const is_refund = refund.answer(&diag) catch |err| switch (err) {
        error.Unsure => {
            try stdout.interface.print("{s}: not sure, send it to a person\n", .{message});
            continue;
        },
        else => {
            try stdout.interface.print("{s}: failed, {s}\n", .{ message, diag.message() });
            continue;
        },
    };
    try stdout.interface.print("{s}: {}\n", .{ message, is_refund });
}
```

After `answer` returns an error, `diag` holds the row's `position`, and either its `probability` (not sure) or its `code` and `message()` (failed). A `switch` on `row.outcome` handles `.answered`, `.unsure` and `.failed` together, as the first example does. `row.isUnsure()`, `row.isFailed()`, `row.status()` and `row.probability()` read the rest.

Print a row with `{f}` to see its outcome and probability, such as `true (0.97)`, `unsure (0.50)` or `failed record.missing: ...`.

A client or call that can't start, such as one with no key or an invalid question, returns its error directly, with the code and message in `diag`, and the first example's `errdefer` line prints that message on stderr.

## The ten functions

### decide: answer a yes-or-no question

The answer is a `bool`.

```zig
const is_refund = try (try client.decide(init.arena.allocator(), "Does the customer request a refund?",
    .{ .text = "Please refund my order." }, .{}, &diag)).answer(&diag);
try stdout.interface.print("{}\n", .{is_refund});
```

### choose: pick one option

The answer is one of your labels. `tt.choices` builds the label list.

```zig
const topic = try (try client.choose(init.arena.allocator(), "What is the message about?",
    tt.choices(&.{ "billing", "shipping", "other" }), .{ .text = "Please refund my order." }, .{}, &diag)).answer(&diag);
try stdout.interface.print("{s}\n", .{topic});
```

### tag: label every fit

The answer is every label that fits, which may be none.

```zig
const topics = try (try client.tag(init.arena.allocator(), "Which topics appear?",
    tt.choices(&.{ "billing", "shipping" }), .{ .text = "I was charged twice and my parcel is late again." }, .{}, &diag)).answer(&diag);
for (topics) |topic| try stdout.interface.print("{s}\n", .{topic});
```

### score: measure along levels

Name the levels lowest first. The answer is an `f64` from 0 to 1 along them.

```zig
const urgency = try (try client.score(init.arena.allocator(), "How urgent is the request?",
    tt.choices(&.{ "routine", "urgent" }), .{ .text = "Please refund my order." }, .{}, &diag)).answer(&diag);
try stdout.interface.print("{d}\n", .{urgency});
```

### filter: keep the records that pass

`filterAll(...).next` yields only the kept records. Not-sure records are left out, and a failed record returns its error in its place, after which reading can go on.

```zig
const messages = [_][]const u8{ "Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice." };
var refunds = try client.filterAll(init.gpa, "Does the customer request a refund?", .{ .texts = &messages }, .{}, &diag);
defer refunds.deinit();
while (try refunds.next(&diag)) |refund| try stdout.interface.print("{s}\n", .{refund.text});
```

To see every row, kept or not, read with `nextRow`, which yields each record with its row. The one-record `client.filter` returns the record when kept and `null` when not, and returns `error.Unsure` when not sure.

### rank: order records by levels

Rows come highest first. The answer is the rank, starting at 1, and `position` is the record's place in your input.

```zig
const messages = [_][]const u8{ "Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice." };
var by_urgency = try client.rankAll(init.gpa, "How urgent is the request?",
    tt.choices(&.{ "routine", "urgent" }), .{ .texts = &messages }, .{}, &diag);
defer by_urgency.deinit();
while (try by_urgency.next(&diag)) |ranked| {
    const rank = try ranked.answer(&diag);
    try stdout.interface.print("{d} {s}\n", .{ rank, ranked.item.text });
}
```

`ranked.item` is the record you passed in, as a `tt.Record`, so `.text` is a text record's text and `.json` a JSON record's bytes. It lives in the row's memory: from `next`, until the next read or `deinit`; from `nextInto(arena, &diag)`, until you free that arena. To keep it longer, copy it with `ranked.item.copy(allocator)`. `.top = 3` in the options keeps only the three highest.

### find: point to the line that answers

The answer is a `?tt.Span`: the line's text with its UTF-8 byte offsets, or `null` when no line answers.

```zig
const action = try (try client.find(init.arena.allocator(), "Which line states the requested action?",
    .{ .text = "Morgan works for Example Group.\nPlease refund my order." }, .{}, &diag)).answer(&diag);
if (action) |line| try stdout.interface.print("{s}\n", .{line.text}) else try stdout.interface.print("no line answers\n", .{});
```

### annotate: fill in a form

Give each question a name, then read each answer by its function and name. The form can't hold rank or annotate.

```zig
const form: tt.AnnotationQuestions = .{ .entries = &.{
    .{ .name = "refund", .question = .{ .decide = .{ .question = "Does the customer request a refund?" } } },
    .{ .name = "topic", .question = .{ .choose = .{ .question = "What is the message about?", .choices = tt.choices(&.{ "billing", "shipping", "other" }) } } },
    .{ .name = "urgency", .question = .{ .score = .{ .question = "How urgent is the request?", .levels = tt.choices(&.{ "routine", "urgent" }) } } },
} };
const messages = [_][]const u8{ "Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice." };
var triage = try client.annotateAll(init.gpa, form, .{ .texts = &messages }, .{}, &diag);
defer triage.deinit();
while (try triage.next(&diag)) |filled| {
    const answers = try filled.answer(&diag);
    const is_refund = try answers.get(.decide, "refund").?.answer(&diag);
    const topic = try answers.get(.choose, "topic").?.answer(&diag);
    const urgency = try answers.get(.score, "urgency").?.answer(&diag);
    try stdout.interface.print("{} {s} {d}\n", .{ is_refund, topic, urgency });
}
```

`get` returns `null` only when the form has no question of that function and name. Each answer it returns can be not sure or failed on its own, and `answer` says so.

### recognize: pull out names

The answer is each name with its `kind`, `text`, byte offsets and probability. Recognize takes kinds, not a question.

```zig
const names = try (try client.recognize(init.arena.allocator(), tt.choices(&.{ "person", "organization" }),
    .{ .text = "Morgan works for Example Group." }, .{}, &diag)).answer(&diag);
for (names) |name| try stdout.interface.print("{s}: {s}\n", .{ name.kind, name.text });
```

### relate: link names

Each rule names a relation and the kinds at each end. The answer is each related pair. Relate finds the names itself, so it needs no recognize call first.

```zig
const works_for = [_]tt.RelationRule{.{ .name = "works_for", .source = "person", .target = "organization" }};
const relations = try (try client.relate(init.arena.allocator(), &works_for,
    .{ .text = "Morgan works for Example Group.\nPlease refund my order." }, .{}, &diag)).answer(&diag);
for (relations) |relation| try stdout.interface.print("{s} --{s}--> {s}\n", .{ relation.source.text, relation.relation, relation.target.text });
```

A rule can add `.reads = "works for"` for how it reads in a sentence and `.either = true` when it holds both ways. The row's detail lists every name found, including names with no relation.

## Many records

A many-record call takes an `tt.Input`:

- `.texts = &messages` for a slice of strings, one record each.
- `.json_lines = bytes` for JSON Lines. Blank lines are skipped.
- `.records = &records` for `tt.Record` values: `.text`, `.json` or `.missing`. `tt.Record.fromValue(allocator, value)` encodes any value `std.json` can write.
- `.source = tt.RecordSource.from(&source)` for input too large to hold, where `source` has `fn next(self) error{RecordSource}!?tt.Record`.

One call over the whole collection lets ThinkThen batch requests. An empty collection yields no rows and sends no request. Keep the input bytes alive until the call is done.

`next(&diag)` reuses the call's memory, so a row is valid until the next read or `deinit`. To keep rows longer, read with `nextInto(arena, &diag)` and they live in your arena instead. `call.cancel()` stops a call from any thread, and the read in progress returns `error.Cancelled`. Always `defer call.deinit()`.

## Options

Each function takes its own options struct as the argument after the record or input:

- `.field = "/text"`: a JSON Pointer to the text to judge in a JSON record.
- `.threshold`: for decide and filter, a cut such as `.{ .cut = 0.8 }` or a band such as `tt.Threshold.band(0.2, 0.8)`, which answers no below 0.2, yes above 0.8, and not sure between. Choose, tag, find, recognize and relate take one `f64` cut.
- `.name = "refund"`: names the question in each row's `source`.
- `.top`: rank only.
- `.yes_means` and `.no_means`: for decide and filter, what a yes and a no mean for this question.
- `.descriptions`: for choose, tag, score and rank, a `std.json.ArrayHashMap([]const u8)` from a label or level to what it means. Describe as many as you like; a key that is not one of the call's labels or levels returns `error.Usage`.

```zig
const is_refund = try (try client.decide(init.arena.allocator(), "Does the customer request a refund?",
    .{ .text = "Please refund my order." }, .{ .yes_means = "The customer wants money back.", .no_means = "The customer wants anything else." }, &diag)).answer(&diag);
try stdout.interface.print("{}\n", .{is_refund});

var urgency_levels: std.json.ArrayHashMap([]const u8) = .{};
try urgency_levels.map.put(init.arena.allocator(), "urgent", "The customer is out of money or waiting on a late order.");
const urgency = try (try client.score(init.arena.allocator(), "How urgent is the request?",
    tt.choices(&.{ "routine", "urgent" }), .{ .text = "Please refund my order." }, .{ .descriptions = urgency_levels }, &diag)).answer(&diag);
try stdout.interface.print("{d}\n", .{urgency});
```

## Save and load questions

A toolbox holds named questions. Save this as `toolbox.json`:

```json
{"entries":{"refund":{"question":{"function":"decide","question":"Does the customer request a refund?"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"]}},"urgency":{"question":{"function":"score","question":"How urgent is the request?","levels":["routine","urgent"]}}}}
```

Load it once. Ask one question by name with `open`, or every question at once with `annotate`:

```zig
const saved = try std.Io.Dir.cwd().readFileAlloc(init.io, "toolbox.json", init.gpa, .limited(1 << 20));
defer init.gpa.free(saved);
var toolbox = try tt.Toolbox.fromJson(init.gpa, saved, &diag);
defer toolbox.deinit();

const topic_question = (toolbox.get("topic") orelse return error.MissingQuestion).as(.choose) orelse return error.WrongQuestionKind;
const topic = try (try client.open(.choose, init.arena.allocator(), topic_question, .{ .text = "Please refund my order." }, &diag)).answer(&diag);
try stdout.interface.print("{s}\n", .{topic});

const form = try toolbox.annotation(init.gpa, &diag);
defer init.gpa.free(form.entries);
const answers = try (try client.annotate(init.arena.allocator(), form, .{ .text = "Please refund my order." }, .{}, &diag)).answer(&diag);
try stdout.interface.print("{}\n", .{try answers.get(.decide, "refund").?.answer(&diag)});
```

`openAll` asks a loaded question of many records. `toolbox.toJson(allocator, &diag)` writes the toolbox back out, and `annotation` returns `error.Usage` when the toolbox holds a rank question. Keep the toolbox alive until the calls that use its questions are done.

## Save rows

Rows write and read JSON through `std.json`, with their position, id, source and outcome: `std.json.Stringify.valueAlloc(allocator, row, .{})` to save, and `std.json.parseFromSlice(tt.Row(.decide), allocator, bytes, .{})` to load.

## Reference

Every public name carries a reference comment in [`sdk/zig/src/thinkthen.zig`](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/zig/src/thinkthen.zig) and the files it imports.
