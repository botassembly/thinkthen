# ThinkThen for C

Call the ten functions through a C11 library. Rust owns execution, input admission, selection, offsets, cancellation and error messages. The [public header](include/thinkthen_c.h) defines the caller API.

The following example uses the built-in stub. It needs no server or key. Settings are Jansson JSON values admitted once when the client opens. A question uses the shared tagged form. Every named function returns a call with the same pull interface.

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <thinkthen_c.h>

int main(void) {
    json_t *settings = json_pack("{s:s}", "adapter", "stub");
    json_t *question = json_pack("{s:s,s:s}",
                                "type", "inline", "value", "Ready?");
    tt_client *client = NULL;
    tt_call *call = NULL;
    tt_row *row = NULL;
    tt_error *error = NULL;
    int result = EXIT_FAILURE;
    bool sent = false;
    if (!settings || !question) goto cleanup;
    client = tt_client_new(settings, &error);
    if (!client) goto cleanup;
    call = tt_decide(client, question, NULL, &error);
    if (!call) goto cleanup;
    for (;;) {
        switch (tt_next(call, &row, &error)) {
        case TT_INPUT:
            if (sent) {
                tt_end_input(call);
            } else {
                const char text[] = "evidence";
                if (!tt_send_text(call, (tt_text){text, sizeof text - 1}, &error))
                    goto cleanup;
                sent = true;
            }
            break;
        case TT_ROW:
            if (tt_row_status(row) == TT_ANSWERED) {
                bool answer;
                if (!tt_row_verdict(row, &answer)) goto cleanup;
                puts(answer ? "yes" : "no");
            } else if (tt_row_status(row) == TT_UNSURE)
                puts("not sure");
            else {
                tt_text message = tt_error_message(tt_row_error(row));
                fwrite(message.data, 1, message.len, stderr);
                fputc('\n', stderr);
            }
            tt_row_free(row);
            row = NULL;
            break;
        case TT_DONE:
            result = EXIT_SUCCESS;
            goto cleanup;
        case TT_ERROR:
            goto cleanup;
        }
    }
cleanup:
    if (error) {
        tt_text message = tt_error_message(error);
        fwrite(message.data, 1, message.len, stderr);
        fputc('\n', stderr);
    }
    tt_row_free(row);
    tt_error_free(error);
    tt_call_free(call);
    tt_client_free(client);
    json_decref(question);
    json_decref(settings);
    return result;
}
```

Send plain text as counted UTF-8 with `tt_send_text`; it requires no JSON escaping. Send structured records as Jansson values with `tt_send_json`. Send counted raw JSON with `tt_send_raw_json` when records contain integers outside Jansson's signed range. All three functions borrow input only until they return. End an empty input with `tt_end_input` before the first pull. A successful send acknowledges transfer. Read each row's status to learn its outcome.

Every call delivers all rows and then reports its first failure. The engine owns filter selection and final failure handling. The wrapper forwards rows and terminal errors without applying another threshold, selection rule or failure policy. Ranking and dependent rounds remain engine work.

Rows are opaque immutable owners. Status has a C enum and probability has a presence accessor. Identifier, answer and detail are counted raw JSON spans borrowed until `tt_row_free`:

```c
tt_text tt_row_id(const tt_row *row);
tt_text tt_row_answer(const tt_row *row);
tt_text tt_row_detail(const tt_row *row);
```

A span with `len == 0` means the member was absent. Otherwise, `data` points to `len` readable bytes of the engine's JSON, without a terminating NUL, until `tt_row_free`. JSON null is the four-byte span `null`. A failed row has no answer and exposes its own error. The spans preserve the engine's JSON values without requiring Jansson to decode them. For example, an identifier `{"a\u0000b":true}` remains readable byte for byte, including its escaped key, and `18446744073709551615` remains an integer rather than becoming a floating point approximation. Callers may copy a span or write it with `fwrite(value.data, 1, value.len, output)` without parsing it. Operation errors have independent owners with counted code and message accessors; the wrapper preserves the engine's text. Row positions and selection flags come directly from the ABI. String positions count UTF-8 bytes.

Parse an ordinary answer with `json_loadb(answer.data, answer.len, JSON_DECODE_ANY | JSON_ALLOW_NUL, &diagnostic)` when a Jansson value helps the caller. Release that separate parsed value with `json_decref`. Jansson can construct counted NUL object keys, but its decoder rejects them. Its integer decoder also has a narrower range than the engine. A Jansson decoding failure leaves the original span available and never changes a successful engine row into a call error. The wrapper retains an optional decoded answer for its typed accessors. Identifier and detail stay unparsed. A row and an operation error survive their call and client until explicitly freed. An open call retains its client state, so the caller may release the client after opening its calls. Original input positions start at one. If `tt_row_probability` returns false, it leaves the output value unchanged.

One thread drives each call. Another thread may call `tt_cancel` while a pull waits. Cancel active host operations and wait for those operations to return before freeing the call. Free releases the caller immediately; backend workers may still finish. The linked native asset must remain loaded for the process lifetime. Independent calls may share a client. The library installs no signal handler.

The [client settings schema](../../schemas/client.schema.json) defines the settings, and the [request schema](../../schemas/request.schema.json) defines tagged questions and each function's options. Pass a toolbox question or a named reference through the same `question` argument. Settings select embedded or proxy mode without changing a function signature. Proxy transport and named references are unavailable until the engine implements them; this wrapper adds no proxy implementation.

`tt_row_verdict` reads decide and filter booleans. `tt_row_score` reads a score. `tt_row_label` reads the choose label at index zero or each tag label. `tt_row_span` reads the found span at index zero or each recognized name's text, start and end. Each returns false without changing its output for a missing or incompatible value. Returned text borrows the row. Raw answer JSON always remains available.

## Package and build

Ship `thinkthen_c.h`, a small `thinkthen_c` shared library, its matching `thinkthen_ffi` native asset, and `thinkthen-c.pc`. Include a CMake install target and an exported `ThinkThen::C` target. The C library links its matching native asset at build time. Both shared libraries install under the same prefix. No runtime asset locator or format negotiation runs. A plugin may unload its wrapper only if the native asset remains loaded independently.

Use [Jansson](https://jansson.readthedocs.io/en/2.14/apiref.html) as the sole C dependency for constructing JSON requests and decoding the fixed row metadata and error objects. Jansson avoids a private JSON parser and exposes familiar C JSON values. It does not validate engine settings or requests. Identifier, answer and detail bypass Jansson parsing. The package records Jansson in pkg-config and CMake dependency metadata. C callers retain their own Jansson inputs; the wrapper does not steal references.

Install Jansson and pkg-config before compiling a consumer. On Debian or Ubuntu, run `sudo apt-get install libjansson-dev pkg-config`. On macOS, run `brew install jansson pkg-config`. The package uses the caller's Jansson installation and installs no Jansson library or headers.

Unpack the archive into a directory named `thinkthen`, save the first example as `example.c`, and compile it:

```sh
export PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
```

Then run:

```sh
cc -std=c11 example.c -o example $(pkg-config --cflags --libs thinkthen-c)
./example
```

On Windows, install the dynamic Jansson package with `vcpkg install jansson:x64-windows`. The upstream [Jansson CMake export](https://github.com/akheron/jansson/blob/v2.14.1/CMakeLists.txt) names its target `jansson::jansson`; the [vcpkg port](https://github.com/microsoft/vcpkg/blob/master/ports/jansson/portfile.cmake) preserves that config. Save the example as `example.c` beside this `CMakeLists.txt`:

```cmake
cmake_minimum_required(VERSION 3.18)
project(example C)
find_package(ThinkThenC CONFIG REQUIRED)
add_executable(example example.c)
target_link_libraries(example PRIVATE ThinkThen::C)
```

Then run in PowerShell with `VCPKG_ROOT` naming your vcpkg checkout and `thinkthen` naming the unpacked C archive:

```powershell
cmake -S . -B build -DCMAKE_TOOLCHAIN_FILE="$env:VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake" -DVCPKG_TARGET_TRIPLET=x64-windows -DCMAKE_PREFIX_PATH="$PWD/thinkthen"
cmake --build build --config Release
$env:PATH = "$PWD/thinkthen/bin;$env:VCPKG_ROOT/installed/x64-windows/bin;$env:PATH"
./build/Release/example.exe
```

The native Windows package check uses `JANSSON_ROOT` to locate the independently installed dependency. Set it to `$env:VCPKG_ROOT/installed/x64-windows` before running that check.

The installed package carries the matching native asset and supplies its runtime search location. A caller does not set a native-library path or build Rust to make a first call. Platform packages follow the native engine's supported targets.

The package tests an installed C consumer against the shared stub cases, preserves usage and backend error codes, and cancels a held loopback request. An installed consumer constructs an identifier with a nested counted NUL key, reads a successful row, frees its call and client, and compares the identifier's raw bytes without Jansson decoding. The consumer also checks absent members, JSON null, ordinary answers, failed rows and raw detail. Engine behavior remains covered by the engine tests.

Build the package from source with Jansson development files, CMake and the Rust toolchain installed. Run `scripts/c-package OUTPUT_DIRECTORY` at the repository root. The output contains an installed prefix and a platform archive. `scripts/c-sdk-gate` compiles consumers against that installed archive through pkg-config and CMake.

Host memory allocation or metadata translation failures use `local.io` and a C binding message. Payload representation never changes an engine row into a decoding error. Engine admission errors retain their original codes and messages; the wrapper does not turn them into host failures. On macOS, a host that supports plugin unloading must retain the directly linked package for the process lifetime. Linux marks the wrapper as non-unloadable so its native dependency remains loaded.
