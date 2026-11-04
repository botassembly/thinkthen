#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

static void print_text(thinkthen_text text) {
    fwrite(text.data, 1, text.len, stdout);
}

int main(void) {
    const char *message =
        "Morgan works for Example Group.\nPlease refund my order.";
    const thinkthen_rule works_for = {
        .relation = {"works_for", THINKTHEN_NUL_TERMINATED},
        .source = {"person", THINKTHEN_NUL_TERMINATED},
        .target = {"organization", THINKTHEN_NUL_TERMINATED},
    };
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    thinkthen_value *relations = NULL;
    size_t count = 0;
    // relate takes rules, not question wording.
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_relate(client, &works_for, 1, message,
                              THINKTHEN_NUL_TERMINATED, &relations, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_value_relation_count(relations, &count, &error);
    for (size_t i = 0; rc == THINKTHEN_OK && i < count; i++) {
        thinkthen_relation relation;
        rc = thinkthen_value_relation_at(relations, i, &relation, &error);
        if (rc == THINKTHEN_OK) {
            print_text(relation.source.text);
            fputs(" --", stdout);
            print_text(relation.relation);
            fputs("--> ", stdout);
            print_text(relation.target.text);
            putchar('\n');
        }
    }
    if (rc != THINKTHEN_OK)
        report(&error);
    thinkthen_value_free(relations);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
