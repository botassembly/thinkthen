/* A C11/C++17 consumer of retained counted constructor signatures. */
#include "thinkthen.h"
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

typedef int (*source_fn)(const thinkthen_engine *, const thinkthen_source_spec_v1 *, thinkthen_source **);
SIGNATURE(thinkthen_source_image_files, source_fn);
typedef int (*image_clone_fn)(const thinkthen_engine *, const uint8_t *, size_t, uint32_t, thinkthen_optional_string_v1, thinkthen_image **);
SIGNATURE(thinkthen_image_clone, image_clone_fn);
typedef int (*image_view_fn)(const thinkthen_image *, thinkthen_image_view_v1 *);
SIGNATURE(thinkthen_image_view, image_view_fn);
/* The Rust-side test supplies its sizes and alignments independently. */
#ifdef RUST_RECORD_SIZE
STATIC_ASSERT(sizeof(thinkthen_record_v1)==RUST_RECORD_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_record_v1)==RUST_RECORD_ALIGN);
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
SIGNATURE(thinkthen_question_parse, load_named_fn);
SIGNATURE(thinkthen_question_load_reference, load_named_fn);
typedef int (*author_fn)(const thinkthen_question *, thinkthen_question_author_v1 *);
SIGNATURE(thinkthen_question_author, author_fn);
STATIC_ASSERT(sizeof(thinkthen_recognition_task_v1)==RUST_RECOGNITION_TASK_SIZE);
STATIC_ASSERT(ALIGNOF(thinkthen_recognition_task_v1)==RUST_RECOGNITION_TASK_ALIGN);
typedef int (*recognition_new_fn)(const thinkthen_engine *,const thinkthen_question_spec_v1 *,const thinkthen_question_author_v1 *,const thinkthen_recognition_task_v1 *,thinkthen_question **);
SIGNATURE(thinkthen_question_new_recognition_v1,recognition_new_fn);
typedef int (*question_task_fn)(const thinkthen_question *,thinkthen_recognition_task_v1 *);
SIGNATURE(thinkthen_question_recognition_task_v1,question_task_fn);
