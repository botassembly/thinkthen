#include <assert.h>
#include <stdlib.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *ask =
    "{\"score\": \"How urgent is this?\", "
    "\"levels\": "
    "[\"Routine.\", \"Soon.\", \"Immediate.\"], "
    "\"evidence\": \"Checkout is down for everyone.\"}";
char *urgency = thinkthen_call(tt, ask);
assert(urgency && atof(urgency) == 2.0);
thinkthen_free_string(urgency);
thinkthen_engine_free(tt);
