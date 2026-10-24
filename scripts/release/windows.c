/* Exercise the packaged Windows C ABI using requests supplied by the shared cases. */
#include "thinkthen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int failure(thinkthen_buffer *error) {
    fwrite(error->data, 1, error->len, stderr);
    thinkthen_buffer_free(error);
    return 1;
}

int main(void) {
    size_t length = 0, capacity = 4096;
    uint8_t *request = malloc(capacity);
    if (!request) return 1;
    int byte;
    while ((byte = getchar()) != EOF) {
        if (length == capacity) {
            capacity *= 2;
            uint8_t *larger = realloc(request, capacity);
            if (!larger) { free(request); return 1; }
            request = larger;
        }
        request[length++] = (uint8_t)byte;
    }
    const char *settings = "{\"adapter\":\"stub\"}";
    thinkthen_client *client = NULL;
    thinkthen_call *call = NULL;
    thinkthen_buffer error = {0};
    int status = thinkthen_client_open((const uint8_t *)settings, strlen(settings), &client, &error);
    if (status == THINKTHEN_OK)
        status = thinkthen_call_open(client, request, length,
                                    THINKTHEN_CODE_POINTS, &call, &error);
    free(request);
    thinkthen_client_free(client);
    if (status != THINKTHEN_OK) return failure(&error);
    thinkthen_end_input(call);
    thinkthen_row row = {0};
    while ((status = thinkthen_next(call, &row, &error)) == THINKTHEN_OK) {
        fwrite(row.fields.data, 1, row.fields.len, stdout);
        putchar('\n');
        fwrite(row.detail.data, 1, row.detail.len, stdout);
        putchar('\n');
        thinkthen_row_free(&row);
    }
    thinkthen_free(call);
    return status == THINKTHEN_END ? 0 : failure(&error);
}
