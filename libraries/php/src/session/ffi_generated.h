/* Generated from selected canonical C declarations; do not edit. */
typedef struct thinkthen_engine thinkthen_engine;
typedef struct thinkthen_session thinkthen_session;
typedef struct thinkthen_session_result thinkthen_session_result;
typedef struct thinkthen_complete_usage_persistence_v1 {
    uint32_t kind;
} thinkthen_complete_usage_persistence_v1;
typedef struct thinkthen_complete_utf8_v1 {
    const char *data;
    size_t len;
} thinkthen_complete_utf8_v1;
typedef struct thinkthen_named_request_v1 {
    struct thinkthen_complete_utf8_v1 function;
    struct thinkthen_complete_utf8_v1 question_json;
    struct thinkthen_complete_utf8_v1 input_json;
    struct thinkthen_complete_utf8_v1 options_json;
} thinkthen_named_request_v1;
int thinkthen_engine_finish_usage_status_v1(const struct thinkthen_engine *engine,
struct thinkthen_complete_usage_persistence_v1 *out_state,
struct thinkthen_complete_utf8_v1 *out_advice);
void thinkthen_engine_free(struct thinkthen_engine *engine);
struct thinkthen_engine *thinkthen_engine_new_with(const char *settings_json);
int thinkthen_engine_usage_persistence_v1(const struct thinkthen_engine *engine,
struct thinkthen_complete_usage_persistence_v1 *out_state,
struct thinkthen_complete_utf8_v1 *out_advice);
int thinkthen_error_code(const struct thinkthen_engine *engine);
const char *thinkthen_error_message(const struct thinkthen_engine *engine);
void thinkthen_session_cancel(struct thinkthen_session *session);
const char *thinkthen_session_error_message(void);
int thinkthen_session_finish(struct thinkthen_session *session,
const char *failure_json,
size_t failure_len);
void thinkthen_session_free(struct thinkthen_session *session);
int thinkthen_session_new_named_with_surface(const struct thinkthen_engine *engine,
const struct thinkthen_named_request_v1 *request,
struct thinkthen_complete_utf8_v1 surface,
struct thinkthen_session **out);
int thinkthen_session_next_packet(struct thinkthen_session *session,
uint32_t interest,
uint32_t *status,
struct thinkthen_session_result **out);
void thinkthen_session_result_free(struct thinkthen_session_result *result);
int thinkthen_session_result_json(const struct thinkthen_session_result *result,
const char **out,
size_t *out_len);
int thinkthen_session_try_push(struct thinkthen_session *session,
const char *descriptor_json,
size_t descriptor_len,
uint32_t *status);
int thinkthen_session_try_read(struct thinkthen_session *session,
uint32_t *status,
struct thinkthen_session_result **out);
int thinkthen_session_wait(const struct thinkthen_session *session,
uint32_t interest,
uint32_t timeout_ms,
uint32_t *out_ready);
