#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    const char *const levels[] = {"routine", "urgent"};
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    double urgency = 0;
    int rc = thinkthen_client_open(NULL, &client, &error);
    // 0 means routine and 1 means urgent.
    if (rc == THINKTHEN_OK)
        rc = thinkthen_score(client, "How urgent is the request?", levels, 2,
                             "Please refund my order.",
                             THINKTHEN_NUL_TERMINATED, &urgency, &error);
    if (rc == THINKTHEN_OK)
        printf("%g\n", urgency);
    else
        report(&error);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
