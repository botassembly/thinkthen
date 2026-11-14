#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

int main(void) {
    const thinkthen_rule rule = {
        .source = {"person", THINKTHEN_NUL_TERMINATED},
        .target = {"organization", THINKTHEN_NUL_TERMINATED},
        .relation = {"works_for", THINKTHEN_NUL_TERMINATED},
    };
    const thinkthen_relation *relations = NULL;
    size_t count = 0;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_row related = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_RELATE, NULL,
                                    0, &question, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_rule(question, &rule, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, question, "Morgan works for Example Group.\nPlease refund my order.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &related, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_relations(&related, &relations, &count, &error);
    if (rc == THINKTHEN_OK) {
        for (size_t i = 0; i < count; i++)
            printf("%.*s --%.*s--> %.*s\n",
                   (int)relations[i].source.text.len, relations[i].source.text.data,
                   (int)relations[i].relation.len, relations[i].relation.data,
                   (int)relations[i].target.text.len, relations[i].target.text.data);
    } else {
        report(&error);
    }
    thinkthen_row_free(&related);
    thinkthen_question_free(question);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
