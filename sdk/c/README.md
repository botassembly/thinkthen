# C SDK

The C SDK asks questions about text from your C code. Its first function, `thinkthen_decide`, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Download the C archive for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) and extract it into `thinkthen`. It holds the header, the library and the engine, so keep them together. You need a C11 compiler and pkg-config. Save the program below as `example.c`, then compile and run it:

```sh
export PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig"
cc -std=c11 -Wall -Wextra -Werror example.c -o example \
    $(pkg-config --cflags --libs thinkthen-c) \
    -Wl,-rpath,"$PWD/thinkthen/lib"
./example
```

The `-rpath` flag tells the program where to find the library, because the pkg-config file names no run-time path. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

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

`thinkthen_decide` returns `THINKTHEN_OK` and writes `is_refund` only when there is an answer. When ThinkThen could not judge the message, it returns a negative code and leaves `is_refund` alone, so a failure never reads as no; `report` prints the code's name, such as `record.empty`, and its message. decide is not sure only when you set a low and a high bar: build the question with `thinkthen_question_new`, set the bars with `thinkthen_question_set_band`, and ask it with `thinkthen_run_one`, which returns `THINKTHEN_UNSURE` between them. Send those messages to a person.

## Next

The [C SDK page](https://thinkthen.dev/sdks/c/) shows all ten functions. The [C and C++ guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/c-cpp.md) and the [header](https://github.com/botassembly/thinkthen/blob/v0.2.0/crates/thinkthen-ffi/include/thinkthen.h) cover every option.
