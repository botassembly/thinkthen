#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};
    TTDecision is_refund = {0};
    char *facts = NULL;

    const char *question =
        "Does the customer ask for a refund?";
    const char *broken =
        "Please refund my order. It arrived broken.";
    TTErrorKind refund_error =
        [client decide:question
                  text:broken
                length:strlen(broken)
              deadline:-1
                 token:nil
                answer:&is_refund
                 facts:&facts
               failure:&failure];
    assert(refund_error == TTErrorNone);
    assert(is_refund.outcome == TTOutcomeYes);
    free(facts);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
