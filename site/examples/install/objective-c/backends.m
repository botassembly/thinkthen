#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};
    TTDecision broken_is_refund = {0};
    TTDecision thanks_is_refund = {0};
    char *facts = NULL;

    const char *question =
        "Does the customer ask for a refund?";
    const char *broken =
        "Please refund my order. It arrived broken.";
    TTErrorKind broken_error =
        [client decide:question
                  text:broken
                length:strlen(broken)
              deadline:-1
                 token:nil
                answer:&broken_is_refund
                 facts:&facts
               failure:&failure];
    assert(broken_error == TTErrorNone);
    assert(broken_is_refund.outcome == TTOutcomeYes);
    free(facts);
    facts = NULL;

    const char *thanks =
        "Thanks for the quick help yesterday!";
    TTErrorKind thanks_error =
        [client decide:question
                  text:thanks
                length:strlen(thanks)
              deadline:-1
                 token:nil
                answer:&thanks_is_refund
                 facts:&facts
               failure:&failure];
    assert(thanks_error == TTErrorNone);
    assert(thanks_is_refund.outcome == TTOutcomeNo);
    free(facts);

    [client dealloc];
    return 0;
}
