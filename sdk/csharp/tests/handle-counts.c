/* macOS test shim: forward the real ABI and count outstanding native owners. */
#include "thinkthen.h"
#include <dlfcn.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>

static _Atomic int owners;
static _Atomic int questions;
static void *engine;
__attribute__((constructor)) static void load_engine(void) {
    engine = dlopen(getenv("THINKTHEN_TEST_ENGINE"), RTLD_NOW | RTLD_LOCAL);
    if (!engine) { fputs(dlerror(), stderr); abort(); }
}
static void *symbol(const char *name) {
    void *result = dlsym(engine, name);
    if (!result) { fputs(dlerror(), stderr); abort(); }
    return result;
}
int thinkthen_test_live_handles(void) { return atomic_load(&owners); }
int thinkthen_test_questions_created(void) { return atomic_load(&questions); }
#define OPEN(name, args, values, is_question) \
    int32_t name args { \
        int32_t (*real) args = symbol(#name); \
        int32_t result = real values; \
        if (*out) { atomic_fetch_add(&owners, 1); if (is_question) atomic_fetch_add(&questions, 1); } \
        return result; \
    }
#define FREE(name, type) \
    void name(struct type *value) { \
        void (*real)(struct type *) = symbol(#name); \
        real(value); if (value) atomic_fetch_sub(&owners, 1); \
    }
#define JSON_OPEN(name, type, question) \
    OPEN(name, (const uint8_t *data, size_t len, struct type **out, struct thinkthen_error *error), \
         (data, len, out, error), question)
JSON_OPEN(thinkthen_question_from_json, thinkthen_question, 1)
JSON_OPEN(thinkthen_toolbox_from_json, thinkthen_toolbox, 0)
JSON_OPEN(thinkthen_toolbox_from_definitions_json, thinkthen_toolbox, 0)
JSON_OPEN(thinkthen_client_open_json, thinkthen_client, 0)
OPEN(thinkthen_question_new,
     (uint32_t function, const uint8_t *data, size_t len, struct thinkthen_question **out, struct thinkthen_error *error),
     (function, data, len, out, error), 1)
OPEN(thinkthen_toolbox_select_as,
     (const struct thinkthen_toolbox *toolbox, const uint8_t *data, size_t len, uint32_t function,
      struct thinkthen_question **out, struct thinkthen_error *error),
     (toolbox, data, len, function, out, error), 1)
OPEN(thinkthen_call_open_question,
     (const struct thinkthen_client *client, const struct thinkthen_question *question,
      const struct thinkthen_text *name, uint32_t offsets, struct thinkthen_call **out, struct thinkthen_error *error),
     (client, question, name, offsets, out, error), 0)
FREE(thinkthen_question_free, thinkthen_question)
FREE(thinkthen_toolbox_free, thinkthen_toolbox)
FREE(thinkthen_client_free, thinkthen_client)
FREE(thinkthen_call_free, thinkthen_call)
