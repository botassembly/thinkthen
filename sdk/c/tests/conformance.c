/* Drive shared cases only through the installed public C SDK. */
#include <thinkthen_c.h>
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rows.h"

static json_t *failure(const tt_error *error) {
    assert(tt_error_message(error).len > 0);
    return json_pack("{s:o}", "error", string(tt_error_code(error)));
}

static void strip_message(json_t *value) {
    json_t *error = json_object_get(value, "error");
    if (error) json_object_del(error, "message");
}

static json_t *project(tt_row *row, const char *name) {
    json_t *borrowed = row_json(row);
    json_t *value = json_deep_copy(borrowed);
    json_decref(borrowed);
    assert(value);
    strip_message(value);
    json_t *answer = json_object_get(value, "answer");
    if (strcmp(name, "annotate") == 0 && json_is_object(answer)) {
        const char *key;
        json_t *entry;
        json_object_foreach(answer, key, entry) {
            (void)key;
            strip_message(entry);
        }
    }
    return value;
}

static json_t *run(json_t *test, const char *url) {
    json_t *settings = json_deep_copy(json_object_get(test, "client"));
    if (!settings) settings = json_object();
    assert(json_object_set_new(settings, "url", json_string(url)) == 0);
    tt_error *error = NULL;
    tt_client *client = tt_client_new(settings, &error);
    json_decref(settings);
    if (!client) {
        assert(!json_object_get(test, "expected_rows"));
        json_t *value = failure(error);
        tt_error_free(error);
        return value;
    }
    json_t *request = json_object_get(test, "request");
    const char *name = json_string_value(json_object_get(request, "function"));
    tt_call *call = named(name)(client, json_object_get(request, "question"),
        json_object_get(request, "options"), &error);
    tt_client_free(client);
    if (!call) {
        assert(!json_object_get(test, "expected_rows"));
        json_t *value = failure(error);
        tt_error_free(error);
        return value;
    }
    bool cancelled = json_is_true(json_object_get(test, "cancelled"));
    if (cancelled) tt_cancel(call);
    json_t *records = json_object_get(request, "records");
    json_t *result = json_array();
    size_t sent = 0;
    for (;;) {
        tt_row *row = NULL;
        tt_event event = tt_next(call, &row, &error);
        if (event == TT_ROW) {
            assert(tt_row_position(row) >= 1);
            if (strcmp(name, "filter") == 0)
                assert(tt_row_kept(row) == equal_text(tt_row_answer(row), "true"));
            assert(json_array_append_new(result, project(row, name)) == 0);
            tt_row_free(row);
        } else if (event == TT_INPUT) {
            if (sent == json_array_size(records)) tt_end_input(call);
            else {
                json_t *record = json_array_get(records, sent++);
                bool ok = json_is_string(record)
                    ? tt_send_text(call, (tt_text){json_string_value(record),
                                                  json_string_length(record)}, &error)
                    : tt_send_json(call, record, &error);
                if (!ok) {
                    assert(!json_object_get(test, "expected_rows"));
                    json_decref(result);
                    result = failure(error);
                    tt_error_free(error);
                    break;
                }
            }
        } else {
            json_t *expected_rows = json_object_get(test, "expected_rows");
            if (expected_rows) assert(json_equal(result, expected_rows));
            if (event == TT_ERROR) {
                if (cancelled) {
                    assert(equal_text(tt_error_code(error), "local.cancelled"));
                    assert(json_array_size(result) == 0 && sent == 0);
                }
                json_decref(result);
                result = failure(error);
                tt_error_free(error);
            } else assert(event == TT_DONE);
            break;
        }
    }
    tt_call_free(call);
    return result;
}

static void options(const char *path) {
    json_t *cases = json_load_file(path, 0, NULL);
    assert(json_is_array(cases));
    size_t index;
    json_t *test;
    json_array_foreach(cases, index, test) {
        json_t *actual = run(test, "http://127.0.0.1");
        json_t *equivalent = json_object_get(test, "equivalent_request");
        if (equivalent) {
            json_t *comparison = json_pack("{s:O,s:O}", "client", json_object_get(test, "client"), "request", equivalent);
            json_t *expected = run(comparison, "http://127.0.0.1");
            assert(json_is_array(actual));
            assert(json_array_size(actual) == json_array_size(json_object_get(equivalent, "records")));
            assert(json_equal(actual, expected));
            json_decref(expected);
            json_decref(comparison);
        } else {
            assert(json_equal(json_object_get(actual, "error"), json_object_get(test, "expected_error")));
        }
        json_decref(actual);
    }
    json_decref(cases);
}
int main(int argc, char **argv) {
    assert(argc == 3);
    if (strcmp(argv[1], "options") == 0) { options(argv[2]); return 0; }
    json_t *test = json_load_file(argv[1], JSON_ALLOW_NUL, NULL);
    assert(test);
    json_t *result = run(test, argv[2]);
    assert(result);
    assert(json_dumpf(result, stdout, JSON_COMPACT) == 0);
    putchar('\n');
    json_decref(result);
    json_decref(test);
    return 0;
}
