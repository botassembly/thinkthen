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
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row refund_request = THINKTHEN_ROW_INIT;
    thinkthen_record_view original;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_FILTER, "Does the customer request a refund?",
                                    THINKTHEN_NUL_TERMINATED, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open(client, question, messages, 3, &call, &error);
    while (rc == THINKTHEN_OK) {
        rc = thinkthen_call_next_kept(call, &refund_request, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_original(&refund_request, &original, &error);
        if (rc == THINKTHEN_OK)
            printf("%.*s\n", (int)original.len, original.data);
    }
    if (rc != THINKTHEN_END)
        report(&error);
    thinkthen_row_free(&refund_request);
    thinkthen_call_free(call);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_END ? EXIT_SUCCESS : EXIT_FAILURE;
}
