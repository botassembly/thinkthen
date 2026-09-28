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
char *deadline_call = thinkthen_call(tt, find);
assert(deadline_call);
assert(strncmp(deadline_call, "{\"value\":", 9) == 0);
const char *refund_deadline = deadline_call + 9;
size_t deadline_len = strlen(expected);
assert(strncmp(
    refund_deadline, expected, deadline_len
) == 0);
assert(strncmp(
    refund_deadline + deadline_len, ",\"facts\":{", 10
) == 0);
thinkthen_free_string(deadline_call);
