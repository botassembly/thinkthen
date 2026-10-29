/* Exercise sized output ownership using the installed public C API. */
#include "typed_api_driver.h"
#include <test_registry.h>
#include <assert.h>

int32_t test_run_missing(thinkthen_row *out, thinkthen_error *error) {
    thinkthen_settings *settings = NULL;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    thinkthen_error setup = {.struct_size = sizeof setup};
    assert(thinkthen_settings_new(&settings, &setup) == THINKTHEN_OK);
    assert(thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, &setup) == THINKTHEN_OK);
    assert(thinkthen_client_open(settings, &client, &setup) == THINKTHEN_OK);
    assert(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, (const uint8_t *)"Ready?", 6, &question, &setup) == THINKTHEN_OK);
    int32_t result = thinkthen_run_one(client, question, NULL, 0, THINKTHEN_RECORD_MISSING, out, error);
    thinkthen_question_free(question);
    thinkthen_client_free(client);
    thinkthen_settings_free(settings);
    thinkthen_error_free(&setup);
    return result;
}
size_t test_registry_count(void) { return sizeof registry_codes / sizeof registry_codes[0]; }
int32_t test_registry_code(size_t index) { return registry_codes[index]; }
const char *test_registry_name(size_t index) { return registry_names[index]; }

const char *test_registry_family(size_t index) { return registry_families[index]; }
