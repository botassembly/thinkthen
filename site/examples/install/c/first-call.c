#include <assert.h>
#include <json-c/json.h>
#include <thinkthen.h>

int main(void) {
    thinkthen_engine *tt = thinkthen_engine_new();
    assert(tt);

    const char *ask =
        "{\"score\": \"How urgent is this?\", "
        "\"levels\": "
        "[\"Routine.\", \"Soon.\", \"Immediate.\"], "
        "\"evidence\": \""
        "Our checkout page is down and "
        "customers cannot pay.\\n\"}";
    char *urgency_call = thinkthen_call(tt, ask);
    assert(urgency_call);

    enum json_tokener_error parse_error;
    struct json_object *envelope =
        json_tokener_parse_verbose(
            urgency_call, &parse_error);
    assert(parse_error == json_tokener_success);
    assert(json_object_get_type(envelope)
        == json_type_object);

    struct json_object *urgency;
    assert(json_object_object_get_ex(
        envelope, "value", &urgency));
    assert(json_object_get_double(urgency) == 2.0);

    struct json_object *facts;
    assert(json_object_object_get_ex(
        envelope, "facts", &facts));
    assert(json_object_get_type(facts)
        == json_type_object);

    json_object_put(envelope);
    thinkthen_free_string(urgency_call);
    thinkthen_engine_free(tt);
    return 0;
}
