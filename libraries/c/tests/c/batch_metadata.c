/* The public header defines the meaning of both batch discriminants. */
#include <thinkthen.h>
#include <assert.h>
#include <string.h>

#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){THINKTHEN_CONTENT_TEXT_V1, STR(s)}

static void check(thinkthen_engine *engine, thinkthen_source *source,
                  const char *question, uint64_t running, uint32_t expected,
                  size_t records, uint32_t tuned, size_t tuned_records) {
    thinkthen_question *parsed = NULL;
    assert(thinkthen_question_parse(engine, THINKTHEN_FUNCTION_DECIDE_V1,
                                   (thinkthen_string_v1){question, strlen(question)}, &parsed) == 0);
    thinkthen_controls_v1 controls = {0};
    controls.deadline_ms = -1;
    if (running) controls.batch = (thinkthen_optional_size_v1){1, running};
    else controls.batch_max = 1;
    thinkthen_result *result = NULL;
    assert(thinkthen_decide_complete(engine, parsed, source, &controls, &result) == 0 && result);
    thinkthen_decide_view_v1 row = {0};
    assert(thinkthen_result_decide(result, 0, &row) == 0);
    assert(row.common.meta.batch_setting.present);
    assert(row.common.meta.batch_setting.value.kind == expected);
    assert(row.common.meta.batch_setting.value.records == records);
    assert(row.common.meta.batch_warning.present == (tuned != 0));
    if (tuned) {
        assert(row.common.meta.batch_warning.value.tuned_for.kind == tuned);
        assert(row.common.meta.batch_warning.value.tuned_for.records == tuned_records);
        assert(row.common.meta.batch_warning.value.running.kind == expected);
        assert(row.common.meta.batch_warning.value.running.records == records);
    }
    thinkthen_result_free(result);
    thinkthen_question_free(parsed);
}

static void check_dynamic(thinkthen_engine *engine, int tuned) {
    const char *with = "{\"choose\":\"Which?\",\"threshold\":0.5,\"batch\":1}";
    const char *without = "{\"choose\":\"Which?\",\"batch\":1}";
    const char *body = tuned ? with : without;
    thinkthen_question *question = NULL;
    assert(thinkthen_question_parse(engine, 3, (thinkthen_string_v1){body, strlen(body)}, &question) == 0);
    thinkthen_choice_v1 choices[2] = {0};
    choices[0].name = STR("first");
    choices[1].name = STR("second");
    thinkthen_record_v1 record = {0};
    record.original = (thinkthen_optional_content_v1){1, TEXT("Refund me.")};
    record.options = (thinkthen_choices_v1){choices, 2};
    thinkthen_source *source = NULL;
    assert(thinkthen_source_records(engine, &record, 1, &source) == 0);
    thinkthen_controls_v1 controls = {0};
    controls.deadline_ms = -1;
    controls.batch = (thinkthen_optional_size_v1){1, 2};
    thinkthen_result *result = NULL;
    assert(thinkthen_choose_complete(engine, question, source, &controls, &result) == 0);
    thinkthen_choose_view_v1 row = {0};
    assert(thinkthen_result_choose(result, 0, &row) == 0);
    assert(row.common.meta.batch_setting.value.kind == THINKTHEN_BATCH_RECORDS_V1);
    assert(row.common.meta.batch_setting.value.records == 2);
    assert(row.common.meta.batch_warning.present == tuned);
    if (tuned) {
        assert(row.common.meta.batch_warning.value.tuned_for.kind == THINKTHEN_BATCH_RECORDS_V1);
        assert(row.common.meta.batch_warning.value.tuned_for.records == 1);
        assert(row.common.meta.batch_warning.value.running.kind == THINKTHEN_BATCH_RECORDS_V1);
        assert(row.common.meta.batch_warning.value.running.records == 2);
    }
    thinkthen_result_free(result);
    thinkthen_source_free(source);
    thinkthen_question_free(question);
}

int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false,\"model\":\"fixed\"}");
    assert(engine);
    thinkthen_record_v1 record = {0};
    record.original = (thinkthen_optional_content_v1){1, TEXT("Refund me.")};
    thinkthen_source *source = NULL;
    assert(thinkthen_source_records(engine, &record, 1, &source) == 0);
    check(engine, source, "{\"decide\":\"Refund?\",\"threshold\":0.5,\"batch\":1}", 2,
          THINKTHEN_BATCH_RECORDS_V1, 2, THINKTHEN_BATCH_RECORDS_V1, 1);
    check(engine, source, "{\"decide\":\"Refund?\",\"threshold\":0.5,\"batch\":2}", 0,
          THINKTHEN_BATCH_MAX_V1, 0, THINKTHEN_BATCH_RECORDS_V1, 2);
    check(engine, source, "{\"decide\":\"Refund?\",\"batch\":1}", 2,
          THINKTHEN_BATCH_RECORDS_V1, 2, 0, 0);
    check_dynamic(engine, 1);
    check_dynamic(engine, 0);
    thinkthen_source_free(source);
    thinkthen_engine_free(engine);
    return 0;
}
