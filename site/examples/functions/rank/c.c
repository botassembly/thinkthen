#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *rank =
    "{\"rank\": \"Is this urgent?\", "
    "\"records\": ["
    "\"Newsletter: our autumn catalog is here. "
    "No reply needed.\", "
    "\"Our checkout page is down and customers "
    "cannot pay\", "
    "\"Reminder: your invoice is due in 30 days\", "
    "\"Please send the signed quote by 5 pm today\"]}";
const char *expected =
    "[\"Our checkout page is down and customers "
    "cannot pay\","
    "\"Please send the signed quote by 5 pm today\","
    "\"Reminder: your invoice is due in 30 days\","
    "\"Newsletter: our autumn catalog is here. "
    "No reply needed.\"]";
char *rank_call = thinkthen_call(tt, rank);
assert(rank_call);
enum json_tokener_error parse_error;
struct json_object *result =
    json_tokener_parse_verbose(rank_call, &parse_error);
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
thinkthen_free_string(rank_call);
