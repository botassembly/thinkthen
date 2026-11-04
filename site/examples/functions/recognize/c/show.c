#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    const char *const kinds[] = {"person", "organization"};
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    thinkthen_value *names = NULL;
    size_t count = 0;
    // recognize takes kinds, not question wording.
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_recognize(client, kinds, 2,
                                 "Morgan works for Example Group.",
                                 THINKTHEN_NUL_TERMINATED, &names, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_value_name_count(names, &count, &error);
    for (size_t i = 0; rc == THINKTHEN_OK && i < count; i++) {
        thinkthen_name name;
        rc = thinkthen_value_name_at(names, i, &name, &error);
        if (rc == THINKTHEN_OK) {
            fwrite(name.kind.data, 1, name.kind.len, stdout);
            fputs(": ", stdout);
            fwrite(name.text.data, 1, name.text.len, stdout);
            putchar('\n');
        }
    }
    if (rc != THINKTHEN_OK)
        report(&error);
    thinkthen_value_free(names);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
