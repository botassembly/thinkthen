#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *filter =
    "{\"filter\": \"Is this a complaint?\", "
    "\"records\": ["
    "\"Arrived a day early. Thank you!\", "
    "\"The zipper broke the first time I used it.\", "
    "\"Does this come in blue?\", "
    "\"The strap snapped on day two.\"]}";
const char *expected =
    "[\"The zipper broke the first time I used it.\","
    "\"The strap snapped on day two.\"]";
char *kept = thinkthen_call(tt, filter);
assert(kept && strcmp(kept, expected) == 0);
thinkthen_free_string(kept);
