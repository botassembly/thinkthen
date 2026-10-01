#include <assert.h>
#include <stdio.h>
#include <json-c/json.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *choose =
    "{\"choose\": \"Which team owns this?\", "
    "\"options\": {"
    "\"billing\": \"Invoices, fees, and refunds.\", "
    "\"shipping\": \"Parcels and delivery.\", "
    "\"account\": \"Logins and passwords.\"}, "
    "\"threshold\": 0.9, \"evidence\": \"%s\"}";
const char *texts[] = {
    "Please refund the extra fee on my invoice.",
    "My parcel went to the wrong address.",
    "I cannot reset my password.",
    "My parcel never came, and now "
        "I cannot log in to track it.",
};
const char *teams[] = {
    "\"billing\"",
    "\"shipping\"",
    "\"account\"",
    "null",
};
char request[320];
for (int i = 0; i < 4; i++) {
    snprintf(request, sizeof request, choose, texts[i]);
    char *team_call = thinkthen_call(tt, request);
    assert(team_call);
    enum json_tokener_error parse_error;
    struct json_object *result =
        json_tokener_parse_verbose(team_call, &parse_error);
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
        json_tokener_parse_verbose(teams[i], &parse_error);
    assert(parse_error == json_tokener_success);
    assert(json_object_equal(value, wanted));
    if (wanted) json_object_put(wanted);
    json_object_put(result);
    thinkthen_free_string(team_call);
}
thinkthen_engine_free(tt);
