// Generated from compiler-derived C declarations. Do not edit.
#include <dlfcn.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#pragma GCC visibility push(hidden)
#include "loader.h"
struct native_functions {
    int (*thinkthen_engine_finish_usage_status_v1)(const thinkthen_engine *, thinkthen_complete_usage_persistence_v1 *, thinkthen_complete_utf8_v1 *);
    void (*thinkthen_engine_free)(thinkthen_engine *);
    thinkthen_engine * (*thinkthen_engine_new_with)(const char *);
    int (*thinkthen_engine_usage_persistence_v1)(const thinkthen_engine *, thinkthen_complete_usage_persistence_v1 *, thinkthen_complete_utf8_v1 *);
    int (*thinkthen_error_code)(const thinkthen_engine *);
    const char * (*thinkthen_error_facts_json)(const thinkthen_engine *);
    const char * (*thinkthen_error_message)(const thinkthen_engine *);
    int (*thinkthen_error_retryable)(const thinkthen_engine *);
    void (*thinkthen_free_string)(char *);
    void (*thinkthen_question_free)(thinkthen_question *);
    int (*thinkthen_question_parse)(const thinkthen_engine *, uint32_t, thinkthen_string_v1, thinkthen_question **);
    int (*thinkthen_request_plan_json)(const thinkthen_engine *, const char *, size_t, char **, size_t *);
    void (*thinkthen_session_cancel)(thinkthen_session *);
    const char * (*thinkthen_session_error_facts_json)(void);
    const char * (*thinkthen_session_error_message)(void);
    int (*thinkthen_session_error_retryable)(void);
    int (*thinkthen_session_finish)(thinkthen_session *, const char *, size_t);
    void (*thinkthen_session_free)(thinkthen_session *);
    int (*thinkthen_session_new_named_with_surface)(const thinkthen_engine *, const thinkthen_named_request_v1 *, thinkthen_complete_utf8_v1, thinkthen_session **);
    int (*thinkthen_session_new_with_surface)(const thinkthen_engine *, const char *, size_t, const char *, size_t, thinkthen_session **);
    int (*thinkthen_session_next_packet)(thinkthen_session *, uint32_t, uint32_t *, thinkthen_session_result **);
    int (*thinkthen_session_push)(thinkthen_session *, const char *, size_t, uint32_t *);
    void (*thinkthen_session_result_free)(thinkthen_session_result *);
    int (*thinkthen_session_result_json)(const thinkthen_session_result *, const char **, size_t *);
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
            symbol = dlsym(handle, "thinkthen_engine_finish_usage_status_v1");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_engine_finish_usage_status_v1) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_engine_finish_usage_status_v1, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_engine_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_engine_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_engine_free, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_question_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_parse");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_parse) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_parse, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_session_new_named_with_surface");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_new_named_with_surface) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_new_named_with_surface, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_new_with_surface");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_new_with_surface) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_new_with_surface, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_next_packet");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_next_packet) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_next_packet, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_push");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_push) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_push, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_result_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_result_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_result_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_result_json");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_result_json) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_result_json, &symbol, sizeof(symbol));
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
int thinkthen_engine_finish_usage_status_v1(const thinkthen_engine * arg0, thinkthen_complete_usage_persistence_v1 * arg1, thinkthen_complete_utf8_v1 * arg2) {
    if (!native.thinkthen_engine_finish_usage_status_v1) abort();
    return native.thinkthen_engine_finish_usage_status_v1(arg0, arg1, arg2);
}
void thinkthen_engine_free(thinkthen_engine * arg0) {
    if (!native.thinkthen_engine_free) abort();
    native.thinkthen_engine_free(arg0);
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
void thinkthen_question_free(thinkthen_question * arg0) {
    if (!native.thinkthen_question_free) abort();
    native.thinkthen_question_free(arg0);
}
int thinkthen_question_parse(const thinkthen_engine * arg0, uint32_t arg1, thinkthen_string_v1 arg2, thinkthen_question ** arg3) {
    if (!native.thinkthen_question_parse) abort();
    return native.thinkthen_question_parse(arg0, arg1, arg2, arg3);
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
int thinkthen_session_new_named_with_surface(const thinkthen_engine * arg0, const thinkthen_named_request_v1 * arg1, thinkthen_complete_utf8_v1 arg2, thinkthen_session ** arg3) {
    if (!native.thinkthen_session_new_named_with_surface) abort();
    return native.thinkthen_session_new_named_with_surface(arg0, arg1, arg2, arg3);
}
int thinkthen_session_new_with_surface(const thinkthen_engine * arg0, const char * arg1, size_t arg2, const char * arg3, size_t arg4, thinkthen_session ** arg5) {
    if (!native.thinkthen_session_new_with_surface) abort();
    return native.thinkthen_session_new_with_surface(arg0, arg1, arg2, arg3, arg4, arg5);
}
int thinkthen_session_next_packet(thinkthen_session * arg0, uint32_t arg1, uint32_t * arg2, thinkthen_session_result ** arg3) {
    if (!native.thinkthen_session_next_packet) abort();
    return native.thinkthen_session_next_packet(arg0, arg1, arg2, arg3);
}
int thinkthen_session_push(thinkthen_session * arg0, const char * arg1, size_t arg2, uint32_t * arg3) {
    if (!native.thinkthen_session_push) abort();
    return native.thinkthen_session_push(arg0, arg1, arg2, arg3);
}
void thinkthen_session_result_free(thinkthen_session_result * arg0) {
    if (!native.thinkthen_session_result_free) abort();
    native.thinkthen_session_result_free(arg0);
}
int thinkthen_session_result_json(const thinkthen_session_result * arg0, const char ** arg1, size_t * arg2) {
    if (!native.thinkthen_session_result_json) abort();
    return native.thinkthen_session_result_json(arg0, arg1, arg2);
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
