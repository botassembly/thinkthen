#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

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
    "\"CSV export fails every time. Steps: open a report,"
    "\\nclick Export, pick CSV. My month-end numbers "
    "are stuck."
    "\\n\", "
    "\"Steps: open the login page, enter a password, "
    "press Enter. The page spins and nobody can "
    "sign in.\", "
    "\"The Pay button on the billing page is a slightly "
    "different blue. No steps, I just noticed it.\"]}";
const char *expected =
    "[{\"steps\":true,\"area\":\"export\","
    "\"impact\":1.99},"
    "{\"steps\":true,\"area\":\"login\","
    "\"impact\":2.0},"
    "{\"steps\":false,\"area\":\"billing\","
    "\"impact\":0.01}]";
char *triage_call = thinkthen_call(tt, annotate);
assert(triage_call);
enum json_tokener_error parse_error;
struct json_object *result =
    json_tokener_parse_verbose(triage_call, &parse_error);
assert(parse_error == json_tokener_success);
assert(result && json_object_get_type(result)
    == json_type_object);
struct json_object *value;
assert(json_object_object_get_ex(result, "value", &value));
struct json_object *facts;
assert(json_object_object_get_ex(result, "facts", &facts));
assert(facts && json_object_get_type(facts)
    == json_type_object);
struct json_object *wanted =
    json_tokener_parse_verbose(expected, &parse_error);
assert(parse_error == json_tokener_success);
assert(json_object_equal(value, wanted));
if (wanted) json_object_put(wanted);
json_object_put(result);
thinkthen_free_string(triage_call);
