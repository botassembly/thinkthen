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
    [client decide:question
              text:broken
            length:strlen(broken)
          deadline:-1
             token:nil
            answer:&is_refund
             facts:&facts
           failure:&failure];
    assert(is_refund.outcome == TTOutcomeYes);
    free(facts);

    const char *refund =
        "{\"decide\": \"Does the customer ask"
        " for a refund?\", \"threshold\": \"0.2:0.8\"}";
    const char *send_back = "I want to send this back.";
    [client decide:refund
              text:send_back
            length:strlen(send_back)
          deadline:-1
             token:nil
            answer:&is_refund
             facts:&facts
           failure:&failure];
    assert(is_refund.outcome == TTOutcomeNotSure);
    free(facts);

    [client dealloc];
    return 0;
}
