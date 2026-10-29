#ifndef THINKTHEN_TYPED_API_TEST_DRIVER_H
#define THINKTHEN_TYPED_API_TEST_DRIVER_H

#include <thinkthen.h>

/* Exercise real typed builders and run_one through the installed declaration.
 * The helper destroys settings, question and client before returning, leaving
 * the actual engine-owned output alive for the consumer.
 */
int32_t test_run_missing(thinkthen_row *out, thinkthen_error *error);

/* This returns the expected integer/name pairs from the regenerated shared
 * contract registry fixture, never from thinkthen_code_name or a copied list.
 * The fixture reader retains these strings until process exit.
 */
size_t test_registry_count(void);
int32_t test_registry_code(size_t index);
const char *test_registry_name(size_t index);
const char *test_registry_family(size_t index);

#endif
