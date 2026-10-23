#ifndef THINKTHEN_PG_PROJECTION_H
#define THINKTHEN_PG_PROJECTION_H
#include <stddef.h>
typedef struct json_t TtJson;
TtJson *tt_json_parse(const char *bytes, size_t length);
void tt_json_free(TtJson *value);
const TtJson *tt_json_get(const TtJson *object, const char *key);
const char *tt_json_string(const TtJson *value);
size_t tt_json_length(const TtJson *value);
double tt_json_number(const TtJson *value);
#endif
