#include "ThinkThen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    TTFailure failure = {0};
    const char *settings = getenv("TT_SETTINGS_JSON");
    TTClient *client = settings ? [TTClient createWithSettings:settings length:strlen(settings) failure:&failure] : [TTClient create];
    if (!client && settings && failure.kind == TTErrorUsage) { puts("{\"error\":\"usage\"}"); tt_failure_clear(&failure); return 0; }
    if (!client) return 3;
    char *answer = [client jsonBytes:argv[1] length:strlen(argv[1]) deadline:-1 token:nil failure:&failure];
    if (answer) { puts(answer); free(answer); }
    else {
        static const char *kinds[] = {"none", "usage", "backend", "deadline", "local", "cancelled", "defect"};
        if (failure.kind < TTErrorUsage || failure.kind > TTErrorDefect) return 4;
        printf("{\"error\":\"%s\"}\n", kinds[failure.kind]);
    }
    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
