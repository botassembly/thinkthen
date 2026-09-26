#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *find =
    "{\"find\": \"Which line gives the refund deadline?\", "
    "\"units\": ["
    "\"Returns need the original receipt.\", "
    "\"Refunds are issued within 30 days of purchase.\", "
    "\"Shipping is free on orders over $50.\", "
    "\"Gift cards cannot be exchanged for cash.\"]}";
const char *expected =
    "\"Refunds are issued within 30 days of purchase.\"";
char *found = thinkthen_call(tt, find);
assert(found && strcmp(found, expected) == 0);
thinkthen_free_string(found);
