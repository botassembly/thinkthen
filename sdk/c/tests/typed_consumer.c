#include "typed_api_driver.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

/* Feature: C consumers retain sized, owned outputs after client destruction.
 * Shared tests own general engine failures; this checks the installed C
 * declaration/layout and the caller's actual output allocation.
 */
static void owned_missing_row(void) {
    struct {
        thinkthen_row row;
        unsigned char suffix[32];
    } output = {0};
    thinkthen_error error = {0};
    memset(output.suffix, 0xa5, sizeof output.suffix);
    output.row.struct_size = sizeof output;
    error.struct_size = sizeof error;
    assert(test_run_missing(&output.row, &error) == THINKTHEN_OK);
    assert(output.row.position == 1);
    assert(strcmp(thinkthen_code_name(output.row.error_code), "record.missing") == 0);
    assert(isnan(output.row.probability));
    assert(output.row.error.len != 0);
    for (size_t i = 0; i < sizeof output.suffix; ++i) assert(output.suffix[i] == 0xa5);
    thinkthen_row_free(&output.row);
    assert(output.row.struct_size == sizeof output);
    thinkthen_row_free(&output.row);
    thinkthen_error_free(&error);
    thinkthen_error_free(&error);
    assert(error.struct_size == sizeof error);
    for (size_t i = 0; i < sizeof output.suffix; ++i) assert(output.suffix[i] == 0xa5);
}

/* Feature: an undersized output cannot overwrite its caller's memory. */
static void short_output(void) {
    thinkthen_row output = {0};
    thinkthen_error error = {0};
    output.struct_size = sizeof output.struct_size;
    error.struct_size = sizeof error;
    unsigned char expected[sizeof output];
    memcpy(expected, &output, sizeof output);
    assert(test_run_missing(&output, &error) == THINKTHEN_ERROR);
    assert(memcmp(expected, &output, sizeof output) == 0);
    assert(strcmp(thinkthen_code_name(error.code), "usage.abi") == 0);
    thinkthen_error_free(&error);

    error.struct_size = sizeof error.struct_size;
    unsigned char expected_error[sizeof error];
    memcpy(expected_error, &error, sizeof error);
    assert(test_run_missing(&output, &error) == THINKTHEN_ERROR);
    assert(memcmp(expected, &output, sizeof output) == 0);
    assert(memcmp(expected_error, &error, sizeof error) == 0);
}

/* Feature: installed C codes preserve the shared contract's names and values. */
static void registry_values(void) {
    const size_t count = test_registry_count();
    assert(count != 0);
    for (size_t i = 0; i < count; ++i) {
        const char *name = thinkthen_code_name(test_registry_code(i));
        assert(name != NULL);
        assert(strcmp(name, test_registry_name(i)) == 0);
    }
}

int main(void) {
    _Static_assert(offsetof(thinkthen_row, struct_size) == 0, "row size prefix");
    _Static_assert(offsetof(thinkthen_error, struct_size) == 0, "error size prefix");
    owned_missing_row();
    short_output();
    registry_values();
    return 0;
}
