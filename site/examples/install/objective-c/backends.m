#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"

int main(void) {
    const char *settings[] = {
        "{\"backend\":\"typesafe\"}",
        "{\"backend\":\"liquid\"}",
        "{\"backend\":\"ollama\",\"base_url\":\"http://localhost:11535/v1\"}",
    };
    for (size_t i = 0; i < 3; i++) {
        TTFailure failure = {0};
        TTClient *client = [TTClient createWithSettings:settings[i]
            length:strlen(settings[i]) failure:&failure];
        assert(client != nil);
        assert(failure.kind == TTErrorNone);
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

        tt_failure_clear(&failure);
        [client dealloc];
    }
    return 0;
}
