#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *filter =
    "{\"filter\": \"Is this a complaint?\", "
    "\"records\": ["
    "\"Arrived a day early. Thank you!\", "
    "\"The zipper broke the first time I used it.\", "
    "\"Does this come in blue?\", "
    "\"The strap snapped on day two.\"]}";
const char *expected =
    "[\"The zipper broke the first time I used it.\","
    "\"The strap snapped on day two.\"]";
char *complaints_call = thinkthen_call(tt, filter);
assert(complaints_call);
enum json_tokener_error parse_error;
struct json_object *result =
    json_tokener_parse_verbose(
        complaints_call, &parse_error);
assert(parse_error == json_tokener_success);
assert(result && json_object_get_type(result)
    == json_type_object);
struct json_object *value;
json_bool has_value = json_object_object_get_ex(
    result, "value", &value);
assert(has_value);
struct json_object *facts;
json_bool has_facts = json_object_object_get_ex(
    result, "facts", &facts);
assert(has_facts);
assert(facts && json_object_get_type(facts)
    == json_type_object);
struct json_object *wanted =
    json_tokener_parse_verbose(expected, &parse_error);
assert(parse_error == json_tokener_success);
assert(json_object_equal(value, wanted));
if (wanted) json_object_put(wanted);
json_object_put(result);
thinkthen_free_string(complaints_call);
thinkthen_engine_free(tt);
