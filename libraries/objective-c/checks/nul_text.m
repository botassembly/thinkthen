#include "ThinkThen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char result[] =
    "{\"entities\":[{\"text\":\"a\\u0000b\",\"start\":0,\"end\":3,"
    "\"length\":3,\"kind\":\"person\",\"strength\":0.9}]}";

int __wrap_thinkthen_recognize(const thinkthen_engine *engine, const char *spec,
                              const char *text, size_t length, char **out,
                              size_t *out_length) {
    if (!engine || !spec || length != 3 || memcmp(text, "a\0b", 3)) return THINKTHEN_EUSAGE;
    *out_length = sizeof(result) - 1;
    *out = malloc(sizeof(result));
    if (!*out) return THINKTHEN_ELOCAL;
    memcpy(*out, result, sizeof(result));
    return 0;
}
void __wrap_thinkthen_free_string(char *text) { free(text); }

int main(void) {
    const char *source = "{\"a\\u0000b\":1,\"a\":2}";
    TTJSON *keys = tt_json_parse(source, strlen(source));
    if (!keys || keys->count != 2 || !tt_json_get_n(keys, "a\0b", 3) ||
        !tt_json_get(keys, "a") || tt_json_get_n(keys, "a\0b", 3)->type != TTJSONNumber)
        return 1;
    tt_json_free(keys);
    const char *duplicate = "{\"a\\u0000b\":1,\"a\\u0000b\":2}";
    keys = tt_json_parse(duplicate, strlen(duplicate));
    if (keys) { tt_json_free(keys); return 2; }

    TTClient *client = [TTClient create];
    if (!client) return 3;
    TTFailure failure = {0};
    char *raw = NULL;
    size_t length = 0;
    int code = [client recognize:"{\"kinds\":{\"person\":\"A person.\"}}"
                           text:"a\0b" length:3 result:&raw size:&length failure:&failure];
    if (code || !raw || length != sizeof(result) - 1 || memcmp(raw, result, length)) return 4;
    TTJSON *answer = tt_json_parse(raw, length);
    const TTJSON *entities = tt_json_get(answer, "entities");
    const TTJSON *entity = entities && entities->count == 1 ? entities->children[0] : NULL;
    const TTJSON *text = tt_json_get(entity, "text");
    if (!answer || !tt_json_answer_shape(answer, "recognize") || !text ||
        text->text_length != 3 || memcmp(text->text, "a\0b", 3)) return 5;
    tt_json_free(answer); free(raw); tt_failure_clear(&failure); [client dealloc];
    puts("OBJC_NUL_TEXT_PUBLIC_RECOGNIZE_PASS");
    return 0;
}
