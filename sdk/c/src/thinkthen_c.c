#include "thinkthen_c.h"
#include <thinkthen.h>
#include <stdlib.h>
#include <string.h>


struct tt_client { thinkthen_client *native; };
struct tt_call { thinkthen_call *native; };
struct tt_error { json_t *value; };
struct tt_row {
    thinkthen_row native;
    json_t *fields;
    json_t *answer;
    struct tt_error error;
};

/* Allocation failure must still be reportable without another allocation. */
static struct tt_error bridge_error = {NULL};

static tt_text text(const char *value, size_t len) {
    return (tt_text){value, len};
}

static tt_text json_text(const json_t *value) {
    return text(json_string_value(value), json_string_length(value));
}

static void free_json_text(char *encoded) {
    json_malloc_t allocate;
    json_free_t release;
    json_get_alloc_funcs(&allocate, &release);
    if (encoded) release(encoded);
}

static bool fail_bridge(tt_error **error) {
    *error = &bridge_error;
    return false;
}

static bool result(int32_t status, thinkthen_buffer *native, tt_error **error) {
    *error = NULL;
    if (status != THINKTHEN_ERROR) return true;
    tt_error *owned = malloc(sizeof *owned);
    json_t *value = json_loadb((const char *)native->data, native->len, JSON_ALLOW_NUL, NULL);
    thinkthen_buffer_free(native);
    if (!owned || !value) {
        free(owned);
        json_decref(value);
        return fail_bridge(error);
    }
    owned->value = value;
    *error = owned;
    return false;
}

tt_client *tt_client_new(const json_t *settings, tt_error **error) {
    tt_client *client = malloc(sizeof *client);
    char *encoded = settings ? json_dumps(settings, JSON_COMPACT | JSON_ENCODE_ANY)
                             : NULL;
    *error = NULL;
    if (!client || (settings && !encoded)) {
        free(client);
        free_json_text(encoded);
        fail_bridge(error);
        return NULL;
    }
    const char *bytes = encoded ? encoded : "{}";
    thinkthen_buffer failure = {0};
    int32_t status = thinkthen_client_open((const uint8_t *)bytes, strlen(bytes),
                                          &client->native, &failure);
    free_json_text(encoded);
    if (!result(status, &failure, error)) {
        free(client);
        return NULL;
    }
    return client;
}

void tt_client_free(tt_client *client) {
    if (client) {
        thinkthen_client_free(client->native);
        free(client);
    }
}

static tt_call *open_call(const tt_client *client, const char *function,
                          const json_t *question, const json_t *options,
                          tt_error **error) {
    tt_call *call = malloc(sizeof *call);
    json_t *request = options
        ? json_pack("{s:s,s:O,s:O}", "function", function, "question", question,
                    "options", options)
        : json_pack("{s:s,s:O}", "function", function, "question", question);
    char *encoded = request ? json_dumps(request, JSON_COMPACT) : NULL;
    json_decref(request);
    *error = NULL;
    if (!call || !encoded) {
        free(call);
        free_json_text(encoded);
        fail_bridge(error);
        return NULL;
    }
    thinkthen_buffer failure = {0};
    int32_t status = thinkthen_call_open(client->native,
        (const uint8_t *)encoded, strlen(encoded),
        THINKTHEN_UTF8, &call->native, &failure);
    free_json_text(encoded);
    if (!result(status, &failure, error)) {
        free(call);
        return NULL;
    }
    return call;
}

tt_call *tt_decide(const tt_client *c, const json_t *q, const json_t *o,
                   tt_error **e) { return open_call(c, "decide", q, o, e); }
tt_call *tt_choose(const tt_client *c, const json_t *q, const json_t *o,
                   tt_error **e) { return open_call(c, "choose", q, o, e); }
tt_call *tt_tag(const tt_client *c, const json_t *q, const json_t *o,
                tt_error **e) { return open_call(c, "tag", q, o, e); }
tt_call *tt_score(const tt_client *c, const json_t *q, const json_t *o,
                  tt_error **e) { return open_call(c, "score", q, o, e); }
tt_call *tt_filter(const tt_client *c, const json_t *q, const json_t *o,
                   tt_error **e) { return open_call(c, "filter", q, o, e); }
tt_call *tt_rank(const tt_client *c, const json_t *q, const json_t *o,
                 tt_error **e) { return open_call(c, "rank", q, o, e); }
tt_call *tt_find(const tt_client *c, const json_t *q, const json_t *o,
                 tt_error **e) { return open_call(c, "find", q, o, e); }
tt_call *tt_annotate(const tt_client *c, const json_t *q, const json_t *o,
                     tt_error **e) { return open_call(c, "annotate", q, o, e); }
tt_call *tt_recognize(const tt_client *c, const json_t *q, const json_t *o,
                      tt_error **e) { return open_call(c, "recognize", q, o, e); }
tt_call *tt_relate(const tt_client *c, const json_t *q, const json_t *o,
                   tt_error **e) { return open_call(c, "relate", q, o, e); }

tt_event tt_next(tt_call *call, tt_row **row, tt_error **error) {
    *row = NULL;
    *error = NULL;
    thinkthen_row native = {0};
    thinkthen_buffer failure = {0};
    int32_t status = thinkthen_next(call->native, &native, &failure);
    if (!result(status, &failure, error)) return TT_ERROR;
    if (status == THINKTHEN_END) return TT_DONE;
    if (status == THINKTHEN_NEEDS_INPUT) return TT_INPUT;
    thinkthen_buffer columns = {0};
    status = thinkthen_row_columns(&native, &columns, &failure);
    if (!result(status, &failure, error)) {
        thinkthen_row_free(&native);
        return TT_ERROR;
    }
    tt_row *owned = malloc(sizeof *owned);
    json_t *fields = json_loadb((const char *)columns.data,
                                columns.len, JSON_ALLOW_NUL, NULL);
    thinkthen_buffer_free(&columns);
    json_t *raw_error = json_object_get(fields, "error");
    json_t *row_error = raw_error ? json_loadb(json_string_value(raw_error),
        json_string_length(raw_error), JSON_ALLOW_NUL, NULL) : NULL;
    if (!owned || !fields || (raw_error && !row_error)) {
        free(owned);
        json_decref(fields);
        json_decref(row_error);
        thinkthen_row_free(&native);
        fail_bridge(error);
        return TT_ERROR;
    }
    owned->native = native;
    owned->fields = fields;
    owned->error.value = row_error;
    tt_text answer = json_text(json_object_get(fields, "answer"));
    owned->answer = answer.len ? json_loadb(answer.data, answer.len,
        JSON_DECODE_ANY | JSON_ALLOW_NUL, NULL) : NULL;
    *row = owned;
    return TT_ROW;
}

bool tt_send_text(tt_call *call, tt_text record, tt_error **error) {
    thinkthen_buffer failure = {0};
    int32_t status = thinkthen_send_text(call->native,
        (const uint8_t *)record.data, record.len, &failure);
    return result(status, &failure, error);
}

bool tt_send_json(tt_call *call, const json_t *record, tt_error **error) {
    char *encoded = json_dumps(record, JSON_COMPACT | JSON_ENCODE_ANY);
    if (!encoded) return fail_bridge(error);
    bool sent = tt_send_raw_json(call, text(encoded, strlen(encoded)), error);
    free_json_text(encoded);
    return sent;
}

bool tt_send_raw_json(tt_call *call, tt_text record, tt_error **error) {
    thinkthen_buffer failure = {0};
    int32_t status = thinkthen_send_json(call->native,
        (const uint8_t *)record.data, record.len, &failure);
    return result(status, &failure, error);
}

void tt_end_input(tt_call *call) { thinkthen_end_input(call->native); }
void tt_cancel(tt_call *call) { thinkthen_cancel(call->native); }
void tt_call_free(tt_call *call) {
    if (call) {
        thinkthen_free(call->native);
        free(call);
    }
}

tt_status tt_row_status(const tt_row *row) {
    const char *status = json_string_value(json_object_get(row->fields, "status"));
    if (strcmp(status, "answered") == 0) return TT_ANSWERED;
    if (strcmp(status, "unsure") == 0) return TT_UNSURE;
    return TT_FAILED;
}
tt_text tt_row_id(const tt_row *row) { return json_text(json_object_get(row->fields, "id")); }
tt_text tt_row_answer(const tt_row *row) { return json_text(json_object_get(row->fields, "answer")); }
bool tt_row_probability(const tt_row *row, double *out) {
    json_t *value = json_object_get(row->fields, "p");
    if (!value) return false;
    *out = json_number_value(value);
    return true;
}
bool tt_row_verdict(const tt_row *row, bool *out) {
    if (!json_is_boolean(row->answer)) return false;
    *out = json_is_true(row->answer);
    return true;
}
bool tt_row_score(const tt_row *row, double *out) {
    if (!json_is_number(row->answer)) return false;
    *out = json_number_value(row->answer);
    return true;
}
static const json_t *answer_item(const tt_row *row, size_t index) {
    return json_is_array(row->answer) ? json_array_get(row->answer, index)
        : index == 0 ? row->answer : NULL;
}
bool tt_row_label(const tt_row *row, size_t index, tt_text *out) {
    const json_t *value = answer_item(row, index);
    if (!json_is_string(value)) return false;
    *out = json_text(value);
    return true;
}
bool tt_row_span(const tt_row *row, size_t index, tt_span *out) {
    const json_t *value = answer_item(row, index);
    const json_t *label = json_object_get(value, "text");
    const json_t *start = json_object_get(value, "start");
    const json_t *end = json_object_get(value, "end");
    if (!json_is_string(label) || !json_is_integer(start) || !json_is_integer(end))
        return false;
    *out = (tt_span){json_text(label), (size_t)json_integer_value(start),
                    (size_t)json_integer_value(end)};
    return true;
}
const tt_error *tt_row_error(const tt_row *row) {
    return row->error.value ? &row->error : NULL;
}
tt_text tt_row_detail(const tt_row *row) {
    return text((const char *)row->native.detail.data, row->native.detail.len);
}
size_t tt_row_position(const tt_row *row) { return row->native.position; }
bool tt_row_kept(const tt_row *row) { return row->native.kept != 0; }
void tt_row_free(tt_row *row) {
    if (row) {
        json_decref(row->error.value);
        json_decref(row->fields);
        json_decref(row->answer);
        thinkthen_row_free(&row->native);
        free(row);
    }
}

tt_text tt_error_code(const tt_error *error) {
    return error == &bridge_error ? text("local.io", sizeof "local.io" - 1)
        : json_text(json_object_get(error->value, "code"));
}
tt_text tt_error_message(const tt_error *error) {
    static const char message[] = "the C binding could not allocate or decode JSON";
    return error == &bridge_error ? text(message, sizeof message - 1)
        : json_text(json_object_get(error->value, "message"));
}
void tt_error_free(tt_error *error) {
    if (error && error != &bridge_error) {
        json_decref(error->value);
        free(error);
    }
}
