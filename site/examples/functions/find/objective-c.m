#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *which_line =
        "{\"find\": \"Which line gives the refund "
        "deadline?\", \"units\": ["
        "\"Returns need the original receipt.\", "
        "\"Refunds are issued within 30 days "
        "of purchase.\", "
        "\"Shipping is free on orders over $50.\", "
        "\"Gift cards cannot be exchanged for cash.\"]}";
    char *deadline = [client json:which_line
                         deadline:-1
                            token:nil
                          failure:&failure];
    assert(deadline);
    TTJSON *envelope =
        tt_json_parse(deadline, strlen(deadline));
    const TTJSON *unit =
        tt_json_get(tt_json_get(envelope, "value"), "unit");
    assert(strcmp(unit->text, "Refunds are issued "
        "within 30 days of purchase.") == 0);
    tt_json_free(envelope);
    free(deadline);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
