#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

static void print_message(const thinkthen_row *row) {
    thinkthen_record_view message;
    if (thinkthen_row_original(row, &message, NULL) == THINKTHEN_OK) {
        fwrite(message.data, 1, message.len, stdout);
        putchar('\n');
    }
}

static const thinkthen_record_input messages[] = {
    {.struct_size = sizeof(thinkthen_record_input),
     .kind = THINKTHEN_RECORD_TEXT, .data = "Please refund my order.",
     .len = THINKTHEN_NUL_TERMINATED},
    {.struct_size = sizeof(thinkthen_record_input),
     .kind = THINKTHEN_RECORD_TEXT, .data = "Where is my parcel?",
     .len = THINKTHEN_NUL_TERMINATED},
    {.struct_size = sizeof(thinkthen_record_input),
     .kind = THINKTHEN_RECORD_TEXT, .data = "I want my money back.",
     .len = THINKTHEN_NUL_TERMINATED},
};

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_question *asks_refund = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row refund_request = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    size_t count = sizeof messages / sizeof messages[0];
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_FILTER,
                                    "Does the customer ask for a refund?",
                                    THINKTHEN_NUL_TERMINATED, &asks_refund,
                                    &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open_records_question(
            client, asks_refund, NULL, messages, count, sizeof messages[0],
            THINKTHEN_OFFSET_UTF8, &call, &error);
    /* Each pass frees the row it held, so one variable serves the loop. */
    while (rc == THINKTHEN_OK) {
        rc = thinkthen_call_next_kept(call, &refund_request, &error);
        if (rc == THINKTHEN_OK)
            print_message(&refund_request);
    }
    const int failed = rc != THINKTHEN_END;
    if (failed) {
        report(&error);
        while (call != NULL
               && thinkthen_call_next_failed(call, &refund_request, &error)
                      == THINKTHEN_OK) {
            fprintf(stderr, "message %" PRIu64 " failed: %s\n",
                    refund_request.position,
                    thinkthen_code_name(refund_request.error_code));
        }
    }
    thinkthen_row_free(&refund_request);
    thinkthen_error_free(&error);
    thinkthen_call_free(call);
    thinkthen_question_free(asks_refund);
    thinkthen_client_free(client);
    return failed ? EXIT_FAILURE : EXIT_SUCCESS;
}
