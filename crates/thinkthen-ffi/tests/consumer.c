#define _POSIX_C_SOURCE 200809L
#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <thinkthen.h>

static void print_buffer(const thinkthen_buffer *buffer) {
    if (buffer->len) assert(fwrite(buffer->data, 1, buffer->len, stdout) == buffer->len);
}

static void print_error(thinkthen_buffer *error) {
    fputs("{\"error\":", stdout);
    print_buffer(error);
    puts("}");
    thinkthen_buffer_free(error);
    assert(error->data == NULL && error->len == 0);
}

static char *line(void) {
    char *text = NULL;
    size_t size = 0;
    ssize_t len = getline(&text, &size, stdin);
    if (len < 0) { free(text); return NULL; }
    if (len && text[len - 1] == '\n') text[len - 1] = '\0';
    return text;
}

static void print_row(thinkthen_row *row) {
    assert(row->fields.len != 0);
    fputs("{\"fields\":", stdout);
    print_buffer(&row->fields);
    fputs(",\"detail\":", stdout);
    if (row->detail.data) print_buffer(&row->detail); else fputs("null", stdout);
    printf(",\"position\":%zu,\"kept\":%u}\n", row->position, row->kept);
    thinkthen_row_free(row);
    assert(row->fields.data == NULL && row->detail.data == NULL);
    assert(row->position == 0 && row->kept == 0);
}

static void *cancel_on_input(void *opaque) {
    assert(getchar() == 'x');
    thinkthen_cancel(opaque);
    return NULL;
}

static void *release_error(void *opaque) {
    thinkthen_buffer *error = opaque;
    assert(error->len != 0);
    /* Buffers are counted, so do not read them as NUL-terminated strings. */
    print_error(error);
    return NULL;
}

static void ownership(void) {
    thinkthen_client *client = NULL;
    thinkthen_call *call = NULL;
    thinkthen_buffer error = {0};
    const char settings[] = "{\"adapter\":\"stub\"}";
    const char request[] = "{\"function\":\"decide\",\"question\":{\"type\":\"inline\",\"value\":\"Ready?\"}}";
    assert(thinkthen_client_open((const uint8_t *)settings, sizeof settings - 1, &client, &error) == THINKTHEN_OK);
    assert(thinkthen_call_open(client, (const uint8_t *)request, sizeof request - 1, THINKTHEN_CODE_POINTS, &call, &error) == THINKTHEN_OK);
    thinkthen_client_free(client);
    assert(thinkthen_send_text(call, NULL, 0, &error) == THINKTHEN_ERROR);
    thinkthen_buffer saved = error;
    error = (thinkthen_buffer){0};
    thinkthen_row row = {0};
    assert(thinkthen_next(call, &row, &error) == THINKTHEN_NEEDS_INPUT);
    const uint8_t invalid[] = {0xff};
    assert(thinkthen_send_text(call, invalid, sizeof invalid, &error) == THINKTHEN_ERROR);
    print_error(&error);
    const char text[] = "raw \"quoted\" 😀\n";
    assert(thinkthen_send_text(call, (const uint8_t *)text, sizeof text - 1, &error) == THINKTHEN_OK);
    thinkthen_end_input(call);
    thinkthen_end_input(call);
    assert(thinkthen_next(call, &row, &error) == THINKTHEN_OK);
    thinkthen_row saved_row = row;
    row = (thinkthen_row){0};
    assert(thinkthen_next(call, &row, &error) == THINKTHEN_END);
    assert(thinkthen_next(call, &row, &error) == THINKTHEN_END);
    thinkthen_free(call);
    print_row(&saved_row);
    pthread_t reader;
    assert(pthread_create(&reader, NULL, release_error, &saved) == 0);
    assert(pthread_join(reader, NULL) == 0);
    thinkthen_buffer_free(&saved);
    thinkthen_buffer_free(NULL);
    thinkthen_row_free(NULL);
    thinkthen_free(NULL);
    thinkthen_client_free(NULL);
}

int main(int argc, char **argv) {
    assert(argc == 2);
    if (strcmp(argv[1], "ownership") == 0) { ownership(); return 0; }
    char *settings = line();
    char *request = line();
    assert(settings != NULL && request != NULL);
    thinkthen_client *client = NULL;
    thinkthen_call *call = NULL;
    thinkthen_buffer error = {0};
    int32_t status = thinkthen_client_open((const uint8_t *)settings, strlen(settings), &client, &error);
    free(settings);
    if (status == THINKTHEN_OK) {
        uint32_t offsets = THINKTHEN_UTF8;
        if (strcmp(argv[1], "utf16") == 0) offsets = THINKTHEN_UTF16;
        if (strcmp(argv[1], "points") == 0) offsets = THINKTHEN_CODE_POINTS;
        if (strcmp(argv[1], "invalid-offset") == 0) offsets = UINT32_MAX;
        status = thinkthen_call_open(client, (const uint8_t *)request, strlen(request), offsets, &call, &error);
    }
    free(request);
    thinkthen_client_free(client);
    if (status == THINKTHEN_ERROR) { print_error(&error); return 0; }
    pthread_t canceller;
    int cancel = strcmp(argv[1], "cancel") == 0;
    if (cancel || strcmp(argv[1], "free-held") == 0) thinkthen_end_input(call);
    if (cancel) assert(pthread_create(&canceller, NULL, cancel_on_input, call) == 0);
    if (strcmp(argv[1], "free-held") == 0) {
        assert(getchar() == 'x');
        thinkthen_free(call);
        puts("{\"freed\":true}");
        return 0;
    }
    size_t needs = 0;
    for (;;) {
        thinkthen_row row = {0};
        status = thinkthen_next(call, &row, &error);
        if (status == THINKTHEN_NEEDS_INPUT) {
            ++needs;
            char *record = line();
            if (record == NULL) { thinkthen_end_input(call); continue; }
            assert(record[0] == 'T' || record[0] == 'J');
            status = record[0] == 'T'
                ? thinkthen_send_text(call, (const uint8_t *)record + 1, strlen(record + 1), &error)
                : thinkthen_send_json(call, (const uint8_t *)record + 1, strlen(record + 1), &error);
            free(record);
            if (status == THINKTHEN_ERROR) { print_error(&error); break; }
        } else if (status == THINKTHEN_OK) {
            print_row(&row);
        } else if (status == THINKTHEN_END) {
            printf("{\"end\":true,\"needs\":%zu}\n", needs);
            break;
        } else {
            assert(row.fields.data == NULL && row.detail.data == NULL);
            print_error(&error);
            /* A terminal error is owned, repeatable, and independent. */
            assert(thinkthen_next(call, &row, &error) == THINKTHEN_ERROR);
            thinkthen_buffer_free(&error);
            break;
        }
    }
    if (cancel) assert(pthread_join(canceller, NULL) == 0);
    thinkthen_free(call);
    return 0;
}
