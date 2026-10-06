/* Private complete integration: compile against reviewed 0426 header. */
#include "thinkthen.h"
static thinkthen_controls_v1 controls_for_cobol(const thinkthen_controls_v1 *input) {
    thinkthen_controls_v1 copy = input ? *input :
        (thinkthen_controls_v1){.deadline_ms=THINKTHEN_NO_DEADLINE};
    copy.surface=(thinkthen_string_v1){"cobol",5};
    return copy;
}
int TT_DECIDE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_decide_complete(e,q,s,&copy,out);
}
int TT_CHOOSE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_choose_complete(e,q,s,&copy,out);
}
int TT_TAG(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_tag_complete(e,q,s,&copy,out);
}
int TT_SCORE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_score_complete(e,q,s,&copy,out);
}
int TT_FILTER(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_filter_complete(e,q,s,&copy,out);
}
int TT_RANK(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_rank_complete(e,q,s,&copy,out);
}
int TT_FIND(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_find_complete(e,q,s,&copy,out);
}
int TT_ANNOTATE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_annotate_complete(e,q,s,&copy,out);
}
int TT_RECOGNIZE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_recognize_complete(e,q,s,&copy,out);
}
int TT_RELATE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out) {
    thinkthen_controls_v1 copy=controls_for_cobol(controls);
    return thinkthen_relate_complete(e,q,s,&copy,out);
}
