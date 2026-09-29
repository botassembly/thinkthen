#include "ThinkThen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
    if (argc != 6) return 2;
    TTFailure failure = {0};
    const char *settings = getenv("TT_PORTABLE_SETTINGS");
    TTClient *client = [TTClient createWithSettings:settings length:strlen(settings) failure:&failure];
    if (!client) return 3;
    const char *texts[5]; size_t lengths[5]; TTDecision answers[5] = {0};
    for (size_t at = 0; at < 5; ++at) {
        texts[at] = argv[at + 1];
        lengths[at] = strlen(texts[at]);
    }
    int code = [client manyBytes:"Is it relevant?" questionLength:15 texts:texts
                       lengths:lengths count:5 deadline:-1 token:nil answers:answers failure:&failure];
    if (code) return 4;
    for (size_t at = 0; at < 5; ++at)
        if (answers[at].outcome != TTOutcomeYes || answers[at].probability != 0.9) return 5;
    tt_failure_clear(&failure);
    [client dealloc];
    puts("OBJC_PORTABLE_BATCH_PASS five typed rows");
    return 0;
}
