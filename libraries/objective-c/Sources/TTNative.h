#ifndef TT_NATIVE_H
#define TT_NATIVE_H
#include <thinkthen.h>
/* These views borrow only this host-owned snapshot, never a native result.
 * Every nested byte/array is copied with checked extents. Free once after all
 * readers finish. Snapshots outlive client, question, source and batch owners. */
typedef struct TTNativeResult TTNativeResult;
int tt_native_snapshot(thinkthen_result *, TTNativeResult **);
void tt_native_result_free(TTNativeResult *);
int tt_native_summary(const TTNativeResult *,thinkthen_summary_v1 *);
int tt_native_observation(const TTNativeResult *,size_t,thinkthen_observation_v1 *);
int tt_native_details(const TTNativeResult *,size_t,thinkthen_details_v1 *);
int tt_native_observation_details(const TTNativeResult *,size_t,thinkthen_details_v1 *);
int tt_native_author(const TTNativeResult *,size_t,thinkthen_question_author_v1 *);
int tt_native_observation_author(const TTNativeResult *,size_t,thinkthen_question_author_v1 *);
int tt_native_member_author(const TTNativeResult *,size_t,size_t,thinkthen_question_author_v1 *);
int tt_native_rank_member_count(const TTNativeResult *,size_t,size_t *);
int tt_native_rank_member(const TTNativeResult *,size_t,size_t,thinkthen_rank_view_v1 *);
int tt_native_source_recognition(const TTNativeResult *,size_t,thinkthen_source_recognition_v1 *);
int tt_native_source_relations(const TTNativeResult *,size_t,thinkthen_source_relations_v1 *);
#define TT_GETTER(name) int tt_native_##name(const TTNativeResult *,size_t,thinkthen_##name##_view_v1 *);
TT_GETTER(decide) TT_GETTER(choose) TT_GETTER(tag) TT_GETTER(score) TT_GETTER(filter)
TT_GETTER(rank) TT_GETTER(find) TT_GETTER(annotate) TT_GETTER(recognize) TT_GETTER(relate)
#undef TT_GETTER
#endif
