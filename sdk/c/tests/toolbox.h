#include <stdio.h>
/* Feature: saved C toolbox outputs and selected questions retain independent ownership. */
static void saved_toolbox(const char *json) {
    thinkthen_error error = {.struct_size = sizeof error};
    thinkthen_toolbox *tools = NULL;
    assert(thinkthen_toolbox_from_json((const char *)json, strlen(json), &tools, &error) == THINKTHEN_OK);
    thinkthen_question *topic = NULL;
    const char name[] = "topic";
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
    const char record[] = "{\"body\":\"Bill\"}";
    thinkthen_row first = {.struct_size = sizeof first}, second = {.struct_size = sizeof second};
    assert(thinkthen_run_one(client, topic, record, sizeof record - 1, THINKTHEN_RECORD_JSON, &first, &error) == THINKTHEN_OK);
    thinkthen_question_free(topic);
    tools = NULL;
    topic = NULL;
    assert(thinkthen_toolbox_from_json((const char *)saved.data, saved.len, &tools, &error) == THINKTHEN_OK);
    assert(thinkthen_toolbox_select(tools, name, sizeof name - 1, &topic, &error) == THINKTHEN_OK);
    assert(thinkthen_run_one(client, topic, record, sizeof record - 1, THINKTHEN_RECORD_JSON, &second, &error) == THINKTHEN_OK);
    assert(first.probability == second.probability);
    assert(first.status==second.status);
    thinkthen_buffer first_json={0},second_json={0};
    assert(thinkthen_row_to_json(&first,&first_json,&error)==THINKTHEN_OK);
    assert(thinkthen_row_to_json(&second,&second_json,&error)==THINKTHEN_OK);
    assert(first_json.len==second_json.len && !memcmp(first_json.data,second_json.data,first_json.len));
    thinkthen_buffer_free(&first_json);thinkthen_buffer_free(&second_json);
    assert(fwrite(saved.data, 1, saved.len, stdout) == saved.len);
    thinkthen_buffer_free(&saved);
    thinkthen_row_free(&first);
    thinkthen_row_free(&second);
    thinkthen_question_free(topic);
    thinkthen_toolbox_free(tools);
    thinkthen_client_free(client);
}
