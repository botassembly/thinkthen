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
    thinkthen_question *by_urgency = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row ranked = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_RANK,
                                    "How urgent is the request?",
                                    THINKTHEN_NUL_TERMINATED, &by_urgency,
                                    &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_level(by_urgency, "routine",
                                          THINKTHEN_NUL_TERMINATED, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_level(by_urgency, "urgent",
                                          THINKTHEN_NUL_TERMINATED, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open_records_question(
            client, by_urgency, NULL, records, 3, sizeof records[0],
            THINKTHEN_OFFSET_UTF8, &call, &error);
    // Rank 1 is the most urgent. position is the message's place in
    // messages, starting at 1. A failed message is a row of its own.
    while (rc == THINKTHEN_OK
           && (rc = thinkthen_call_next(call, &ranked, &error))
                  == THINKTHEN_OK) {
        if (ranked.status == THINKTHEN_STATUS_ANSWERED)
            printf("%" PRIu64 " %s\n", ranked.rank,
                   messages[ranked.position - 1]);
        else
            fprintf(stderr, "message %" PRIu64 " failed: %s\n",
                    ranked.position, thinkthen_code_name(ranked.error_code));
    }
    if (rc != THINKTHEN_END)
        report(&error);
    thinkthen_row_free(&ranked);
    thinkthen_error_free(&error);
    thinkthen_call_free(call);
    thinkthen_question_free(by_urgency);
    thinkthen_client_free(client);
    return rc == THINKTHEN_END ? EXIT_SUCCESS : EXIT_FAILURE;
}
