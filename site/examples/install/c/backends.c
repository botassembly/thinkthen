#include <assert.h>
#include <string.h>
#include <thinkthen.h>

int main(void) {
    thinkthen_engine *tt = thinkthen_engine_new();
    assert(tt);

    const char *question =
        "Does the customer ask for a refund?";
    const char *broken =
        "Please refund my order. It arrived broken.";
    const char *thanks =
        "Thanks for the quick help yesterday!";

    thinkthen_answer broken_answer;
    int broken_code = thinkthen_decide(
        tt, question, broken, strlen(broken),
        &broken_answer);
    assert(broken_code == THINKTHEN_OK);
    assert(broken_answer.outcome == THINKTHEN_YES);

    thinkthen_answer thanks_answer;
    int thanks_code = thinkthen_decide(
        tt, question, thanks, strlen(thanks),
        &thanks_answer);
    assert(thanks_code == THINKTHEN_OK);
    assert(thanks_answer.outcome == THINKTHEN_NO);

    thinkthen_engine_free(tt);
    return 0;
}
