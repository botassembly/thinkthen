# The C door

`libthinkthen` exports the engine to any language that can call a C library. `include/thinkthen.h` declares its functions, and [DESIGN.md](DESIGN.md) gives the ownership, threading, and error rules behind them.

## Recommended development 0.2 API

Use the owned `thinkthen_session_*` interface for all ten judgments. Pass length-delimited canonical Request JSON to `thinkthen_session_new`; the shared [Request contract](../../sdlc/planning/adr/0125-one-request-contract-and-native-admission.md) owns grammar and admission. The [session contract](../../sdlc/planning/adr/0129-owned-json-sessions.md) owns streaming, packet ordering and terminal results. These development interfaces do not describe a published 0.2 package.

`thinkthen_session_new` identifies the caller as C. Language bindings over this interface use `thinkthen_session_new_with_surface` with their length-delimited native `Surface` token, so the engine identifies the outer wrapper accurately. The token describes the caller; it grants no authority and changes no answer or cache identity. The generated header defines both constructors with the same ownership and diagnostic rules.

The constructor copies request input and gives the worker its own engine. After it returns successfully, release the request buffer and the original engine if no other call needs them. For caller-fed input, `thinkthen_session_try_push` retains an owned descriptor only on ACCEPTED; FULL requires retrying that same descriptor, and CLOSED stops the producer. Finish intake with `thinkthen_session_finish`. Read with `thinkthen_session_try_read`: RESULT transfers an independent packet owner, PENDING means try again later, and END follows the terminal packet. Inspect terminal failure and final facts even when the read itself succeeds.

`thinkthen_session_result_view` borrows the generated complete typed packet graph. Select its `kind` before reading its data union, and check a field’s `presence` before reading its value. Strings and arrays have counted extents; unknown members retain their raw JSON in ordered extension entries at each object depth. Repeated access returns the same immutable view. Every nested pointer remains valid until `thinkthen_session_result_free`.

`thinkthen_session_result_json` borrows immutable bytes until `thinkthen_session_result_free`; never pass that borrowed pointer to `thinkthen_free_string`. A packet survives destruction of its session and engine. Cancellation signals work without waiting. Session free closes the receiver without joining native work. Free each session and packet once after concurrent operations and borrowers finish. Immediate API refusals use the borrowed calling-thread `thinkthen_session_error_message`; execution failures belong to terminal packets.

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

[examples/session.c](examples/session.c) is a complete caller using the recommended typed packet views and terminal failure facts. Supply the backend environment before a live call. The focused local package check runs that caller and the existing typed owner case against an extracted archive with a synthetic loopback backend:

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
