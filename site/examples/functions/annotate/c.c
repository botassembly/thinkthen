#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *annotate =
    "{\"annotate\": {\"version\": 1, \"questions\": {"
    "\"steps\": {\"decide\": "
    "\"Does the report give steps to reproduce?\"}, "
    "\"area\": {\"choose\": "
    "\"Which part of the app is this?\", "
    "\"options\": [\"export\", \"login\", \"billing\"]}, "
    "\"impact\": {\"score\": "
    "\"How much does this block the user?\", "
    "\"levels\": [\"None.\", \"Slows them.\", "
    "\"Blocks work.\"]}}}, "
    "\"records\": ["
    "\"Steps: click Log in. Nobody gets in.\"]}";
const char *expected =
    "[{\"steps\":true,\"area\":\"login\","
    "\"impact\":1.98}]";
char *triage_call = thinkthen_call(tt, annotate);
assert(triage_call);
struct json_object *result =
    json_tokener_parse(triage_call);
assert(result);
struct json_object *triage;
json_bool has_triage = json_object_object_get_ex(
    result, "value", &triage);
assert(has_triage);
struct json_object *wanted = json_tokener_parse(expected);
assert(wanted);
json_bool same = json_object_equal(triage, wanted);
assert(same);
json_object_put(wanted);
json_object_put(result);
thinkthen_free_string(triage_call);
thinkthen_engine_free(tt);
