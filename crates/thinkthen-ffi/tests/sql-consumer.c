#include "thinkthen_sql.h"
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
static void rejected(int result, int32_t code, thinkthen_error *error) {
    assert(result == THINKTHEN_ERROR && error->code == code);
    assert(error->message.len != 0);
    thinkthen_error_free(error);
}

/* The obsolete column-JSON export is replaced by a typed snapshot. This checks
 * SQL output ownership, not the canonical suite's per-function semantics. */
static void projected(thinkthen_client *client, const char *request, int annotation) {
    thinkthen_sql_call *call = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    assert(thinkthen_sql_call_open_json(client, BYTES(request), &call, &error) == THINKTHEN_OK);
    thinkthen_row retained = {.struct_size = sizeof retained};
    int supplied = 0, rows = 0, result;
    do {
        thinkthen_row row = {.struct_size = sizeof row};
        result = thinkthen_sql_call_poll(call, 20, NULL, NULL, &row, &error);
        if (result == THINKTHEN_NEEDS_INPUT) {
            assert(!supplied++);
            const char record[] = "{\"id\":18446744073709551615,\"body\":\"evidence\"}";
            thinkthen_sql_input input = {THINKTHEN_RECORD_JSON, (const uint8_t *)record, sizeof record - 1};
            assert(thinkthen_sql_call_send_chunk(call, &input, 1, &error) == THINKTHEN_OK);
            thinkthen_sql_call_end_input(call);
        } else if (result == THINKTHEN_OK) {
            assert(++rows == 1);
            retained = row;
        }
    } while (result != THINKTHEN_END && result != THINKTHEN_ERROR);
    assert(result == THINKTHEN_END && rows == 1);
    thinkthen_sql_call_free(call);
    thinkthen_value *value = NULL;
    assert(thinkthen_row_values(&retained, &value, &error) == THINKTHEN_OK);
    thinkthen_row_free(&retained);
    thinkthen_id id = {0};
    assert(thinkthen_value_id(value, &id, &error) == THINKTHEN_OK);
    assert(id.kind == THINKTHEN_ID_UNSIGNED && id.unsigned_value == UINT64_MAX);
    if (annotation) {
        size_t count = 0;
        assert(thinkthen_value_annotation_count(value, &count, &error) == THINKTHEN_OK && count == 1);
        thinkthen_text name = {0};
        thinkthen_value *child = NULL;
        assert(thinkthen_value_annotation_at(value, 0, &name, &child, &error) == THINKTHEN_OK);
        assert(name.len == 5 && memcmp(name.data, "named", 5) == 0);
        thinkthen_value_free(value);
        uint32_t status = 0;
        assert(thinkthen_value_status(child, &status, &error) == THINKTHEN_OK);
        assert(status == THINKTHEN_STATUS_ANSWERED);
        thinkthen_value_free(child);
    } else {
        thinkthen_text choice = {0};
        assert(thinkthen_value_choice(value, &choice, &error) == THINKTHEN_OK && choice.len != 0);
        thinkthen_value_free(value);
    }
    thinkthen_error_free(&error);
}

int main(int argc, char **argv) {
    (void)argv;
    driver = pthread_self();
    assert(thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()));
    thinkthen_client *client = NULL;
    thinkthen_sql_call *call = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    const char *settings = argc > 1 ? "{\"batch\":1}" : "{\"backend\":\"stub\"}";
    rejected(thinkthen_sql_client_open(BYTES("{\"url\":null}"), &client, &error), THINKTHEN_CODE_USAGE_SETTINGS, &error);
    assert(client == NULL);
    rejected(thinkthen_sql_client_open(BYTES("{\"api_key\":null}"), &client, &error), THINKTHEN_CODE_USAGE_SETTINGS, &error);
    assert(client == NULL);
    assert(thinkthen_sql_client_open(BYTES(settings), &client, &error) == THINKTHEN_OK);
    if (argc == 1) {
        projected(client, "{\"question\":{\"function\":\"choose\",\"question\":\"Which?\",\"choices\":[\"first\",\"second\"]}}", 0);
        projected(client, "{\"question\":{\"function\":\"annotate\",\"questions\":{\"named\":{\"function\":\"decide\",\"question\":\"Ready?\"}}}}", 1);
    }
    rejected(thinkthen_sql_call_open_json(client, BYTES(" {bad"), &call, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    assert(call == NULL);
    rejected(thinkthen_sql_call_open_json(client, BYTES(""), &call, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    assert(call == NULL);
    char wording[] = "Ready?";
    thinkthen_question *question = NULL;
    assert(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, BYTES(wording), &question, &error) == THINKTHEN_OK);
    memset(wording, 'x', strlen(wording));
    assert(thinkthen_sql_call_open_question(client, question, NULL, &call, &error) == THINKTHEN_OK);
    thinkthen_question_free(question);
    thinkthen_client_free(client);
    char record[] = "evidence";
    thinkthen_sql_input input = {THINKTHEN_RECORD_TEXT, (const uint8_t *)record, strlen(record)};
    rejected(thinkthen_sql_call_send_chunk(call, &input, 1, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    thinkthen_row retained = {.struct_size = sizeof retained};
    int supplied = 0, rows = 0, result;
    pthread_t stopper;
    if (argc > 1) assert(pthread_create(&stopper, NULL, stop_input, NULL) == 0);
    do {
        thinkthen_row row = {.struct_size = sizeof row};
        result = thinkthen_sql_call_poll(call, 20, interrupted, &probes, &row, &error);
        if (result == THINKTHEN_NEEDS_INPUT) {
            assert(!supplied);
            assert(thinkthen_sql_call_send_chunk(call, NULL, 0, &error) == THINKTHEN_OK);
            input.kind = UINT32_MAX;
            rejected(thinkthen_sql_call_send_chunk(call, &input, 1, &error), THINKTHEN_CODE_USAGE_ABI, &error);
            input.kind = THINKTHEN_RECORD_TEXT;
            assert(thinkthen_sql_call_send_chunk(call, &input, 1, &error) == THINKTHEN_OK);
            memset(record, 'z', strlen(record));
            thinkthen_sql_call_end_input(call);
            supplied = 1;
        } else if (result == THINKTHEN_OK) {
            assert(++rows == 1 && row.position == 1);
            retained = row;
        }
    } while (result != THINKTHEN_END && result != THINKTHEN_ERROR);
    assert(probes > 0);
    thinkthen_sql_call_free(call);
    if (argc > 1) {
        assert(pthread_join(stopper, NULL) == 0);
        assert(result == THINKTHEN_ERROR && error.code == THINKTHEN_CODE_LOCAL_CANCELLED);
        assert(rows == 0);
    } else {
        assert(result == THINKTHEN_END && rows == 1);
        assert(retained.status == THINKTHEN_STATUS_ANSWERED && retained.source.len != 0);
    }
    thinkthen_error_free(&error);
    thinkthen_row_free(&retained);
    return 0;
}
