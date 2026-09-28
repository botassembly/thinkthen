#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *ask =
    "{\"score\": \"How urgent is this?\", "
    "\"levels\": "
    "[\"Routine.\", \"Soon.\", \"Immediate.\"], "
    "\"evidence\": \"Checkout is down for everyone.\"}";
char *urgency_call = thinkthen_call(tt, ask);
assert(urgency_call);
assert(strncmp(urgency_call, "{\"value\":", 9) == 0);
const char *urgency = urgency_call + 9;
assert(strncmp(urgency, "2.0,\"facts\":{", 14) == 0);
thinkthen_free_string(urgency_call);
thinkthen_engine_free(tt);
