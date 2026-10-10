# thinkthen-cpp

This C++17 header-only CMake package installs its matching native library and header with the C++ headers. It owns C++ argument, result and error lifetimes; Rust owns question grammar and judgment. The package and its installed consumers have passed locally on Linux x86_64. Final release archives and other hosts need separate qualification.

Build the matching native C artifacts from this checkout, then install the CMake package into a local prefix:

```sh
cargo build --locked --offline --manifest-path libraries/c/Cargo.toml --lib
sh libraries/c/localize.sh libraries/c/target/debug/libthinkthen_c.a target/libthinkthen.a
cmake -S libraries/cpp -B target/thinkthen-cpp -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX="$HOME/.local/thinkthen-cpp" \
  -DTHINKTHEN_C_HEADER="$PWD/libraries/c/include/thinkthen.h" \
  -DTHINKTHEN_NATIVE_SHARED="$PWD/libraries/c/target/debug/libthinkthen_c.so" \
  -DTHINKTHEN_NATIVE_STATIC="$PWD/target/libthinkthen.a"
cmake --build target/thinkthen-cpp --parallel 2
cmake --install target/thinkthen-cpp
```

A separate CMake project can set `CMAKE_PREFIX_PATH` to that prefix and call `find_package(thinkthen-cpp CONFIG REQUIRED)`. Link `thinkthen::thinkthen_cpp_shared` or `thinkthen::thinkthen_cpp_static`; both provide `<thinkthen/client.hpp>`. Shared mode needs the installed library directory in the runtime loader path. Static-C mode still depends on Linux system libraries. The package config uses `PACKAGE_PREFIX_DIR` and also resolves a multi-component `lib/x86_64-linux-gnu` installation. It does not ship a shim library or download a native binary.

`sh libraries/cpp/check.sh` tests the installed CMake package with shared and static consumers, all ten typed calls, parser boundaries, presence and null, usage persistence, locale independence, file locations and cancellation. The routine shared cases use a counted local backend. Set `THINKTHEN_TEST_PROFILE=full` only at a candidate to run the entire required shared inventory through the same owned API.

For Linux x86-64, `sdlc/scripts/release-pack TARGET OUT c go cpp` produces versioned Go, C++ and C archives from one clean source commit. Run `sdlc/scripts/release-go-cpp-pair OUT` before using the files together. Unpack the C++ source and the matching C archive into separate folders, then pass the C archive's `include/thinkthen.h`, `lib/libthinkthen.so`, and `lib/libthinkthen.a` to CMake through the three `THINKTHEN_` inputs above. The C++ archive's `THINKTHEN-PACKAGE-INPUTS` records the exact C archive digest. Each GitHub release ships these archives, and an installed `find_package` consumer check covers them. No CMake registry package is published.

The owned caller API uses `tt::Client` and ten named methods: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. Each starts a native-owned session and returns a move-only `tt::Call`. The native engine admits the generated input values and owns validation, defaults, files, images, cache and replay behavior.

```cpp
#include <thinkthen/client.hpp>
using namespace tt::inputs;

tt::Client client;
auto call = client.decide(
    RequestQuestionText().set_text("Does the customer ask for money back?"),
    RequestInputText().set_text("Please refund my order."));
call.finish();
for (const auto& packet : call.collect()) {
    if (auto row = packet.as_SessionPacketDecideRow()) {
        auto value = row->value().value->value();
        // Presence distinguishes an absent value, explicit null and a present value.
        (void)value;
    }
}
```

`collect` blocks explicitly. An event loop uses `poll`, whose state is `pending`, `result` or `end`; a worker can own a moved call and use `collect`. A single host thread owns each call. `try_push` admits one generated feed descriptor without waiting and returns false when the native queue is full. `finish` declares input EOF. `cancel` signals stop without waiting; `close` and destruction cancel and free the native owner without joining the provider. Closing the client leaves existing calls valid. Generated C++ packet values own their data after native result cleanup.

Generated input builders accept standard strings, vectors, ordered member lists, booleans and integers. Generated result classes expose known fields and typed alternatives. Each field returns `tt::results::Presence<T>` with `absent`, `null` or `value` state. `document()` retains unknown extensions and unrestricted authored JSON. Integer conversion preserves all signed and unsigned 64-bit values. `tt::SessionFailure` retains the generated `CallError`, including native kind, retryability and facts; immediate admission errors throw `tt::NativeFailure` with the actual native error kind.

The installed consumer covers all ten named calls, presence, null, false, unknown fields, wide integers, native failure facts, file locations and held-provider cancellation and destruction. Shared cases also exercise images, cache and replay; full parity and platform qualification run at the candidate.

| Removed API | Replacement |
| --- | --- |
| `tt::create`, `tt::call`, `tt::Engine` | `tt::Client` and its ten named methods |
| `tt::many` | Named calls with `RequestInputRecords`, or `Call::try_push` |
| `tt::native` and `tt::complete` | Generated `tt::inputs` and `tt::results` values |
| Cancel token and joined worker | `Call::cancel`, `close` and RAII cleanup |
| JSON result envelopes and hand-copied views | Generated session packets with explicit presence |

The old headers and public names have been removed. The local JSON parser preserves insertion order, rejects duplicate keys and invalid strings, and retains signed and unsigned 64-bit integers without rounding. Runtime model selection, grammar, file reading and cache behavior remain in Rust.

Client settings accept a named backend, for example `tt::Client({{"backend", "local"}})`, or a direct base URL. A missing or invalid backend name fails before sending. Explicit files follow the [library reader contract](../files.md); pass a generated `RequestInputSource` with its paths and reading unit.

## Live usage persistence

The ordinary `tt::Client` exposes `usage_persistence()` and `finish_usage_status()`. Each returns a `UsageStatus` with constant `state` and optional copied `advice` members. The state uses the generated `UsagePersistenceState` enum.

These methods observe the native engine directly and refuse calls after close. Failed persistence leaves successful answers and their earlier facts intact. Written covers this engine's current deltas, not future calls or other engines. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Returned observations remain readable after close. See [the C engine contract](../c/DESIGN.md) for native observation semantics.
