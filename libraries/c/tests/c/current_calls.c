/* Ten named typed helpers and file carriers, through the installed header. */
#include <thinkthen.h>
#include "platform.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){THINKTHEN_CONTENT_TEXT_V1, STR(s)}
static int equal(thinkthen_string_v1 got, const char *want) {
    return got.len == strlen(want) && (got.len == 0 || memcmp(got.data, want, got.len) == 0);
}
static thinkthen_question *make(thinkthen_engine *e, unsigned kind) {
    thinkthen_question_spec_v1 spec = {0};
    spec.kind = kind;
    if (kind < 8) spec.text = TEXT("Does this need attention?");
    thinkthen_choice_v1 choices[2] = {0};
    choices[0].name = STR("z-first"); choices[1].name = STR("a-second");
    if (kind >= 2 && kind <= 4) spec.choices = (thinkthen_choices_v1){choices, 2};
    thinkthen_member_spec_v1 members[2] = {0};
    if (kind == 8) {
        members[0].name = STR("z_first"); members[0].question = make(e, 1);
        members[1].name = STR("a_second"); members[1].question = make(e, 2);
        spec.members = (thinkthen_member_specs_v1){members, 2};
    }
    thinkthen_relation_v1 rule = {0};
    if (kind == 10) {
        rule.name = STR("supports"); rule.source = STR("*"); rule.target = STR("*");
        spec.relations = (thinkthen_relations_v1){&rule, 1};
    }
    thinkthen_question *q = NULL;
    int code = thinkthen_question_new(e, &spec, &q);
    if (code != 0) fprintf(stderr, "question %u: %s\n", kind, thinkthen_error_message(e));
    assert(code == 0 && q);
    if (kind == 8) {
        thinkthen_question_free((thinkthen_question *)members[0].question);
        thinkthen_question_free((thinkthen_question *)members[1].question);
    }
    return q;
}
static thinkthen_source *records(thinkthen_engine *e) {
    thinkthen_record_v1 rows[2] = {0};
    rows[0].original.present = rows[1].original.present = 1;
    rows[0].original.value = TEXT("Maria Chen joined Northwind Freight.");
    rows[1].original.value = TEXT("A different document.");
    thinkthen_source *source = NULL;
    assert(thinkthen_source_records(e, rows, 2, &source) == 0);
    return source;
}
static void location(thinkthen_current_row_v1 r, int files) {
    assert(r.input.present == 1 && r.index.present == 1);
    assert(r.position.present == files);
    if (files) {
        assert(r.position.value.file.present);
        assert(r.position.value.first_line.present && r.position.value.first_line.value == 1);
        assert(r.position.value.last_line.present && r.position.value.last_line.value == 4);
    }
}
static void ten(thinkthen_engine *e, int files) {
    typedef int (*ask_fn)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    const ask_fn calls[] = {thinkthen_decide_current, thinkthen_choose_current, thinkthen_tag_current, thinkthen_score_current, thinkthen_filter_current, thinkthen_rank_current, thinkthen_find_current, thinkthen_annotate_current, thinkthen_recognize_current, thinkthen_relate_current};
    thinkthen_source *source;
    if (files) {
        const char *path = getenv("TYPED_FILES"); assert(path);
        thinkthen_string_v1 name = {path, strlen(path)};
        thinkthen_source_spec_v1 spec = {{&name, 1}, THINKTHEN_SOURCE_FILE_V1, 0};
        source = NULL; assert(thinkthen_source_files(e, &spec, &source) == 0);
    } else source = records(e);
    thinkthen_result *results[10] = {0};
    for (unsigned i=0; i<10; ++i) {
        thinkthen_question *q = make(e, i+1);
        thinkthen_controls_v1 controls = {0};
        controls.deadline_ms = THINKTHEN_NO_DEADLINE; controls.attempts = 1;
        int code = calls[i](e, q, source, &controls, &results[i]);
        if (code != 0) fprintf(stderr,"call %u: %s\n", i+1, thinkthen_error_message(e));
        assert(code == 0 && results[i]);
        thinkthen_question_free(q);
    }
    thinkthen_source_free(source); thinkthen_engine_free(e);
    for (unsigned i=0; i<10; ++i) {
        thinkthen_current_summary_v1 summary;
        assert(thinkthen_result_current_summary(results[i], &summary) == 0);
        assert(summary.function == i+1 && summary.attempts_present && summary.attempt_count > 0);
        assert(summary.facts.requests_sent == (i==8 ? 4u : 1u) && summary.facts.seconds >= 0);
        assert(summary.question_count > 0);
        thinkthen_current_question_v1 observed;
        assert(thinkthen_result_current_question(results[i],0,&observed) == 0);
        assert(observed.state == THINKTHEN_MEMBER_SUCCESS_V1 && observed.failure == 0);
        assert(observed.meta.requests.len > 0 && observed.meta.model.len > 0);
        thinkthen_current_attempt_v1 attempt;
        assert(thinkthen_result_current_attempt(results[i],0,&attempt) == 0);
        assert(attempt.ordinal == 1 && attempt.outcome == THINKTHEN_ATTEMPT_OK_V1);
        if (i<4) {
            thinkthen_current_atomic_v1 r;
            assert(summary.count == 2);
            assert(thinkthen_result_current_atomic(results[i],0,&r) == 0); location(r.common,files);
            assert(r.value.kind == i+1 && r.meta.cached == 0 && r.meta.question_sha256.len == 64);
            if (i==0) assert(r.value.decide_kind == THINKTHEN_DECIDE_BOOLEAN_V1 && r.value.boolean == 1 && r.yes_probability.value == 0.9);
            if (i==1) assert(r.value.choice.present && equal(r.value.choice.value,"z-first"));
            if (i==2) assert(r.value.tags.len == 2 && equal(r.value.tags.data[0],"z-first") && equal(r.value.tags.data[1],"a-second"));
            if (i==1 || i==3) assert(r.probabilities.len == 2 && equal(r.probabilities.data[0].name,"z-first") && equal(r.probabilities.data[1].name,"a-second"));
            if (i==3) assert(r.value.score == 0.1 && r.nearest.present && equal(r.nearest.value,"z-first"));
        } else if (i==4 || i==5) {
            thinkthen_current_row_v1 r; assert(summary.count == 2);
            assert((i==4 ? thinkthen_result_current_filter(results[i],0,&r) : thinkthen_result_current_rank(results[i],0,&r)) == 0);
            location(r,files); if(i==5) assert(r.index.value == 0 && r.probability.value == 0.9);
        } else if (i==6) {
            thinkthen_current_find_v1 r; assert(summary.count == 1);
            assert(thinkthen_result_current_find(results[i],0,&r) == 0); location(r.selected,files);
            assert(r.candidates.len == 2 && r.selected.index.value == 0 && r.candidates.data[0].probability == 0.9);
        } else if (i==7) {
            thinkthen_current_annotation_v1 r; assert(summary.count == 2);
            assert(thinkthen_result_current_annotate(results[i],0,&r) == 0); location(r.common,files);
            assert(r.members.len == 2 && equal(r.members.data[0].name,"z_first") && equal(r.members.data[1].name,"a_second"));
            assert(r.members.data[0].state == THINKTHEN_MEMBER_SUCCESS_V1 && r.members.data[0].value.boolean == 1);
            assert(r.members.data[1].value.choice.present && equal(r.members.data[1].value.choice.value,"z-first"));
        } else if (i==8) {
            thinkthen_current_recognition_v1 r; assert(summary.count == 2);
            assert(thinkthen_result_current_recognize(results[i],0,&r) == 0); location(r.common,files);
            assert(r.relations_present == 0 && r.relations.len == 0 && r.relations.data == NULL);
            if(!files) assert(r.entities.len == 3 && r.entities.data[0].start == 0 && r.entities.data[0].end == 10 && equal(r.entities.data[0].text,"Maria Chen"));
            assert(r.entities.len > 0 && equal(r.entities.data[0].kind,"ENTITY"));
            for(size_t at=0;at<r.entities.len;++at) assert(r.entities.data[at].end-r.entities.data[at].start == r.entities.data[at].length);
        } else {
            thinkthen_current_relations_v1 r; assert(summary.count == 1);
            assert(thinkthen_result_current_relate(results[i],0,&r) == 0);
            assert(r.edges.len == 2 && equal(r.edges.data[0].relation,"supports") && r.edges.data[0].probability == 0.9);
            assert(r.inputs.len == 2); location(r.inputs.data[0],files);
        }
        thinkthen_result_free(results[i]);
    }
}
static void refusal(thinkthen_engine *e) {
    thinkthen_question_spec_v1 spec = {0}; spec.kind=1; spec.text=TEXT("Q?");
    thinkthen_question *q = NULL;
    assert(thinkthen_question_new(e,&spec,&q) == 0);
    thinkthen_source *source = records(e);
    thinkthen_result *out = (thinkthen_result *)(uintptr_t)19;
    assert(thinkthen_choose_current(e,q,source,NULL,&out) == THINKTHEN_EUSAGE && out == (thinkthen_result *)(uintptr_t)19);
    assert(thinkthen_decide_current(e,NULL,source,NULL,&out) == THINKTHEN_EUSAGE);
    assert(thinkthen_decide_current(e,q,NULL,NULL,&out) == THINKTHEN_EUSAGE);
    assert(thinkthen_decide_current(e,q,source,NULL,NULL) == THINKTHEN_EUSAGE);
    thinkthen_controls_v1 c = {0}; c.deadline_ms=THINKTHEN_NO_DEADLINE;
    c.batch.present=2; assert(thinkthen_decide_current(e,q,source,&c,&out) == THINKTHEN_EUSAGE);
    c.batch.present=1; c.batch.value=2; c.batch_max=1; assert(thinkthen_decide_current(e,q,source,&c,&out) == THINKTHEN_EUSAGE);
    c.batch.present=c.batch_max=0; c.deadline_ms=0; assert(thinkthen_decide_current(e,q,source,&c,&out) == THINKTHEN_EDEADLINE);
    c.deadline_ms=-1; c.cancel=thinkthen_cancel_token_new(); assert(c.cancel); thinkthen_cancel(c.cancel);
    assert(thinkthen_decide_current(e,q,source,&c,&out) == THINKTHEN_ECANCELLED); thinkthen_cancel_token_free(c.cancel);
    assert(out == (thinkthen_result *)(uintptr_t)19);
    thinkthen_current_summary_v1 summary={0}; summary.count=37;
    assert(thinkthen_result_current_summary(NULL,&summary) == THINKTHEN_EUSAGE && summary.count==37);
    thinkthen_question *unchanged=(thinkthen_question *)(uintptr_t)17;
    spec.text.data=(thinkthen_string_v1){NULL,1}; assert(thinkthen_question_new(e,&spec,&unchanged)==THINKTHEN_EUSAGE);
    spec.text.data=(thinkthen_string_v1){"x",SIZE_MAX}; assert(thinkthen_question_new(e,&spec,&unchanged)==THINKTHEN_EUSAGE);
    const char invalid[]={ (char)0xff }; spec.text.data=(thinkthen_string_v1){invalid,1}; assert(thinkthen_question_new(e,&spec,&unchanged)==THINKTHEN_EUSAGE);
    assert(unchanged==(thinkthen_question *)(uintptr_t)17);
    spec=(thinkthen_question_spec_v1){0}; spec.kind=THINKTHEN_FUNCTION_RECOGNIZE_V1;
    spec.relation_threshold.kind=THINKTHEN_RULE_NULL_V1;
    assert(thinkthen_question_new(e,&spec,&unchanged)==THINKTHEN_EUSAGE);
    assert(unchanged==(thinkthen_question *)(uintptr_t)17);
    thinkthen_source *s=(thinkthen_source *)(uintptr_t)23;
    assert(thinkthen_source_records(e,NULL,1,&s)==THINKTHEN_EUSAGE);
    assert(thinkthen_source_records(e,(const thinkthen_record_v1 *)"x",SIZE_MAX,&s)==THINKTHEN_EUSAGE);
    assert(s==(thinkthen_source *)(uintptr_t)23);
    thinkthen_source_free(source); thinkthen_question_free(q); thinkthen_engine_free(e);
    thinkthen_source_free(NULL); thinkthen_question_free(NULL); thinkthen_result_free(NULL);
}
int main(void) {
    binary_streams(); int mode=getchar();
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}"); assert(e);
    if(mode=='R') refusal(e); else ten(e,mode=='F');
    return 0;
}
