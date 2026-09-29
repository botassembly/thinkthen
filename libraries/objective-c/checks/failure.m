#include "ThinkThen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(void) {
    TTFailure failure = {0};
    TTClient *client = [TTClient create];
    if (!client) return 1;
    const char *bad = "{\"decide\":\"Is it?\",\"evidence\":\"failure-two\"}";
    char *answer = [client jsonBytes:bad length:strlen(bad) deadline:-1 token:nil failure:&failure];
    if (answer || failure.kind != TTErrorBackend || !failure.facts_json) return 2;
    char *facts = strdup(failure.facts_json), *message = strdup(failure.message);
    if (!facts || !message) return 3;
    /* A second refusal reuses the same failure object and must release its old buffers. */
    answer = [client jsonBytes:bad length:strlen(bad) deadline:-1 token:nil failure:&failure];
    if (answer || failure.kind != TTErrorBackend || !failure.facts_json) return 4;
    const char *good = "{\"decide\":\"Is it?\",\"evidence\":\"first\"}";
    answer = [client jsonBytes:good length:strlen(good) deadline:-1 token:nil failure:&failure];
    if (!answer || !strstr(answer,"\"value\":true") || failure.kind != TTErrorNone) return 5;
    if (!strstr(facts,"\"requests_sent\"") || !message[0]) return 6;
    free(answer); free(facts); free(message); tt_failure_clear(&failure); [client dealloc];
    puts("OBJC_FAILURE_FACTS_AND_REUSE_PASS"); return 0;
}
