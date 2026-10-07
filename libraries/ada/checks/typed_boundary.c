/* Independent hand-authored C carrier cases. These test layout/conversion,
 * not native execution or result/2 parity. Compile against the reviewed header. */
#include "thinkthen.h"
#include <string.h>
#define STR(x) {x, sizeof(x)-1}
_Static_assert(sizeof(thinkthen_record_v1) == 96, "counted record ABI");
_Static_assert(sizeof(thinkthen_question_spec_v1) == 344, "question ABI");
_Static_assert(sizeof(thinkthen_summary_v1) == 712, "summary ABI");
static const thinkthen_probability_v1 probabilities[] = {
    {STR("same"), 0.0}, {STR("same"), 1.0}
};
static const thinkthen_string_v1 requests[] = {STR("repeat"), STR("repeat")};
static const thinkthen_question_source_v1 sources[] = {
    {THINKTHEN_ORIGIN_REPLAY_V1, STR("observed")},
    {THINKTHEN_ORIGIN_REPLAY_V1, STR("observed")}
};
static const thinkthen_observation_identity_v1 identities[] = {
    {.kind=THINKTHEN_ID_OBSERVATION_V1, .data.observation_id=STR("observation")},
    {.kind=THINKTHEN_ID_FAILURE_V1, .data.failure_id=STR("failure")}
};
int tt_carrier_case(thinkthen_summary_v1 *summary, thinkthen_decide_view_v1 *row,
                    thinkthen_member_v1 *member, thinkthen_record_v1 *record) {
    if (!summary || !row || !member || !record) return 1;
    *summary=(thinkthen_summary_v1){
        .state=THINKTHEN_RESULT_SUCCESS_V1, .schema=STR("thinkthen.result/2"),
        .answer_id={1, STR("answer")}, .function={1, THINKTHEN_FUNCTION_DECIDE_V1},
        .count=1, .observation_count=2, .meta={.present=1, .value={
            .model=STR("historical"), .requests={requests,2},
            .origin={1,THINKTHEN_ORIGIN_REPLAY_V1}, .question_sources={sources,2},
            .observations={identities,2}, .answered_by={1,STR("observed")}}},
        .facts={.present=1, .value={.call_id=STR("call"),
            .estimated_cost_usd={1,STR("0.000123")},
            .input_tokens={1,UINT64_MAX}, .records=1}},
        .attempts={.present=1, .value={NULL,0}}
    };
    *row=(thinkthen_decide_view_v1){.common={.answer_id=STR("answer"),
        .position={.present=1,.value={.file={1,STR("explicit.txt")},
            .first_line={1,2},.last_line={1,4}}}},
        .value={.kind=THINKTHEN_DECIDE_BOOLEAN_V1,.data.boolean=0}};
    *member=(thinkthen_member_v1){.name=STR("member"),
        .state=THINKTHEN_MEMBER_SUCCESS_V1,.data.success={
            .answer_id=STR("member-answer"), .value={
                .kind=THINKTHEN_FUNCTION_DECIDE_V1,
                .data.decide={.kind=THINKTHEN_DECIDE_NULL_V1}},
            .answer={.kind=THINKTHEN_ANSWER_TAG_V1,.data.tag={probabilities,2}}}};
    *record=(thinkthen_record_v1){.original={.present=1,.value={
        .kind=THINKTHEN_CONTENT_TEXT_V1,.data=STR("\xc3\xa9\r\n\0z")}},
        .context={.present=1,.value={.kind=THINKTHEN_CONTENT_TEXT_V1,.data={NULL,0}}}};
    return 0;
}
int tt_carrier_request(const thinkthen_question_spec_v1 *spec,
                       const thinkthen_record_v1 *record, uint32_t kind) {
    static const char description[]="{\"what\":\"keep\",\"examples\":[\"a\",\"a\"]}";
    if (!spec || !record || spec->kind != kind || spec->choices.len != 2 ||
        !spec->choices.data || spec->on.len != 2 || !spec->on.data ||
        !record->original.present || record->original.value.data.len != 9000 ||
        !record->original.value.data.data || record->context.present != 1 ||
        record->context.value.data.len != 0 || record->options.len != 2 ||
        record->options.data != spec->choices.data) return 1;
    for (size_t i=0; i<2; i++) {
        const thinkthen_choice_v1 *choice=&spec->choices.data[i];
        if (choice->name.len != 4 || memcmp(choice->name.data,"same",4) ||
            !choice->description.present ||
            choice->description.value.kind != THINKTHEN_CONTENT_JSON_V1 ||
            choice->description.value.data.len != sizeof(description)-1 ||
            memcmp(choice->description.value.data.data,description,sizeof(description)-1) ||
            spec->on.data[i].len != 6 || memcmp(spec->on.data[i].data,"/field",6)) return 1;
    }
    return 0;
}
