# thinkthen-cpp

This C++17 header-only package wraps the separately installed ThinkThen C library. It owns C++ argument, result and error lifetimes; Rust owns question grammar and judgment. The package and its installed consumers have passed locally on Linux x86_64. Final release archives and other hosts need separate qualification.

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

A separate CMake project can set `CMAKE_PREFIX_PATH` to that prefix and call `find_package(thinkthen-cpp CONFIG REQUIRED)`. Link `thinkthen::thinkthen_cpp_shared` or `thinkthen::thinkthen_cpp_static`; both provide `<thinkthen/door.hpp>`. Shared mode needs the installed library directory in the runtime loader path. Static-C mode still depends on Linux system libraries. The package config uses `PACKAGE_PREFIX_DIR` and also resolves a multi-component `lib/x86_64-linux-gnu` installation. It does not ship a shim library or download a native binary.

`sh libraries/cpp/check.sh 0` checks the current 31-export native ABI, parser boundaries, the 55-case schema and 30 executable public corpus cases, four installed shared/static/multilib/sanitized consumers with 16 exact request bodies each, and planted failures. It uses only a counted loopback backend. The C++ sanitizer checks the C++ consumer, not Rust allocations.

For Linux x86-64, `sdlc/scripts/release-pack TARGET OUT c go cpp` produces versioned Go, C++ and C archives from one clean source commit. Run `sdlc/scripts/release-go-cpp-pair OUT` before using the files together. Unpack the C++ source and the matching C archive into separate folders, then pass the C archive's `include/thinkthen.h`, `lib/libthinkthen.so`, and `lib/libthinkthen.a` to CMake through the three `THINKTHEN_` inputs above. The C++ archive's `THINKTHEN-PACKAGE-INPUTS` records the exact C archive digest. Each GitHub release ships these archives, and an installed `find_package` consumer check covers them. No CMake registry package is published.

`tt::Engine`, `tt::CancelToken`, and `tt::OwnedString` are move-only RAII owners. Join worker and canceller threads before freeing the token or engine. `tt::create(settings)` accepts the current C settings JSON. `tt::call` returns the JSON success envelope, including `value` and `facts`. The typed `decide`, `many`, `recognize`, and `relate` helpers each return `CallResult<T>` with the former value in `.value` and that call's facts object in `.facts` as `tt::Json`. `specification/result.schema.json` describes every JSON value; read the members you need and ignore the rest. Recognize and relate values stay JSON, and entity offsets count Unicode scalars, not UTF-16 units. `call`, `recognize` and `relate` take the same `deadline` milliseconds and cancel token as `decide` and `many`. `tt::plan(engine, verb, question, input, settings)` previews a `decide`, `choose`, `score` or `tag` call through `thinkthen_plan_json` and returns the result schema's `plan` object; the question is a JSON string of bare text or one question object. It needs no key, reads no cache and sends nothing. `tt::create(settings)` passes engine settings such as `max_requests_total` to the C constructor unchanged. The package offers no probability option on score or tag, so it has no probability refusal to make.

`tt::Failure` names one of six `tt::ErrorKind` values, codes 1 to 6, and copies the message, retry flag and borrowed final facts before another native call on the same thread. `tt::annotatedField` reads one annotate answer member as `tt::Unresolved` (JSON null), its JSON value, or a `tt::FailedField` with kind and cause; `tt::annotatedDecision` does the same for a decide member with a `tt::Outcome`. An unresolved `null` value is never a failure.

The local MIT JSON parser preserves object insertion order for label maps and rejects invalid strings and duplicate keys. Numbers are `double`; this is not arbitrary-precision number support. Synthetic replies prove the integration boundary, not model accuracy or a published release.
