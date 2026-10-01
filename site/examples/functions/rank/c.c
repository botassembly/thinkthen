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
const int expected[] = {1, 3, 2, 0};
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
assert(json_object_array_length(value) == 4);
for (size_t i = 0; i < 4; i++) {
    struct json_object *index;
    assert(json_object_object_get_ex(
        json_object_array_get_idx(value, i),
        "index", &index));
    assert(json_object_get_int(index) == expected[i]);
}
json_object_put(result);
thinkthen_free_string(rank_call);
