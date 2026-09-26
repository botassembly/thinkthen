#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *question =
    "Does the customer ask for a refund?";
const char *text =
    "Please refund my order. It arrived broken.";
thinkthen_answer is_refund;
int rc = thinkthen_decide(
    tt,
    question,
    text,
    strlen(text),
    &is_refund
);
assert(rc == THINKTHEN_OK);
assert(is_refund.outcome == THINKTHEN_YES);
