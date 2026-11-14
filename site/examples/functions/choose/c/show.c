#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

int main(void) {
    enum topic { BILLING, SHIPPING, OTHER };
    const char *const words[] = {[BILLING] = "billing", [SHIPPING] = "shipping", [OTHER] = "other"};
    int choice = 0;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_row topic = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE, "What is the message about?",
                                    THINKTHEN_NUL_TERMINATED, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_choices(question, words, 3, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, question, "Please refund my order.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &topic, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_choice(&topic, &choice, &error);
    if (rc == THINKTHEN_OK) {
        puts(words[choice]);
    } else {
        report(&error);
    }
    thinkthen_row_free(&topic);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
