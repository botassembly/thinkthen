#include <assert.h>
#include <json-c/json.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *choose =
    "{\"choose\": \"Which team owns this?\", "
    "\"options\": {"
    "\"billing\": \"Invoices, fees, and refunds.\", "
    "\"shipping\": \"Parcels and delivery.\", "
    "\"account\": \"Logins and passwords.\"}, "
    "\"evidence\": "
    "\"My parcel went to the wrong address.\"}";
char *team_call = thinkthen_call(tt, choose);
assert(team_call);
struct json_object *result = json_tokener_parse(team_call);
assert(result);
struct json_object *value;
json_bool has_value = json_object_object_get_ex(
    result, "value", &value);
assert(has_value);
const char *team = json_object_get_string(value);
assert(strcmp(team, "shipping") == 0);
json_object_put(result);
thinkthen_free_string(team_call);
thinkthen_engine_free(tt);
