#include "loader.h"
#ifdef _WIN32
#include <windows.h>
#include <stdlib.h>
#else
#include <dlfcn.h>
#endif
#define SYMBOLS(X) X(toolbox_new) X(toolbox_insert) X(toolbox_from_json) X(toolbox_to_json) X(toolbox_definitions_json) X(toolbox_free) X(buffer_free) X(code_name) X(settings_conversion_error) X(question_conversion_error) X(client_open) X(client_free) X(call_open_question) X(call_open_records_question) X(call_send_text) X(call_send_json) X(call_send_invalid) X(call_end_input) X(call_next) X(call_cancel) X(call_free) X(error_free) X(row_free) X(question_new) X(question_add_choice) X(question_add_label) X(question_add_level) X(question_add_kind) X(question_add_rule) X(question_set_cut) X(question_set_band) X(question_set_evidence) X(question_set_top) X(question_add_entry) X(question_free) X(settings_new) X(settings_set_backend) X(settings_set_api_key) X(settings_set_model) X(settings_set_url) X(settings_set_batch) X(settings_set_retry) X(settings_free)
#define DECLARE(name) static __typeof__(&thinkthen_##name) p_##name;
SYMBOLS(DECLARE)
static int32_t load_code = THINKTHEN_CODE_USAGE_ABI;
int32_t tt_load_code(void) { return load_code; }
static __typeof__(&thinkthen_abi_version) p_abi_version;
static __typeof__(&thinkthen_abi_compatible) p_abi_compatible;
const char *tt_load(const char *path) {
#ifdef _WIN32
    if (!path) return THINKTHEN_ABI_ERROR_MESSAGE;
    int count = MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, path, -1, NULL, 0);
    if (!count) return "Invalid native library path";
    wchar_t *wide = malloc((size_t)count * sizeof(*wide));
    if (!wide) return "Cannot allocate native library path";
    MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, path, -1, wide, count);
    HMODULE handle = LoadLibraryW(wide);
    free(wide);
    if (!handle) { load_code = THINKTHEN_CODE_LOCAL_IO; return "cannot load native engine"; }
#define LOAD(name) p_##name = (__typeof__(p_##name))GetProcAddress(handle, "thinkthen_" #name); if (!p_##name) return THINKTHEN_ABI_ERROR_MESSAGE;
#else
    void *handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
    if (!handle) { load_code = THINKTHEN_CODE_LOCAL_IO; return dlerror(); }
    /* The handle stays loaded, including after a symbol lookup failure. */
#define LOAD(name) p_##name = (__typeof__(p_##name))dlsym(handle, "thinkthen_" #name); if (!p_##name) return THINKTHEN_ABI_ERROR_MESSAGE;
#endif
    LOAD(abi_version)
    LOAD(abi_compatible)
    if (!p_abi_compatible(THINKTHEN_ABI_VERSION, p_abi_version())) return THINKTHEN_ABI_ERROR_MESSAGE;
    SYMBOLS(LOAD)
    return NULL;
}
const char * tt_code_name(int32_t value) { return p_code_name(value); }
int32_t tt_settings_conversion_error(struct thinkthen_error *error) { return p_settings_conversion_error(error); }
int32_t tt_question_conversion_error(struct thinkthen_error *error) { return p_question_conversion_error(error); }
int32_t tt_client_open(const struct thinkthen_settings *settings, struct thinkthen_client **out, struct thinkthen_error *error) { return p_client_open(settings,out,error); }
void tt_client_free(struct thinkthen_client *client) { p_client_free(client); }
int32_t tt_call_open_question(const struct thinkthen_client *client, const struct thinkthen_question *question, const struct thinkthen_text *name, uint32_t offsets, struct thinkthen_call **out, struct thinkthen_error *error) { return p_call_open_question(client,question,name,offsets,out,error); }
int32_t tt_call_send_text(struct thinkthen_call *call, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_call_send_text(call,data,len,error); }
int32_t tt_call_send_json(struct thinkthen_call *call, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_call_send_json(call,data,len,error); }
int32_t tt_call_send_invalid(struct thinkthen_call *call, struct thinkthen_error *error) { return p_call_send_invalid(call,error); }
void tt_call_end_input(struct thinkthen_call *call) { p_call_end_input(call); }
int32_t tt_call_next(struct thinkthen_call *call, struct thinkthen_row *out, struct thinkthen_error *error) { return p_call_next(call,out,error); }
void tt_call_cancel(struct thinkthen_call *call) { p_call_cancel(call); }
void tt_call_free(struct thinkthen_call *call) { p_call_free(call); }
void tt_error_free(struct thinkthen_error *error) { p_error_free(error); }
void tt_row_free(struct thinkthen_row *row) { p_row_free(row); }
int32_t tt_question_new(uint32_t function, const uint8_t *data, size_t len, struct thinkthen_question **out, struct thinkthen_error *error) { return p_question_new(function,data,len,out,error); }
int32_t tt_question_add_choice(struct thinkthen_question *question, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_question_add_choice(question,data,len,error); }
int32_t tt_question_add_label(struct thinkthen_question *question, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_question_add_label(question,data,len,error); }
int32_t tt_question_add_level(struct thinkthen_question *question, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_question_add_level(question,data,len,error); }
int32_t tt_question_add_kind(struct thinkthen_question *question, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_question_add_kind(question,data,len,error); }
int32_t tt_question_add_rule(struct thinkthen_question *question, const struct thinkthen_rule *rule, struct thinkthen_error *error) { return p_question_add_rule(question,rule,error); }
int32_t tt_question_set_cut(struct thinkthen_question *question, double cut, struct thinkthen_error *error) { return p_question_set_cut(question,cut,error); }
int32_t tt_question_set_band(struct thinkthen_question *question, double low, double high, struct thinkthen_error *error) { return p_question_set_band(question,low,high,error); }
int32_t tt_question_set_evidence(struct thinkthen_question *question, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_question_set_evidence(question,data,len,error); }
int32_t tt_question_set_top(struct thinkthen_question *question, size_t top, struct thinkthen_error *error) { return p_question_set_top(question,top,error); }
int32_t tt_question_add_entry(struct thinkthen_question *question, const uint8_t *name, size_t len, const struct thinkthen_question *child, struct thinkthen_error *error) { return p_question_add_entry(question,name,len,child,error); }
void tt_question_free(struct thinkthen_question *question) { p_question_free(question); }
int32_t tt_settings_new(struct thinkthen_settings **out, struct thinkthen_error *error) { return p_settings_new(out,error); }
int32_t tt_settings_set_backend(struct thinkthen_settings *settings, uint32_t backend, struct thinkthen_error *error) { return p_settings_set_backend(settings,backend,error); }
int32_t tt_settings_set_api_key(struct thinkthen_settings *settings, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_settings_set_api_key(settings,data,len,error); }
int32_t tt_settings_set_model(struct thinkthen_settings *settings, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_settings_set_model(settings,data,len,error); }
int32_t tt_settings_set_url(struct thinkthen_settings *settings, const uint8_t *data, size_t len, struct thinkthen_error *error) { return p_settings_set_url(settings,data,len,error); }
int32_t tt_settings_set_batch(struct thinkthen_settings *settings, uint64_t batch, struct thinkthen_error *error) { return p_settings_set_batch(settings,batch,error); }
int32_t tt_settings_set_retry(struct thinkthen_settings *settings, uint32_t max_retries, uint64_t initial_backoff_ms, uint64_t attempt_timeout_ms, struct thinkthen_error *error) { return p_settings_set_retry(settings,max_retries,initial_backoff_ms,attempt_timeout_ms,error); }
void tt_settings_free(struct thinkthen_settings *settings) { p_settings_free(settings); }

int32_t tt_call_open_records_question(const thinkthen_client *c, const thinkthen_question *q, const thinkthen_text *name, const thinkthen_record_input *r, size_t count, size_t stride, uint32_t offsets, thinkthen_call **out, thinkthen_error *e) { return p_call_open_records_question(c,q,name,r,count,stride,offsets,out,e); }

int32_t tt_toolbox_new(thinkthen_toolbox **out, thinkthen_error *e) { return p_toolbox_new(out,e); }
int32_t tt_toolbox_insert(thinkthen_toolbox *t, const uint8_t *n, size_t l, const thinkthen_question *q, thinkthen_error *e) { return p_toolbox_insert(t,n,l,q,e); }
int32_t tt_toolbox_from_json(const uint8_t *d, size_t l, thinkthen_toolbox **out, thinkthen_error *e) { return p_toolbox_from_json(d,l,out,e); }
int32_t tt_toolbox_to_json(const thinkthen_toolbox *t, thinkthen_buffer *out, thinkthen_error *e) { return p_toolbox_to_json(t,out,e); }
int32_t tt_toolbox_definitions_json(const thinkthen_toolbox *t, thinkthen_buffer *out, thinkthen_error *e) { return p_toolbox_definitions_json(t,out,e); }
void tt_toolbox_free(thinkthen_toolbox *t) { p_toolbox_free(t); }
void tt_buffer_free(thinkthen_buffer *b) { p_buffer_free(b); }
