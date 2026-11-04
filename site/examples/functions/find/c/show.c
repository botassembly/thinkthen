#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    const char *message =
        "Morgan works for Example Group.\nPlease refund my order.";
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    thinkthen_span action = {.present = 0};
    int rc = thinkthen_client_open(NULL, &client, &error);
    // action points into message, so keep message while you read it.
    if (rc == THINKTHEN_OK)
        rc = thinkthen_find(client, "Which line states the requested action?",
                            message, THINKTHEN_NUL_TERMINATED, &action,
                            &error);
    if (rc == THINKTHEN_OK && action.present) {
        fwrite(action.text.data, 1, action.text.len, stdout);
        putchar('\n');
    } else if (rc == THINKTHEN_OK) {
        puts("no line answers");
    } else if (rc == THINKTHEN_UNSURE) {
        puts("not sure");
    } else {
        report(&error);
    }
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc < 0 ? EXIT_FAILURE : EXIT_SUCCESS;
}
