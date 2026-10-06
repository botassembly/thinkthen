/* Typed current-native calls own buffers and survive engine destruction. */
#include <thinkthen.h>
#include <assert.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false}");
    assert(engine);
    char question_text[] = "asks for a refund";
    thinkthen_question_spec_v1 spec = {0};
    spec.kind = THINKTHEN_FUNCTION_DECIDE_V1;
    spec.text = (thinkthen_content_v1){THINKTHEN_CONTENT_TEXT_V1, {question_text, sizeof(question_text)-1}};
    thinkthen_question *question = NULL;
    assert(thinkthen_question_new(engine, &spec, &question) == 0);
    memset(question_text, 'x', sizeof(question_text)-1);
    char text[] = "Refund me.";
    thinkthen_record_v1 record = {0};
    record.original.present = 1;
    record.original.value = (thinkthen_content_v1){THINKTHEN_CONTENT_TEXT_V1, {text, sizeof(text)-1}};
    thinkthen_source *source = NULL;
    assert(thinkthen_source_records(engine, &record, 1, &source) == 0);
    memset(text, 'x', sizeof(text)-1);
    thinkthen_result *result = NULL;
    assert(thinkthen_decide_current(engine, question, source, NULL, &result) == 0);
    thinkthen_question_free(question);
    thinkthen_source_free(source);
    thinkthen_engine_free(engine);
    thinkthen_current_summary_v1 summary;
    assert(thinkthen_result_current_summary(result, &summary) == 0);
    assert(summary.count == 1 && summary.facts.records == 1 && summary.facts.requests_sent == 1);
    assert(summary.facts.input_tokens.present == 0);
    thinkthen_current_atomic_v1 row;
    assert(thinkthen_result_current_atomic(result, 0, &row) == 0);
    assert(row.value.kind == THINKTHEN_FUNCTION_DECIDE_V1);
    assert(row.value.decide_kind == THINKTHEN_DECIDE_BOOLEAN_V1 && row.value.boolean == 1);
    assert(row.yes_probability.present && row.yes_probability.value == 0.9);
    assert(row.common.input.value.data.len == 10 && memcmp(row.common.input.value.data.data, "Refund me.", 10) == 0);
    thinkthen_result_free(result);
    return 0;
}
