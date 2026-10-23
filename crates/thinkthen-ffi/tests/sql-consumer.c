#include "thinkthen.h"
#include <assert.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <string.h>

#define BYTES(value) (const uint8_t *)(value), strlen(value)
static pthread_t driver;
static atomic_bool stopped;
static unsigned probes;
static uint8_t interrupted(void *context) {
    assert(context == &probes && pthread_equal(driver, pthread_self()));
    ++probes;
    return atomic_load(&stopped);
}
static void *stop_input(void *unused) {
    (void)unused;
    assert(getchar() == 'c');
    atomic_store(&stopped, 1);
    return NULL;
}
static int contains(thinkthen_buffer value, const char *text) {
    size_t length = strlen(text);
    for (size_t i = 0; i + length <= value.len; ++i)
        if (memcmp(value.data + i, text, length) == 0) return 1;
    return 0;
}
static void projected(thinkthen_client *client, const char *function,
                      const char *question, const char *options,
                      const char *record, const char *expected) {
    thinkthen_sql_call *call = NULL;
    thinkthen_buffer error = {0}, columns = {0};
    int opened = thinkthen_sql_open(client, BYTES(function), BYTES(question), BYTES(options), &call, &error);
    if (opened != THINKTHEN_OK) {
        fprintf(stderr, "SQL projection case %s: ", function);
        fwrite(error.data, 1, error.len, stderr);
        fputc('\n', stderr);
    }
    assert(opened == THINKTHEN_OK);
    thinkthen_row retained = {0};
    int supplied = 0, rows = 0, result;
    do {
        thinkthen_row row = {0};
        result = thinkthen_sql_poll(call, 20, NULL, NULL, &row, &error);
        if (result == THINKTHEN_NEEDS_INPUT) {
            assert(!supplied++);
            thinkthen_sql_record input = {THINKTHEN_SQL_JSON, (const uint8_t *)record, strlen(record)};
            assert(thinkthen_sql_send_chunk(call, &input, 1, &error) == THINKTHEN_OK);
            thinkthen_sql_end_input(call);
        } else if (result == THINKTHEN_OK) {
            assert(++rows == 1);
            retained = row;
        }
    } while (result != THINKTHEN_END && result != THINKTHEN_ERROR);
    assert(result == THINKTHEN_END && rows == 1);
    thinkthen_sql_free(call);
    assert(contains(retained.fields, "18446744073709551615"));
    assert(thinkthen_row_columns(&retained, &columns, &error) == THINKTHEN_OK);
    thinkthen_row_free(&retained);
    assert(contains(columns, expected));
    assert(contains(columns, "18446744073709551615"));
    thinkthen_buffer_free(&columns);
}
int main(int argc, char **argv) {
    (void)argv;
    driver = pthread_self();
    thinkthen_client *client = NULL;
    thinkthen_sql_call *call = NULL;
    thinkthen_buffer error = {0};
    const char *settings = argc > 1 ? "{\"batch\":1}" : "{\"adapter\":\"stub\"}";
    assert(thinkthen_sql_client_open(BYTES("{\"url\":null}"), &client, &error) == THINKTHEN_ERROR);
    assert(!client && contains(error, "usage.settings"));
    thinkthen_buffer_free(&error);
    assert(thinkthen_sql_client_open(BYTES(settings), &client, &error) == THINKTHEN_OK);
    if (argc == 1) {
        const char *record = "{\"id\":{\"a\\u0000b\":18446744073709551615},\"body\":\"evidence\"}";
        projected(client, "choose", "Which?", "{\"options\":[\"first\",\"second\"]}", record, "\"answer\":");
        projected(client, "annotate", "{\"type\":\"toolbox\",\"value\":{\"questions\":{\"named\":{\"decide\":\"Ready?\"}}}}", "{}", record, "named");
        projected(client, "decide", "Ready?", "{\"threshold\":\"0.01:1\"}", record, "\"answer\":\"null\"");
    }
    assert(thinkthen_sql_open(client, BYTES("decide"), BYTES(" {bad"), NULL, 0, &call, &error) == THINKTHEN_ERROR);
    assert(!call && contains(error, "usage.question"));
    thinkthen_buffer_free(&error);
    assert(thinkthen_sql_open(client, BYTES("decide"), BYTES("Ready?"), BYTES(""), &call, &error) == THINKTHEN_ERROR);
    assert(!call && contains(error, "usage.question"));
    thinkthen_buffer_free(&error);
    char question[] = "Ready?";
    assert(thinkthen_sql_open(client, BYTES("decide"), BYTES(question), NULL, 0, &call, &error) == THINKTHEN_OK);
    memset(question, 'x', strlen(question));
    thinkthen_client_free(client);
    char record[] = "evidence";
    thinkthen_sql_record input = {THINKTHEN_SQL_TEXT, (const uint8_t *)record, strlen(record)};
    assert(thinkthen_sql_send_chunk(call, &input, 1, &error) == THINKTHEN_ERROR);
    assert(contains(error, "usage.record"));
    thinkthen_buffer_free(&error);
    thinkthen_row retained = {0};
    int supplied = 0, rows = 0, result;
    pthread_t stopper;
    if (argc > 1) assert(pthread_create(&stopper, NULL, stop_input, NULL) == 0);
    do {
        thinkthen_row row = {0};
        result = thinkthen_sql_poll(call, 20, interrupted, &probes, &row, &error);
        if (result == THINKTHEN_NEEDS_INPUT) {
            assert(!supplied);
            assert(thinkthen_sql_send_chunk(call, NULL, 0, &error) == THINKTHEN_OK);
            input.kind = 99;
            assert(thinkthen_sql_send_chunk(call, &input, 1, &error) == THINKTHEN_ERROR);
            assert(contains(error, "usage.record"));
            thinkthen_buffer_free(&error);
            input.kind = THINKTHEN_SQL_TEXT;
            assert(thinkthen_sql_send_chunk(call, &input, 1, &error) == THINKTHEN_OK);
            memset(record, 'z', strlen(record));
            thinkthen_sql_end_input(call);
            supplied = 1;
        } else if (result == THINKTHEN_OK) {
            assert(++rows == 1 && row.position == 1);
            retained = row;
        }
    } while (result != THINKTHEN_END && result != THINKTHEN_ERROR);
    assert(probes > 0);
    thinkthen_sql_free(call);
    if (argc > 1) {
        assert(pthread_join(stopper, NULL) == 0);
        assert(result == THINKTHEN_ERROR && contains(error, "local.cancelled"));
        if (rows) assert(contains(retained.fields, "local.cancelled"));
    } else {
        assert(result == THINKTHEN_END && rows == 1);
        assert(contains(retained.fields, "answered") && contains(retained.detail, "question_identity"));
    }
    thinkthen_buffer_free(&error);
    thinkthen_row_free(&retained);
    return 0;
}
