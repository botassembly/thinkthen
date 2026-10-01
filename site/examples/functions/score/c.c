#include <assert.h>
#include <stdio.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *score =
    "{\"score\": \"How urgent is this?\", "
    "\"levels\": [\"Routine.\", \"Soon.\", "
    "\"Immediate.\"], \"evidence\": \"%s\"}";
const char *texts[] = {
    "Please update my mailing address when you can.",
    "Can you send the signed contract by Friday?",
    "Nobody can log in to the site right now.",
};
const double expected[] = {0.06, 0.99, 2.0};
char request[320];
for (int i = 0; i < 3; i++) {
    snprintf(request, sizeof request, score, texts[i]);
    char *urgency_call = thinkthen_call(tt, request);
    assert(urgency_call);
    enum json_tokener_error parse_error;
    struct json_object *result = json_tokener_parse_verbose(
        urgency_call, &parse_error);
    assert(parse_error == json_tokener_success);
    assert(result && json_object_get_type(result)
        == json_type_object);
    struct json_object *value;
    json_bool has_value = json_object_object_get_ex(
        result, "value", &value);
    assert(has_value);
    double urgency = json_object_get_double(value);
    assert(urgency == expected[i]);
    json_object_put(result);
    thinkthen_free_string(urgency_call);
}
thinkthen_engine_free(tt);
