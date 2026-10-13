// Generated from compiler-derived C declarations. Do not edit.
#include <dlfcn.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#pragma GCC visibility push(hidden)
#include "loader.h"
struct native_functions {
    char * (*thinkthen_call)(const thinkthen_engine *, const char *);
    char * (*thinkthen_call_opts)(const thinkthen_engine *, const char *, int64_t, thinkthen_cancel_token *);
    void (*thinkthen_cancel)(thinkthen_cancel_token *);
    void (*thinkthen_cancel_token_free)(thinkthen_cancel_token *);
    thinkthen_cancel_token * (*thinkthen_cancel_token_new)(void);
    int (*thinkthen_decide)(const thinkthen_engine *, const char *, const char *, size_t, thinkthen_answer *);
    int (*thinkthen_decide_many)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, thinkthen_answer *);
    int (*thinkthen_decide_many_opts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, thinkthen_answer *);
    int (*thinkthen_decide_many_with_facts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, thinkthen_answer *, char **, size_t *);
    int (*thinkthen_decide_many_with_facts_opts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, thinkthen_answer *, char **, size_t *);
    int (*thinkthen_decide_opts)(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, thinkthen_answer *);
    int (*thinkthen_decide_with_facts)(const thinkthen_engine *, const char *, const char *, size_t, thinkthen_answer *, char **, size_t *);
    int (*thinkthen_decide_with_facts_opts)(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, thinkthen_answer *, char **, size_t *);
    int (*thinkthen_engine_finish_usage_status_v1)(const thinkthen_engine *, thinkthen_complete_usage_persistence_v1 *, thinkthen_complete_utf8_v1 *);
    void (*thinkthen_engine_free)(thinkthen_engine *);
    thinkthen_engine * (*thinkthen_engine_new)(void);
    thinkthen_engine * (*thinkthen_engine_new_with)(const char *);
    int (*thinkthen_engine_usage_persistence_v1)(const thinkthen_engine *, thinkthen_complete_usage_persistence_v1 *, thinkthen_complete_utf8_v1 *);
    int (*thinkthen_error_code)(const thinkthen_engine *);
    const char * (*thinkthen_error_facts_json)(const thinkthen_engine *);
    const char * (*thinkthen_error_message)(const thinkthen_engine *);
    int (*thinkthen_error_retryable)(const thinkthen_engine *);
    void (*thinkthen_free_string)(char *);
    int (*thinkthen_plan_json)(const thinkthen_engine *, const char *, char **, size_t *);
    int (*thinkthen_question_author)(const thinkthen_question *, thinkthen_question_author_v1 *);
    int (*thinkthen_question_file)(const thinkthen_engine *, const char *, char **, size_t *);
    void (*thinkthen_question_free)(thinkthen_question *);
    int (*thinkthen_question_parse)(const thinkthen_engine *, uint32_t, thinkthen_string_v1, thinkthen_question **);
    int (*thinkthen_recognize)(const thinkthen_engine *, const char *, const char *, size_t, char **, size_t *);
    int (*thinkthen_recognize_opts)(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *);
    int (*thinkthen_recognize_with_facts)(const thinkthen_engine *, const char *, const char *, size_t, char **, size_t *, char **, size_t *);
    int (*thinkthen_recognize_with_facts_opts)(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *, char **, size_t *);
    int (*thinkthen_relate)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, char **, size_t *);
    int (*thinkthen_relate_opts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *);
    int (*thinkthen_relate_with_facts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, char **, size_t *, char **, size_t *);
    int (*thinkthen_relate_with_facts_opts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *, char **, size_t *);
    int (*thinkthen_request_plan_json)(const thinkthen_engine *, const char *, size_t, char **, size_t *);
    void (*thinkthen_session_cancel)(thinkthen_session *);
    const char * (*thinkthen_session_error_facts_json)(void);
    const char * (*thinkthen_session_error_message)(void);
    int (*thinkthen_session_error_retryable)(void);
    int (*thinkthen_session_finish)(thinkthen_session *, const char *, size_t);
    void (*thinkthen_session_free)(thinkthen_session *);
    int (*thinkthen_session_new)(const thinkthen_engine *, const char *, size_t, thinkthen_session **);
    int (*thinkthen_session_new_named_with_surface)(const thinkthen_engine *, const thinkthen_named_request_v1 *, thinkthen_complete_utf8_v1, thinkthen_session **);
    int (*thinkthen_session_new_with_surface)(const thinkthen_engine *, const char *, size_t, const char *, size_t, thinkthen_session **);
    void (*thinkthen_session_result_free)(thinkthen_session_result *);
    int (*thinkthen_session_result_json)(const thinkthen_session_result *, const char **, size_t *);
    int (*thinkthen_session_result_view)(const thinkthen_session_result *, const thinkthen_complete_session_packet_v1 **);
    int (*thinkthen_session_try_push)(thinkthen_session *, const char *, size_t, uint32_t *);
    int (*thinkthen_session_try_read)(thinkthen_session *, uint32_t *, thinkthen_session_result **);
    int (*thinkthen_session_wait)(const thinkthen_session *, uint32_t, uint32_t, uint32_t *);
};
static struct native_functions native;
static pthread_mutex_t loading = PTHREAD_MUTEX_INITIALIZER;
static int state;
static void *native_handle;
int thinkthen_swift_load(const char *path) {
    pthread_mutex_lock(&loading);
    if (!state) {
        state = -1;
        void *handle = path && path[0] == '/' ? dlopen(path, RTLD_NOW | RTLD_LOCAL) : NULL;
        if (handle) {
            struct native_functions candidate;
            void *symbol;
            symbol = dlsym(handle, "thinkthen_call");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_call) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_call, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_call_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_call_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_call_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_cancel");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_cancel) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_cancel, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_cancel_token_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_cancel_token_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_cancel_token_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_cancel_token_new");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_cancel_token_new) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_cancel_token_new, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_many");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_many) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_many, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_many_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_many_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_many_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_many_with_facts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_many_with_facts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_many_with_facts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_many_with_facts_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_many_with_facts_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_many_with_facts_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_with_facts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_with_facts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_with_facts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_with_facts_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_with_facts_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_with_facts_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_engine_finish_usage_status_v1");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_engine_finish_usage_status_v1) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_engine_finish_usage_status_v1, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_engine_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_engine_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_engine_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_engine_new");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_engine_new) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_engine_new, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_engine_new_with");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_engine_new_with) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_engine_new_with, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_engine_usage_persistence_v1");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_engine_usage_persistence_v1) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_engine_usage_persistence_v1, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_error_code");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_error_code) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_error_code, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_error_facts_json");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_error_facts_json) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_error_facts_json, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_error_message");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_error_message) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_error_message, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_error_retryable");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_error_retryable) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_error_retryable, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_free_string");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_free_string) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_free_string, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_plan_json");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_plan_json) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_plan_json, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_author");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_author) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_author, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_file");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_file) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_file, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_parse");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_parse) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_parse, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_recognize");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_recognize) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_recognize, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_recognize_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_recognize_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_recognize_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_recognize_with_facts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_recognize_with_facts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_recognize_with_facts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_recognize_with_facts_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_recognize_with_facts_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_recognize_with_facts_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_relate");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_relate) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_relate, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_relate_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_relate_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_relate_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_relate_with_facts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_relate_with_facts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_relate_with_facts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_relate_with_facts_opts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_relate_with_facts_opts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_relate_with_facts_opts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_request_plan_json");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_request_plan_json) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_request_plan_json, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_cancel");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_cancel) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_cancel, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_error_facts_json");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_error_facts_json) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_error_facts_json, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_error_message");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_error_message) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_error_message, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_error_retryable");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_error_retryable) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_error_retryable, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_finish");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_finish) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_finish, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_new");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_new) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_new, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_new_named_with_surface");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_new_named_with_surface) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_new_named_with_surface, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_new_with_surface");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_new_with_surface) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_new_with_surface, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_result_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_result_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_result_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_result_json");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_result_json) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_result_json, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_result_view");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_result_view) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_result_view, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_try_push");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_try_push) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_try_push, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_try_read");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_try_read) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_try_read, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_wait");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_wait) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_wait, &symbol, sizeof(symbol));
            native = candidate;
            native_handle = handle; // Workers may outlive Swift owners; retain until process exit.
            state = 1;
            goto done;
failed:
            dlclose(handle);
        }
    }
done:;
    int result = state == 1 ? 0 : THINKTHEN_ELOCAL;
    pthread_mutex_unlock(&loading);
    return result;
}
char * thinkthen_call(const thinkthen_engine * arg0, const char * arg1) {
    if (!native.thinkthen_call) abort();
    return native.thinkthen_call(arg0, arg1);
}
char * thinkthen_call_opts(const thinkthen_engine * arg0, const char * arg1, int64_t arg2, thinkthen_cancel_token * arg3) {
    if (!native.thinkthen_call_opts) abort();
    return native.thinkthen_call_opts(arg0, arg1, arg2, arg3);
}
void thinkthen_cancel(thinkthen_cancel_token * arg0) {
    if (!native.thinkthen_cancel) abort();
    native.thinkthen_cancel(arg0);
}
void thinkthen_cancel_token_free(thinkthen_cancel_token * arg0) {
    if (!native.thinkthen_cancel_token_free) abort();
    native.thinkthen_cancel_token_free(arg0);
}
thinkthen_cancel_token * thinkthen_cancel_token_new(void) {
    if (!native.thinkthen_cancel_token_new) abort();
    return native.thinkthen_cancel_token_new();
}
int thinkthen_decide(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, thinkthen_answer * arg4) {
    if (!native.thinkthen_decide) abort();
    return native.thinkthen_decide(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_decide_many(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, thinkthen_answer * arg5) {
    if (!native.thinkthen_decide_many) abort();
    return native.thinkthen_decide_many(arg0, arg1, arg2, arg3, arg4, arg5);
}
int thinkthen_decide_many_opts(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, int64_t arg5, thinkthen_cancel_token * arg6, thinkthen_answer * arg7) {
    if (!native.thinkthen_decide_many_opts) abort();
    return native.thinkthen_decide_many_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
}
int thinkthen_decide_many_with_facts(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, thinkthen_answer * arg5, char ** arg6, size_t * arg7) {
    if (!native.thinkthen_decide_many_with_facts) abort();
    return native.thinkthen_decide_many_with_facts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
}
int thinkthen_decide_many_with_facts_opts(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, int64_t arg5, thinkthen_cancel_token * arg6, thinkthen_answer * arg7, char ** arg8, size_t * arg9) {
    if (!native.thinkthen_decide_many_with_facts_opts) abort();
    return native.thinkthen_decide_many_with_facts_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
}
int thinkthen_decide_opts(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, int64_t arg4, thinkthen_cancel_token * arg5, thinkthen_answer * arg6) {
    if (!native.thinkthen_decide_opts) abort();
    return native.thinkthen_decide_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6);
}
int thinkthen_decide_with_facts(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, thinkthen_answer * arg4, char ** arg5, size_t * arg6) {
    if (!native.thinkthen_decide_with_facts) abort();
    return native.thinkthen_decide_with_facts(arg0, arg1, arg2, arg3, arg4, arg5, arg6);
}
int thinkthen_decide_with_facts_opts(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, int64_t arg4, thinkthen_cancel_token * arg5, thinkthen_answer * arg6, char ** arg7, size_t * arg8) {
    if (!native.thinkthen_decide_with_facts_opts) abort();
    return native.thinkthen_decide_with_facts_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}
int thinkthen_engine_finish_usage_status_v1(const thinkthen_engine * arg0, thinkthen_complete_usage_persistence_v1 * arg1, thinkthen_complete_utf8_v1 * arg2) {
    if (!native.thinkthen_engine_finish_usage_status_v1) abort();
    return native.thinkthen_engine_finish_usage_status_v1(arg0, arg1, arg2);
}
void thinkthen_engine_free(thinkthen_engine * arg0) {
    if (!native.thinkthen_engine_free) abort();
    native.thinkthen_engine_free(arg0);
}
thinkthen_engine * thinkthen_engine_new(void) {
    if (!native.thinkthen_engine_new) abort();
    return native.thinkthen_engine_new();
}
thinkthen_engine * thinkthen_engine_new_with(const char * arg0) {
    if (!native.thinkthen_engine_new_with) abort();
    return native.thinkthen_engine_new_with(arg0);
}
int thinkthen_engine_usage_persistence_v1(const thinkthen_engine * arg0, thinkthen_complete_usage_persistence_v1 * arg1, thinkthen_complete_utf8_v1 * arg2) {
    if (!native.thinkthen_engine_usage_persistence_v1) abort();
    return native.thinkthen_engine_usage_persistence_v1(arg0, arg1, arg2);
}
int thinkthen_error_code(const thinkthen_engine * arg0) {
    if (!native.thinkthen_error_code) abort();
    return native.thinkthen_error_code(arg0);
}
const char * thinkthen_error_facts_json(const thinkthen_engine * arg0) {
    if (!native.thinkthen_error_facts_json) abort();
    return native.thinkthen_error_facts_json(arg0);
}
const char * thinkthen_error_message(const thinkthen_engine * arg0) {
    if (!native.thinkthen_error_message) abort();
    return native.thinkthen_error_message(arg0);
}
int thinkthen_error_retryable(const thinkthen_engine * arg0) {
    if (!native.thinkthen_error_retryable) abort();
    return native.thinkthen_error_retryable(arg0);
}
void thinkthen_free_string(char * arg0) {
    if (!native.thinkthen_free_string) abort();
    native.thinkthen_free_string(arg0);
}
int thinkthen_plan_json(const thinkthen_engine * arg0, const char * arg1, char ** arg2, size_t * arg3) {
    if (!native.thinkthen_plan_json) abort();
    return native.thinkthen_plan_json(arg0, arg1, arg2, arg3);
}
int thinkthen_question_author(const thinkthen_question * arg0, thinkthen_question_author_v1 * arg1) {
    if (!native.thinkthen_question_author) abort();
    return native.thinkthen_question_author(arg0, arg1);
}
int thinkthen_question_file(const thinkthen_engine * arg0, const char * arg1, char ** arg2, size_t * arg3) {
    if (!native.thinkthen_question_file) abort();
    return native.thinkthen_question_file(arg0, arg1, arg2, arg3);
}
void thinkthen_question_free(thinkthen_question * arg0) {
    if (!native.thinkthen_question_free) abort();
    native.thinkthen_question_free(arg0);
}
int thinkthen_question_parse(const thinkthen_engine * arg0, uint32_t arg1, thinkthen_string_v1 arg2, thinkthen_question ** arg3) {
    if (!native.thinkthen_question_parse) abort();
    return native.thinkthen_question_parse(arg0, arg1, arg2, arg3);
}
int thinkthen_recognize(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, char ** arg4, size_t * arg5) {
    if (!native.thinkthen_recognize) abort();
    return native.thinkthen_recognize(arg0, arg1, arg2, arg3, arg4, arg5);
}
int thinkthen_recognize_opts(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, int64_t arg4, thinkthen_cancel_token * arg5, char ** arg6, size_t * arg7) {
    if (!native.thinkthen_recognize_opts) abort();
    return native.thinkthen_recognize_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
}
int thinkthen_recognize_with_facts(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, char ** arg4, size_t * arg5, char ** arg6, size_t * arg7) {
    if (!native.thinkthen_recognize_with_facts) abort();
    return native.thinkthen_recognize_with_facts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
}
int thinkthen_recognize_with_facts_opts(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, int64_t arg4, thinkthen_cancel_token * arg5, char ** arg6, size_t * arg7, char ** arg8, size_t * arg9) {
    if (!native.thinkthen_recognize_with_facts_opts) abort();
    return native.thinkthen_recognize_with_facts_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
}
int thinkthen_relate(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, char ** arg5, size_t * arg6) {
    if (!native.thinkthen_relate) abort();
    return native.thinkthen_relate(arg0, arg1, arg2, arg3, arg4, arg5, arg6);
}
int thinkthen_relate_opts(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, int64_t arg5, thinkthen_cancel_token * arg6, char ** arg7, size_t * arg8) {
    if (!native.thinkthen_relate_opts) abort();
    return native.thinkthen_relate_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}
int thinkthen_relate_with_facts(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, char ** arg5, size_t * arg6, char ** arg7, size_t * arg8) {
    if (!native.thinkthen_relate_with_facts) abort();
    return native.thinkthen_relate_with_facts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}
int thinkthen_relate_with_facts_opts(const thinkthen_engine * arg0, const char * arg1, const char *const * arg2, const size_t * arg3, size_t arg4, int64_t arg5, thinkthen_cancel_token * arg6, char ** arg7, size_t * arg8, char ** arg9, size_t * arg10) {
    if (!native.thinkthen_relate_with_facts_opts) abort();
    return native.thinkthen_relate_with_facts_opts(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
}
int thinkthen_request_plan_json(const thinkthen_engine * arg0, const char * arg1, size_t arg2, char ** arg3, size_t * arg4) {
    if (!native.thinkthen_request_plan_json) abort();
    return native.thinkthen_request_plan_json(arg0, arg1, arg2, arg3, arg4);
}
void thinkthen_session_cancel(thinkthen_session * arg0) {
    if (!native.thinkthen_session_cancel) abort();
    native.thinkthen_session_cancel(arg0);
}
const char * thinkthen_session_error_facts_json(void) {
    if (!native.thinkthen_session_error_facts_json) abort();
    return native.thinkthen_session_error_facts_json();
}
const char * thinkthen_session_error_message(void) {
    if (!native.thinkthen_session_error_message) abort();
    return native.thinkthen_session_error_message();
}
int thinkthen_session_error_retryable(void) {
    if (!native.thinkthen_session_error_retryable) abort();
    return native.thinkthen_session_error_retryable();
}
int thinkthen_session_finish(thinkthen_session * arg0, const char * arg1, size_t arg2) {
    if (!native.thinkthen_session_finish) abort();
    return native.thinkthen_session_finish(arg0, arg1, arg2);
}
void thinkthen_session_free(thinkthen_session * arg0) {
    if (!native.thinkthen_session_free) abort();
    native.thinkthen_session_free(arg0);
}
int thinkthen_session_new(const thinkthen_engine * arg0, const char * arg1, size_t arg2, thinkthen_session ** arg3) {
    if (!native.thinkthen_session_new) abort();
    return native.thinkthen_session_new(arg0, arg1, arg2, arg3);
}
int thinkthen_session_new_named_with_surface(const thinkthen_engine * arg0, const thinkthen_named_request_v1 * arg1, thinkthen_complete_utf8_v1 arg2, thinkthen_session ** arg3) {
    if (!native.thinkthen_session_new_named_with_surface) abort();
    return native.thinkthen_session_new_named_with_surface(arg0, arg1, arg2, arg3);
}
int thinkthen_session_new_with_surface(const thinkthen_engine * arg0, const char * arg1, size_t arg2, const char * arg3, size_t arg4, thinkthen_session ** arg5) {
    if (!native.thinkthen_session_new_with_surface) abort();
    return native.thinkthen_session_new_with_surface(arg0, arg1, arg2, arg3, arg4, arg5);
}
void thinkthen_session_result_free(thinkthen_session_result * arg0) {
    if (!native.thinkthen_session_result_free) abort();
    native.thinkthen_session_result_free(arg0);
}
int thinkthen_session_result_json(const thinkthen_session_result * arg0, const char ** arg1, size_t * arg2) {
    if (!native.thinkthen_session_result_json) abort();
    return native.thinkthen_session_result_json(arg0, arg1, arg2);
}
int thinkthen_session_result_view(const thinkthen_session_result * arg0, const thinkthen_complete_session_packet_v1 ** arg1) {
    if (!native.thinkthen_session_result_view) abort();
    return native.thinkthen_session_result_view(arg0, arg1);
}
int thinkthen_session_try_push(thinkthen_session * arg0, const char * arg1, size_t arg2, uint32_t * arg3) {
    if (!native.thinkthen_session_try_push) abort();
    return native.thinkthen_session_try_push(arg0, arg1, arg2, arg3);
}
int thinkthen_session_try_read(thinkthen_session * arg0, uint32_t * arg1, thinkthen_session_result ** arg2) {
    if (!native.thinkthen_session_try_read) abort();
    return native.thinkthen_session_try_read(arg0, arg1, arg2);
}
int thinkthen_session_wait(const thinkthen_session * arg0, uint32_t arg1, uint32_t arg2, uint32_t * arg3) {
    if (!native.thinkthen_session_wait) abort();
    return native.thinkthen_session_wait(arg0, arg1, arg2, arg3);
}
#pragma GCC visibility pop
