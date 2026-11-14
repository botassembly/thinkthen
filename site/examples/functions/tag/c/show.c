#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

int main(void) {
    enum topic { BILLING, SHIPPING };
    const char *const words[] = {[BILLING] = "billing", [SHIPPING] = "shipping"};
    bool fits[2] = {false};
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_row topics = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_TAG, "Which topics appear?",
                                    THINKTHEN_NUL_TERMINATED, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_labels(question, words, 2, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, question, "I was charged twice and my parcel is late again.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &topics, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_tags(&topics, fits, 2, &error);
    if (rc == THINKTHEN_OK) {
        for (size_t i = 0; i < 2; i++)
            if (fits[i]) puts(words[i]);
    } else {
        report(&error);
    }
    thinkthen_row_free(&topics);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
