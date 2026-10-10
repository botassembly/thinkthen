// Generated from compiler-derived C declarations. Do not edit.
#include <dlfcn.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#pragma GCC visibility push(hidden)
#include "loader.h"
struct native_functions {
    int (*thinkthen_annotate_batch_start)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
    int (*thinkthen_annotate_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    int (*thinkthen_batch_facts)(const thinkthen_batch *, thinkthen_result **);
    void (*thinkthen_batch_free)(thinkthen_batch *);
    int (*thinkthen_batch_next)(thinkthen_batch *, thinkthen_result **);
    char * (*thinkthen_call)(const thinkthen_engine *, const char *);
    char * (*thinkthen_call_opts)(const thinkthen_engine *, const char *, int64_t, thinkthen_cancel_token *);
    void (*thinkthen_cancel)(thinkthen_cancel_token *);
    void (*thinkthen_cancel_token_free)(thinkthen_cancel_token *);
    thinkthen_cancel_token * (*thinkthen_cancel_token_new)(void);
    int (*thinkthen_choose_batch_start)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
    int (*thinkthen_choose_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    int (*thinkthen_decide)(const thinkthen_engine *, const char *, const char *, size_t, thinkthen_answer *);
    int (*thinkthen_decide_batch_start)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
    int (*thinkthen_decide_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
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
    int (*thinkthen_error_complete)(const thinkthen_engine *, thinkthen_result **);
    const char * (*thinkthen_error_facts_json)(const thinkthen_engine *);
    const char * (*thinkthen_error_message)(const thinkthen_engine *);
    int (*thinkthen_error_retryable)(const thinkthen_engine *);
    int (*thinkthen_filter_batch_start)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
    int (*thinkthen_filter_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    int (*thinkthen_find_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    void (*thinkthen_free_string)(char *);
    int (*thinkthen_image_clone)(const thinkthen_engine *, const uint8_t *, size_t, uint32_t, thinkthen_optional_string_v1, thinkthen_image **);
    void (*thinkthen_image_free)(thinkthen_image *);
    int (*thinkthen_image_view)(const thinkthen_image *, thinkthen_image_view_v1 *);
    int (*thinkthen_plan_json)(const thinkthen_engine *, const char *, char **, size_t *);
    int (*thinkthen_question_author)(const thinkthen_question *, thinkthen_question_author_v1 *);
    int (*thinkthen_question_file)(const thinkthen_engine *, const char *, char **, size_t *);
    void (*thinkthen_question_free)(thinkthen_question *);
    int (*thinkthen_question_load)(const thinkthen_engine *, thinkthen_string_v1, thinkthen_question **);
    int (*thinkthen_question_load_named)(const thinkthen_engine *, uint32_t, thinkthen_string_v1, thinkthen_question **);
    int (*thinkthen_question_load_reference)(const thinkthen_engine *, uint32_t, thinkthen_string_v1, thinkthen_question **);
    int (*thinkthen_question_new)(const thinkthen_engine *, const thinkthen_question_spec_v1 *, thinkthen_question **);
    int (*thinkthen_question_new_authored)(const thinkthen_engine *, const thinkthen_question_spec_v1 *, const thinkthen_question_author_v1 *, thinkthen_question **);
    int (*thinkthen_question_new_recognition_v1)(const thinkthen_engine *, const thinkthen_question_spec_v1 *, const thinkthen_question_author_v1 *, const thinkthen_recognition_task_v1 *, thinkthen_question **);
    int (*thinkthen_question_parse)(const thinkthen_engine *, uint32_t, thinkthen_string_v1, thinkthen_question **);
    int (*thinkthen_question_recognition_task_v1)(const thinkthen_question *, thinkthen_recognition_task_v1 *);
    int (*thinkthen_rank_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    int (*thinkthen_recognize)(const thinkthen_engine *, const char *, const char *, size_t, char **, size_t *);
    int (*thinkthen_recognize_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    int (*thinkthen_recognize_opts)(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *);
    int (*thinkthen_recognize_with_facts)(const thinkthen_engine *, const char *, const char *, size_t, char **, size_t *, char **, size_t *);
    int (*thinkthen_recognize_with_facts_opts)(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *, char **, size_t *);
    int (*thinkthen_relate)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, char **, size_t *);
    int (*thinkthen_relate_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    int (*thinkthen_relate_opts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *);
    int (*thinkthen_relate_with_facts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, char **, size_t *, char **, size_t *);
    int (*thinkthen_relate_with_facts_opts)(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *, char **, size_t *);
    int (*thinkthen_request_plan_json)(const thinkthen_engine *, const char *, size_t, char **, size_t *);
    int (*thinkthen_result_annotate)(const thinkthen_result *, size_t, thinkthen_annotate_view_v1 *);
    int (*thinkthen_result_choose)(const thinkthen_result *, size_t, thinkthen_choose_view_v1 *);
    int (*thinkthen_result_decide)(const thinkthen_result *, size_t, thinkthen_decide_view_v1 *);
    int (*thinkthen_result_details)(const thinkthen_result *, size_t, thinkthen_details_v1 *);
    int (*thinkthen_result_filter)(const thinkthen_result *, size_t, thinkthen_filter_view_v1 *);
    int (*thinkthen_result_find)(const thinkthen_result *, size_t, thinkthen_find_view_v1 *);
    void (*thinkthen_result_free)(thinkthen_result *);
    int (*thinkthen_result_member_author)(const thinkthen_result *, size_t, size_t, thinkthen_question_author_v1 *);
    int (*thinkthen_result_observation)(const thinkthen_result *, size_t, thinkthen_observation_v1 *);
    int (*thinkthen_result_observation_author)(const thinkthen_result *, size_t, thinkthen_question_author_v1 *);
    int (*thinkthen_result_observation_details)(const thinkthen_result *, size_t, thinkthen_details_v1 *);
    int (*thinkthen_result_question_author)(const thinkthen_result *, size_t, thinkthen_question_author_v1 *);
    int (*thinkthen_result_rank)(const thinkthen_result *, size_t, thinkthen_rank_view_v1 *);
    int (*thinkthen_result_rank_member)(const thinkthen_result *, size_t, size_t, thinkthen_rank_view_v1 *);
    int (*thinkthen_result_rank_member_count)(const thinkthen_result *, size_t, size_t *);
    int (*thinkthen_result_rank_member_details)(const thinkthen_result *, size_t, size_t, thinkthen_details_v1 *);
    int (*thinkthen_result_recognition_task_v1)(const thinkthen_result *, size_t, thinkthen_recognition_task_v1 *);
    int (*thinkthen_result_recognize)(const thinkthen_result *, size_t, thinkthen_recognize_view_v1 *);
    int (*thinkthen_result_relate)(const thinkthen_result *, size_t, thinkthen_relate_view_v1 *);
    int (*thinkthen_result_row)(const thinkthen_result *, size_t, thinkthen_row_observation_v1 *);
    int (*thinkthen_result_score)(const thinkthen_result *, size_t, thinkthen_score_view_v1 *);
    int (*thinkthen_result_source_recognition)(const thinkthen_result *, size_t, thinkthen_source_recognition_v1 *);
    int (*thinkthen_result_source_relations)(const thinkthen_result *, size_t, thinkthen_source_relations_v1 *);
    int (*thinkthen_result_summary)(const thinkthen_result *, thinkthen_summary_v1 *);
    int (*thinkthen_result_tag)(const thinkthen_result *, size_t, thinkthen_tag_view_v1 *);
    int (*thinkthen_score_batch_start)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
    int (*thinkthen_score_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
    void (*thinkthen_session_cancel)(thinkthen_session *);
    const char * (*thinkthen_session_error_message)(void);
    int (*thinkthen_session_finish)(thinkthen_session *, const char *, size_t);
    void (*thinkthen_session_free)(thinkthen_session *);
    int (*thinkthen_session_new)(const thinkthen_engine *, const char *, size_t, thinkthen_session **);
    int (*thinkthen_session_new_with_surface)(const thinkthen_engine *, const char *, size_t, const char *, size_t, thinkthen_session **);
    void (*thinkthen_session_result_free)(thinkthen_session_result *);
    int (*thinkthen_session_result_json)(const thinkthen_session_result *, const char **, size_t *);
    int (*thinkthen_session_result_view)(const thinkthen_session_result *, const thinkthen_complete_session_packet_v1 **);
    int (*thinkthen_session_try_push)(thinkthen_session *, const char *, size_t, uint32_t *);
    int (*thinkthen_session_try_read)(thinkthen_session *, uint32_t *, thinkthen_session_result **);
    int (*thinkthen_source_files)(const thinkthen_engine *, const thinkthen_source_spec_v1 *, thinkthen_source **);
    void (*thinkthen_source_free)(thinkthen_source *);
    int (*thinkthen_source_image_files)(const thinkthen_engine *, const thinkthen_source_spec_v1 *, thinkthen_source **);
    int (*thinkthen_source_records)(const thinkthen_engine *, const thinkthen_record_v1 *, size_t, thinkthen_source **);
    int (*thinkthen_tag_batch_start)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
    int (*thinkthen_tag_complete)(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
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
            symbol = dlsym(handle, "thinkthen_annotate_batch_start");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_annotate_batch_start) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_annotate_batch_start, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_annotate_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_annotate_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_annotate_complete, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_batch_facts");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_batch_facts) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_batch_facts, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_batch_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_batch_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_batch_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_batch_next");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_batch_next) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_batch_next, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_choose_batch_start");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_choose_batch_start) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_choose_batch_start, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_choose_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_choose_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_choose_complete, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_batch_start");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_batch_start) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_batch_start, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_decide_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_decide_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_decide_complete, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_error_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_error_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_error_complete, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_filter_batch_start");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_filter_batch_start) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_filter_batch_start, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_filter_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_filter_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_filter_complete, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_find_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_find_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_find_complete, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_free_string");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_free_string) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_free_string, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_image_clone");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_image_clone) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_image_clone, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_image_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_image_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_image_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_image_view");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_image_view) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_image_view, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_question_load");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_load) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_load, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_load_named");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_load_named) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_load_named, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_load_reference");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_load_reference) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_load_reference, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_new");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_new) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_new, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_new_authored");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_new_authored) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_new_authored, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_new_recognition_v1");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_new_recognition_v1) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_new_recognition_v1, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_parse");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_parse) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_parse, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_question_recognition_task_v1");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_question_recognition_task_v1) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_question_recognition_task_v1, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_rank_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_rank_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_rank_complete, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_recognize");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_recognize) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_recognize, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_recognize_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_recognize_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_recognize_complete, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_relate_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_relate_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_relate_complete, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_result_annotate");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_annotate) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_annotate, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_choose");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_choose) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_choose, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_decide");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_decide) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_decide, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_details");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_details) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_details, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_filter");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_filter) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_filter, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_find");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_find) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_find, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_member_author");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_member_author) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_member_author, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_observation");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_observation) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_observation, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_observation_author");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_observation_author) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_observation_author, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_observation_details");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_observation_details) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_observation_details, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_question_author");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_question_author) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_question_author, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_rank");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_rank) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_rank, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_rank_member");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_rank_member) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_rank_member, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_rank_member_count");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_rank_member_count) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_rank_member_count, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_rank_member_details");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_rank_member_details) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_rank_member_details, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_recognition_task_v1");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_recognition_task_v1) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_recognition_task_v1, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_recognize");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_recognize) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_recognize, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_relate");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_relate) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_relate, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_row");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_row) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_row, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_score");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_score) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_score, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_source_recognition");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_source_recognition) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_source_recognition, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_source_relations");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_source_relations) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_source_relations, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_summary");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_summary) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_summary, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_result_tag");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_result_tag) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_result_tag, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_score_batch_start");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_score_batch_start) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_score_batch_start, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_score_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_score_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_score_complete, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_cancel");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_cancel) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_cancel, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_session_error_message");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_session_error_message) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_session_error_message, &symbol, sizeof(symbol));
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
            symbol = dlsym(handle, "thinkthen_source_files");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_source_files) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_source_files, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_source_free");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_source_free) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_source_free, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_source_image_files");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_source_image_files) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_source_image_files, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_source_records");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_source_records) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_source_records, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_tag_batch_start");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_tag_batch_start) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_tag_batch_start, &symbol, sizeof(symbol));
            symbol = dlsym(handle, "thinkthen_tag_complete");
            if (!symbol) goto failed;
            _Static_assert(sizeof(candidate.thinkthen_tag_complete) == sizeof(symbol), "function pointer width");
            memcpy(&candidate.thinkthen_tag_complete, &symbol, sizeof(symbol));
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
int thinkthen_annotate_batch_start(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_batch ** arg4) {
    if (!native.thinkthen_annotate_batch_start) abort();
    return native.thinkthen_annotate_batch_start(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_annotate_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_annotate_complete) abort();
    return native.thinkthen_annotate_complete(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_batch_facts(const thinkthen_batch * arg0, thinkthen_result ** arg1) {
    if (!native.thinkthen_batch_facts) abort();
    return native.thinkthen_batch_facts(arg0, arg1);
}
void thinkthen_batch_free(thinkthen_batch * arg0) {
    if (!native.thinkthen_batch_free) abort();
    native.thinkthen_batch_free(arg0);
}
int thinkthen_batch_next(thinkthen_batch * arg0, thinkthen_result ** arg1) {
    if (!native.thinkthen_batch_next) abort();
    return native.thinkthen_batch_next(arg0, arg1);
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
int thinkthen_choose_batch_start(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_batch ** arg4) {
    if (!native.thinkthen_choose_batch_start) abort();
    return native.thinkthen_choose_batch_start(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_choose_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_choose_complete) abort();
    return native.thinkthen_choose_complete(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_decide(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, thinkthen_answer * arg4) {
    if (!native.thinkthen_decide) abort();
    return native.thinkthen_decide(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_decide_batch_start(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_batch ** arg4) {
    if (!native.thinkthen_decide_batch_start) abort();
    return native.thinkthen_decide_batch_start(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_decide_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_decide_complete) abort();
    return native.thinkthen_decide_complete(arg0, arg1, arg2, arg3, arg4);
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
int thinkthen_error_complete(const thinkthen_engine * arg0, thinkthen_result ** arg1) {
    if (!native.thinkthen_error_complete) abort();
    return native.thinkthen_error_complete(arg0, arg1);
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
int thinkthen_filter_batch_start(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_batch ** arg4) {
    if (!native.thinkthen_filter_batch_start) abort();
    return native.thinkthen_filter_batch_start(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_filter_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_filter_complete) abort();
    return native.thinkthen_filter_complete(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_find_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_find_complete) abort();
    return native.thinkthen_find_complete(arg0, arg1, arg2, arg3, arg4);
}
void thinkthen_free_string(char * arg0) {
    if (!native.thinkthen_free_string) abort();
    native.thinkthen_free_string(arg0);
}
int thinkthen_image_clone(const thinkthen_engine * arg0, const uint8_t * arg1, size_t arg2, uint32_t arg3, thinkthen_optional_string_v1 arg4, thinkthen_image ** arg5) {
    if (!native.thinkthen_image_clone) abort();
    return native.thinkthen_image_clone(arg0, arg1, arg2, arg3, arg4, arg5);
}
void thinkthen_image_free(thinkthen_image * arg0) {
    if (!native.thinkthen_image_free) abort();
    native.thinkthen_image_free(arg0);
}
int thinkthen_image_view(const thinkthen_image * arg0, thinkthen_image_view_v1 * arg1) {
    if (!native.thinkthen_image_view) abort();
    return native.thinkthen_image_view(arg0, arg1);
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
int thinkthen_question_load(const thinkthen_engine * arg0, thinkthen_string_v1 arg1, thinkthen_question ** arg2) {
    if (!native.thinkthen_question_load) abort();
    return native.thinkthen_question_load(arg0, arg1, arg2);
}
int thinkthen_question_load_named(const thinkthen_engine * arg0, uint32_t arg1, thinkthen_string_v1 arg2, thinkthen_question ** arg3) {
    if (!native.thinkthen_question_load_named) abort();
    return native.thinkthen_question_load_named(arg0, arg1, arg2, arg3);
}
int thinkthen_question_load_reference(const thinkthen_engine * arg0, uint32_t arg1, thinkthen_string_v1 arg2, thinkthen_question ** arg3) {
    if (!native.thinkthen_question_load_reference) abort();
    return native.thinkthen_question_load_reference(arg0, arg1, arg2, arg3);
}
int thinkthen_question_new(const thinkthen_engine * arg0, const thinkthen_question_spec_v1 * arg1, thinkthen_question ** arg2) {
    if (!native.thinkthen_question_new) abort();
    return native.thinkthen_question_new(arg0, arg1, arg2);
}
int thinkthen_question_new_authored(const thinkthen_engine * arg0, const thinkthen_question_spec_v1 * arg1, const thinkthen_question_author_v1 * arg2, thinkthen_question ** arg3) {
    if (!native.thinkthen_question_new_authored) abort();
    return native.thinkthen_question_new_authored(arg0, arg1, arg2, arg3);
}
int thinkthen_question_new_recognition_v1(const thinkthen_engine * arg0, const thinkthen_question_spec_v1 * arg1, const thinkthen_question_author_v1 * arg2, const thinkthen_recognition_task_v1 * arg3, thinkthen_question ** arg4) {
    if (!native.thinkthen_question_new_recognition_v1) abort();
    return native.thinkthen_question_new_recognition_v1(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_question_parse(const thinkthen_engine * arg0, uint32_t arg1, thinkthen_string_v1 arg2, thinkthen_question ** arg3) {
    if (!native.thinkthen_question_parse) abort();
    return native.thinkthen_question_parse(arg0, arg1, arg2, arg3);
}
int thinkthen_question_recognition_task_v1(const thinkthen_question * arg0, thinkthen_recognition_task_v1 * arg1) {
    if (!native.thinkthen_question_recognition_task_v1) abort();
    return native.thinkthen_question_recognition_task_v1(arg0, arg1);
}
int thinkthen_rank_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_rank_complete) abort();
    return native.thinkthen_rank_complete(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_recognize(const thinkthen_engine * arg0, const char * arg1, const char * arg2, size_t arg3, char ** arg4, size_t * arg5) {
    if (!native.thinkthen_recognize) abort();
    return native.thinkthen_recognize(arg0, arg1, arg2, arg3, arg4, arg5);
}
int thinkthen_recognize_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_recognize_complete) abort();
    return native.thinkthen_recognize_complete(arg0, arg1, arg2, arg3, arg4);
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
int thinkthen_relate_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_relate_complete) abort();
    return native.thinkthen_relate_complete(arg0, arg1, arg2, arg3, arg4);
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
int thinkthen_result_annotate(const thinkthen_result * arg0, size_t arg1, thinkthen_annotate_view_v1 * arg2) {
    if (!native.thinkthen_result_annotate) abort();
    return native.thinkthen_result_annotate(arg0, arg1, arg2);
}
int thinkthen_result_choose(const thinkthen_result * arg0, size_t arg1, thinkthen_choose_view_v1 * arg2) {
    if (!native.thinkthen_result_choose) abort();
    return native.thinkthen_result_choose(arg0, arg1, arg2);
}
int thinkthen_result_decide(const thinkthen_result * arg0, size_t arg1, thinkthen_decide_view_v1 * arg2) {
    if (!native.thinkthen_result_decide) abort();
    return native.thinkthen_result_decide(arg0, arg1, arg2);
}
int thinkthen_result_details(const thinkthen_result * arg0, size_t arg1, thinkthen_details_v1 * arg2) {
    if (!native.thinkthen_result_details) abort();
    return native.thinkthen_result_details(arg0, arg1, arg2);
}
int thinkthen_result_filter(const thinkthen_result * arg0, size_t arg1, thinkthen_filter_view_v1 * arg2) {
    if (!native.thinkthen_result_filter) abort();
    return native.thinkthen_result_filter(arg0, arg1, arg2);
}
int thinkthen_result_find(const thinkthen_result * arg0, size_t arg1, thinkthen_find_view_v1 * arg2) {
    if (!native.thinkthen_result_find) abort();
    return native.thinkthen_result_find(arg0, arg1, arg2);
}
void thinkthen_result_free(thinkthen_result * arg0) {
    if (!native.thinkthen_result_free) abort();
    native.thinkthen_result_free(arg0);
}
int thinkthen_result_member_author(const thinkthen_result * arg0, size_t arg1, size_t arg2, thinkthen_question_author_v1 * arg3) {
    if (!native.thinkthen_result_member_author) abort();
    return native.thinkthen_result_member_author(arg0, arg1, arg2, arg3);
}
int thinkthen_result_observation(const thinkthen_result * arg0, size_t arg1, thinkthen_observation_v1 * arg2) {
    if (!native.thinkthen_result_observation) abort();
    return native.thinkthen_result_observation(arg0, arg1, arg2);
}
int thinkthen_result_observation_author(const thinkthen_result * arg0, size_t arg1, thinkthen_question_author_v1 * arg2) {
    if (!native.thinkthen_result_observation_author) abort();
    return native.thinkthen_result_observation_author(arg0, arg1, arg2);
}
int thinkthen_result_observation_details(const thinkthen_result * arg0, size_t arg1, thinkthen_details_v1 * arg2) {
    if (!native.thinkthen_result_observation_details) abort();
    return native.thinkthen_result_observation_details(arg0, arg1, arg2);
}
int thinkthen_result_question_author(const thinkthen_result * arg0, size_t arg1, thinkthen_question_author_v1 * arg2) {
    if (!native.thinkthen_result_question_author) abort();
    return native.thinkthen_result_question_author(arg0, arg1, arg2);
}
int thinkthen_result_rank(const thinkthen_result * arg0, size_t arg1, thinkthen_rank_view_v1 * arg2) {
    if (!native.thinkthen_result_rank) abort();
    return native.thinkthen_result_rank(arg0, arg1, arg2);
}
int thinkthen_result_rank_member(const thinkthen_result * arg0, size_t arg1, size_t arg2, thinkthen_rank_view_v1 * arg3) {
    if (!native.thinkthen_result_rank_member) abort();
    return native.thinkthen_result_rank_member(arg0, arg1, arg2, arg3);
}
int thinkthen_result_rank_member_count(const thinkthen_result * arg0, size_t arg1, size_t * arg2) {
    if (!native.thinkthen_result_rank_member_count) abort();
    return native.thinkthen_result_rank_member_count(arg0, arg1, arg2);
}
int thinkthen_result_rank_member_details(const thinkthen_result * arg0, size_t arg1, size_t arg2, thinkthen_details_v1 * arg3) {
    if (!native.thinkthen_result_rank_member_details) abort();
    return native.thinkthen_result_rank_member_details(arg0, arg1, arg2, arg3);
}
int thinkthen_result_recognition_task_v1(const thinkthen_result * arg0, size_t arg1, thinkthen_recognition_task_v1 * arg2) {
    if (!native.thinkthen_result_recognition_task_v1) abort();
    return native.thinkthen_result_recognition_task_v1(arg0, arg1, arg2);
}
int thinkthen_result_recognize(const thinkthen_result * arg0, size_t arg1, thinkthen_recognize_view_v1 * arg2) {
    if (!native.thinkthen_result_recognize) abort();
    return native.thinkthen_result_recognize(arg0, arg1, arg2);
}
int thinkthen_result_relate(const thinkthen_result * arg0, size_t arg1, thinkthen_relate_view_v1 * arg2) {
    if (!native.thinkthen_result_relate) abort();
    return native.thinkthen_result_relate(arg0, arg1, arg2);
}
int thinkthen_result_row(const thinkthen_result * arg0, size_t arg1, thinkthen_row_observation_v1 * arg2) {
    if (!native.thinkthen_result_row) abort();
    return native.thinkthen_result_row(arg0, arg1, arg2);
}
int thinkthen_result_score(const thinkthen_result * arg0, size_t arg1, thinkthen_score_view_v1 * arg2) {
    if (!native.thinkthen_result_score) abort();
    return native.thinkthen_result_score(arg0, arg1, arg2);
}
int thinkthen_result_source_recognition(const thinkthen_result * arg0, size_t arg1, thinkthen_source_recognition_v1 * arg2) {
    if (!native.thinkthen_result_source_recognition) abort();
    return native.thinkthen_result_source_recognition(arg0, arg1, arg2);
}
int thinkthen_result_source_relations(const thinkthen_result * arg0, size_t arg1, thinkthen_source_relations_v1 * arg2) {
    if (!native.thinkthen_result_source_relations) abort();
    return native.thinkthen_result_source_relations(arg0, arg1, arg2);
}
int thinkthen_result_summary(const thinkthen_result * arg0, thinkthen_summary_v1 * arg1) {
    if (!native.thinkthen_result_summary) abort();
    return native.thinkthen_result_summary(arg0, arg1);
}
int thinkthen_result_tag(const thinkthen_result * arg0, size_t arg1, thinkthen_tag_view_v1 * arg2) {
    if (!native.thinkthen_result_tag) abort();
    return native.thinkthen_result_tag(arg0, arg1, arg2);
}
int thinkthen_score_batch_start(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_batch ** arg4) {
    if (!native.thinkthen_score_batch_start) abort();
    return native.thinkthen_score_batch_start(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_score_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_score_complete) abort();
    return native.thinkthen_score_complete(arg0, arg1, arg2, arg3, arg4);
}
void thinkthen_session_cancel(thinkthen_session * arg0) {
    if (!native.thinkthen_session_cancel) abort();
    native.thinkthen_session_cancel(arg0);
}
const char * thinkthen_session_error_message(void) {
    if (!native.thinkthen_session_error_message) abort();
    return native.thinkthen_session_error_message();
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
int thinkthen_source_files(const thinkthen_engine * arg0, const thinkthen_source_spec_v1 * arg1, thinkthen_source ** arg2) {
    if (!native.thinkthen_source_files) abort();
    return native.thinkthen_source_files(arg0, arg1, arg2);
}
void thinkthen_source_free(thinkthen_source * arg0) {
    if (!native.thinkthen_source_free) abort();
    native.thinkthen_source_free(arg0);
}
int thinkthen_source_image_files(const thinkthen_engine * arg0, const thinkthen_source_spec_v1 * arg1, thinkthen_source ** arg2) {
    if (!native.thinkthen_source_image_files) abort();
    return native.thinkthen_source_image_files(arg0, arg1, arg2);
}
int thinkthen_source_records(const thinkthen_engine * arg0, const thinkthen_record_v1 * arg1, size_t arg2, thinkthen_source ** arg3) {
    if (!native.thinkthen_source_records) abort();
    return native.thinkthen_source_records(arg0, arg1, arg2, arg3);
}
int thinkthen_tag_batch_start(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_batch ** arg4) {
    if (!native.thinkthen_tag_batch_start) abort();
    return native.thinkthen_tag_batch_start(arg0, arg1, arg2, arg3, arg4);
}
int thinkthen_tag_complete(const thinkthen_engine * arg0, const thinkthen_question * arg1, const thinkthen_source * arg2, const thinkthen_controls_v1 * arg3, thinkthen_result ** arg4) {
    if (!native.thinkthen_tag_complete) abort();
    return native.thinkthen_tag_complete(arg0, arg1, arg2, arg3, arg4);
}
#pragma GCC visibility pop
