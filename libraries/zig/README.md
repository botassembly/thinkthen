# ThinkThen for Zig

This Zig 0.15.2 source module wraps the ThinkThen C library. The package gate proves Ubuntu 24.04 x86_64 glibc through its bundled static C library. Static-C linkage does not make the executable fully static. Static mode builds the consumer with LLVM and LLD, because Zig 0.15.2's own linker drops the 16-byte alignment of Rust's constants, and the flags go away once Zig fixes it (`sdlc/issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md`). Other targets and Zig versions remain unproved.

## Recommended development session API

The unpublished 0.2 package bundles the generated C header and static engine under `native/x86_64-unknown-linux-gnu`. A downstream build calls `linkNative(b, exe, module, null, .static)`; the module's dependency root supplies the native files. Installation and the first call need no Rust build or manual library path. Source developers may still pass an explicit native root.

`thinkthen.session` provides the ten named functions over native sessions. Each function accepts its generated `inputs.RequestCallDecide`, `RequestCallChoose` or corresponding descriptor. Questions, evidence selectors, authored definitions and options use generated tagged unions, structs and optionals. The facade omits absent options and preserves present false, zero and null. Rust admits every request; Zig adds no grammar or semantic validation. `Session.push` accepts a generated `inputs.RequestSessionDescriptor`, which contains its `item` and optional physical `location`. `Session.read()` returns `.pending`, `.end` or an owned `.packet`; `cancel`, `finish`, `push` and `deinit` use native session ownership. Immediate refusals use Zig error unions and the borrowed `session.message()` diagnostic. Terminal packets retain typed failures and final facts independently of immediate errors.

`Question.init(engine, role, definition)` sends a generated `inputs.RequestDefinition` through native authored-question admission and returns an owned question or a copied admission `Failure`. `Question.author()` borrows the generated native author view until question destruction. The explicit role selects the native grammar.

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

`Tests/session.zig` demonstrates an installed typed caller. `python3 libraries/zig/Tests/installed-session.py PATH_TO_ZIG_ARCHIVE` checks no-send preview, authored admission, retained nested row/failure facts and usage persistence. `check.sh` runs the shared routine cases through a downstream consumer linked only to the installed static engine. Each complete typed field is compared with its native packet after session and engine destruction. `THINKTHEN_TEST_PROFILE=full` selects the full shared inventory at the release candidate.

`Engine.usagePersistence()` observes live usage persistence without waiting. `Engine.finishUsageStatus()` drains this engine's current deltas. Both return an owned `UsageStatus` with a `UsagePersistence` state and optional copied advice. Release it with `status.deinit(allocator)` using the engine allocator. Status values survive engine destruction; both methods return `error.ClosedEngine` afterward. Written covers only current deltas; failed stays latched and does not turn a good answer into a call failure. Join callers before engine destruction.

`Engine.init` and `initWithSettings` return an owned engine or copied admission `Failure`. Use `releaseFailure(allocator, failure)` after its fields are unused. `initWithSettings` takes one NUL-terminated settings JSON object; native Rust validates it. Session operation errors use the six native Zig errors and `session.message()`. An execution failure belongs to an owned terminal packet and retains its failure facts.

## Upgrade from the earlier Zig APIs

| Earlier call | Current call |
| --- | --- |
| `engine.decide`, `decideMany`, `recognize`, `relate`, `call` and `files` | The ten `session.FUNCTION` calls with generated question and input descriptors |
| `engine.plan` | `Request.init(...).plan(engine)` |
| `complete` builders and readers | `inputs` descriptors and `Packet.view` |
| `native` complete calls, result snapshots and lazy batches | `session` calls, `Session.read` and owned packets |
| JSON facts and `readField` | Generated fields under `Packet.view`, with `session.presence` or `optional` |
| Old cancellation tokens | `Session.cancel()` |

The older Zig families have been removed. The C library retains its frozen compatibility exports.

## Build from source

Build the matching native library, stage its generated header and static archive in an absolute native root, and pass that root to the module build:

```sh
cargo build --locked --offline --manifest-path libraries/c/Cargo.toml --lib
mkdir -p /tmp/thinkthen-native/include /tmp/thinkthen-native/lib
cp libraries/c/include/thinkthen.h /tmp/thinkthen-native/include/
sh libraries/c/localize.sh libraries/c/target/debug/libthinkthen_c.a /tmp/thinkthen-native/lib/libthinkthen.a
cd libraries/zig
zig build -Dnative=/tmp/thinkthen-native -Dlink-mode=static
```

A downstream project adds this package as a dependency named `thinkthen`, imports `dep.module("thinkthen")` and calls `linkNative(b, exe, module, null, .static)`. The package supplies its own header and engine. `Tests/session-build.zig` demonstrates these build lines; `examples/decide.zig` demonstrates the typed call. Configure backend environment settings before intentionally running a live example. The installed gate uses synthetic loopback responses.

Zig 0.15.2's own linker can misalign Rust constants, so `linkNative` selects LLVM and LLD for static C linkage. This does not make the executable fully static. Linux checks close the migration; other targets remain release qualification work under the [native package design](../../sdlc/decisions/2026-10-09-native-package-design.md).
