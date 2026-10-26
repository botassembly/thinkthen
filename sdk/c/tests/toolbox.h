#include <stdio.h>
/* Feature: saved C toolbox outputs and selected questions retain independent ownership. */
static void saved_toolbox(const char *json) {
    thinkthen_error error = {.struct_size = sizeof error};
    thinkthen_toolbox *tools = NULL;
    assert(thinkthen_toolbox_from_json((const uint8_t *)json, strlen(json), &tools, &error) == THINKTHEN_OK);
    thinkthen_question *topic = NULL;
    const uint8_t name[] = "topic";
    assert(thinkthen_toolbox_select_as(tools, name, sizeof name - 1, THINKTHEN_FUNCTION_CHOOSE, &topic, &error) == THINKTHEN_OK);
    thinkthen_buffer saved = {0};
    assert(thinkthen_toolbox_to_json(tools, &saved, &error) == THINKTHEN_OK);
    thinkthen_toolbox_free(tools);
    thinkthen_settings *settings = NULL;
    thinkthen_client *client = NULL;
    assert(thinkthen_settings_new(&settings, &error) == THINKTHEN_OK);
    assert(thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, &error) == THINKTHEN_OK);
    assert(thinkthen_client_open(settings, &client, &error) == THINKTHEN_OK);
    thinkthen_settings_free(settings);
    const uint8_t record[] = "{\"body\":\"Bill\"}";
    thinkthen_row first = {.struct_size = sizeof first}, second = {.struct_size = sizeof second};
    assert(thinkthen_run_one(client, topic, record, sizeof record - 1, THINKTHEN_RECORD_JSON, &first, &error) == THINKTHEN_OK);
    thinkthen_question_free(topic);
    tools = NULL;
    topic = NULL;
    assert(thinkthen_toolbox_from_json(saved.data, saved.len, &tools, &error) == THINKTHEN_OK);
    assert(thinkthen_toolbox_select(tools, name, sizeof name - 1, &topic, &error) == THINKTHEN_OK);
    assert(thinkthen_run_one(client, topic, record, sizeof record - 1, THINKTHEN_RECORD_JSON, &second, &error) == THINKTHEN_OK);
    assert(first.probability == second.probability);
    assert(first.detail.len == second.detail.len && memcmp(first.detail.data, second.detail.data, first.detail.len) == 0);
    assert(first.status == second.status && first.answer.len == second.answer.len);
    assert(memcmp(first.answer.data, second.answer.data, first.answer.len) == 0);
    assert(first.source.len == second.source.len && memcmp(first.source.data, second.source.data, first.source.len) == 0);
    assert(fwrite(saved.data, 1, saved.len, stdout) == saved.len);
    thinkthen_buffer_free(&saved);
    thinkthen_row_free(&first);
    thinkthen_row_free(&second);
    thinkthen_question_free(topic);
    thinkthen_toolbox_free(tools);
    thinkthen_client_free(client);
}
