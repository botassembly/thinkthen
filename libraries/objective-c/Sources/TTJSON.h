/* MIT license: Copyright (c) 2026 local experiment 295 contributors.
 * See LICENSE. Self-contained JSON tree reader. */
#ifndef TT_JSON_H
#define TT_JSON_H
#include <stddef.h>
typedef enum { TTJSONObject, TTJSONArray, TTJSONString, TTJSONNumber, TTJSONBoolean, TTJSONNull } TTJSONType;
typedef struct TTJSON {
    TTJSONType type;
    char *text;
    size_t text_length; /* decoded bytes; text may contain embedded NUL */
    struct TTJSON **children;
    char **keys;
    size_t *key_lengths; /* decoded bytes for each object key */
    size_t count;
} TTJSON;
/* Null return means malformed JSON or a duplicate object key.
 * The caller owns the returned tree and frees it with tt_json_free. */
TTJSON *tt_json_parse(const char *text, size_t length);
void tt_json_free(TTJSON *node);
const TTJSON *tt_json_get(const TTJSON *object, const char *key);
const TTJSON *tt_json_get_n(const TTJSON *object, const char *key, size_t key_length);
#endif
