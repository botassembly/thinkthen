#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_question *refund = NULL;
    thinkthen_row result = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                                    "Does the customer request a refund?",
                                    THINKTHEN_NUL_TERMINATED, &refund, &error);
    // Below 0.2 is no, 0.8 and above is yes, and between is not sure.
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_band(refund, 0.2, 0.8, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, refund,
                               "I was charged twice for one invoice.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &result, &error);
    if (rc == THINKTHEN_OK)
        puts(result.answer_bool ? "true" : "false");
    else if (rc == THINKTHEN_UNSURE)
        printf("not sure (%g): ask a person\n", result.probability);
    else
        report(&error);
    thinkthen_row_free(&result);
    thinkthen_question_free(refund);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc < 0 ? EXIT_FAILURE : EXIT_SUCCESS;
}
