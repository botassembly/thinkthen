/* Promote after 0426 constructors/header land. No host parser or allocation. */
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
