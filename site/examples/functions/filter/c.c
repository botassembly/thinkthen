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
char *complaints_call = thinkthen_call(tt, filter);
assert(complaints_call);
assert(strncmp(complaints_call, "{\"value\":", 9) == 0);
const char *complaints = complaints_call + 9;
size_t complaints_len = strlen(expected);
assert(strncmp(complaints, expected, complaints_len) == 0);
assert(strncmp(
    complaints + complaints_len, ",\"facts\":{", 10
) == 0);
thinkthen_free_string(complaints_call);
