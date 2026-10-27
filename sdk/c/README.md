# ThinkThen for C

Download the matching C archive from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) and extract it into `thinkthen`. The package requires a C11 compiler and pkg-config. Save the example as `example.c`, then compile and run it:

```sh
cc -std=c11 example.c -o example $(PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig" pkg-config --cflags --libs thinkthen-c)
case "$(uname -s)" in
    Linux) export LD_LIBRARY_PATH="$PWD/thinkthen/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ;;
    Darwin) export DYLD_LIBRARY_PATH="$PWD/thinkthen/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}" ;;
esac
./example
```

The loader uses `LD_LIBRARY_PATH` on Linux and `DYLD_LIBRARY_PATH` on macOS to find the installed shared library. Keep the selected variable set when running the program.

The example uses the stub backend and needs no key or service.

```c
#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    thinkthen_settings *settings = NULL;
    thinkthen_question *question = NULL;
    thinkthen_client *client = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    thinkthen_row row = {.struct_size = sizeof row};
    int result = EXIT_FAILURE;
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version())) {
        fputs(THINKTHEN_ABI_ERROR_MESSAGE "\n", stderr);
        return result;
    }
    if (thinkthen_settings_new(&settings, &error) != THINKTHEN_OK) goto cleanup;
    if (thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, &error) != THINKTHEN_OK)
        goto cleanup;
    if (thinkthen_client_open(settings, &client, &error) != THINKTHEN_OK) goto cleanup;
    const uint8_t wording[] = "Ready?";
    if (thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, wording, sizeof wording - 1,
                               &question, &error) != THINKTHEN_OK) goto cleanup;
    const uint8_t evidence[] = "evidence";
    if (thinkthen_run_one(client, question, evidence, sizeof evidence - 1,
                          THINKTHEN_RECORD_TEXT, &row, &error) != THINKTHEN_OK) goto cleanup;
    if (row.status == THINKTHEN_STATUS_ANSWERED)
        puts(row.answer_bool ? "yes" : "no");
    else if (row.status == THINKTHEN_STATUS_UNSURE)
        puts("not sure");
    else {
        fprintf(stderr, "%s\n", thinkthen_code_name(row.error_code));
        goto cleanup;
    }
    result = EXIT_SUCCESS;
cleanup:
    if (error.code) {
        fwrite(error.message.data, 1, error.message.len, stderr);
        fputc('\n', stderr);
    }
    thinkthen_row_free(&row);
    thinkthen_error_free(&error);
    thinkthen_question_free(question);
    thinkthen_client_free(client);
    thinkthen_settings_free(settings);
    return result;
}
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/crates/thinkthen-ffi/include/thinkthen.h).
