#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

static const thinkthen_record_input messages[] = {
    THINKTHEN_TEXT_RECORD("Please refund my order."),
    THINKTHEN_TEXT_RECORD("Where is my parcel?"),
    THINKTHEN_TEXT_RECORD("I was charged twice for one invoice."),
};

int main(void) {
    const char *const levels[] = {"routine", "urgent"};
    uint64_t rank = 0;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row ranked = THINKTHEN_ROW_INIT;
    thinkthen_record_view original;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_RANK, "How urgent is the request?",
                                    THINKTHEN_NUL_TERMINATED, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_levels(question, levels, 2, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open(client, question, messages, 3, &call, &error);
    while (rc == THINKTHEN_OK) {
        rc = thinkthen_call_next(call, &ranked, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_answer_rank(&ranked, &rank, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_original(&ranked, &original, &error);
        if (rc == THINKTHEN_OK)
            printf("%" PRIu64 " %.*s\n", rank, (int)original.len, original.data);
    }
    if (rc != THINKTHEN_END)
        report(&error);
    thinkthen_row_free(&ranked);
    thinkthen_call_free(call);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_END ? EXIT_SUCCESS : EXIT_FAILURE;
}
