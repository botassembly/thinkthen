# C SDK

The C SDK asks questions about text from your C code. Its first function, `thinkthen_decide`, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Download the C archive for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) and extract it into `thinkthen`. It holds the header, the library and the engine, so keep them together. You need a C11 compiler and pkg-config. Save the program below as `example.c`, then compile and run it. These commands are for Linux and macOS; [Build on Windows](#build-on-windows) has the Windows ones.

```sh
export PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig"
cc -std=c11 -Wall -Wextra -Werror example.c -o example \
    $(pkg-config --cflags --libs thinkthen-c) \
    -Wl,-rpath,"$PWD/thinkthen/lib"
./example
```

The `-rpath` flag tells the program where to find the library, because the pkg-config file names no run-time path. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Build on Windows

Extract the C archive for Windows, `thinkthen-c-0.2.0-windows-x86_64.zip`, into `thinkthen`. You need clang-cl and the Visual Studio C++ tools, and no pkg-config. Open Developer PowerShell for Visual Studio in the folder that holds `thinkthen` and `example.c`, then run:

```powershell
$flags = "/nologo", "/std:c11", "/MD", "/Ithinkthen\include"
$lib = "thinkthen\lib\thinkthen_ffi.lib"
clang-cl @flags example.c /Fe:example.exe /link $lib
$env:PATH = "$PWD\thinkthen\bin;$env:PATH"
.\example.exe
```

`thinkthen_ffi.lib` links the program. When it runs, Windows looks for `thinkthen_ffi.dll` on `PATH`, so the `$env:PATH` line adds `thinkthen\bin`; copying the DLL beside `example.exe` works too. Releases are tested with clang-cl.

## Ask a question

Ask whether a support message requests a refund:

```c
#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool is_refund = false;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_decide(client, "Does the customer request a refund?",
                              "Please refund my order.",
                              THINKTHEN_NUL_TERMINATED, &is_refund, &error);
    if (rc == THINKTHEN_OK)
        puts(is_refund ? "true" : "false");
    else
        report(&error);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
```

Output:

```text
true
```

`true` means yes: the message asks for a refund.

## Not sure and failures

`thinkthen_decide` returns `THINKTHEN_OK` and writes `is_refund` only when there is an answer. When ThinkThen could not judge the message, it returns a negative code and leaves `is_refund` alone, so a failure never reads as no; `report` prints the code's name, such as `record.empty`, and its message. decide is not sure only when you set a low and a high bar: `thinkthen_decide_band(client, question, 0.3, 0.7, text, len, &is_refund, &error)` answers no below 0.3 and yes at 0.7 or above, and returns `THINKTHEN_UNSURE` between them with `is_refund` unchanged. Send those messages to a person.

## Next

The [C SDK page](https://thinkthen.dev/sdks/c/) shows all ten functions. The [C and C++ guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/c-cpp.md) and the [header](https://github.com/botassembly/thinkthen/blob/v0.2.0/crates/thinkthen-ffi/include/thinkthen.h) cover every option.

## Request allowance

This complete program opens an explicit client, caps its remote attempts and copies usage before freeing it:

```c
#include <thinkthen.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    thinkthen_settings *settings = NULL;
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool ready = false;
    int status = thinkthen_settings_new(&settings, &error);
    if (status == THINKTHEN_OK)
        status = thinkthen_settings_set_max_requests(settings, 100, &error);
    if (status == THINKTHEN_OK)
        status = thinkthen_client_open(settings, &client, &error);
    thinkthen_settings_free(settings);
    if (status == THINKTHEN_OK)
        status = thinkthen_decide(client, "Ready?", "Ready to send.",
                                  THINKTHEN_NUL_TERMINATED, &ready, &error);
    thinkthen_usage usage = {0};
    usage.struct_size = sizeof usage;
    if (status == THINKTHEN_OK)
        status = thinkthen_client_usage_values(client, &usage, &error);
    thinkthen_client_free(client);
    if (status == THINKTHEN_OK) {
        printf("ready=%s attempts=%" PRIu64 "\n",
               ready ? "true" : "false", usage.attempts);
        if (usage.input_tokens.reported_known)
            printf("reported input tokens=%" PRIu64 " unknown attempts=%"
                   PRIu64 "\n", usage.input_tokens.reported,
                   usage.input_tokens.unknown_attempts);
    } else {
        fprintf(stderr, "%s: ", thinkthen_code_name(error.code));
        fwrite(error.message.data, 1, error.message.len, stderr);
        fputc('\n', stderr);
    }
    thinkthen_error_free(&error);
    return status == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.
