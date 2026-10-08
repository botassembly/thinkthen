#ifndef TT_NATIVE_H
#define TT_NATIVE_H
#include "thinkthen.h"
int TT_DECIDE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_CHOOSE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_TAG(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_SCORE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_FILTER(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_RANK(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_FIND(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_ANNOTATE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_RECOGNIZE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_RELATE(const thinkthen_engine *e, const thinkthen_question *q,
                 const thinkthen_source *s, const thinkthen_controls_v1 *controls,
                 thinkthen_result **out);
int TT_DECIDE_BATCH_START(const thinkthen_engine *e,const thinkthen_question *q,const thinkthen_source *s,const thinkthen_controls_v1 *controls,thinkthen_batch **out);
int TT_CHOOSE_BATCH_START(const thinkthen_engine *e,const thinkthen_question *q,const thinkthen_source *s,const thinkthen_controls_v1 *controls,thinkthen_batch **out);
int TT_TAG_BATCH_START(const thinkthen_engine *e,const thinkthen_question *q,const thinkthen_source *s,const thinkthen_controls_v1 *controls,thinkthen_batch **out);
int TT_SCORE_BATCH_START(const thinkthen_engine *e,const thinkthen_question *q,const thinkthen_source *s,const thinkthen_controls_v1 *controls,thinkthen_batch **out);
int TT_FILTER_BATCH_START(const thinkthen_engine *e,const thinkthen_question *q,const thinkthen_source *s,const thinkthen_controls_v1 *controls,thinkthen_batch **out);
int TT_ANNOTATE_BATCH_START(const thinkthen_engine *e,const thinkthen_question *q,const thinkthen_source *s,const thinkthen_controls_v1 *controls,thinkthen_batch **out);
int TT_QUESTION_NEW(const thinkthen_engine *engine,
                    const thinkthen_question_spec_v1 *spec, thinkthen_question **out);
int TT_QUESTION_LOAD(const thinkthen_engine *engine,
                     const thinkthen_string_v1 *path, thinkthen_question **out);
int TT_IMAGE_CLONE(const thinkthen_engine *engine, const uint8_t *bytes,
                   uint64_t count, uint32_t media,
                   const thinkthen_optional_string_v1 *filename, thinkthen_image **out);
int TT_SOURCE_RECORDS(const thinkthen_engine *engine, const thinkthen_record_v1 *records,
                      uint64_t count, thinkthen_source **out);
int TT_SOURCE_FILES(const thinkthen_engine *engine,
                    const thinkthen_source_spec_v1 *spec, thinkthen_source **out);
int TT_QUESTION_PARSE(const thinkthen_engine *e, uint32_t role, const thinkthen_string_v1 *value, thinkthen_question **out);
int TT_QUESTION_LOAD_NAMED(const thinkthen_engine *e, uint32_t role, const thinkthen_string_v1 *value, thinkthen_question **out);
int TT_QUESTION_LOAD_REFERENCE(const thinkthen_engine *e, uint32_t role, const thinkthen_string_v1 *value, thinkthen_question **out);
int TT_QUESTION_NEW_AUTHORED(const thinkthen_engine *e,const thinkthen_question_spec_v1 *s,const thinkthen_question_author_v1 *a,thinkthen_question **out);
int TT_SOURCE_IMAGE_FILES(const thinkthen_engine *e,const thinkthen_source_spec_v1 *s,thinkthen_source **out);
int TT_QUESTION_NEW_RECOGNITION_V1(const thinkthen_engine *,const thinkthen_question_spec_v1 *,const thinkthen_question_author_v1 *,const thinkthen_recognition_task_v1 *,thinkthen_question **);
int TT_RESULT_RECOGNITION_TASK_V1(const thinkthen_result *,uint64_t,thinkthen_recognition_task_v1 *);
#endif
