# ThinkThen for C++

Use C++17, RAII and a range-for loop to call the ten functions. The package calls the C ABI directly. Rust owns admission, batching, thresholds, selection, offsets, errors and cancellation.

The package contains a header-only C++ adapter and its matching native engine.

```cpp
#include <thinkthen/thinkthen.hpp>
#include <iostream>

int main() {
    try {
        thinkthen::client client(thinkthen::json{{"adapter", "stub"}});
        for (const auto& row : client.decide("Ready?", {"evidence"})) {
            if (row.status() == thinkthen::status::answered)
                std::cout << *row.answer() << '\n';
            else if (row.error())
                std::cerr << row.error()->code() << ": "
                          << row.error()->what() << '\n';
        }
    } catch (const thinkthen::error& error) {
        std::cerr << error.code() << ": " << error.what() << '\n';
        return 1;
    }
}
```

The built-in stub needs no key or service. A default-constructed client uses the engine's default settings. The [client schema](../../schemas/client.schema.json) owns settings, including backend and mode. Embedded and proxy settings use the same constructor and functions. Proxy availability comes from the engine.

## Calls and input

`thinkthen::json` aliases `nlohmann::json`. Each function accepts its inline question value, records and optional JSON options. Named methods return `typed_stream<Answer>`; each row has `std::optional<Answer> answer()`.

| Function | Answer type |
| --- | --- |
| decide, filter | `bool` |
| choose | `std::string` |
| tag | `std::vector<std::string>` |
| score | `double` |
| rank | `std::size_t` |
| find | `span` with text, start and end |
| recognize | `std::vector<name>` with text, start, end, kind and p |
| relate | `std::vector<relation>` with source, target, relation, p and optional either |
| annotate | `annotations`, a map of names to typed outcome containers |

An annotation exposes status, optional probability and error, and its raw answer JSON. Use `entry.answer<Answer>()` to decode a saved question's known answer type. Relation endpoints have text, start, end and kind.

A JSON string record supplies text evidence. Other JSON values supply structured evidence. The call owns the input vector and feeds one record when the ABI requests input, then closes input. It never pushes the entire vector into the engine before pulling output. Caller memory includes that vector and the engine's bounded queues and batching. Full ranking retains records in the engine.

Each function also accepts `record_source` in the input position. This is `std::function<std::optional<json>()>`: a null optional means input ended. The stream owns the source and invokes it only on its driver thread when input is requested. A returned JSON null is a record. Sources may capture an input iterator or reader and must keep their captured resources alive. A source exception cancels and closes the call before propagating unchanged. Cancellation cannot interrupt arbitrary caller code inside a source.

```cpp
std::vector<thinkthen::json> input = {"alpha", "beta"};
std::size_t index = 0;
auto source = [&]() -> std::optional<thinkthen::json> {
    if (index == input.size()) return std::nullopt;
    return input[index++];
};
for (const auto& row : client.decide("Ready?", source)) {
    std::cout << row.position() << '\n';
}
```

A named function wraps its question in the shared inline tag. It does not parse question strings as filenames. Options pass unchanged to engine admission. The [request schema](../../schemas/request.schema.json) owns their names and values. No C++ options builder repeats that grammar. A saved question uses `tagged_question` in any named method, with either a record vector or a lazy source:

```cpp
auto question = thinkthen::tagged_question{thinkthen::json{
    {"type", "toolbox"}, {"name", "color"},
    {"value", {{"questions", {{"color", {{"choose", "Pick a color"},
        {"options", {"red", "blue"}}}}}}}}
}};
auto rows = client.choose(question, source);
```

Saved choose, tag and score questions need no repeated domain options. A caller with a complete tagged request uses `client.open(json request)`. This forwards the complete shared request, including toolbox tags and initial records, then ends input. It adds no function dispatcher or validation. Normal callers use the ten methods.

## All ten functions

These expressions use the client above. Each returns a stream and can appear directly after the colon in a range-for loop. Constructing a stream admits its request; iteration drives execution.

```cpp
client.decide("Ready?", {"evidence"});
client.choose("Pick a color", {"evidence"},
              {{"options", {"red", "green", "blue"}}});
client.tag("Which labels apply?", {"evidence"},
           {{"labels", {"red", "green", "blue"}}});
client.score("How urgent?", {"evidence"},
             {{"levels", {"low", "high"}}});
client.filter("Ready?", {"alpha", "beta"});
client.rank("How relevant?", {"alpha", "beta"}, {{"top", 1}});
client.find("When is the meeting?", {"The meeting is Monday."});
client.annotate({{"questions", {{"ready", {{"decide", "Ready?"}}}}}},
                {"evidence"});
client.recognize({"person", "organization"}, {"Ada works at Acme."});
client.relate(thinkthen::json::array({
    {{"name", "works_for"}, {"source", "person"},
     {"target", "organization"}, {"reads", "works for"}}
}), {"Ada works at Acme."});
```

Every call delivers all rows, including failed rows, before reporting its first failure. An operation failure throws `thinkthen::error`, derived from `std::runtime_error`, with `const std::string& code() const noexcept`. The exception copies the engine code and message and survives every handle. Failed rows carry the same error value. Rust owns the completion policy. All ten methods expose ABI rows. In particular, `filter` returns rows; callers inspect `kept()` to select retained records. Rows arrive in ABI order, and `position()` identifies the original input record. The adapter neither filters rows nor sorts them. Positions and selection flags come from the ABI.

## Rows and JSON ownership

`row` is a small hand-written immutable value. It exposes const accessors and permits copying and moving. Its private owned data has no mutable public reference. A copied row survives iterator advancement, stream destruction and client destruction.

| Accessor | Result |
| --- | --- |
| `status()` | `thinkthen::status`: answered, unsure or failed |
| `id()` | `const json*`, null pointer when absent |
| `answer()` | named function's optional answer type; raw `row` returns `const json*` |
| `raw_answer()` | `const json*`, null pointer when absent |
| `probability()` | `std::optional<double>` |
| `error()` | `const thinkthen::error*`, null pointer when absent |
| `detail()` | `std::optional<json>`, parsed only on request |
| `position()` | original one-based input position |
| `kept()` | engine selection flag |

A present JSON null differs from an absent member. Identifier and raw answer values use the ordinary JSON type, including nested objects with counted NUL keys and unsigned 64-bit integers. Detail remains owned raw bytes until `detail()` parses it into a separate JSON value. Repeated detail access parses again; there is no shared mutable cache. Accessor pointers remain valid until that row is destroyed, assigned to or moved from. String offsets count UTF-8 bytes.

C++17 supplies no JSON value. The sole C++ dependency is [JSON for Modern C++](https://github.com/nlohmann/json). Its [default representation](https://json.nlohmann.me/home/architecture/) stores strings and keys in `std::string` and unsigned integers in `std::uint64_t`. This fits the engine's JSON payloads without a custom parser or Jansson's narrower decoder. Counted strings retain embedded NUL bytes. Callers construct such keys with `std::string("a\0b", 3)` and large integers with `std::uint64_t`. Conversion to an application-selected narrower number remains the application's choice. The package does not normalize payload values through floating point.

The adapter serializes inputs and decodes output only for host representation. It performs no schema or engine rule checks. Caller JSON construction and accessor conversion errors remain nlohmann exceptions; allocation failures remain standard allocation exceptions. Engine failures retain their dotted code and message. No host exception crosses the C ABI.

## Iteration, cancellation and threads

A stream is a move-only single-pass input range. `begin()` obtains its first row; increment pulls the next row. Dereference exposes a const row until the next increment or close. Copy the row to retain it. A range may be iterated once. There is one driver thread per stream. `typed_stream<Answer>::next()` returns `std::optional<typed_row<Answer>>` for callers who prefer an explicit loop; do not mix it with an active range iterator. End of range means successful engine completion. Terminal errors throw from begin, increment or next.

A stream destructor cancels and frees its call without joining backend work. A temporary range in `for (const auto& row : client.decide(...))` is destroyed when the loop exits, including `break` and exceptions. A named range remains alive after a break; call its idempotent `close() noexcept` or end its scope to release it. Closed streams have no further rows. Iterator use after close is invalid.

`stream.cancel_handle()` returns a copyable `thinkthen::cancel_handle` with `request_cancel() const noexcept`. It safely requests cancellation from another thread while next waits for a held backend response. The handle uses a weak reference to call state and does nothing after closure. A cancellation request briefly retains the state; native free cannot race that request. The driver retains state throughout each host operation. No mutex needed by cancellation remains locked around a blocking pull. Stream close and move belong to the driver thread and cannot race another driver operation. The cancel handle is the sole operation permitted from another thread on that stream.

```cpp
auto rows = client.decide("Ready?", {"evidence"});
auto stop = rows.cancel_handle();
// Give stop to the application's existing control thread.
// That thread calls stop.request_cancel() when the user requests stop.
for (const auto& row : rows) {
    if (application_has_enough(row)) {
        rows.close();
        break;
    }
}
```

This C++17 handle avoids requiring C++20 stop tokens or a wrapper-owned worker thread. Cancellation closes engine connections and preserves `local.cancelled`. A failed row may precede the terminal cancellation error, as the shared engine contract permits. DNS and connection establishment remain subject to the engine's timeout. The adapter installs no signal handlers.

Clients are move-only RAII owners. Existing calls retain their engine after their client is destroyed. Const function calls may run concurrently on one live client. Client destruction or move must not race opening calls. Independent streams may run on separate threads. Immutable rows may be read on separate threads while their owners remain alive; `detail()` creates an independent value.

## CMake package

Ship headers, the matching release `thinkthen_ffi` shared library, dependency notices and `ThinkThenCppConfig.cmake` in platform archives for Linux x86-64 and arm64, macOS arm64, and Windows x86-64. The C++ adapter is header-only and adds no compiled C++ ABI or second native layer. The exported `ThinkThen::Cpp` interface target supplies C++17, headers, the JSON dependency and the native link dependency. The native asset comes from the package build. Consumers do not build Rust.

```cmake
cmake_minimum_required(VERSION 3.20)
project(example LANGUAGES CXX)
set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)
find_package(ThinkThenCpp CONFIG REQUIRED)
add_executable(example main.cpp)
target_link_libraries(example PRIVATE ThinkThen::Cpp)
```

Save the first example as `main.cpp` and the CMake block as `CMakeLists.txt`. Unpack the archive into a directory named `thinkthen`, then run:

```sh
cmake -S . -B build -DCMAKE_PREFIX_PATH="$PWD/thinkthen"
cmake --build build
./build/example
```

The package uses the JSON dependency's [CMake target](https://json.nlohmann.me/integration/cmake/) through `find_dependency(nlohmann_json CONFIG REQUIRED)`. Consumers install that header dependency before configuring. Preparation supplies it offline for package proofs. Configuration performs no downloads or FetchContent operations. Consumer prerequisites are CMake, a C++17 compiler, its matching standard library and the JSON development package. Apple Clang with libc++ and GCC with libstdc++ are the first proof compilers. Platform archives inherit the release engine's deployment floors and system dependencies.

The native dependency installs beside the package's library files. The imported CMake target supplies its relocated link location. `CMAKE_INSTALL_RPATH_USE_LINK_PATH` retains that dependency directory when installing an application, as in the example; configure the consumer after moving the package prefix. There is no runtime search API, path override, downloader, version probe or fallback native asset. The OS loads the matching directly linked library. The native asset remains loaded for the process lifetime because workers can finish after call cleanup. A host that unloads plugins must retain the native asset independently; ordinary application linkage supplies its lifetime. No loader framework enters the adapter.

## Package proof

Package tests run an installed consumer against the shared stub cases on all four platforms. Linux and macOS tests install the package in a fresh prefix, move that prefix, then configure and install a consumer against the moved prefix. They run the installed executable with the source tree unavailable and loader-path environment variables unset. This proves that the installed CMake target and native runtime location survive relocation. Windows tests compile a consumer against the extracted headers and native import library and run it with the packaged DLL on PATH. One small public-API driver reads the shared construction cases, forwards their JSON through `client.open`, reconstructs rows including requested detail, and compares the existing expected values and request counts. Named methods receive public examples for all ten functions. The driver does not implement a second expectation checker.

Surface checks cover RAII cleanup after range break and exceptions, source exceptions, rows retained after call destruction, absent versus null, NUL keys and unsigned 64-bit values, lazy detail, move ownership and a cancel handle retained after closure. Usage and unavailable-backend errors preserve engine codes. Held-loopback cancellation waits for the fixture's request-arrived event, cancels from another thread and verifies connection closure and termination. A broken cancellation implementation must fail that proof. There are no sleeps or timing-only success conditions. Shared fixture isolation supplies clean settings and credentials. Builds and tests run offline through the project's job runner. Agents make no live backend calls. Publication is outside this ticket.
