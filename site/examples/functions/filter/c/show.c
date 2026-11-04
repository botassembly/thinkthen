#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

static const char *const messages[] = {
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
};

int main(void) {
    thinkthen_record_input records[3];
    for (size_t i = 0; i < 3; i++) {
        records[i].struct_size = sizeof records[i];
        records[i].kind = THINKTHEN_RECORD_TEXT;
        records[i].data = messages[i];
        records[i].len = THINKTHEN_NUL_TERMINATED;
    }
    thinkthen_client *client = NULL;
    thinkthen_question *asks_refund = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row refund_request = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_FILTER,
                                    "Does the customer request a refund?",
                                    THINKTHEN_NUL_TERMINATED, &asks_refund,
                                    &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open_records_question(
            client, asks_refund, NULL, records, 3, sizeof records[0],
            THINKTHEN_OFFSET_UTF8, &call, &error);
    // Only the messages that answer yes. A failed message returns its
    // negative code in its place, and the rest carry on.
    while (rc == THINKTHEN_OK
           || refund_request.status == THINKTHEN_STATUS_FAILED) {
        rc = thinkthen_call_next_kept(call, &refund_request, &error);
        if (rc == THINKTHEN_OK) {
            puts(messages[refund_request.position - 1]);
        } else if (refund_request.status == THINKTHEN_STATUS_FAILED) {
            fprintf(stderr, "message %" PRIu64 " failed: ",
                    refund_request.position);
            report(&error);
        }
    }
    if (rc != THINKTHEN_END)
        report(&error);
    thinkthen_row_free(&refund_request);
    thinkthen_error_free(&error);
    thinkthen_call_free(call);
    thinkthen_question_free(asks_refund);
    thinkthen_client_free(client);
    return rc == THINKTHEN_END ? EXIT_SUCCESS : EXIT_FAILURE;
}
