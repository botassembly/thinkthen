/* Private test artifacts route both backends without a product URL setting. */
#define THINKTHEN_TEST_BUILD
#define THINKTHEN_TEST_FIXTURE
#include "thinkthen_fixture.h"
#include <assert.h>
#include <string.h>
static void ok(int32_t status, thinkthen_error *error) { assert(status == THINKTHEN_OK); assert(error->code == 0); }
int main(int argc, char **argv) {
    assert(argc == 3); thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_fixture_routes *owner = NULL;
    const char *remote = "https://remote.invalid/v1";
    assert(thinkthen_fixture_routes_install((const uint8_t *)remote, strlen(remote), (const uint8_t *)argv[2], strlen(argv[2]), &owner, &error) == THINKTHEN_ERROR);
    assert(error.code == THINKTHEN_CODE_USAGE_URL && owner == NULL); thinkthen_error_free(&error);
    ok(thinkthen_fixture_routes_install((const uint8_t *)argv[1], strlen(argv[1]), (const uint8_t *)argv[2], strlen(argv[2]), &owner, &error), &error);
    thinkthen_fixture_routes *duplicate = NULL;
    assert(thinkthen_fixture_routes_install((const uint8_t *)argv[1], strlen(argv[1]), (const uint8_t *)argv[2], strlen(argv[2]), &duplicate, &error) == THINKTHEN_ERROR);
    assert(error.code == THINKTHEN_CODE_USAGE_SETTINGS && duplicate == NULL); thinkthen_error_free(&error);
    uint32_t backends[] = { THINKTHEN_BACKEND_SYSTEMONE, THINKTHEN_BACKEND_OPENAI };
    for (size_t i = 0; i < sizeof(backends) / sizeof(backends[0]); i++) {
        thinkthen_settings *settings = NULL; ok(thinkthen_settings_new(&settings, &error), &error);
        ok(thinkthen_settings_set_backend(settings, backends[i], &error), &error);
        const char *model = "fixture-model"; ok(thinkthen_settings_set_model(settings, (const uint8_t *)model, strlen(model), &error), &error);
        thinkthen_client *client = NULL; ok(thinkthen_client_open(settings, &client, &error), &error); thinkthen_settings_free(settings);
        thinkthen_question *question = NULL; ok(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, (const uint8_t *)"Ready?", 6, &question, &error), &error);
        thinkthen_row row = {0}; row.struct_size = sizeof(row); ok(thinkthen_run_one(client, question, (const uint8_t *)"evidence", 8, THINKTHEN_RECORD_TEXT, &row, &error), &error);
        assert(row.status == THINKTHEN_STATUS_ANSWERED && row.answer_bool);
        assert(row.probability == 0.8123456789012345);
        thinkthen_value *value = NULL; ok(thinkthen_row_values(&row, &value, &error), &error);
        thinkthen_source source = {0}; ok(thinkthen_value_source(value, &source, &error), &error);
        assert(source.backend == backends[i]); assert(source.model.len == strlen(model)); assert(memcmp(source.model.data, model, source.model.len) == 0);
        thinkthen_value_free(value); thinkthen_row_free(&row); thinkthen_question_free(question); thinkthen_client_free(client);
    }
    thinkthen_fixture_routes_free(owner);
    const char *invalid = "{\"backend\":\"openai\",\"url\":\"http://remote.invalid/v1\"}";
    thinkthen_client *client = NULL;
    assert(thinkthen_client_open_json((const uint8_t *)invalid, strlen(invalid), &client, &error) == THINKTHEN_ERROR);
    assert(error.code == THINKTHEN_CODE_USAGE_URL && client == NULL); thinkthen_error_free(&error); return 0;
}
