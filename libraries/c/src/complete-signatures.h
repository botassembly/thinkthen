/* Private 0426 integration contract; not an installed API. */
#ifndef THINKTHEN_COMPLETE_SIGNATURES_H
#define THINKTHEN_COMPLETE_SIGNATURES_H
#include "thinkthen.h"
#ifdef __cplusplus
extern "C" {
#endif
int thinkthen_decide_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_choose_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_tag_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_score_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_filter_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_rank_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_find_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_annotate_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_recognize_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_relate_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
void thinkthen_result_free(thinkthen_result *);
int thinkthen_result_summary(const thinkthen_result *, thinkthen_summary_v1 *);
int thinkthen_result_observation(const thinkthen_result *, size_t, thinkthen_observation_v1 *);
int thinkthen_result_decide(const thinkthen_result *, size_t, thinkthen_decide_view_v1 *);
int thinkthen_result_choose(const thinkthen_result *, size_t, thinkthen_choose_view_v1 *);
int thinkthen_result_tag(const thinkthen_result *, size_t, thinkthen_tag_view_v1 *);
int thinkthen_result_score(const thinkthen_result *, size_t, thinkthen_score_view_v1 *);
int thinkthen_result_filter(const thinkthen_result *, size_t, thinkthen_filter_view_v1 *);
int thinkthen_result_rank(const thinkthen_result *, size_t, thinkthen_rank_view_v1 *);
int thinkthen_result_find(const thinkthen_result *, size_t, thinkthen_find_view_v1 *);
int thinkthen_result_annotate(const thinkthen_result *, size_t, thinkthen_annotate_view_v1 *);
int thinkthen_result_recognize(const thinkthen_result *, size_t, thinkthen_recognize_view_v1 *);
int thinkthen_result_relate(const thinkthen_result *, size_t, thinkthen_relate_view_v1 *);
/* Snapshot the calling thread's last failure for engine, or its failed-build
 * slot for engine=NULL. Never clears/replaces that slot.
 * With no saved failure: returns OK and writes *out=NULL (no allocation).
 * With a pre-start failure: returns OK and writes an owned FAILURE result;
 * facts/attempts are absent. With a started failure: returns OK and writes an
 * owned FAILURE result with final facts and opt-in attempts, even if []
 * because no send occurred. Both failure snapshots have absent schema,
 * answer_id and function, count=observation_count=0, and meta.present=0.
 * error is present; no origin/model/request/answer provenance is invented.
 * The return value describes snapshot creation, not the saved failure code.
 * out=NULL returns EUSAGE without changing the saved failure.
 */
int thinkthen_error_complete(const thinkthen_engine *, thinkthen_result **);

#ifdef __cplusplus
}
#endif
#endif
