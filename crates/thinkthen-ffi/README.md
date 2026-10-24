# ThinkThen C ABI

This interface opens a client from [client settings](../../schemas/client.schema.json), opens calls from the [shared request](../../schemas/request.schema.json), and pulls [result rows](../../schemas/row.schema.json). The [header](include/thinkthen.h) defines the calls and ownership rules. Rust owns admission, execution, settings, errors, cancellation, offset conversion and selection rules.

## Drive a call on one thread

This C11 consumer opens the built-in stub client and sends one text record when the engine requests input. It needs no service or key. Client settings stay on the client; the call contains its function and question.

```c
#include <stdio.h>
#include <stdlib.h>
#include <thinkthen.h>

static void print_error(const thinkthen_buffer *error) {
    if (error->len != 0) {
        fwrite(error->data, 1, error->len, stderr);
        fputc('\n', stderr);
    }
}

int main(void) {
    const char settings[] = "{\"adapter\":\"stub\"}";
    const char request[] =
        "{\"function\":\"decide\",\"question\":{"
        "\"type\":\"inline\",\"value\":\"Ready?\"}}";
    const char text[] = "evidence";
    thinkthen_client *client = NULL;
    thinkthen_call *call = NULL;
    thinkthen_buffer error = {0};
    int result = EXIT_FAILURE;
    int sent = 0;

    if (thinkthen_client_open((const uint8_t *)settings, sizeof settings - 1,
                             &client, &error) != THINKTHEN_OK) {
        goto failed;
    }
    if (thinkthen_call_open(client, (const uint8_t *)request, sizeof request - 1,
                           THINKTHEN_UTF8, &call, &error)
        != THINKTHEN_OK) {
        goto failed;
    }
    for (;;) {
        thinkthen_row row = {0};
        int32_t status = thinkthen_next(call, &row, &error);
        if (status == THINKTHEN_NEEDS_INPUT) {
            if (sent) {
                thinkthen_end_input(call);
            } else {
                if (thinkthen_send_text(call, (const uint8_t *)text,
                                       sizeof text - 1, &error) != THINKTHEN_OK) {
                    goto failed;
                }
                sent = 1;
            }
        } else if (status == THINKTHEN_OK) {
            /* fields is row JSON without detail. Parse detail only if needed. */
            fwrite(row.fields.data, 1, row.fields.len, stdout);
            fputc('\n', stdout);
            thinkthen_row_free(&row);
        } else if (status == THINKTHEN_END) {
            result = EXIT_SUCCESS;
            break;
        } else {
            goto failed;
        }
    }
    goto cleanup;
failed:
    print_error(&error);
cleanup:
    thinkthen_buffer_free(&error);
    thinkthen_free(call);
    thinkthen_client_free(client);
    return result;
}
```

After `THINKTHEN_NEEDS_INPUT`, send one record or end input before pulling again. A text record crosses as raw UTF-8 through `thinkthen_send_text`. Hosts with UTF-16 strings can send counted native-endian `uint16_t` units through `thinkthen_send_utf16`. Its length counts code units and excludes a terminator. The shared strict decoder returns `usage.record` for an unpaired surrogate before transfer. Embedded NUL and valid supplementary characters retain their text. A structured record crosses as JSON through `thinkthen_send_json`. Rust owns its parsing and evidence checks. A successful send transfers the record; it does not promise an answered row. Initial records in a call request precede sent records. When all records are already in the request, call `thinkthen_end_input` before pulling. An empty input follows the same path: after opening the call, replace the example's loop with this drain loop. It sends no records and closes input before the first pull.

```c
thinkthen_end_input(call);
for (;;) {
    thinkthen_row row = {0};
    int32_t status = thinkthen_next(call, &row, &error);
    if (status == THINKTHEN_OK) {
        thinkthen_row_free(&row);
    } else if (status == THINKTHEN_END) {
        result = EXIT_SUCCESS;
        break;
    } else {
        goto failed;
    }
}
```

The same drain loop handles requests containing initial records. It releases each returned row; a consumer can read its fields before releasing it.

`next` waits for a row, completion or a request for input. It never requires a second producer thread. While it waits for backend work, a host can request cancellation from another thread. Bindings run that blocking work off their host's main thread. Input and output queues remain bounded. Memory also depends on record and row sizes, the initial request's records, and engine batching. Full ranking retains all records; a top selection retains its best records.

## Read rows and errors

`thinkthen_row_columns` returns an owned JSON envelope for foreign parsers with narrower payload limits. Status and probability stay scalars. Present identifier, answer and error fields become strings containing their original JSON bytes. Absent members remain absent; a present JSON null becomes the string `"null"`. Detail stays on the original row. Release the envelope through `thinkthen_buffer_free`. The C library and SQL hosts share this projection.

A successful `next` transfers a `thinkthen_row` to the caller. `fields` contains JSON for the row except its top-level `detail`. `detail` contains the raw JSON text for that member, or an empty buffer if the member is absent. Parse the ordinary fields for the typed answer, and parse `detail` only when a caller requests it. To reconstruct the complete contract row, insert the parsed detail under `detail` when present. `position` preserves the input position for host object identity, and `kept` carries the engine's selection result. String offsets use the convention selected at call open.

Stream calls deliver failed rows alongside answered and unsure rows. Every call reports its first failure after delivering all rows. `THINKTHEN_END` means successful completion; `THINKTHEN_ERROR` means that the operation has returned its own error JSON. Read a row's `status` separately from the operation result.

Every failing operation returns an owned error buffer containing the shared `code` and `message` object. A bad call request therefore returns its usage code from `thinkthen_call_open`, with the call pointer left null. Errors remain readable after another operation, after a thread exits and after the client or call is freed. No diagnostic state lives on a thread. Free an error through `thinkthen_buffer_free` before reusing its output slot. Non-error results leave the error slot empty.

Rows and their buffers outlive the client and call. Free each row through `thinkthen_row_free`. Free each error buffer through `thinkthen_buffer_free`. Never use the C allocator to release either. A buffer or row has one owner; move that ownership between threads rather than copying it. Cleanup clears the released struct, and accepts null pointers and empty structs. Input bytes need no NUL terminator and remain borrowed only through the entry point's return.

## Cancel and release

One host driver owns sends, input completion and pulls for each call. Those operations do not run concurrently on the same call. `thinkthen_cancel` may run concurrently from another thread while the call remains alive. Independent calls may run concurrently through the same client. A call retains its engine, so the client owner can be freed after opening calls.

Cancel requests stop, and the next pull returns `local.cancelled` in its error buffer. Already delivered rows remain valid. Cancellation aborts in-flight backend requests, and host call cleanup does not wait for their completion. `thinkthen_free` cancels, discards unread output and releases the host owner immediately without joining the worker.

Cancel active host operations and wait for those host operations to return before freeing their call pointer. Free must never race with another operation or cancellation on that pointer. Later operations on a freed pointer are invalid. Repeated terminal reads retain the shared call's terminal result, including after later cancellation. The native library must remain loaded for the process lifetime because a freed call may leave a backend worker finishing in the background.

## Build and link

The ABI crate will produce a shared library named `thinkthen_ffi` and ship this hand-written header from the same build. A C consumer includes `thinkthen.h` and links `thinkthen_ffi`. Language packages ship their own matching native asset through their installation route and retain its loader for the process lifetime. A host that unloads plugins must keep this native asset loaded independently of the plugin. The ABI adds no proxy transport or routing behavior; those remain in Rust behind the shared client and call contracts.
