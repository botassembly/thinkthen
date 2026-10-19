# The C door

`libthinkthen` exports the engine to any language that can call a C library. `include/thinkthen.h` declares its functions, and [DESIGN.md](DESIGN.md) gives the ownership, threading, and error rules behind them.

## Recommended development 0.2 API

C call input uses counted canonical JSON. The [input decision](../../sdlc/decisions/2026-10-19-c-input-json.md) records the choice and the migration boundary for existing typed and component-based entry points.

Use `thinkthen_run_call` for all ten judgments. Supply a `thinkthen_named_request_v1` containing counted canonical function name, question, input and options JSON. The [Request contract](../../sdlc/planning/adr/0125-one-request-contract-and-native-admission.md) owns grammar and admission. The call blocks until Rust settles execution and transfers an independent result or error owner. The [result decision](../../sdlc/decisions/2026-10-19-c-call-owned-json.md) records the representation and its cost.

Read the complete canonical Call JSON through `thinkthen_call_result_json`, then release it with `thinkthen_call_result_free`. Its bytes survive engine destruction and the original calling thread. They remain valid until the result is freed.

For fed input, declare a named feed in the input JSON and supply a producer. It returns one counted canonical `RequestItem` JSON value, end, or failure. The producer runs only on the thread calling `thinkthen_run_call`; its bytes remain readable until its next invocation or call return. Keep private host exceptions in the host and return the failure status. Async hosts dedicate one worker to the blocking call. Another thread may fire `thinkthen_call_cancel_fire` on the shared cancellation owner. Keep the engine and token alive until the call and every concurrent operation return.

On failure, read kind, safe message, retryability and optional native Facts JSON from the `thinkthen_call_error_*` accessors and free the error with `thinkthen_call_error_free`. The immutable owner may move to another thread. This call never changes legacy thread-local error slots. Both output slots must be distinct writable storage. Success publishes only result; failure publishes only error. Initialize both slots before calling.

The older session and generated view exports remain for migrating language hosts. Their ABI ownership and pointer safety rules still apply until retirement.

`thinkthen_request_plan_json` previews canonical Requests for fixed atomic decide, choose, tag and score questions only. Other judgments and dynamic question definitions refuse; this preview limit does not limit the ten-judgment session API. It reads no key or cache and sends nothing. Its successful plan string is owned: free it with `thinkthen_free_string`.

## Use a prebuilt development archive

Extract the matching C archive supplied by your reviewed development build. It carries the generated `include/thinkthen.h`, static engine, shared library and relocatable `lib/pkgconfig/thinkthen.pc`. A C caller compiles its own program; it needs no Rust compiler. The package-relative static archive supplies the engine without a runtime library search path. For Linux, from the extracted directory:

```sh
PKG_CONFIG_PATH="$PWD/lib/pkgconfig" cc -std=c11 consumer.c -o consumer \
  $(PKG_CONFIG_PATH="$PWD/lib/pkgconfig" pkg-config --cflags thinkthen) \
  "$PWD/lib/libthinkthen.a" \
  $(PKG_CONFIG_PATH="$PWD/lib/pkgconfig" pkg-config --libs --static thinkthen | sed 's/-lthinkthen//')
./consumer
```

[examples/session.c](examples/session.c) is a complete caller using the blocking call and owned canonical JSON. Supply the backend environment before a live call. The focused local package check runs that caller and the JSON session and plan cases against an extracted archive with a synthetic loopback backend:

```sh
python3 libraries/c/tests/installed-session.py PATH_TO_C_ARCHIVE
```

The [native package design](../../sdlc/decisions/2026-10-09-native-package-design.md) defines intended targets and runtime prerequisites. This example proves Linux static integration; other platform qualification belongs to the authorized release candidate.

## Build from source

```sh
cargo build --release
mkdir -p lib
cp target/release/libthinkthen_c.so lib/libthinkthen.so
ln -sf libthinkthen.so lib/libthinkthen.so.0
cc -std=c11 -I include examples/session.c -L lib -lthinkthen -Wl,-rpath,"$PWD/lib" -o session
```

A release renames the built files `libthinkthen.so` and `libthinkthen.a`, with the soname `libthinkthen.so.0`. `localize.sh` writes the release `libthinkthen.a` so that it defines only the header's `thinkthen_` functions as global names, and a program can link its own SQLite beside it. On macOS it needs the `rust-objcopy` that ships inside the Rust toolchain. `thinkthen_engine_new` reads `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE` as the command does.

`thinkthen_engine_new_with` accepts JSON for the address, model, throttle, request limit, request-byte ceiling, cache, timeout, retries, profile, batch default, record, and strict replay; the key stays in `THINKTHEN_API_KEY`.

`thinkthen_request_plan_value` accepts the same counted full Request UTF-8 as `thinkthen_request_plan_json` and returns an independent immutable value owner through the same Rust admission and Plan serializer. Unknown fields refuse before planning. `thinkthen_value_parse` parses arbitrary counted JSON without Request admission and retains exact number bytes and counted object keys. Free both owners with `thinkthen_value_free`; roots borrowed through `thinkthen_value_view` remain valid until that free, even after source bytes and engines are destroyed. NULL input requires zero length; empty input, malformed JSON or UTF-8, nonrepresentable extents, and NULL output return `THINKTHEN_EUSAGE` without changing the output. Failures use the calling thread immediate session diagnostic and preserve constructor and engine diagnostics.
