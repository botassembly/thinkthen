#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

int main(void) {
    thinkthen_span span;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_row action = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_FIND, "Which line states the requested action?",
                                    THINKTHEN_NUL_TERMINATED, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, question, "Morgan works for Example Group.\nPlease refund my order.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &action, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_span(&action, &span, &error);
    if (rc == THINKTHEN_OK) {
        if (span.present) printf("%.*s\n", (int)span.text.len, span.text.data);
        else puts("no line answers");
    } else {
        report(&error);
    }
    thinkthen_row_free(&action);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
