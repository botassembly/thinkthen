#include <thinkthen.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    thinkthen_settings *settings = NULL;
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool ready = false;
    int status = thinkthen_settings_new(&settings, &error);
    if (status == THINKTHEN_OK)
        status = thinkthen_settings_set_max_requests(settings, 100, &error);
    if (status == THINKTHEN_OK)
        status = thinkthen_client_open(settings, &client, &error);
    thinkthen_settings_free(settings);
    if (status == THINKTHEN_OK)
        status = thinkthen_decide(client, "Ready?", "Ready to send.",
                                  THINKTHEN_NUL_TERMINATED, &ready, &error);
    thinkthen_usage usage = {0};
    usage.struct_size = sizeof usage;
    if (status == THINKTHEN_OK)
        status = thinkthen_client_usage_values(client, &usage, &error);
    thinkthen_client_free(client);
    if (status == THINKTHEN_OK) {
        printf("ready=%s attempts=%" PRIu64 "\n",
               ready ? "true" : "false", usage.attempts);
        if (usage.input_tokens.reported_known)
            printf("reported input tokens=%" PRIu64 " unknown attempts=%"
                   PRIu64 "\n", usage.input_tokens.reported,
                   usage.input_tokens.unknown_attempts);
    } else {
        fprintf(stderr, "%s: ", thinkthen_code_name(error.code));
        fwrite(error.message.data, 1, error.message.len, stderr);
        fputc('\n', stderr);
    }
    thinkthen_error_free(&error);
    return status == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
