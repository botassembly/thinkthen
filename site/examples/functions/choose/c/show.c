#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    const char *const topics[] = {"billing", "shipping", "other"};
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    size_t topic = 0;
    int rc = thinkthen_client_open(NULL, &client, &error);
    // topic is the index of the chosen string in topics.
    if (rc == THINKTHEN_OK)
        rc = thinkthen_choose(client, "What is the message about?", topics,
                              3, "Please refund my order.",
                              THINKTHEN_NUL_TERMINATED, &topic, &error);
    if (rc == THINKTHEN_OK)
        puts(topics[topic]);
    else if (rc == THINKTHEN_UNSURE)
        puts("not sure");
    else
        report(&error);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc < 0 ? EXIT_FAILURE : EXIT_SUCCESS;
}
