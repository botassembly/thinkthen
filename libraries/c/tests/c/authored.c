/* Parsed question metadata owns its bytes and preserves output on refusal. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <string.h>

static void same(thinkthen_string_v1 text, const char *expected) {
    assert(text.len == strlen(expected));
    assert(memcmp(text.data, expected, text.len) == 0);
}
int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false}");
    assert(engine);
    char json[] = "{\"decide\":\"Private question?\",\"name\":\"refund\",\"wording_version\":2,\"item_schema\":{\"type\":\"object\",\"properties\":{\"body\":{\"type\":\"string\"}},\"required\":[\"body\"]},\"context_schema\":{\"type\":\"string\"}}";
    thinkthen_question *question = NULL;
    assert(thinkthen_question_parse(engine, THINKTHEN_LOAD_ATOMIC_V1,
        (thinkthen_string_v1){json, sizeof(json)-1}, &question) == THINKTHEN_OK);
    memset(json, 0, sizeof(json));
    const unsigned char invalid[] = {0xff};
    const thinkthen_string_v1 cases[] = {
        {NULL, 1}, {"x", SIZE_MAX}, {(const char *)invalid, 1}, {"{", 1}
    };
    for (size_t i = 0; i < sizeof(cases)/sizeof(cases[0]); ++i) {
        thinkthen_question *output = (thinkthen_question *)(uintptr_t)17;
        assert(thinkthen_question_parse(engine, THINKTHEN_LOAD_ATOMIC_V1, cases[i], &output) == THINKTHEN_EUSAGE);
        assert(output == (thinkthen_question *)(uintptr_t)17);
    }
    thinkthen_question_author_v1 sentinel = {0}; sentinel.name.present = 7;
    assert(thinkthen_question_author(NULL, &sentinel) == THINKTHEN_EUSAGE);
    assert(sentinel.name.present == 7);
    assert(thinkthen_question_author(question, NULL) == THINKTHEN_EUSAGE);
    thinkthen_engine_free(engine);
    thinkthen_question_author_v1 view;
    assert(thinkthen_question_author(question, &view) == THINKTHEN_OK);
    assert(view.name.present == 1); same(view.name.value, "refund");
    assert(view.wording_version.present == 1 && view.wording_version.value == 2);
    assert(view.item_schema.kind == THINKTHEN_DECLARATION_OBJECT_V1);
    assert(view.item_schema.properties.len == 1);
    same(view.item_schema.properties.data[0].name, "body");
    assert(view.item_schema.properties.data[0].kind == THINKTHEN_PROPERTY_STRING_V1);
    assert(view.item_schema.required.len == 1); same(view.item_schema.required.data[0], "body");
    assert(view.context_schema.kind == THINKTHEN_DECLARATION_STRING_V1);
    thinkthen_question_free(question); thinkthen_question_free(NULL);
    return 0;
}
