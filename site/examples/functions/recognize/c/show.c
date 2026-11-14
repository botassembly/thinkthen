#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

int main(void) {
    const thinkthen_name *names = NULL;
    size_t count = 0;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_row recognized = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_RECOGNIZE, NULL,
                                    0, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_kind(question, "person", THINKTHEN_NUL_TERMINATED, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_kind(question, "organization", THINKTHEN_NUL_TERMINATED, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, question, "Morgan works for Example Group.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &recognized, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_names(&recognized, &names, &count, &error);
    if (rc == THINKTHEN_OK) {
        for (size_t i = 0; i < count; i++)
            printf("%.*s: %.*s\n", (int)names[i].kind.len, names[i].kind.data,
                   (int)names[i].text.len, names[i].text.data);
    } else {
        report(&error);
    }
    thinkthen_row_free(&recognized);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
