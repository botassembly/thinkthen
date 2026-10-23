/* Keep Jansson's C names out of PostgreSQL's builtin function declarations.
 * Its static archive symbols are hidden by --exclude-libs,ALL at link time. */
#include "projection.h"
#include <jansson.h>
TtJson *tt_json_parse(const char *bytes, size_t length)
{
    return json_loadb(bytes, length, JSON_ALLOW_NUL, NULL);
}
void tt_json_free(TtJson *value) { json_decref(value); }
const TtJson *tt_json_get(const TtJson *object, const char *key)
{
    return json_object_get(object, key);
}
const char *tt_json_string(const TtJson *value) { return json_string_value(value); }
size_t tt_json_length(const TtJson *value) { return json_string_length(value); }
double tt_json_number(const TtJson *value) { return json_number_value(value); }
