#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *how_urgent =
        "{\"score\": \"How urgent is this?\", "
        "\"levels\": [\"Routine.\", \"Soon.\", "
        "\"Immediate.\"], "
        "\"evidence\": \"Our checkout page is down "
        "and customers cannot pay.\\n\"}";
    char *urgency = [client json:how_urgent
                        deadline:-1
                           token:nil
                         failure:&failure];
    assert(urgency);
    TTJSON *envelope =
        tt_json_parse(urgency, strlen(urgency));
    const TTJSON *level = tt_json_get(envelope, "value");
    assert(strcmp(level->text, "2.0") == 0);
    tt_json_free(envelope);
    free(urgency);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
