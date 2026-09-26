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
char *fitting_labels = thinkthen_call(tt, tag);
assert(
    fitting_labels
    && strcmp(fitting_labels, expected) == 0
);
thinkthen_free_string(fitting_labels);
