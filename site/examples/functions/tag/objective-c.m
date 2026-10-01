#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *which_labels =
        "{\"tag\": \"Which labels fit this message?\", "
        "\"labels\": [\"praise\", \"bug\", \"billing\"], "
        "\"evidence\": \"Love the new dashboard, but "
        "export crashes the app,\\nand I was charged "
        "twice.\\n\"}";
    char *fitting = [client json:which_labels
                        deadline:-1
                           token:nil
                         failure:&failure];
    assert(fitting);
    TTJSON *envelope =
        tt_json_parse(fitting, strlen(fitting));
    const TTJSON *labels = tt_json_get(envelope, "value");
    assert(labels->count == 3);
    TTJSON **label = labels->children;
    assert(strcmp(label[0]->text, "praise") == 0);
    assert(strcmp(label[1]->text, "bug") == 0);
    assert(strcmp(label[2]->text, "billing") == 0);
    tt_json_free(envelope);
    free(fitting);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
