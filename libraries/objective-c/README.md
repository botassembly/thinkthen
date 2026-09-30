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

On Ubuntu 24.04 the GNU Objective-C compiler and runtime are `gobjc gcc libobjc4`; no Foundation is required. Running this checkout's `check.sh` also requires Node and Python with `jsonschema`. A consuming project must install or name the matching native library and its runtime search path. The archive instructions below describe local pilot files, not public release assets.

## Install from independently supplied archives

Unpack the local `thinkthen-objective-c-0.0.1-x86_64-unknown-linux-gnu.tar.gz` and the *separate matching* `thinkthen-c-0.0.1-x86_64-unknown-linux-gnu.tar.gz`. Verify their adjacent SHA-256 files and matching `THINKTHEN-PACKAGE-INPUTS` source and C digest. The C archive supplies `include/thinkthen.h` and `lib/libthinkthen.so` (soname `libthinkthen.so.0`); the wrapper takes its C header from there and ships none. Never use a C library from another build. GNU Objective-C (`gobjc` version 4:13.2.0-7ubuntu1; gobjc-13 13.3.0), GCC, libobjc4, glibc, pthreads and the separately built Rust C native library are required. On Ubuntu 24.04 install `gobjc gcc libobjc4` (native build requires Rust 1.95 and offline locked Cargo dependencies). Example:

```
gcc -std=gnu11 -x objective-c -I native/include -I package/Sources \
  package/Sources/ThinkThen.m package/Sources/TTJSON.c package/Examples/consumer.m \
  -L native/lib -Wl,-rpath,/absolute/path/to/native/lib -lthinkthen -lobjc -pthread -lm -o example
```

The future release path: GitHub Actions `ubuntu-24.04` builds source checks and native archive, checks its hashes/ABI, attaches platform-specific native binaries to a GitHub Release, and publishes an Objective-C source package for direct download. The queue owner decides publication, toolchain pins, and distribution routes. No public native archive has shipped yet.

## Contract

`TTOutcomeNo`, `TTOutcomeYes`, `TTOutcomeNotSure` and `TTErrorKind` (usage, backend, deadline, local, cancelled, defect) name the domain; `TTFailure` owns a same-thread copied message and failure-facts JSON plus a retryable bit. Initialize it to zero and clear it after use. `createWithSettings:length:failure:` validates configured construction. `tt_failure_clear` frees that message. Scalar and bulk output use `TTDecision` and `TTOutcome`; the C ABI `thinkthen_answer` and raw error integers remain inside the binding. A failed call leaves every `TTDecision` untouched. Returned JSON strings are host-owned and freed with `free`; parsed `TTJSON` trees are freed with `tt_json_free`. Evidence uses counted bytes and may contain NUL; every untrusted C-string input has a length-aware variant (`decideBytes`, `manyBytes`, `jsonBytes`, `recognizeBytes`, `relateBytes`) that refuses interior NUL before crossing the ABI. The non-Bytes methods accept only trusted, terminated C strings.

Every typed `decide`, `many`, `recognize`, and `relate` selector, including its counted `*Bytes` form, sets a required `char **facts` to host-owned facts JSON text alongside the former value on success; free it with `free` and read it with `tt_json_parse`, ignoring members you do not know. `recognize` and `relate` return their value as host-owned JSON text too, and take `deadline:` and `token:` like `decide`, `many` and `json`. A failed call leaves both outputs untouched. `tt_field_read` reads one annotate answer member from a parsed row: JSON null is `TTFieldUnresolved`, `{"failed":{"kind":...,"cause":...}}` is `TTFieldFailed` with its `TTErrorKind` and borrowed cause, and any other value is `TTFieldValue`; it returns 0 for any other object. `plan:question:texts:lengths:count:settings:failure:` previews a decide, choose, score or tag call through `thinkthen_plan_json` and returns the result schema's `plan` object as host-owned JSON text; it needs no key and sends nothing. A question that starts with `{` is a question object. `max_requests_total` in `createWithSettings:` caps the process's live sends.

Ownership: create clients/tokens; join all callers before deallocating either. The facade tracks in-flight native calls and waits before native release, but callers must stop initiating new calls and stop calling `fire` before teardown. It cannot make concurrent use-after-free of the Objective-C object safe. Error copying occurs on the calling native thread, before tracking release.

The JSON door accepts question-file grammar: bare label arrays and ordered label→description objects for options/labels/kinds; structured `{ "what", "not_for", "examples" }` descriptions pass through without modification. For native callers, validate the question against `specification/question-file.schema.json` at the application boundary (the pinned file is not included in this minimal source archive). `TTJSON` parses JSON and rejects duplicate keys and malformed or truncated input; it checks no answer shape. Unresolved annotate fields are `TTJSONNull`; failure is an object `{"failed":{"kind":...,"cause":...}}`, never null. Entity offsets count **zero-based Unicode code points**, end exclusive; not UTF-16 units. The product check runs the current J1 corpus through the public GNU Objective-C JSON door. No C structs added.

This proof tests synthetic answers, ownership, UTF-8, native cancellation, response shapes and counted requests, not answer quality, Rust allocator instrumentation, other operating systems or Apple Objective-C runtimes.
