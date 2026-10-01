#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

char *refund;
size_t refund_len;
int loaded = thinkthen_question_file(
    tt, "refund.json", &refund, &refund_len);
assert(loaded == THINKTHEN_OK);

const char *money_back =
    "I would like to return this and get "
    "my money back.\n";
thinkthen_answer is_refund;
int rc = thinkthen_decide(
    tt,
    refund,
    money_back,
    strlen(money_back),
    &is_refund
);
assert(rc == THINKTHEN_OK);
assert(is_refund.outcome == THINKTHEN_YES);
thinkthen_free_string(refund);
thinkthen_engine_free(tt);
