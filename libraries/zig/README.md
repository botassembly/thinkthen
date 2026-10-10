# ThinkThen for Zig

This Zig 0.15.2 source module wraps the ThinkThen C library. The package gate proves Ubuntu 24.04 x86_64 glibc with a separately installed native shared or static C library. Static-C linkage does not make the executable fully static. Static mode builds the consumer with LLVM and LLD, because Zig 0.15.2's own linker drops the 16-byte alignment of Rust's constants, and the flags go away once Zig fixes it (`sdlc/issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md`). Other targets and Zig versions remain unproved.

## Recommended development session API

The unpublished 0.2 package bundles the generated C header and static engine under `native/x86_64-unknown-linux-gnu`. A downstream build calls `linkNative(b, exe, module, null, .static)`; the module's dependency root supplies the native files. Installation and the first call need no Rust build or manual library path. Source developers may still pass an explicit native root.

`thinkthen.session` provides the ten named functions over native sessions. Each function accepts its generated `inputs.RequestCallDecide`, `RequestCallChoose` or corresponding descriptor. Questions, evidence selectors, authored definitions and options use generated tagged unions, structs and optionals. The facade omits absent options and preserves present false, zero and null. Rust admits every request; Zig adds no grammar or semantic validation. `Session.push` accepts a generated `inputs.RequestSessionDescriptor`, which contains its `item` and optional physical `location`. `Session.read()` returns `.pending`, `.end` or an owned `.packet`; `cancel`, `finish`, `push` and `deinit` use native session ownership. Immediate refusals use Zig error unions and the borrowed `session.message()` diagnostic. Terminal packets retain typed failures and final facts independently of immediate errors.

`Question.init(engine, role, definition)` sends a generated `inputs.RequestDefinition` through native authored-question admission and returns an owned question or the existing typed native failure snapshot. `Question.author()` borrows the generated native author view until question destruction. The explicit role selects the native grammar.

`Request.init(allocator, call)` owns a serialized canonical request independently of caller buffers. `request.start(engine)` admits it as a native session. `request.plan(engine)` previews the same request and returns an owned `Plan` with generated fields under `plan.value`, including byte/token estimates and the optional first body. The plan owns its strings after engine and request destruction. `plan.json()` retains unknown extension members in the original native JSON. Release questions, requests and plans with `deinit()` after their borrowers finish. Preview currently supports native fixed atomic decide, choose, tag and score; native admission refuses the other functions and dynamic questions.

```zig
const request = try tt.Request.init(allocator, .{ .decide = .{
    .question = .{ .text = .{ .text = "Does it pass?" } },
    .input = .{ .text = .{ .text = "Evidence." } },
    .options = .{ .details = false },
} });
defer request.deinit();
var plan = try request.plan(&engine);
defer plan.deinit();
const records = plan.value.records;
```

A packet borrows the complete Rust-generated header graph through `packet.view`; all known fields and unknown extension entries remain accessible without a result JSON parser. `packet.terminal()`, `packet.facts()` and `packet.failure()` return optionals. `session.presence(field)` distinguishes missing, null and value; `session.optional(field)` is for fields that do not allow null. `session.bytes` exposes counted UTF-8. Free each packet with `deinit()` after all nested reads. Packets survive session and engine destruction.

`Tests/session.zig` and `Tests/session-build.zig` demonstrate an installed typed caller. The focused package check is `python3 libraries/zig/Tests/installed-session.py PATH_TO_ZIG_ARCHIVE`. It builds one downstream consumer, reads a typed plan without a key with zero sends, checks native authored admission and retained author metadata, then sends one synthetic loopback request and reads typed facts after session destruction. The older executable APIs and reachable preparation helpers remain until their replacement passes the shared release cases.

## Build from source

Build the matching native library from the same checkout, then supply an absolute native root to Zig:

```sh
cargo build --locked --offline --manifest-path libraries/c/Cargo.toml --lib
mkdir -p /tmp/thinkthen-native/include /tmp/thinkthen-native/lib
cp libraries/c/include/thinkthen.h /tmp/thinkthen-native/include/
cp libraries/c/target/debug/libthinkthen_c.so /tmp/thinkthen-native/lib/libthinkthen.so
sh libraries/c/localize.sh libraries/c/target/debug/libthinkthen_c.a /tmp/thinkthen-native/lib/libthinkthen.a
ln -s libthinkthen.so /tmp/thinkthen-native/lib/libthinkthen.so.0
cd libraries/zig
zig build -Dnative=/tmp/thinkthen-native -Dlink-mode=shared
```

In another Zig project, add this source folder as a dependency named `thinkthen`, import `dep.module("thinkthen")`, and call `@import("thinkthen").linkNative(b, exe, module, native, .shared)` in its build script. The `Tests/build.zig` installed consumer is an executed example of the dependency and native-link lines. `examples/decide.zig` shows the runtime API. Set `THINKTHEN_BASE_URL` and `THINKTHEN_API_KEY` only when intentionally running that example against a backend. The gate uses a synthetic loopback backend.

`Engine.init(allocator)` reads environment settings. `Engine.initWithSettings(allocator, settings)` accepts NUL-terminated JSON and passes it to the C constructor; invalid settings fail before sending. Free a constructor failure with `releaseFailure(allocator, failure)`. Calls return `.ok` or `.failed`, in addition to Zig allocation and validation errors. A failure has one of six named `kind` values and copied `message` and optional `facts_json` slices. Free it with `engine.freeFailure` while the engine exists, or `releaseFailure` with the original allocator after teardown. Free JSON `call` bytes with that allocator. The generic JSON call returns the C `{value,facts}` envelope. Each typed `.ok` is an owned `CallResult` with the former value in `.value` and the per-call facts in `.facts`, a `Json` (`std.json.Parsed(std.json.Value)`) that follows `specification/result.schema.json`; read `success.facts.value.object.get("records")` and ignore members you do not know. Call `success.deinit(allocator)` for every typed success, including scalar answers and after engine teardown. An absent usage member is simply missing from the object. `Outcome` carries the C values: `yes` 1, `no` 0, `unsure` 2. `readField(member)` reads one annotate member as `.unresolved` (JSON null), `.failed` with its `FailureKind` and cause, or `.answered` with the value, and returns `error.NotAField` for any other object. `engine.plan(verb, question, input, settings)` previews a `decide`, `choose`, `score` or `tag` call through `thinkthen_plan_json` with no key, cache read or send, and returns the result schema's `plan` object as a `Json` to `deinit`. A question starting with `{` is a question object and is sent as written; other text is the bare question. `settings` is null or one settings object; a spliced text that is not one JSON object is `error.NotJsonObject`. Every sending call takes `Options.deadline_ms`, and `initWithSettings` passes engine settings such as `{"max_requests_total":0}` through unchanged. Native failure values still use `releaseFailure`. Null inside a successful JSON value is not a failure. Inputs use counted evidence and refuse interior NUL in C strings. Keep engine and token owner structs un-copied, join in-flight calls before `deinit`, and use a thread-safe allocator across concurrent callers. Native cancellation is one shot: already-sent requests drain, the cancelled call returns no output, and recovery uses a fresh token.

`sh libraries/zig/check.sh` builds the current C door offline, checks all current header-derived exports, runs the public J1 corpus, and proves the exact 42-request body multiset through two isolated shared and two isolated static-C installed consumers. From a clean committed Linux x86-64 checkout, `sdlc/scripts/release-pack x86_64-unknown-linux-gnu OUT c swift zig` makes a local checksummed Zig source archive with a matching C archive. `sdlc/scripts/release-go-cpp-pair OUT swift-zig` validates the three-file set. Passing its Zig and C archive paths as `THINKTHEN_ARTIFACT` and `THINKTHEN_C_ARTIFACT` to `check.sh` builds an isolated consumer whose dependency points at the unpacked Zig package and proves three complete public bulk requests. These local artifacts are disposable. Each GitHub release ships the checksummed Zig source archive and the matching C archive for Linux x86-64.

`Engine.initWithSettings(allocator, settings)` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

Typed 0.2 host descriptors are available through `thinkthen.complete`. Ten named request builders prepare explicit questions/question files and record/file/image sources; known answer, metadata, fact, identity, location, span and edge fields have typed carriers. This is independent carrier preparation, not complete-call runtime parity. The host descriptor codec is not the native wire format. Existing engine calls remain the executable compatibility API. Adoption of the real 0426 constructors, complete calls, result views and failure snapshots is still required; no legacy result is promoted to result/2.

Descriptor slices borrow caller memory. `complete.clone` makes an independent deep copy in an owned parse arena; call its `deinit` after all borrowed fields are unused. `complete.readFacts` similarly owns decoded field storage.


## Complete native calls (0428 integration)

The additive complete API uses C's native question grammar and readers for all ten named functions. Bare and generic JSON calls remain available. Known result fields are typed; arbitrary original JSON is retained as counted content.

Question input is explicit: construct a typed descriptor, parse saved JSON with a loader role, load an exact file, load a configured name, or load a reference. The eight roles are atomic, question set, per-record choose, recognize, record relate, rank, rank set and find. A string is never guessed to be a path. Native declarations, named/versioned authors, record context, options, rank members, observations, answer/request/call IDs, final facts and owned failure snapshots remain distinct.

Records may carry explicit text or JSON, per-record context and ordered candidate replacements. Images use explicit PNG/JPEG media and preserve original compressed bytes, order and duplicates. Decide, choose and score admit native image routes; the other seven functions refuse before sending. File sources explicitly select line, window, whole file, image file or JSONL. Physical filename/line ranges and absent image line coordinates come from the native reader.

Complete results copy all borrowed native views before `thinkthen_result_free`. They survive destruction of the engine, question, source and input buffers. Presence remains explicit, including absent aggregate IDs/meta, nullable selections and independently unknown reported token dimensions. Cost strings are copied without floating-point conversion. Failures retain the six native kinds, safe messages, available stop details and final facts/attempts.

Decide, choose, tag, score, filter and annotate also have owned lazy native batches. Start/next/facts/free belong to the creating thread; keep the engine live until the batch closes. Starting clones question/source/context/cancellation state. Returned rows own independent snapshots. Rank, find, recognize and relate retain aggregate complete calls. No host parser, cache, scheduler or model-routing policy is added.

This is an unpublished integration API. The family ticket and root review own final qualification; the shared consumer reports actual failures instead of counting generic JSON as typed parity.

Import `thinkthen.native`. Its `question`, `parse`, `load`, `named`, `reference`, `records`, `files` and `image` constructors return `Outcome` values with owned success handles or a failure `Snapshot`. `Role` and `Unit` select the exact native grammar/reader. The ten named calls return an owned `Snapshot`; its checked named getters expose typed C layouts backed by the snapshot arena, with detail/author/observation/rank/located arrays. No known result is decoded through JSON. `Snapshot.kind()` maps a failure to the six-language enum.

Call `deinit()` once for each success handle, image view and snapshot, after every borrowed field/slice is unused. Counted descriptors borrow caller buffers through construction only. `image.view(allocator)` returns independently owned bytes and properties. The six `FUNCTIONBatch` functions return `LazyBatch`; its engine must stay live through `deinit`, and start/next/facts/deinit stay on the creating thread. Each yielded snapshot is independent of that batch.

Rank-set rows retain every member in saved declaration order. Each member exposes its native positive rank position, probability, answer identity, author declarations and complete details. Details preserve independently reported token dimensions and source batch sizes. Parent and member metadata overlap; read final call facts for invocation usage.

`Engine.usagePersistence()` observes live usage persistence without waiting. `Engine.finishUsageStatus()` drains this engine’s current deltas. Both return an owned `UsageStatus` with a `UsagePersistence` state and optional copied native advice. Written covers current deltas only; later calls and other engines can write more. Failed is latched and does not turn a good answer into a call failure. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Observation never derives state from earlier call facts.
Zig callers release status advice with `status.deinit(allocator)` using the engine’s original allocator. Status values survive engine destruction; both methods return `error.ClosedEngine` afterward. Join callers before destroying an engine.
