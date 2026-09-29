/* MIT license: Copyright (c) 2026 Ian Maurer.
 * See LICENSE. Self-contained strict JSON tree and answer-shape validator. */
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
/* Null return means malformed JSON, duplicate object key, or wrong answer shape.
 * The caller owns the returned tree and frees it with tt_json_free. */
TTJSON *tt_json_parse(const char *text, size_t length);
void tt_json_free(TTJSON *node);
const TTJSON *tt_json_get(const TTJSON *object, const char *key);
const TTJSON *tt_json_get_n(const TTJSON *object, const char *key, size_t key_length);
/* kind is "annotate", "recognize", or "relate". */
int tt_json_answer_shape(const TTJSON *root, const char *kind);
#endif
