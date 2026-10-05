# The C door's design

The C door serves every language that can call a C library (ADR 0037, Ian: "Whatever you need, let's support everything through C"). The library team owns its ABI. Ticket 0094 ported it from the retired `surfaces` branch (tag `surfaces-wave7-final`) onto the public API. This page keeps the branch's options and ownership design and records what the port changed.

The shape: one engine value built from the environment, one JSON door that carries any request, typed doors for the hot paths, and JSON text for every result of open size. The drawn slide's four plain signatures stay frozen. Each control sits on an `_opts` twin beside them.

## The header table

The header adds a borrowed failure-facts accessor beside its existing error functions. Ticket 0148 added the settings constructor before 0.1 shipped.

| Symbol | Verdict | Change |
|---|---|---|
| `thinkthen_engine_new` | changed | Builds through `Engine::from_env`, the constructor `default_engine` uses. It reads the variables that constructor reads. `THINKTHEN_TIMEOUT_SECS` and `THINKTHEN_MAX_RETRIES` are not read. NULL means the environment settings are invalid. A cache folder or configuration file the engine cannot read also returns NULL, as the local kind. The calling thread keeps that failure in its own slot; the error accessors called with NULL name it until the thread's next engine builds. No throttle argument and no throttle variable exist. |
| `thinkthen_engine_new_with` | added | Reads one closed JSON settings object over `EngineBuilder::from_env`. The keys include `base_url`, `model`, `throttle`, `max_requests`, `max_request_bytes`, `cache`, `timeout`, `max_retries`, `profile`, `batch`, `record`, and `replay`; `api_key` is refused. Null or `{}` matches `thinkthen_engine_new`. Wrong types, unknown or repeated keys, and builder refusals enter the calling thread's null-engine error slot. |
| `thinkthen_engine_free` | kept | |
| `thinkthen_error_message` | kept | Messages come from the engine's `Error` display text. |
| `thinkthen_error_code` | kept | Codes 1 to 6 map from `ErrorKind` in the header's order. |
| `thinkthen_error_retryable` | changed | Returns the engine's retryable signal under ticket 0089's rule: a retried status earns 1, and a transport failure, a 401, and every kind but backend earn 0. |
| `thinkthen_error_facts_json` | added | Borrows the last started failure's final facts on this thread and engine; NULL means absent. Its pointer follows the message accessor's lifetime. |
| `thinkthen_cancel_token_new`, `thinkthen_cancel`, `thinkthen_cancel_token_free` | kept | The handle wraps `CancelToken` and fires from any thread. |
| `thinkthen_decide`, `thinkthen_decide_opts` | kept | `question_json` goes through `Question::from_json`. `deadline_ms` stays `int64_t` and goes through `CallOptions::deadline_millis` (ADR 0041). |
| `thinkthen_question_file` | added | A bounded named-file read validates through `Question::from_json`, returns owned original JSON through two unchanged-on-error outputs, and records non-retryable Local on a file failure. It sends nothing. |
| `thinkthen_plan_json` | added | Ticket 0291: one closed `thinkthen.plan-input/1` object (`verb`, `question`, `input`, optional `settings`) goes through the core settings parser and `Engine::plan_with`. It returns the crate's serialized `PlanEstimate`, the result schema's `plan` definition, through two unchanged-on-error outputs. It reads no key or cache and sends nothing. |
| `thinkthen_decide_many`, `thinkthen_decide_many_opts` | kept | A cancelled or expired call still returns no rows. |
| Four typed `*_with_facts` pairs | added | Decide, decide-many, recognize and relate each have plain and options forms. A successful call returns a separate owned final-facts JSON string beside its unchanged typed result. Existing typed names remain bare compatibility calls. A nonzero code changes no output slot; started failures keep the borrowed error-facts accessor. |
| `thinkthen_call`, `thinkthen_call_opts` | changed | The envelope grammar below replaces the branch's eight verbs. |
| `thinkthen_recognize`, `thinkthen_recognize_opts` | changed | The spec is a version-one question file. Entities take the engine's `{text, start, end, length, kind, strength}` through `Recognized::to_json`. Recognize follows the engine's cache and replay settings like every call. |
| `thinkthen_relate`, `thinkthen_relate_opts` | changed | The spec is a version-one relate file read by `Relate::from_json`. Each text is one JSON record with `name` and `kind` at the default fields. A record with `text` and no `name`, as `recognize` writes, is read by its `text`. A non-default `fields` pointer is refused as usage. Edges take `Edge::to_json`'s shape inside `{"edges":[...]}`. The 255 cap stays. |
| `thinkthen_free_string` | kept | |

The version macros equal the `thinkthen-c` crate version. `tests/door/main.rs` compares the exported symbols and the macros with the header.

## The JSON door's envelope

A request names one verb of ten: `decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`, `recognize`, or `relate`. Beside the verb, the door reads six envelope keys:

| Key | Verbs | Value |
|---|---|---|
| `evidence` | `decide`, `choose`, `score`, `tag`, `recognize` | One string |
| `records` | Four judgments, `filter`, `rank`, `annotate`, `relate` | An array of strings; for `relate`, an array of JSON records |
| `units` | `find` | An array of strings |
| `details` | the four judgments | `true` or `false` |
| `usage` | alone | `true` |
| `call` | eligible record arrays | A closed object with `batch` (`"max"` or a positive integer) and optional shared text `context` where supported |

Every other key forms the question object. `Question::from_json` validates it, so an unknown question key is a usage failure there. `filter` asks its text as a `decide` question at a single cut. `rank` takes its text alone. `find` takes its text and an optional `"none": true`, which offers a none candidate. `annotate` carries the question set as its value, and a member's `on` reads that part of each record as JSON text.

The `value` inside a successful `{"value":VALUE,"facts":FACTS}` reply has the prior bare shape:

- `decide`: `true`, `false`, or `null`.
- `choose`: a label or `null`.
- `score`: a number.
- `tag`: an array of labels.
- `filter`: an array of the kept records.
- `rank`: an array of `{"index":N,"record":TEXT,"probability":P}` rows, most likely yes first, with a zero-based index.
- `find`: `{"index":N,"unit":TEXT,"probability":P}` for the chosen unit, with a zero-based index, or `null`.
- `annotate`: an array holding each record's `value_json` object.
- `recognize`: the `Recognized::to_json` object.
- `relate`: `{"edges":[...]}`.
- `usage`: `{"requests_sent","retries","input_tokens","output_tokens","cache_answers"}`, this process's totals.

With `"details": true`, a single judgment puts the `thinkthen.result/1` object under `value`. A record array holds one full detailed object per input. The four record-array verbs use the shared Rust stopping-details bridge, preserving runtime labels and descriptions. A failed row returns NULL and no partial array. `facts` is scoped to this call; `{"usage":true}` remains a direct process-counter object.

The door writes the `value` for `decide`, `choose`, `score`, `tag`, `filter`, `rank`, and `find` with the existing result writer. `tests/door/bytes.rs` compares that value with the command's output on the shared cases.

## 1. Cancellation

The door owns a token handle. `thinkthen_cancel_token_new` creates it, `thinkthen_cancel` fires it, and `thinkthen_cancel_token_free` frees it. Every `_opts` spelling takes a token, and a null token means none. A fired token stays fired, and one token can stop many calls. No new request or retry starts after the fire. A request already sent finishes within its attempt timeout and the budget. A complete answer reaches the cache, and every reply reaches the counters, so the same call with a fresh token replays it and pays nothing more. The call that carried the token then returns `THINKTHEN_ECANCELLED` with no results, whatever the reply held, a failure included. The call reads the token on every thread it uses, and once more after its last request ends. A fire after that last read does not change the result.

C has no standard interrupt channel, and every host that binds this door has threads. A Go, Java, or C++ host fires the token from whatever thread receives its stop gesture. A fire allocates nothing, so a POSIX signal handler can fire it too.

## 2. Deadlines

Every `_opts` spelling takes an `int64_t deadline_ms` beside the token. The door hands it to `CallOptions::deadline_millis` (ADR 0041), which holds the rules:

- `THINKTHEN_NO_DEADLINE` (-1) sets none.
- Zero is a spent budget. The call returns `THINKTHEN_EDEADLINE` before anything is sent.
- Any other negative value, and any value past 4,294,967,295 seconds, is a usage failure before anything is sent.
- A positive value is that many milliseconds from the call.

Every plain spelling is its `_opts` twin called with `THINKTHEN_NO_DEADLINE` and a null token. `tests/c/opts.c` proves this on the answer path and the failure path.

Flat arguments suit Go's cgo and Java's FFI, which marshal scalars and pointers directly and structs by value awkwardly. A struct would also freeze a layout.

## 3. Partial completion

A cancelled or expired call returns its code and no results. Every out parameter keeps what it held. The engine reports a bulk call as one unit, and the door does not chunk the call to fake partial delivery. Partial rows would also need a count that the frozen argument list does not carry.

On success the door requires one judgment per record. A short list is `THINKTHEN_EDEFECT` with every slot untouched.

## 4. Null and length inputs

The door checks its pointers before it asks the engine, so a refusal sends nothing:

- A null `engine` returns `THINKTHEN_EUSAGE`, or NULL from `thinkthen_call`. The error functions with a null engine name the calling thread's last failed build, or else the usage code, zero, and a message that no engine came.
- A null or non-UTF-8 `question_json`, `request_json`, or `spec_json` is a usage failure. These strings end in NUL and carry no length.
- A null `text` with a zero length is the empty text, and the engine's blank-evidence rule refuses it. A null `text` with a nonzero length is a usage failure. A non-null `text` reads exactly `text_len` bytes.
- Null `texts`, `lengths`, or bulk `out` arrays are usage failures when `count` is nonzero. A zero count reads and writes nothing.
- A null `out` or `out_len` for decide, recognize, or relate is a usage failure.
- `thinkthen_relate` refuses more than 255 records before it reads any pointer.

`tests/c/nulls.c` pins each row's code and message, and checks that the backend received no request.

## 5. Allocated results

- A string from `thinkthen_call`, `thinkthen_recognize`, or `thinkthen_relate` is freed with `thinkthen_free_string`, once.
- Each new typed facts string, and each new recognition or relation result string, is independently owned and freed with `thinkthen_free_string` once.
- The token is freed with `thinkthen_cancel_token_free` after every call that carried it has returned.
- The engine is freed with `thinkthen_engine_free` after every call on it has returned.
- The message from `thinkthen_error_message` is borrowed. It stays valid until the calling thread records its next failure on that engine, the thread exits, or the engine is freed.
- The JSON text from `thinkthen_error_facts_json` is borrowed under the same rule and is NULL when the last failure has no started-call facts.
- Every buffer the caller passes is borrowed for the call and never kept.

## 6. Concurrent callers

Any number of threads may call on one engine at once. The failure table is per thread and per engine. Each engine carries its own table keyed by thread. `thinkthen_error_code`, `thinkthen_error_message`, and `thinkthen_error_retryable` read the calling thread's entry on that engine. A thread that recorded no failure reads `no failure yet`.

The branch measured the two failures this prevents. A shared slot let a second thread's failure free the first thread's message while it was read (R1-9). A thread-local slot keyed by engine address let one engine's failure answer for another (R2-16).

A thread's entries leave every table when the thread exits (R3-24). The table registers a weak link in the thread's local storage on its first failure, and the storage's destructor removes the entries. A host that spawns a thread per request no longer grows the table without end.

`tests/c/threads.c` runs two threads with distinct failures behind a barrier, and each reads its own message. `tests/c/engines.c` reads two engines' failures on one thread and a fresh engine at a freed engine's address. The unit test `a_threads_failure_leaves_the_table_when_the_thread_exits` checks that 200 exited threads leave no entry.

## 7. The error surface

`thinkthen_error_code` returns the code of the calling thread's last failure on the engine, and `THINKTHEN_OK` before any. Success does not clear it. The failing call's own return value is always that call's code.

Every exported symbol runs behind one guard. A panic from the engine or the door becomes the defect kind with the panic's text in the message. It never unwinds into the host. The failure path reaches thread-local storage only through `try_with`, so a call from an `atexit` handler after teardown still returns its code (R2-7). `tests/c/atexit.c` fails once on one engine before exit, then fails on a second engine and frees both from an `atexit` handler, and must exit 0.

## 8. ABI stability

Version 0.0.1 freezes:

- the prior 19 symbol names and eight additive facts-returning typed names,
- the `thinkthen_answer` layout, `int` then `double`,
- the return codes 0 through 6, with new kinds appended and none renumbered,
- `THINKTHEN_NO_DEADLINE` and the `thinkthen_` prefix,
- the argument types: `size_t` lengths and counts, `int64_t` budgets.

Adding a symbol or a code is a minor bump. Changing a signature, a layout, or a code is a major bump.

**Soname.** The shared library carries the soname `libthinkthen.so.0` (ADR 0047 item 6), and on macOS the install name `@rpath/libthinkthen.0.dylib`. `build.rs` sets it. The crate's library is named `thinkthen_c` so its output files never collide with the engine's, and a release renames them `libthinkthen.so` and `libthinkthen.a`. `tests/door/main.rs` reads the soname with `readelf -d` and checks that a clean build prints no collision warning (R2-26).

**Windows DLL.** The x86-64 MSVC archive contains exactly `include/thinkthen.h`, `bin/thinkthen.dll`, and `lib/thinkthen.dll.lib`. It has no wrapper directory, static implementation, `.pc`, runtime DLL, PDB or EXP. The checksum sidecar covers the ZIP. The declaration reader supplies a sorted header-derived DEF to the cdylib linker. Final packaging separately generates the import library with `lib.exe /MACHINE:X64` against a DEF naming `thinkthen.dll`; renaming Cargo's import library would retain the wrong descriptor. Native `dumpbin` checks compare all exports with the header, including unexpected names, and inspect the import descriptor and consumer's imports. Owned allocations return to the DLL's free functions; borrowed failures never cross into host `free`.

The Windows source door runs shared behavioral assertions through MSVC, except the Unix fork case. A separate consumer links the downloaded archive's header/import library and runs its public DLL, proving an exact spend refusal with zero counted loopback sends followed by a successful call with one send. Its fixtures alone define `_CRT_SECURE_NO_WARNINGS` under `/W4 /WX`. Threads use checked native creation, bounded join and handle close; byte-sensitive streams stay binary. The MSVC ASan check requires a detected owned negative before exercising C ownership and concurrency. It cannot establish Rust DLL instrumentation or Unix leak checking. Actual native linker, loader, execution, sanitizer availability and runtime dependency results remain pending; portable structural fixtures are not native receipts. The Windows host-exit test does not establish identical Unix destructor ordering.

Every result of open size crosses as JSON text, so a new field never changes a layout. Options are flat scalars for the same reason.

## Deferred

- A checked throttle constructor landed as `thinkthen_engine_new_with` in ticket 0148.
- Partial rows, which need an engine capability that exposes a stopped call's finished judgments.
- Windows 0.2 distributes the DLL and its import library only. Raw Cargo static output is never shipped. Stage 3 owns COFF static localization, static-link coexistence and metadata in [the static-library issue](../../sdlc/issues/2026-10-05-windows-static-library-waits-for-stage-3.md). No native localization experiment has run. Stage 2 bindings remain separate.
