# ThinkThen GNU Objective-C

A Linux x86_64/glibc GNU Objective-C 13 source package. No Foundation or GNUstep. This is not an Apple Objective-C or supported release claim.

## Build from a checkout and use in another project

Clone the ThinkThen repository at the matching source pin. From the checkout, build the native C library with Rust 1.95, an installed Cargo registry, and no network:

```
git clone https://github.com/botassembly/thinkthen.git
cd thinkthen
CARGO_NET_OFFLINE=true cargo build --locked --offline --release -j2 --manifest-path libraries/c/Cargo.toml
```

The result is `libraries/c/target/release/libthinkthen_c.so` (if Cargo uses a different target directory, use that directory). Install the **matching** `libraries/c/include/thinkthen.h`, `LICENSE`, and shared library into a separate native directory; name the shared library `libthinkthen.so` and point `libthinkthen.so.0` at it. Copy `libraries/objective-c/Sources/` and your chosen example into the consuming project's source tree, then compile from that project:

```
gcc -std=gnu11 -x objective-c -I /absolute/native/include -I /absolute/objc/Sources \
  /absolute/objc/Sources/ThinkThen.m /absolute/objc/Sources/TTJSON.c \
  /absolute/objc/Examples/consumer.m -L /absolute/native/lib \
  -Wl,-rpath,/absolute/native/lib -lthinkthen -lobjc -pthread -lm -o example
./example
```

On Ubuntu 24.04 the GNU Objective-C compiler and runtime are `gobjc gcc libobjc4`; no Foundation is required. Running this checkout's `check.sh` also requires Node and Python with `jsonschema`. A consuming project must install or name the matching native library and its runtime search path. The archive instructions below describe the files each GitHub release ships.

## Install from independently supplied archives

Unpack the release's `thinkthen-objective-c-0.1.2-x86_64-unknown-linux-gnu.tar.gz` and the *separate matching* `thinkthen-c-0.1.2-x86_64-unknown-linux-gnu.tar.gz`. Verify their adjacent SHA-256 files and matching `THINKTHEN-PACKAGE-INPUTS` source and C digest. The C archive supplies `include/thinkthen.h` and `lib/libthinkthen.so` (soname `libthinkthen.so.0`); the wrapper takes its C header from there and ships none. Never use a C library from another build. GNU Objective-C (`gobjc` version 4:13.2.0-7ubuntu1; gobjc-13 13.3.0), GCC, libobjc4, glibc, pthreads and the separately built Rust C native library are required. On Ubuntu 24.04 install `gobjc gcc libobjc4` (native build requires Rust 1.95 and offline locked Cargo dependencies). Example:

```
gcc -std=gnu11 -x objective-c -I native/include -I package/Sources \
  package/Sources/ThinkThen.m package/Sources/TTJSON.c package/Examples/consumer.m \
  -L native/lib -Wl,-rpath,/absolute/path/to/native/lib -lthinkthen -lobjc -pthread -lm -o example
```

The release workflow builds the source package and the native archive on GitHub Actions `ubuntu-24.04`, checks their hashes and ABI, and attaches both to the GitHub release for direct download.

## Contract

`TTOutcomeNo`, `TTOutcomeYes`, `TTOutcomeNotSure` and `TTErrorKind` (usage, backend, deadline, local, cancelled, defect) name the domain; `TTFailure` owns a same-thread copied message and failure-facts JSON plus a retryable bit. Initialize it to zero and clear it after use. `createWithSettings:length:failure:` validates configured construction. `tt_failure_clear` frees that message. Scalar and bulk output use `TTDecision` and `TTOutcome`; the C ABI `thinkthen_answer` and raw error integers remain inside the binding. A failed call leaves every `TTDecision` untouched. Returned JSON strings are host-owned and freed with `free`; parsed `TTJSON` trees are freed with `tt_json_free`. Evidence uses counted bytes and may contain NUL; every untrusted C-string input has a length-aware variant (`decideBytes`, `manyBytes`, `jsonBytes`, `recognizeBytes`, `relateBytes`) that refuses interior NUL before crossing the ABI. The non-Bytes methods accept only trusted, terminated C strings.

Every typed `decide`, `many`, `recognize`, and `relate` selector, including its counted `*Bytes` form, sets a required `char **facts` to host-owned facts JSON text alongside the former value on success; free it with `free` and read it with `tt_json_parse`, ignoring members you do not know. `recognize` and `relate` return their value as host-owned JSON text too, and take `deadline:` and `token:` like `decide`, `many` and `json`. A failed call leaves both outputs untouched. `tt_field_read` reads one annotate answer member from a parsed row: JSON null is `TTFieldUnresolved`, `{"failed":{"kind":...,"cause":...}}` is `TTFieldFailed` with its `TTErrorKind` and borrowed cause, and any other value is `TTFieldValue`; it returns 0 for any other object. `plan:question:texts:lengths:count:settings:failure:` previews a decide, choose, score or tag call through `thinkthen_plan_json` and returns the result schema's `plan` object as host-owned JSON text; it needs no key and sends nothing. A question that starts with `{` is a question object. `max_requests_total` in `createWithSettings:` caps the process's live sends.

Ownership: create clients/tokens; join all callers before deallocating either. The facade tracks in-flight native calls and waits before native release, but callers must stop initiating new calls and stop calling `fire` before teardown. It cannot make concurrent use-after-free of the Objective-C object safe. Error copying occurs on the calling native thread, before tracking release.

The JSON door accepts question-file grammar: bare label arrays and ordered label→description objects for options/labels/kinds; structured `{ "what", "not_for", "examples" }` descriptions pass through without modification. For native callers, validate the question against `specification/question-file.schema.json` at the application boundary (the pinned file is not included in this minimal source archive). `TTJSON` parses JSON and rejects duplicate keys and malformed or truncated input; it checks no answer shape. Unresolved annotate fields are `TTJSONNull`; failure is an object `{"failed":{"kind":...,"cause":...}}`, never null. Entity offsets count **zero-based Unicode code points**, end exclusive; not UTF-16 units. The product check runs the current J1 corpus through the public GNU Objective-C JSON door. No C structs added.

This proof tests synthetic answers, ownership, UTF-8, native cancellation, response shapes and counted requests, not answer quality, Rust allocator instrumentation, other operating systems or Apple Objective-C runtimes.

`createWithSettings:length:failure:` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

Typed 0.2 host descriptors are available through `TTComplete.h`. Ten named request builders prepare explicit questions/question files and record/file/image sources; known answer, metadata, fact, identity, location, span and edge fields have typed carriers. This is independent carrier preparation, not complete-call runtime parity. The host descriptor codec is not the native wire format. Existing engine calls remain the executable compatibility API. Adoption of the real 0426 constructors, complete calls, result views and failure snapshots is still required; no legacy result is promoted to result/2.

Compile `Sources/TTComplete.c` with `Sources/TTJSON.c` when using `ttc_atomic_read` or `ttc_facts_read`. Descriptors borrow caller storage; parsed field strings borrow the `TTJSON` tree, and probability entries use caller-provided storage. Release the tree only after those borrows finish. Invalid field reads leave outputs unchanged.


## Complete native calls (0428 integration)

The additive complete API uses C's native question grammar and readers for all ten named functions. Bare and generic JSON calls remain available. Known result fields are typed; arbitrary original JSON is retained as counted content.

Question input is explicit: construct a typed descriptor, parse saved JSON with a loader role, load an exact file, load a configured name, or load a reference. The eight roles are atomic, question set, per-record choose, recognize, record relate, rank, rank set and find. A string is never guessed to be a path. Native declarations, named/versioned authors, record context, options, rank members, observations, answer/request/call IDs, final facts and owned failure snapshots remain distinct.

Records may carry explicit text or JSON, per-record context and ordered candidate replacements. Images use explicit PNG/JPEG media and preserve original compressed bytes, order and duplicates. Decide, choose and score admit native image routes; the other seven functions refuse before sending. File sources explicitly select line, window, whole file, image file or JSONL. Physical filename/line ranges and absent image line coordinates come from the native reader.

Complete results copy all borrowed native views before `thinkthen_result_free`. They survive destruction of the engine, question, source and input buffers. Presence remains explicit, including absent aggregate IDs/meta, nullable selections and independently unknown reported token dimensions. Cost strings are copied without floating-point conversion. Failures retain the six native kinds, safe messages, available stop details and final facts/attempts.

Decide, choose, tag, score, filter and annotate also have owned lazy native batches. Start/next/facts/free belong to the creating thread; keep the engine live until the batch closes. Starting clones question/source/context/cancellation state. Returned rows own independent snapshots. Rank, find, recognize and relate retain aggregate complete calls. No host parser, cache, scheduler or model-routing policy is added.

This is an unpublished integration API. The family ticket and root review own final qualification; the shared consumer reports actual failures instead of counting generic JSON as typed parity.

Include `TTNativeAPI.h`, and compile `TTNativeAPI.m` plus `TTNativeViews.c` with the existing sources. `TTQuestion`, `TTSource` and `TTImage` own native handles and release them in `dealloc`. The `TTClient (Complete)` category supplies counted constructors/loaders and ten `FUNCTIONComplete:source:controls:output:failure:` selectors. Success and native failure outputs own `TTNativeResult` snapshots; free each with `tt_native_result_free` after every reader finishes. `tt_native_FUNCTION`, summary/details/author/observation/rank-member/located getters return checked typed views borrowing that host snapshot. `tt_native_failure_kind` maps its six native errors.

The six `FUNCTIONBatch:source:controls:output:failure:` selectors return `TTBatch`. Keep the client live until batch `dealloc`; `next:failure:`, `facts:failure:` and `dealloc` run on its creating thread. Completed row snapshots outlive the batch and client. Failed calls leave success outputs unchanged. Counted input descriptors must stay initialized/readable through construction. Native constructors clone all nested buffers. Existing ABI/static and ASan/UBSan checks remain in the family gate.

Rank-set rows retain every member in saved declaration order. Each member exposes its native positive rank position, probability, answer identity, author declarations and complete details. Details preserve independently reported token dimensions and source batch sizes. Parent and member metadata overlap; read final call facts for invocation usage.
