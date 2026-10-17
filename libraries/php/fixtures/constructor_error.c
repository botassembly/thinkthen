#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include "ffi_generated.h"

static const thinkthen_complete_immediate_facts_v1 facts = {
    .records = 7, .requests_sent = 3,
};
static const thinkthen_error_view_v1 failure = {
    .code = 2, .retryable = 1, .message = { .data = "synthetic refusal", .len = 17 }, .facts = &facts,
};
struct thinkthen_engine *thinkthen_engine_new_with(const char *settings) { (void)settings; return NULL; }
int thinkthen_error_code(const struct thinkthen_engine *engine) { (void)engine; return failure.code; }
const thinkthen_error_view_v1 *thinkthen_error_view(const struct thinkthen_engine *engine) { (void)engine; return &failure; }
