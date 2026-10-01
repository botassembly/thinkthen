#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *is_urgent =
        "{\"rank\": \"Is this urgent?\", \"records\": ["
        "\"Newsletter: our autumn catalog is here. "
        "No reply needed.\", "
        "\"Our checkout page is down and customers "
        "cannot pay\", "
        "\"Reminder: your invoice is due in 30 days\", "
        "\"Please send the signed quote by 5 pm today\"]}";
    char *most_urgent = [client json:is_urgent
                            deadline:-1
                               token:nil
                             failure:&failure];
    assert(most_urgent);
    TTJSON *envelope =
        tt_json_parse(most_urgent, strlen(most_urgent));
    const TTJSON *order = tt_json_get(envelope, "value");
    const char *expected[] = {"1", "3", "2", "0"};
    assert(order->count == 4);
    for (size_t i = 0; i < 4; i++) {
        const TTJSON *index =
            tt_json_get(order->children[i], "index");
        assert(strcmp(index->text, expected[i]) == 0);
    }
    tt_json_free(envelope);
    free(most_urgent);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
