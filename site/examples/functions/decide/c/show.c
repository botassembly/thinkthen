#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

int main(void) {
    bool yes = false;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_row refund = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, "Does the customer request a refund?",
                                    THINKTHEN_NUL_TERMINATED, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, question, "Please refund my order.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_bool(&refund, &yes, &error);
    if (rc == THINKTHEN_OK) {
        puts(yes ? "true" : "false");
    } else {
        report(&error);
    }
    thinkthen_row_free(&refund);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
