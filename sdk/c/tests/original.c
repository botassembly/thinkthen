#include <thinkthen.h>
#include <assert.h>
#include <string.h>

/* Feature: each filter output owns original input bytes and an independent snapshot. */
int main(void) {
    thinkthen_error error = {.struct_size = sizeof error};
    thinkthen_settings *settings = NULL;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_call *call = NULL;
    assert(thinkthen_settings_new(&settings, &error) == THINKTHEN_OK);
    assert(thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, &error) == THINKTHEN_OK);
    assert(thinkthen_client_open(settings, &client, &error) == THINKTHEN_OK);
    thinkthen_settings_free(settings);
    assert(thinkthen_question_new(THINKTHEN_FUNCTION_FILTER, (const uint8_t *)"Ready?", 6, &question, &error) == THINKTHEN_OK);
    assert(thinkthen_question_set_band(question, 0.001, 1, &error) == THINKTHEN_OK);
    const char json[] = "{\"id\":{\"invalid\":true},\"body\":\"original\"}";
    const uint8_t invalid_utf8[] = {0xff};
    thinkthen_record_input records[] = {
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_TEXT, .data = (const uint8_t *)"evidence", .len = 8},
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_MISSING},
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_JSON, .data = (const uint8_t *)json, .len = sizeof json - 1},
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_INVALID},
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_TEXT, .data = invalid_utf8, .len = 1},
    };
    assert(thinkthen_call_open_records_question(client, question, NULL, records, 5, sizeof records[0], THINKTHEN_OFFSET_UTF8, &call, &error) == THINKTHEN_OK);
    thinkthen_question_free(question);
    thinkthen_client_free(client);
    struct output { thinkthen_row row; unsigned char suffix[16]; } outputs[5] = {0};
    thinkthen_value *snapshot = NULL;
    for (size_t i = 0; i < 5; ++i) {
        outputs[i].row.struct_size = sizeof outputs[i];
        memset(outputs[i].suffix, 0xa5, sizeof outputs[i].suffix);
        assert(thinkthen_call_next(call, &outputs[i].row, &error) == THINKTHEN_OK);
        assert(outputs[i].row.position == i + 1);
        if (i == 0) assert(outputs[i].row.status == THINKTHEN_STATUS_UNSURE);
        else assert(outputs[i].row.status == THINKTHEN_STATUS_FAILED);
    }
    assert(thinkthen_row_values(&outputs[2].row, &snapshot, &error) == THINKTHEN_OK);
    thinkthen_row terminal = {.struct_size = sizeof terminal};
    assert(thinkthen_call_next(call, &terminal, &error) == THINKTHEN_END);
    thinkthen_call_free(call);
    for (size_t i = 0; i < 5; ++i) {
        thinkthen_record_view original = {0};
        assert(thinkthen_row_original(&outputs[i].row, &original, &error) == THINKTHEN_OK);
        assert(original.kind == records[i].kind && original.len == records[i].len);
        if (original.len) assert(memcmp(original.data, records[i].data, original.len) == 0);
        for (size_t j = 0; j < sizeof outputs[i].suffix; ++j) assert(outputs[i].suffix[j] == 0xa5);
        const uint32_t size = outputs[i].row.struct_size;
        outputs[i].row.struct_size = sizeof(uint32_t);
        assert(thinkthen_row_original(&outputs[i].row, &original, &error) == THINKTHEN_ERROR);
        assert(error.code == THINKTHEN_CODE_USAGE_ABI);
        thinkthen_error_free(&error);
        outputs[i].row.struct_size = size;
        thinkthen_row_free(&outputs[i].row);
        thinkthen_row_free(&outputs[i].row);
        assert(outputs[i].row.struct_size == size);
        for (size_t j = 0; j < sizeof outputs[i].suffix; ++j) assert(outputs[i].suffix[j] == 0xa5);
    }
    thinkthen_record_view original = {0};
    assert(thinkthen_value_original(snapshot, &original, &error) == THINKTHEN_OK);
    assert(original.kind == THINKTHEN_RECORD_JSON && original.len == sizeof json - 1);
    assert(memcmp(original.data, json, original.len) == 0);
    thinkthen_value_free(snapshot);
    thinkthen_row_free(&terminal);
    thinkthen_error_free(&error);
    return 0;
}
