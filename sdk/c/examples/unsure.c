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
    thinkthen_row row = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                                    "Does the customer ask for a refund?",
                                    THINKTHEN_NUL_TERMINATED, &refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_yes_means(
            refund, "the customer wants their money back",
            THINKTHEN_NUL_TERMINATED, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_no_means(
            refund, "the customer wants something else",
            THINKTHEN_NUL_TERMINATED, &error);
    /* Below 0.3 is no, 0.7 and above is yes, and between is not sure. */
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_band(refund, 0.3, 0.7, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, refund, "I may send it back.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &row, &error);
    if (rc == THINKTHEN_OK) {
        bool is_refund = row.answer_bool;
        puts(is_refund ? "refund" : "no refund");
    } else if (rc == THINKTHEN_UNSURE) {
        puts("not sure");
    } else {
        report(&error);
    }
    thinkthen_row_free(&row);
    thinkthen_error_free(&error);
    thinkthen_question_free(refund);
    thinkthen_client_free(client);
    return rc < 0 ? EXIT_FAILURE : EXIT_SUCCESS;
}
