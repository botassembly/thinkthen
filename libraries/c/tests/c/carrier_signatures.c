/* A C11/C++17 consumer of the unpublished canonical integration signatures. */
#include "../../src/complete-signatures.h"
#ifdef __cplusplus
#include <type_traits>
#define SIGNATURE(name,type) static_assert(std::is_same<decltype(&(name)), type>::value, #name)
#define ALIGNOF(type) alignof(type)
#define STATIC_ASSERT(condition) static_assert(condition, #condition)
#else
#define SIGNATURE(name,type) _Static_assert(_Generic(&(name), type: 1, default: 0), #name)
#define ALIGNOF(type) _Alignof(type)
#define STATIC_ASSERT(condition) _Static_assert(condition, #condition)
#endif

typedef int (*complete_fn)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
SIGNATURE(thinkthen_decide_complete, complete_fn);
SIGNATURE(thinkthen_choose_complete, complete_fn);
SIGNATURE(thinkthen_tag_complete, complete_fn);
SIGNATURE(thinkthen_score_complete, complete_fn);
SIGNATURE(thinkthen_filter_complete, complete_fn);
SIGNATURE(thinkthen_rank_complete, complete_fn);
SIGNATURE(thinkthen_find_complete, complete_fn);
SIGNATURE(thinkthen_annotate_complete, complete_fn);
SIGNATURE(thinkthen_recognize_complete, complete_fn);
SIGNATURE(thinkthen_relate_complete, complete_fn);
typedef int (*image_clone_fn)(const thinkthen_engine *, const uint8_t *, size_t, uint32_t, thinkthen_optional_string_v1, thinkthen_image **);
SIGNATURE(thinkthen_image_clone, image_clone_fn);
typedef int (*image_view_fn)(const thinkthen_image *, thinkthen_image_view_v1 *);
SIGNATURE(thinkthen_image_view, image_view_fn);
typedef int (*summary_fn)(const thinkthen_result *, thinkthen_summary_v1 *);
SIGNATURE(thinkthen_result_summary, summary_fn);
typedef int (*observation_fn)(const thinkthen_result *, size_t, thinkthen_observation_v1 *);
SIGNATURE(thinkthen_result_observation, observation_fn);
typedef int (*details_fn)(const thinkthen_result *, size_t, thinkthen_details_v1 *);
SIGNATURE(thinkthen_result_details, details_fn);
SIGNATURE(thinkthen_result_observation_details, details_fn);
typedef int (*failure_fn)(const thinkthen_engine *, thinkthen_result **);
SIGNATURE(thinkthen_error_complete, failure_fn);

/* The Rust-side test supplies its sizes and alignments independently. */
#ifdef RUST_RECORD_SIZE
STATIC_ASSERT(sizeof(thinkthen_record_v1)==RUST_RECORD_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_record_v1)==RUST_RECORD_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_question_view_v1)==RUST_QUESTION_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_question_view_v1)==RUST_QUESTION_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_details_v1)==RUST_DETAILS_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_details_v1)==RUST_DETAILS_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_answer_v1)==RUST_ANSWER_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_answer_v1)==RUST_ANSWER_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_meta_v1)==RUST_META_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_meta_v1)==RUST_META_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_summary_v1)==RUST_SUMMARY_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_summary_v1)==RUST_SUMMARY_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_observation_v1)==RUST_OBSERVATION_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_observation_v1)==RUST_OBSERVATION_ALIGN);
#endif

#ifdef RUST_AUTHOR_SIZE
STATIC_ASSERT(sizeof(thinkthen_question_author_v1)==RUST_AUTHOR_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_question_author_v1)==RUST_AUTHOR_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_input_declaration_v1)==RUST_DECLARATION_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_input_declaration_v1)==RUST_DECLARATION_ALIGN);
STATIC_ASSERT(sizeof(thinkthen_input_property_v1)==RUST_PROPERTY_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_input_property_v1)==RUST_PROPERTY_ALIGN);
#endif
typedef int (*authored_fn)(const thinkthen_engine *, const thinkthen_question_spec_v1 *, const thinkthen_question_author_v1 *, thinkthen_question **);
SIGNATURE(thinkthen_question_new_authored, authored_fn);
typedef int (*load_named_fn)(const thinkthen_engine *, uint32_t, thinkthen_string_v1, thinkthen_question **);
SIGNATURE(thinkthen_question_load_named, load_named_fn);
SIGNATURE(thinkthen_question_load_reference, load_named_fn);
typedef int (*author_fn)(const thinkthen_question *, thinkthen_question_author_v1 *);
SIGNATURE(thinkthen_question_author, author_fn);
typedef int (*result_author_fn)(const thinkthen_result *, size_t, thinkthen_question_author_v1 *);
SIGNATURE(thinkthen_result_question_author, result_author_fn);
SIGNATURE(thinkthen_result_observation_author, result_author_fn);
typedef int (*member_author_fn)(const thinkthen_result *, size_t, size_t, thinkthen_question_author_v1 *);
SIGNATURE(thinkthen_result_member_author, member_author_fn);
