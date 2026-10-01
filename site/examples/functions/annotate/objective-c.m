#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *annotate =
        "{\"annotate\": {\"version\": 1, \"questions\": {"
        "\"steps\": {\"decide\": "
        "\"Does the report give steps to reproduce?\"}, "
        "\"area\": {\"choose\": "
        "\"Which part of the app is this?\", "
        "\"options\": [\"export\", \"login\", "
        "\"billing\"]}, "
        "\"impact\": {\"score\": "
        "\"How much does this block the user?\", "
        "\"levels\": [\"None.\", \"Slows them.\", "
        "\"Blocks work.\"]}}}, "
        "\"records\": ["
        "\"Steps: click Log in. Nobody gets in.\"]}";
    char *rows = [client json:annotate
                     deadline:-1
                        token:nil
                      failure:&failure];
    assert(rows);
    TTJSON *envelope = tt_json_parse(rows, strlen(rows));
    const TTJSON *triage = tt_json_get(envelope, "value");
    assert(triage->count == 1);
    const TTJSON *row = triage->children[0];
    const TTJSON *steps = tt_json_get(row, "steps");
    const TTJSON *area = tt_json_get(row, "area");
    const TTJSON *impact = tt_json_get(row, "impact");
    assert(strcmp(steps->text, "true") == 0);
    assert(strcmp(area->text, "login") == 0);
    assert(strcmp(impact->text, "1.98") == 0);
    tt_json_free(envelope);
    free(rows);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
