/* Generated private test declarations. Do not ship this header. */
#if !defined(THINKTHEN_TEST_BUILD) || !defined(THINKTHEN_TEST_FIXTURE)
#error private fixture declarations require an explicit test build
#endif


#ifndef THINKTHEN_FIXTURE_H
#define THINKTHEN_FIXTURE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>
#include "thinkthen.h"

#if (defined(THINKTHEN_TEST_FIXTURE) && defined(THINKTHEN_TEST_BUILD))
/**
 * Process-owned test routes. Existing clients retain their admitted snapshots.
 */
typedef struct thinkthen_fixture_routes thinkthen_fixture_routes;
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if (defined(THINKTHEN_TEST_FIXTURE) && defined(THINKTHEN_TEST_BUILD))
/**
 * Install both loopback routes only in an explicit debug test artifact.
 * # Safety
 * URL bytes are readable; out/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_fixture_routes_install(const uint8_t *systemone,
                                         size_t systemone_len,
                                         const uint8_t *openai,
                                         size_t openai_len,
                                         struct thinkthen_fixture_routes **out,
                                         thinkthen_error *error);
#endif

#if (defined(THINKTHEN_TEST_FIXTURE) && defined(THINKTHEN_TEST_BUILD))
/**
 * Clear the owned test routes; opened clients keep their copied destinations.
 * # Safety
 * NULL or the unique live guard owner after all readers finish.
 */
THINKTHEN_API void thinkthen_fixture_routes_free(struct thinkthen_fixture_routes *owner);
#endif

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THINKTHEN_FIXTURE_H */
