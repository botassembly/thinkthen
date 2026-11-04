/* Generated private test declarations. Do not ship this header. */
#if !defined(THINKTHEN_TEST_BUILD) || !defined(THINKTHEN_TEST_FIXTURE)
#error private fixture declarations require an explicit test build
#endif


#ifndef THINKTHEN_FIXTURE_H
#define THINKTHEN_FIXTURE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include "thinkthen.h"

#if (defined(THINKTHEN_TEST_FIXTURE) && defined(THINKTHEN_TEST_BUILD))
/**
 * Test routes to local servers, in a debug test build. Free with thinkthen_fixture_routes_free.
 */
typedef struct thinkthen_fixture_routes thinkthen_fixture_routes;
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if (defined(THINKTHEN_TEST_FIXTURE) && defined(THINKTHEN_TEST_BUILD))
/**
 * Send backend requests to two local test servers, in a debug test build only.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
 * for a bad URL or a build without test routes, or -THINKTHEN_CODE_USAGE_ABI when the text is
 * not UTF-8. Free *out with thinkthen_fixture_routes_free.
 * Pointers and threads:
 * Any thread may call this. systemone points to systemone_len readable bytes, or to a NUL-
 * terminated string when systemone_len is THINKTHEN_NUL_TERMINATED; it is copied before
 * return. openai points to openai_len readable bytes, or to a NUL-terminated string when
 * openai_len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error
 * is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_fixture_routes_install(const char *systemone,
                                         size_t systemone_len,
                                         const char *openai,
                                         size_t openai_len,
                                         struct thinkthen_fixture_routes **out,
                                         thinkthen_error *error);
#endif

#if (defined(THINKTHEN_TEST_FIXTURE) && defined(THINKTHEN_TEST_BUILD))
/**
 * Remove the test routes. Clients already open keep theirs.
 *
 * NULL is ignored.
 * Pointers and threads:
 * Call this once, after every other use of owner has returned.
 */
THINKTHEN_API void thinkthen_fixture_routes_free(struct thinkthen_fixture_routes *owner);
#endif

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THINKTHEN_FIXTURE_H */
