#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    const char *const labels[] = {"billing", "shipping"};
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool appears[2] = {false, false};
    int rc = thinkthen_client_open(NULL, &client, &error);
    // appears gets one answer per label, in label order.
    if (rc == THINKTHEN_OK)
        rc = thinkthen_tag(client, "Which topics appear?", labels, 2,
                           "I was charged twice and my parcel is late again.",
                           THINKTHEN_NUL_TERMINATED, appears, &error);
    if (rc == THINKTHEN_OK) {
        // Every label that fits, possibly none.
        for (size_t i = 0; i < 2; i++)
            if (appears[i])
                puts(labels[i]);
    } else {
        report(&error);
    }
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
