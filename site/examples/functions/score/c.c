#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *score =
    "{\"score\": \"How urgent is this?\", "
    "\"levels\": [\"Routine.\", \"Soon.\", "
    "\"Immediate.\"], "
    "\"evidence\": \"Our checkout page is down "
    "and customers cannot pay.\\n\"}";
char *urgency_call = thinkthen_call(tt, score);
assert(urgency_call);
struct json_object *result =
    json_tokener_parse(urgency_call);
assert(result);
struct json_object *value;
json_bool has_value = json_object_object_get_ex(
    result, "value", &value);
assert(has_value);
double urgency = json_object_get_double(value);
assert(urgency == 2.0);
json_object_put(result);
thinkthen_free_string(urgency_call);
thinkthen_engine_free(tt);
