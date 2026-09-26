#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *annotate =
    "{\"annotate\": {\"version\": 1, \"questions\": {"
    "\"steps\": {\"decide\": "
    "\"Does the report give steps to reproduce?\"}, "
    "\"area\": {\"choose\": "
    "\"Which part of the app is this?\", "
    "\"options\": [\"export\", \"login\", \"billing\"]}}}, "
    "\"records\": ["
    "\"CSV export fails. Steps: click Export.\", "
    "\"The login page spins and nobody can sign in.\", "
    "\"The Pay button on the billing page is too blue.\"]}";
const char *expected =
    "[{\"steps\":true,\"area\":\"export\"},"
    "{\"steps\":false,\"area\":\"login\"},"
    "{\"steps\":false,\"area\":\"billing\"}]";
char *triage = thinkthen_call(tt, annotate);
assert(triage && strcmp(triage, expected) == 0);
thinkthen_free_string(triage);
