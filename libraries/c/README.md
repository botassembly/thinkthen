# The C door

`libthinkthen` exports the engine to any language that can call a C library. `include/thinkthen.h` declares its functions, and [DESIGN.md](DESIGN.md) gives the ownership, threading, and error rules behind them.

```sh
cargo build --release
mkdir -p lib
cp target/release/libthinkthen_c.so lib/libthinkthen.so
ln -sf libthinkthen.so lib/libthinkthen.so.0
cc -std=c11 -I include examples/slide.c -L lib -lthinkthen -Wl,-rpath,"$PWD/lib" -o slide
```

A release renames the built files `libthinkthen.so` and `libthinkthen.a`, with the soname `libthinkthen.so.0`. `localize.sh` writes the release `libthinkthen.a` so that it defines only the header's `thinkthen_` functions as global names, and a program can link its own SQLite beside it. On macOS it needs the `rust-objcopy` that ships inside the Rust toolchain. `thinkthen_engine_new` reads `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE` as the command does.

`thinkthen_engine_new_with` accepts JSON for the address, model, throttle, request limit, request-byte ceiling, cache, timeout, retries, profile, batch default, record, and strict replay; the key stays in `THINKTHEN_API_KEY`.

`thinkthen_question_file(engine, path, &json, &length)` reads one named UTF-8 question file of at most 1 MiB and returns its validated source JSON. Pass that owned string to `thinkthen_decide_with_facts` or use it to construct a JSON-door request, then free it once with `thinkthen_free_string`. A bad file returns non-retryable `THINKTHEN_ELOCAL` without changing either output or sending a request. The existing bare question and inline JSON arguments remain literal and report typed grammar errors as `THINKTHEN_EUSAGE`.

`thinkthen_plan_json(engine, plan_json, &json, &length)` previews a judgment call and sends nothing. Pass `{"verb":"decide","question":"asks for a refund","input":["Refund me please."],"settings":{}}`: bare question text takes its fields, such as `options`, from `settings`, and a question object takes only `batch`, `context` and `deadline_ms` from it. The owned JSON holds `records`, `requests`, `estimated_bytes`, `estimated_input_tokens` as `{"lower","upper"}`, `upper_bound` and `first_body_utf8`, the result schema's `plan` definition. It needs no key. Free it once with `thinkthen_free_string`.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `thinkthen_engine_new_with("{\"cache\":false}")`. Set `THINKTHEN_CACHE` before `thinkthen_engine_new` runs to move its folder.

## Counted typed calls

[The typed C contract](TYPED.md) describes 0426 slice A: owned counted question/source handles, ten named `*_current` calls, and typed current-native values, probabilities, structures, locations, facts and observations. These additive helpers expose native result/1. Complete result/2 and image execution remain open with their native owners; the prior calls stay compatible.

## Windows x86-64

The development archive `thinkthen-c-0.2.0-x86_64-pc-windows-msvc.zip` contains exactly `include/thinkthen.h`, `bin/thinkthen.dll` and `lib/thinkthen.dll.lib`, with a separate SHA-256 sidecar. The `.lib` imports the public DLL; it is not a static implementation. From an x64 MSVC developer shell, compile a consumer after extraction:

```console
cl.exe /nologo /TC /std:c11 /W4 /WX /MD /Iinclude consumer.c /link lib\thinkthen.dll.lib
copy bin\thinkthen.dll .
consumer.exe
```

Free owned strings and facts through `thinkthen_free_string`, and engines and tokens through their DLL free functions. Error messages and error facts remain borrowed. Windows static libraries and other bindings remain separate work. Development files are unsigned; public distribution and native runner proof remain pending.

## Run facts

Every successful asking JSON call returns `{"value":...,"facts":...}`. `value` keeps the verb's prior bare JSON shape. `facts` reports this call's finished records, sent attempts, cache answers and elapsed seconds, with provider token counts and model only when available. A failed call still returns `NULL`; after it, `thinkthen_error_facts_json` borrows final facts from the same calling-thread and engine slot as `thinkthen_error_message`. A refusal before a call starts has no facts. The direct `{"usage":true}` process totals remain a separate shape.

For typed calls, use `thinkthen_decide_with_facts`, `thinkthen_decide_many_with_facts`, `thinkthen_recognize_with_facts`, or `thinkthen_relate_with_facts`. Each also has an `_opts` form. Pass result outputs followed by `char **facts_json` and `size_t *facts_len`; free the owned facts JSON with `thinkthen_free_string`. Recognition and relation return a separate owned result JSON string. The older eight typed names remain compatible bare-result calls and do not return facts. The new forms preserve their results, options and failure codes. A failure changes no output slot; started-failure facts remain available through the borrowed error accessor.

The four judgments also accept `"records":["...",...]` with one runtime question and ordered answers. The closed `"call":{"batch":10,"context":"..."}` object controls eligible record arrays; `"batch":"max"` selects maximal packing. `call.batch` outranks the engine default, which outranks a saved question's batch. A top-level `batch` remains part of the saved question. A context is shared evidence, changes request identity, and is refused on unsupported routes. With `"details":true`, `value` is an array of complete `thinkthen.result/1` record objects including each whole `input`, request and batch metadata.

For one text, `"details":true` puts the command's `--details` object under `value`. The backend's reply supplies `meta.model`, `meta.usage` and probabilities, with `answer.confidence` when present. `meta.requests` holds request digests; `meta.url` names the address. A field the backend did not report is absent.

The JSON call keeps a question's saved calibration `profile` in `meta.question_sha256`. When `thinkthen_engine_new_with` selects a different runtime profile, details include `meta.profile_warning` with `tuned_for` and `running`. The selected profile enforces request limits before a send.

A call of `{"usage": true}` returns this engine's running totals of requests sent, retries, cache answers and tokens.

## Throttle is per loaded copy

The engine's throttle is the limit on requests in flight at once. It holds per loaded copy of the engine. A process that loads two copies, such as a C host and a native extension built from another surface, can run up to twice the throttle (ADR 0047 item 5).

## Checks

`cargo test` builds the door and compiles each C program against it. Unix uses AddressSanitizer and leak checking; Windows uses MSVC `/W4 /WX /MD`, linked through the header and the public library as a host links it. The Windows fixture compiler alone defines `_CRT_SECURE_NO_WARNINGS` for the framing driver. A separate native MSVC ASan test first requires a detected use-after-free; it covers C consumers, not the Rust DLL or Unix LSan. `tests/door/main.rs` checks the soname and the exported symbols, the examples' pinned output, and the C rows in `tests/c/`. `tests/door/cases.rs` feeds every applicable shared case through `tests/c/driver.c`. `tests/door/bytes.rs` holds the `value` field's bytes to the command's output.

The loopback backend's generic arm answers every question by a fixed rule: the first option, level, label, or yes gets 0.9. The pinned text shows that rule's answers. A model would answer differently.

`check.sh` is this surface's entry in the surface rung, `sdlc/scripts/surfaces`. It exits 77 when no C compiler is found. It runs the formatter, Clippy, and the tests, and then runs the slide against the rung's own loopback backend.

`thinkthen_engine_new_with("{\"backend\":\"liquid\"}")` selects a named backend in code. The Rust builder captures its key from that backend's environment variable. Explicit `backend` outranks environment and configuration selection. With `base_url` too, the selected key, posting path, description form and setup prices/profile apply at that address. Explicit model, profile and price settings override their setup values. The door accepts no key. Every package forwarding settings JSON to this door inherits `"backend"` without an ABI change.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.
