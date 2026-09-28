#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *tag =
    "{\"tag\": \"Which labels fit this message?\", "
    "\"labels\": [\"praise\", \"bug\", \"billing\"], "
    "\"evidence\": \"Love the new dashboard, but export "
    "crashes the app, and I was charged twice.\"}";
const char *expected = "[\"praise\",\"bug\",\"billing\"]";
char *tag_call = thinkthen_call(tt, tag);
assert(tag_call);
assert(strncmp(tag_call, "{\"value\":", 9) == 0);
const char *fitting_labels = tag_call + 9;
size_t labels_len = strlen(expected);
assert(strncmp(fitting_labels, expected, labels_len) == 0);
assert(strncmp(
    fitting_labels + labels_len, ",\"facts\":{", 10
) == 0);
thinkthen_free_string(tag_call);
