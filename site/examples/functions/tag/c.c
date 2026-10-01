#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *tag =
    "{\"tag\": \"Which labels fit this message?\", "
    "\"labels\": [\"praise\", \"bug\", \"billing\"], "
    "\"evidence\": \"Love the new dashboard, but export "
    "crashes the app,\\nand I was charged twice.\\n\"}";
const char *expected = "[\"praise\",\"bug\",\"billing\"]";
char *tag_call = thinkthen_call(tt, tag);
assert(tag_call);
enum json_tokener_error parse_error;
struct json_object *result =
    json_tokener_parse_verbose(tag_call, &parse_error);
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
thinkthen_free_string(tag_call);
thinkthen_engine_free(tt);
