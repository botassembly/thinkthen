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
        "\"Steps: click Export. It is very slow.\", "
        "\"Steps: click Log in. Nobody gets in.\", "
        "\"The Pay button on billing is too blue.\"]}";
    char *triage = [client json:annotate
                       deadline:-1
                          token:nil
                        failure:&failure];
    assert(triage);
    TTJSON *envelope =
        tt_json_parse(triage, strlen(triage));
    const TTJSON *rows = tt_json_get(envelope, "value");
    const char *areas[] = {"export", "login", "billing"};
    const char *impacts[] = {"1.04", "1.98", "0.09"};
    assert(rows->count == 3);
    for (size_t i = 0; i < 3; i++) {
        const TTJSON *row = rows->children[i];
        const TTJSON *area = tt_json_get(row, "area");
        const TTJSON *impact = tt_json_get(row, "impact");
        assert(strcmp(area->text, areas[i]) == 0);
        assert(strcmp(impact->text, impacts[i]) == 0);
    }
    tt_json_free(envelope);
    free(triage);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
