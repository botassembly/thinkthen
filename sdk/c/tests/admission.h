/* Public serialized C boundaries retain the library's diagnostic. */
static void admission_error(const thinkthen_error *error) {
    puts(thinkthen_code_name(error->code));
    assert(fwrite(error->message.data, 1, error->message.len, stdout) == error->message.len);
    putchar('\n');
}

static void admission(const char *boundary, const char *json) {
    const uint8_t *data = (const uint8_t *)json;
    const size_t size = strlen(json);
    thinkthen_error error = {.struct_size = sizeof error};
    int32_t status = THINKTHEN_OK;
    if (strcmp(boundary, "question") == 0) {
        thinkthen_question *question = NULL;
        status = thinkthen_question_from_json(data, size, &question, &error);
        assert(question == NULL);
        thinkthen_question_free(question);
    } else if (strcmp(boundary, "settings") == 0) {
        thinkthen_settings *settings = NULL;
        status = thinkthen_settings_from_json(data, size, &settings, &error);
        assert(settings == NULL);
        thinkthen_settings_free(settings);
    } else if (strcmp(boundary, "toolbox") == 0) {
        thinkthen_toolbox *toolbox = NULL;
        status = thinkthen_toolbox_from_json(data, size, &toolbox, &error);
        assert(toolbox == NULL);
        thinkthen_toolbox_free(toolbox);
    } else {
        assert(strcmp(boundary, "request") == 0);
        thinkthen_settings *settings = NULL;
        thinkthen_client *client = NULL;
        thinkthen_call *call = NULL;
        assert(thinkthen_settings_new(&settings, &error) == THINKTHEN_OK);
        assert(thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, &error) == THINKTHEN_OK);
        assert(thinkthen_client_open(settings, &client, &error) == THINKTHEN_OK);
        status = thinkthen_call_open_json(client, data, size, THINKTHEN_OFFSET_CODE_POINTS, &call, &error);
        assert(call == NULL);
        thinkthen_call_free(call);
        thinkthen_client_free(client);
        thinkthen_settings_free(settings);
    }
    assert(status == THINKTHEN_ERROR);
    admission_error(&error);
    thinkthen_error_free(&error);
}
