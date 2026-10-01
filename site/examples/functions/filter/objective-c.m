#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *is_complaint =
        "{\"filter\": \"Is this a complaint?\", "
        "\"records\": ["
        "\"Arrived a day early. Thank you!\", "
        "\"The zipper broke the first time I used it.\", "
        "\"Does this come in blue?\", "
        "\"The strap snapped on day two.\"]}";
    char *complaints = [client json:is_complaint
                           deadline:-1
                              token:nil
                            failure:&failure];
    assert(complaints);
    TTJSON *envelope =
        tt_json_parse(complaints, strlen(complaints));
    const TTJSON *reviews = tt_json_get(envelope, "value");
    assert(reviews->count == 2);
    assert(strcmp(reviews->children[0]->text,
        "The zipper broke the first time I used it.") == 0);
    assert(strcmp(reviews->children[1]->text,
        "The strap snapped on day two.") == 0);
    tt_json_free(envelope);
    free(complaints);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
