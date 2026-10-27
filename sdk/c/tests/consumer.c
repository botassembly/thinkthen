#include <thinkthen.h>
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <string.h>

/* The installed C package needs neither the removed wrapper nor a JSON library. */
#include "toolbox.h"
#include "admission.h"

int main(int argc, char **argv) {
    if (argc == 2 && strcmp(argv[1], "admission-surface") == 0) { puts("c"); return 0; }
    if (argc == 3 && strncmp(argv[1], "admission-", 10) == 0) {
        admission(argv[1] + 10, argv[2]);
        return 0;
    }
    if (argc == 3 && strcmp(argv[1], "toolbox-invalid") == 0) {
        admission("toolbox", argv[2]);
        return 0;
    }
    if (argc == 3 && strcmp(argv[1], "toolbox") == 0) {
        saved_toolbox(argv[2]);
        return 0;
    }
    assert(thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()));
    thinkthen_error error = {.struct_size = sizeof error};
    thinkthen_settings *settings = NULL;
    thinkthen_client *client = NULL;
    thinkthen_question *question = NULL;
    assert(thinkthen_settings_new(&settings, &error) == THINKTHEN_OK);
    assert(thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, &error) == THINKTHEN_OK);
    assert(thinkthen_client_open(settings, &client, &error) == THINKTHEN_OK);
    thinkthen_settings_free(settings);
    const uint8_t text[] = "Ready?";
    assert(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, text, sizeof text - 1,
                                  &question, &error) == THINKTHEN_OK);
    thinkthen_row row = {.struct_size = sizeof row};
    assert(thinkthen_run_one(client, question, NULL, 0, THINKTHEN_RECORD_MISSING,
                              &row, &error) == THINKTHEN_OK);
    thinkthen_row_free(&row);
    const uint8_t pointer[] = "/text";
    const uint8_t array[] = "[{\"text\":\"first\"},{\"text\":\"second\"}]";
    assert(thinkthen_question_set_evidence(question, pointer, sizeof pointer - 1, &error) == THINKTHEN_OK);
    assert(thinkthen_run_one(client, question, array, sizeof array - 1, THINKTHEN_RECORD_JSON,
                              &row, &error) == THINKTHEN_OK);
    thinkthen_value *value = NULL;
    assert(thinkthen_row_values(&row, &value, &error) == THINKTHEN_OK);
    thinkthen_error failure = {.struct_size = sizeof failure};
    assert(thinkthen_value_error(value, &failure, &error) == THINKTHEN_OK);
    const char expected[] = "the record is an array of 2 values and holds nothing at /text. Use the stream form of the call to judge each value";
    assert(failure.message.len == sizeof expected - 1);
    assert(memcmp(failure.message.data, expected, sizeof expected - 1) == 0);
    thinkthen_error_free(&failure);
    thinkthen_value_free(value);
    thinkthen_question_free(question);
    thinkthen_client_free(client);
    assert(row.status == THINKTHEN_STATUS_FAILED && row.position == 1);
    assert(strcmp(thinkthen_code_name(row.error_code), "record.missing") == 0);
    assert(isnan(row.probability));
    thinkthen_row_free(&row);
    thinkthen_row_free(&row);
    thinkthen_error_free(&error);
    assert(row.struct_size == sizeof row && error.struct_size == sizeof error);
    return 0;
}
