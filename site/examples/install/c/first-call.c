#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *ask =
    "{\"score\": \"How urgent is this?\", "
    "\"levels\": "
    "[\"Routine.\", \"Soon.\", \"Immediate.\"], "
    "\"evidence\": \"Our checkout page is down and "
    "customers cannot pay.\\n\"}";
char *urgency_call = thinkthen_call(tt, ask);
assert(urgency_call);
enum json_tokener_error parse_error;
struct json_object *result =
    json_tokener_parse_verbose(urgency_call, &parse_error);
assert(parse_error == json_tokener_success);
assert(result && json_object_get_type(result)
    == json_type_object);
struct json_object *value;
assert(json_object_object_get_ex(result, "value", &value));
assert(value && (json_object_get_type(value)
    == json_type_double || json_object_get_type(value)
    == json_type_int));
assert(json_object_get_double(value) == 2.0);
struct json_object *facts;
assert(json_object_object_get_ex(result, "facts", &facts));
assert(facts && json_object_get_type(facts)
    == json_type_object);
json_object_put(result);
thinkthen_free_string(urgency_call);
thinkthen_engine_free(tt);
