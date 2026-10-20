# Writing a thinkthen binding

A binding gives callers named functions, native language values and complete typed results. Rust owns input admission, execution and the meaning of every answer and error. The host owns naming, value conversion, async scheduling, cancellation idiom, cleanup and packaging. Each package's README describes its public calls and installation.

## Call Rust once

C-interface bindings call `thinkthen_run_call`. Direct Rust bridges call `Engine::call(function, RequestArguments, RequestEnvironment)`. Both run one named call to completion and return an owned result or error. Use these entry points for all ten judgments. The [Rust API](../specification/rust-api-0.2.md) and [generated C header](c/include/thinkthen.h) define the signatures and lifetimes.

For C, supply a `thinkthen_named_request_v1` with counted function name, question, input and options JSON. Its components use the [canonical Request grammar](../specification/request.schema.json). Keep the request and its bytes readable until return. Supply distinct writable result and error slots, initialized to null. A successful call publishes the result; a failed call publishes the error. Invalid output slots return a usage status without publishing an owner.

For fed input, declare the feed in the input selector and supply a producer callback. Each invocation returns one counted canonical RequestItem JSON value, end, or failure. Rust pulls the producer on the thread calling `thinkthen_run_call`. Its bytes remain readable until the next invocation or call return. Keep host exceptions in the host and return the producer failure status; never throw or unwind across the C boundary. Direct Rust bridges supply the named pull feed through `RequestEnvironment`.

Rust owns defaults, limits, input validation, request scheduling, settlement and answer projection. The binding converts language values and forwards native errors. It adds no private semantic validator, request scheduler or answer rules. Explicit file selection follows the [shared file behavior](files.md); ordinary text remains evidence.

The existing [C example](c/examples/session.c) calls the blocking entry point and releases its independent result. The [Rust decide example](rust/examples/decide.rs) calls `Engine::call` and reads its typed value. Use those compiled examples when explaining the contract.

## Run async calls on one worker

An async host dedicates one worker to the blocking call and its producer. Its event loop remains free to run other work. Map the host's Task, Future, Promise or equivalent to that worker's completion. Keep host callback values and exceptions reachable until the worker returns.

Create a `thinkthen_call_cancel` owner with `thinkthen_call_cancel_new`, pass it to the call and map the host cancellation idiom to `thinkthen_call_cancel_fire` from another thread. Keep the engine and cancellation owner alive until the call and every concurrent cancellation operation return. Release the token with `thinkthen_call_cancel_free` after those operations finish. Direct Rust bridges pass borrowed cancellation and deadline controls through `RequestEnvironment` and keep their owners alive through the call.

Cancellation requests native cancellation. The binding waits for native completion before releasing resources or presenting final facts. It must not invent an answer or final failure facts while the worker is still running. Synchronous hosts keep their normal execution model and need no async runtime.

## Preserve results, errors and presence

`thinkthen_call_result_json` borrows the complete canonical Call JSON from its result owner. The result survives engine destruction and the original calling thread. Its bytes remain valid until `thinkthen_call_result_free`; copy or convert borrowed data before freeing the owner. Direct Rust bridges retain the returned `Call<RequestValue>` through Rust ownership.

On failure, retain the independent `thinkthen_call_error` owner. Read its kind with `thinkthen_call_error_code`, safe message with `thinkthen_call_error_message`, retryability with `thinkthen_call_error_retryable` and optional native Facts JSON with `thinkthen_call_error_facts_json`. Absent facts return empty text. The message and facts are borrowed until `thinkthen_call_error_free`. The immutable error may move to another thread and survives engine destruction. Free it only after all readers finish. Direct Rust bridges preserve the typed `Error` and its facts.

If the host's public failure exposes a record position, read it through `thinkthen_call_error_position` and preserve a null pointer as an absent position.

Map these native fields to the host's typed error idiom. Preserve safe messages and facts without interpreting message text or replacing facts with host guesses. Keep private producer exceptions available to their host caller.

The [result contract](../specification/result.md), [generated result schema](../specification/result.schema.json) and [shared type corpus](../specification/fixtures/types/) own result meaning. Preserve missing, explicit null and false separately. Preserve unresolved answers separately from failed answers. Complete results retain native observations, identifiers, provenance, usage and failure facts. Accept unknown output members. Host conversion must not fabricate observations or turn a failure into null.

## Use generated declarations

Use the generated C header for C declarations and the package's generated request, result, presence and error declarations for host types and editor support. Direct Rust bridges use public Rust types and their generated host declarations. The [generator](../build-tools/results/generate.py) and its [output registry](../build-tools/results/registry.py) identify generated files; the [C header generator](../sdlc/scripts/generate-c-header.py) owns C declarations. Regenerate from their Rust-owned sources instead of editing generated output or maintaining a private schema or field inventory.

Generation supplies declarations and mechanical value conversion. Handwritten host code supplies the language's calling and ownership idioms. It must not move Rust validation, settlement or projection rules into a language template. Share loading and ownership mechanics within language families. Expose one coherent named function family with useful typed answer and complete-result views.

## Check the native build at load

Check that the wrapper and native library come from the same build before creating an engine or calling other operations. Treat this check as build identity. It does not select an input or output format, negotiate compatibility or accept an older shape.

The current C bootstrap is `thinkthen_version`, with writable storage for the value compared against the generated `THINKTHEN_INTERFACE_NUMBER`. Refuse a missing bootstrap symbol or a mismatch and report the expected value, loaded value and returned library release string. The returned string is borrowed until unload and must never be freed. Keep the generated declarations and native assets together in the package; equality of the bootstrap value alone does not identify a unique build.

Package the supported native assets through the language's normal installation route. Installation and the first call require no manual native library path. Runtime floors and intended platform support come from the [native package design](../sdlc/decisions/2026-10-09-native-package-design.md). A source import or generated declaration alone does not prove the installed package works.

## Compare native packets once

[packets.json](../conformance/fixtures/packets.json) is the shared deterministic packet oracle. It retains native rows, aggregates, terminal output, answers, probabilities, picks, resolution, presence and facts. The [native currency test](../crates/thinkthen/tests/packet_oracle.rs) regenerates it on request and fails when it is stale; the existing [lint rung](../sdlc/scripts/lint) runs that check.

A language dumper reads each shared case's request, calls the language's public API and prints the packet JSON returned by Rust under `packets`. It adds only the values, presence states and errors actually converted by the host, using the comparator's `host` and `error` fields where applicable. It does not serialize every public carrier again or keep a second expected-output corpus. A new native field must reach the comparison without a language-specific field list.

The [shared comparator](../conformance/children/packet_dump.py) parses dumper JSON and compares it with the oracle, retaining numbers as exact decimal strings rather than comparing text bytes. It owns normalization of runtime identities, timing and temporary path prefixes. Authored values and presence remain intact. The [installed Rust consumer](rust/consumer/src/main.rs) and its [dumper](rust/consumer/src/dump.rs) provide the existing reference. The [shared cases](../conformance/cases.json) and [suite](../conformance/children/fixture_suite.py) retain their boundary assertions and request counts. Private invariant injection remains outside public input parity.

## Verify the host boundary

Use the package's existing compiler, import, surface and installed-package checks. Verify named calls, native value conversion, typed results, error facts, presence and resource ownership through a real consumer. Exercise host cancellation while a provider is held and show that unrelated host work continues. Verify that cleanup retains native owners until active calls and readers finish. The native tests own the shared execution rules; the host checks cover what the host adds.

Install a local artifact in a clean environment and run a real call using its packaged native assets. Keep source-tree and installed-artifact evidence separate. Use an explicit isolated environment, owned home/cache/state folders and the shared loopback backend; assert zero sends with request counts where required. Keep existing test and example folders. The [surface checks](../sdlc/scripts/surfaces), [policy check](../sdlc/scripts/policy.py) and package checks supply the existing enforcement.

SQL and MCP retain their own [value](../specification/types.md) and [protocol](../specification/mcp.md) contracts while sharing Rust admission and results. Their adapters own database or protocol conversion and authority.

Engine constructors accept a nullable `thinkthen_call_error **error` output. Failure returns NULL and publishes an independent error with the native code, safe message, retryability and optional facts. Success writes NULL to the output, including when it previously held a sentinel. A NULL output discards diagnostics. Read construction failures with the `thinkthen_call_error_*` accessors and free each returned error once with `thinkthen_call_error_free`. Constructors leave thread-local diagnostic slots unchanged.
