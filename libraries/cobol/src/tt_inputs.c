/* Native constructors clone counted caller storage. */
#include "thinkthen.h"
int TT_QUESTION_NEW(const thinkthen_engine *engine,
                    const thinkthen_question_spec_v1 *spec, thinkthen_question **out) {
    return thinkthen_question_new(engine, spec, out);
}
int TT_QUESTION_LOAD(const thinkthen_engine *engine,
                     const thinkthen_string_v1 *path, thinkthen_question **out) {
    if (!path) return THINKTHEN_EUSAGE;
    return thinkthen_question_load(engine, *path, out);
}
int TT_IMAGE_CLONE(const thinkthen_engine *engine, const uint8_t *bytes,
                   uint64_t count, uint32_t media,
                   const thinkthen_optional_string_v1 *filename, thinkthen_image **out) {
    if (!filename || count > SIZE_MAX) return THINKTHEN_EUSAGE;
    return thinkthen_image_clone(engine, bytes, (size_t)count, media, *filename, out);
}
int TT_SOURCE_RECORDS(const thinkthen_engine *engine, const thinkthen_record_v1 *records,
                      uint64_t count, thinkthen_source **out) {
    if (count > SIZE_MAX) return THINKTHEN_EUSAGE;
    return thinkthen_source_records(engine, records, (size_t)count, out);
}
int TT_SOURCE_FILES(const thinkthen_engine *engine,
                    const thinkthen_source_spec_v1 *spec, thinkthen_source **out) {
    return thinkthen_source_files(engine, spec, out);
}

int TT_QUESTION_PARSE(const thinkthen_engine *e, uint32_t role, const thinkthen_string_v1 *value, thinkthen_question **out) {
    if (!value) return THINKTHEN_EUSAGE;
    return thinkthen_question_parse(e,role,*value,out);
}
int TT_QUESTION_LOAD_NAMED(const thinkthen_engine *e, uint32_t role, const thinkthen_string_v1 *value, thinkthen_question **out) {
    if (!value) return THINKTHEN_EUSAGE;
    return thinkthen_question_load_named(e,role,*value,out);
}
int TT_QUESTION_LOAD_REFERENCE(const thinkthen_engine *e, uint32_t role, const thinkthen_string_v1 *value, thinkthen_question **out) {
    if (!value) return THINKTHEN_EUSAGE;
    return thinkthen_question_load_reference(e,role,*value,out);
}
int TT_QUESTION_NEW_AUTHORED(const thinkthen_engine *e,const thinkthen_question_spec_v1 *s,const thinkthen_question_author_v1 *a,thinkthen_question **out) { return thinkthen_question_new_authored(e,s,a,out); }
int TT_SOURCE_IMAGE_FILES(const thinkthen_engine *e,const thinkthen_source_spec_v1 *s,thinkthen_source **out) { return thinkthen_source_image_files(e,s,out); }
