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

static void print_error(thinkthen_error *error) {
    assert(error->code != 0 && error->message.len != 0);
    printf("{\"error\":{\"code\":\"%s\"}}\n", thinkthen_code_name(error->code));
    thinkthen_error_free(error);
    assert(error->code == 0 && error->message.data == NULL && error->message.len == 0);
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
    const char *status = row->status == THINKTHEN_STATUS_ANSWERED ? "answered"
        : row->status == THINKTHEN_STATUS_UNSURE ? "unsure" : "failed";
    printf("{\"fields\":{\"status\":\"%s\"", status);
    if (row->answer.len) { fputs(",\"answer\":", stdout); print_buffer(&row->answer); }
    if (row->error.len) { fputs(",\"error\":", stdout); print_buffer(&row->error); }
    if (row->id.len) { fputs(",\"id\":", stdout); print_buffer(&row->id); }
    if (row->source.len) { fputs(",\"source\":", stdout); print_buffer(&row->source); }
    fputs("},\"detail\":", stdout);
    if (row->detail.len) print_buffer(&row->detail); else fputs("null", stdout);
    printf(",\"position\":%llu}\n", (unsigned long long)row->position);
    thinkthen_row_free(row);
    assert(row->answer.data == NULL && row->detail.data == NULL && row->source.data == NULL);
    assert(row->struct_size == sizeof *row);
}

static void *cancel_on_input(void *opaque) {
    assert(getchar() == 'x');
    thinkthen_call_cancel(opaque);
    return NULL;
}

static void *release_error(void *opaque) {
    thinkthen_error *error = opaque;
    assert(error->message.len != 0);
    /* Buffers are counted, so do not read them as NUL-terminated strings. */
    print_error(error);
    return NULL;
}

static void ownership(void) {
    thinkthen_client *client = NULL;
    thinkthen_call *call = NULL, *invalid_call = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    const char settings[] = "{\"backend\":\"stub\"}";
    const char request[] = "{\"question\":{\"function\":\"decide\",\"question\":\"Ready?\"}}";
    assert(thinkthen_client_open_json((const uint8_t *)settings, sizeof settings - 1, &client, &error) == THINKTHEN_OK);
    assert(thinkthen_call_open_json(client, (const uint8_t *)request, sizeof request - 1,
                                    THINKTHEN_OFFSET_CODE_POINTS, &call, &error) == THINKTHEN_OK);
    assert(thinkthen_call_open_json(client, (const uint8_t *)request, sizeof request - 1,
                                    THINKTHEN_OFFSET_CODE_POINTS, &invalid_call, &error) == THINKTHEN_OK);
    thinkthen_client_free(client);
    assert(thinkthen_call_send_text(call, NULL, 0, &error) == THINKTHEN_ERROR);
    assert(error.code == THINKTHEN_CODE_USAGE_ABI);
    thinkthen_error saved = error;
    error = (thinkthen_error){.struct_size = sizeof error};
    thinkthen_row row = {.struct_size = sizeof row};
    assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_NEEDS_INPUT);
    char text[] = "raw \"quoted\" 😀\n";
    assert(thinkthen_call_send_text(call, (const uint8_t *)text, sizeof text - 1, &error) == THINKTHEN_OK);
    memset(text, 'x', sizeof text - 1);
    thinkthen_call_end_input(call);
    thinkthen_call_end_input(call);
    assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_OK);
    thinkthen_row saved_row = row;
    row = (thinkthen_row){.struct_size = sizeof row};
    assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END);
    assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END);
    thinkthen_call_free(call);
    print_row(&saved_row);
    assert(thinkthen_call_next(invalid_call, &row, &error) == THINKTHEN_NEEDS_INPUT);
    const uint8_t invalid[] = {0xff};
    assert(thinkthen_call_send_text(invalid_call, invalid, sizeof invalid, &error) == THINKTHEN_OK);
    thinkthen_call_end_input(invalid_call);
    assert(thinkthen_call_next(invalid_call, &row, &error) == THINKTHEN_OK);
    assert(row.status == THINKTHEN_STATUS_FAILED && row.error_code == THINKTHEN_CODE_RECORD_INVALID);
    saved_row = row;
    row = (thinkthen_row){.struct_size = sizeof row};
    assert(thinkthen_call_next(invalid_call, &row, &error) == THINKTHEN_END);
    thinkthen_call_free(invalid_call);
    print_row(&saved_row);
    pthread_t reader;
    assert(pthread_create(&reader, NULL, release_error, &saved) == 0);
    assert(pthread_join(reader, NULL) == 0);
    thinkthen_error_free(&saved);
    thinkthen_error_free(NULL);
    thinkthen_row_free(NULL);
    thinkthen_call_free(NULL);
    thinkthen_client_free(NULL);
}

/* Each worker owns its calls and rows; only the immutable client is shared. */
typedef struct {
    thinkthen_client *client;
    pthread_mutex_t lock;
    pthread_cond_t start;
    int ready;
} concurrent_worker;

static void *run_concurrent(void *opaque) {
    concurrent_worker *worker = opaque;
    assert(pthread_mutex_lock(&worker->lock) == 0);
    while (!worker->ready) assert(pthread_cond_wait(&worker->start, &worker->lock) == 0);
    assert(pthread_mutex_unlock(&worker->lock) == 0);
    for (size_t iteration = 0; iteration < 8; ++iteration) {
        thinkthen_call *call = NULL;
        thinkthen_error error = {.struct_size = sizeof error};
        const char request[] = "{\"question\":{\"function\":\"decide\",\"question\":\"Ready?\",\"threshold\":0.5},\"records\":[\"shared client\"]}";
        assert(thinkthen_call_open_json(worker->client, (const uint8_t *)request, sizeof request - 1,
                                        THINKTHEN_OFFSET_UTF8, &call, &error) == THINKTHEN_OK);
        thinkthen_call_end_input(call);
        thinkthen_row row = {.struct_size = sizeof row};
        assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_OK);
        assert(row.status == THINKTHEN_STATUS_ANSWERED && row.position == 1);
        assert((row.answer.len == 4 && memcmp(row.answer.data, "true", 4) == 0) ||
               (row.answer.len == 5 && memcmp(row.answer.data, "false", 5) == 0));
        thinkthen_row_free(&row);
        assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END);
        assert(error.code == 0);
        thinkthen_call_free(call);
    }
    return NULL;
}

static void concurrent(void) {
    thinkthen_client *client = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    const char settings[] = "{\"backend\":\"stub\"}";
    assert(thinkthen_client_open_json((const uint8_t *)settings, sizeof settings - 1, &client, &error) == THINKTHEN_OK);
    pthread_t threads[4];
    concurrent_worker worker = {.client = client, .lock = PTHREAD_MUTEX_INITIALIZER,
                                .start = PTHREAD_COND_INITIALIZER, .ready = 0};
    for (size_t index = 0; index < 4; ++index)
        assert(pthread_create(&threads[index], NULL, run_concurrent, &worker) == 0);
    assert(pthread_mutex_lock(&worker.lock) == 0);
    worker.ready = 1;
    assert(pthread_cond_broadcast(&worker.start) == 0);
    assert(pthread_mutex_unlock(&worker.lock) == 0);
    for (size_t index = 0; index < 4; ++index)
        assert(pthread_join(threads[index], NULL) == 0);
    assert(pthread_cond_destroy(&worker.start) == 0);
    assert(pthread_mutex_destroy(&worker.lock) == 0);
    thinkthen_client_free(client);
    puts("{\"concurrent\":true,\"rows\":32}");
}

#include "workload.h"

int main(int argc, char **argv) {
    assert(argc == 2);
    assert(thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()));
    if (strcmp(argv[1], "workload") == 0) { workload(); return 0; }
    if (strcmp(argv[1], "concurrent") == 0) { concurrent(); return 0; }
    if (strcmp(argv[1], "ownership") == 0) { ownership(); return 0; }
    char *settings = line();
    char *request = line();
    assert(settings != NULL && request != NULL);
    thinkthen_client *client = NULL;
    thinkthen_call *call = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    int32_t status = thinkthen_client_open_json((const uint8_t *)settings, strlen(settings), &client, &error);
    free(settings);
    if (status == THINKTHEN_OK && strcmp(argv[1], "single") == 0) {
        thinkthen_question *question = NULL;
        assert(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, (const uint8_t *)"Ready?", 6,
                                      &question, &error) == THINKTHEN_OK);
        assert(thinkthen_question_set_evidence(question, (const uint8_t *)"/body", 5, &error) == THINKTHEN_OK);
        thinkthen_row row = {.struct_size = sizeof row};
        assert(thinkthen_run_one(client, question, (const uint8_t *)"{}", 2,
                                 THINKTHEN_RECORD_JSON, &row, &error) == THINKTHEN_OK);
        thinkthen_question_free(question);
        thinkthen_client_free(client);
        free(request);
        assert(error.code == 0);
        print_row(&row);
        puts("{\"end\":true}");
        return 0;
    }
    if (status == THINKTHEN_OK) {
        uint32_t offsets = THINKTHEN_OFFSET_UTF8;
        if (strcmp(argv[1], "utf16") == 0) offsets = THINKTHEN_OFFSET_UTF16;
        if (strcmp(argv[1], "points") == 0) offsets = THINKTHEN_OFFSET_CODE_POINTS;
        if (strcmp(argv[1], "invalid-offset") == 0) offsets = UINT32_MAX;
        status = thinkthen_call_open_json(client, (const uint8_t *)request, strlen(request), offsets, &call, &error);
    }
    free(request);
    thinkthen_client_free(client);
    if (status == THINKTHEN_ERROR) { print_error(&error); return 0; }
    pthread_t canceller;
    int cancel = strcmp(argv[1], "cancel") == 0;
    if (cancel || strcmp(argv[1], "free-held") == 0) thinkthen_call_end_input(call);
    if (cancel) assert(pthread_create(&canceller, NULL, cancel_on_input, call) == 0);
    if (strcmp(argv[1], "free-held") == 0) {
        assert(getchar() == 'x');
        thinkthen_call_free(call);
        puts("{\"freed\":true}");
        return 0;
    }
    size_t needs = 0;
    for (;;) {
        thinkthen_row row = {.struct_size = sizeof row};
        status = thinkthen_call_next(call, &row, &error);
        if (status == THINKTHEN_NEEDS_INPUT) {
            ++needs;
            char *record = line();
            if (record == NULL) { thinkthen_call_end_input(call); continue; }
            assert(record[0] == 'T' || record[0] == 'J');
            status = record[0] == 'T'
                ? thinkthen_call_send_text(call, (const uint8_t *)record + 1, strlen(record + 1), &error)
                : thinkthen_call_send_json(call, (const uint8_t *)record + 1, strlen(record + 1), &error);
            free(record);
            if (status == THINKTHEN_ERROR) { print_error(&error); break; }
        } else if (status == THINKTHEN_OK) {
            print_row(&row);
        } else if (status == THINKTHEN_END) {
            printf("{\"end\":true,\"needs\":%zu}\n", needs);
            break;
        } else {
            assert(row.answer.data == NULL && row.detail.data == NULL);
            print_error(&error);
            /* A terminal error is owned and yielded once; later pulls end. */
            assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END);
            thinkthen_error_free(&error);
            break;
        }
    }
    if (cancel) assert(pthread_join(canceller, NULL) == 0);
    thinkthen_call_free(call);
    return 0;
}
