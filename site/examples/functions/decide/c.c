#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *question =
    "Does the customer ask for a refund?";
const char *text =
    "Please refund my order. It arrived broken.";
thinkthen_answer answer;
int rc = thinkthen_decide(
    tt,
    question,
    text,
    strlen(text),
    &answer
);
assert(rc == THINKTHEN_OK);
assert(answer.outcome == THINKTHEN_YES);
