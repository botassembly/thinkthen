/* Parsed question metadata owns its bytes and preserves output on refusal. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>

static void same(thinkthen_string_v1 text, const char *expected) {
    assert(text.len == strlen(expected));
    assert(memcmp(text.data, expected, text.len) == 0);
}
int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false}");
    assert(engine);
    char json[] = "{\"decide\":\"Private question?\",\"name\":\"refund\",\"wording_version\":2,\"item_schema\":{\"type\":\"object\",\"properties\":{\"body\":{\"type\":\"string\"}},\"required\":[\"body\"]},\"context_schema\":{\"type\":\"string\"}}";
    char *source=malloc(sizeof(json));assert(source);memcpy(source,json,sizeof(json));
    thinkthen_question *question = NULL;
    assert(thinkthen_question_parse(engine, THINKTHEN_LOAD_ATOMIC_V1,
        (thinkthen_string_v1){source, sizeof(json)-1}, &question) == THINKTHEN_OK);
    memset(source,0xff,sizeof(json));free(source);
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
    const thinkthen_complete_json_v1 *root=NULL,*again=NULL;
    assert(thinkthen_question_value(question,&root)==THINKTHEN_OK && root->kind==THINKTHEN_COMPLETE_JSON_OBJECT_V1);
    assert(root->data.object.len==5);
    const thinkthen_complete_json_entry_v1 *fields=root->data.object.data;
    assert(fields[0].name.len==6 && !memcmp(fields[0].name.data,"decide",6));
    assert(fields[0].value->kind==THINKTHEN_COMPLETE_JSON_STRING_V1);
    assert(fields[0].value->data.string.len==17 && !memcmp(fields[0].value->data.string.data,"Private question?",17));
    assert(fields[2].value->kind==THINKTHEN_COMPLETE_JSON_NUMBER_V1 && fields[2].value->data.number.len==1 && fields[2].value->data.number.data[0]=='2');
    const thinkthen_error_view_v1 *error=thinkthen_error_view(engine);
    assert(thinkthen_question_value(question,&again)==THINKTHEN_OK && again==root && thinkthen_error_view(engine)==error);
    const thinkthen_complete_json_v1 *missing=(const thinkthen_complete_json_v1 *)(uintptr_t)17;
    assert(thinkthen_question_value(NULL,&missing)==THINKTHEN_EUSAGE && missing==(const thinkthen_complete_json_v1 *)(uintptr_t)17);
    assert(thinkthen_question_value(question,NULL)==THINKTHEN_EUSAGE);
    thinkthen_engine_free(engine);
    assert(thinkthen_question_value(question,&again)==THINKTHEN_OK && again==root);
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
