#include <thinkthen_c.h>
#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "rows.h"

static tt_client *client(const json_t *settings) {
    tt_error *error = NULL;
    tt_client *value = tt_client_new(settings, &error);
    assert(value && !error);
    return value;
}

static void typed_answer(const tt_row *row, const char *function) {
    json_t *answer = parse(tt_row_answer(row));
    if (json_is_boolean(answer)) {
        bool value = !json_is_true(answer);
        assert(tt_row_verdict(row, &value) && value == json_is_true(answer));
    } else if (strcmp(function, "score") == 0) {
        double value = -1;
        assert(tt_row_score(row, &value) && value == json_number_value(answer));
    } else if (strcmp(function, "choose") == 0 || strcmp(function, "tag") == 0) {
        size_t count = json_is_string(answer) ? 1 : json_array_size(answer);
        for (size_t i = 0; i < count; ++i) {
            tt_text label;
            const json_t *expected = json_is_string(answer) ? answer : json_array_get(answer, i);
            assert(tt_row_label(row, i, &label));
            assert(label.len == json_string_length(expected));
            assert(memcmp(label.data, json_string_value(expected), label.len) == 0);
        }
        tt_text absent = {"unchanged", 9};
        assert(!tt_row_label(row, count, &absent) && absent.len == 9);
    } else if (strcmp(function, "find") == 0 || strcmp(function, "recognize") == 0) {
        size_t count = json_is_array(answer) ? json_array_size(answer) : 1;
        for (size_t i = 0; i < count; ++i) {
            tt_span span;
            const json_t *expected = json_is_array(answer) ? json_array_get(answer, i) : answer;
            assert(tt_row_span(row, i, &span));
            assert(equal_text(span.text, json_string_value(json_object_get(expected, "text"))));
            assert(span.start == (size_t)json_integer_value(json_object_get(expected, "start")));
            assert(span.end == (size_t)json_integer_value(json_object_get(expected, "end")));
        }
    }
    json_decref(answer);
}

static void construction(const char *path) {
    json_t *cases = json_load_file(path, 0, NULL);
    assert(json_is_array(cases) && json_array_size(cases) == 10);
    size_t index;
    json_t *test;
    json_array_foreach(cases, index, test) {
        json_t *request = json_object_get(test, "request");
        json_t *records = json_object_get(request, "records");
        json_t *expected = json_object_get(test, "expected");
        const char *name = json_string_value(json_object_get(request, "function"));
        tt_client *owner = client(json_object_get(test, "client"));
        tt_error *error = NULL;
        tt_call *call = named(name)(owner, json_object_get(request, "question"),
            json_object_get(request, "options"), &error);
        assert(call && !error);
        tt_client_free(owner); /* Calls retain their engine. */
        json_t *actual = json_array();
        tt_row *retained = NULL;
        size_t sent = 0;
        for (;;) {
            tt_row *row = NULL;
            tt_event event = tt_next(call, &row, &error);
            assert(!error);
            if (event == TT_INPUT) {
                assert(!row);
                if (sent == json_array_size(records)) tt_end_input(call);
                else {
                    json_t *record = json_array_get(records, sent++);
                    bool ok = json_is_string(record)
                        ? tt_send_text(call, (tt_text){json_string_value(record),
                                                      json_string_length(record)}, &error)
                        : tt_send_json(call, record, &error);
                    assert(ok && !error);
                }
            } else if (event == TT_ROW) {
                assert(row && tt_row_position(row) >= 1);
                typed_answer(row, name);
                if (strcmp(name, "filter") == 0)
                    assert(tt_row_kept(row) == equal_text(tt_row_answer(row), "true"));
                if (retained) {
                    assert(json_array_append_new(actual, row_json(retained)) == 0);
                    tt_row_free(retained);
                }
                retained = row;
            } else {
                assert(event == TT_DONE && !row);
                break;
            }
        }
        tt_call_free(call);
        if (retained) {
            assert(json_array_append_new(actual, row_json(retained)) == 0);
            tt_row_free(retained); /* The row survives its call. */
        }
        if (!json_equal(actual, expected)) {
            char *got = json_dumps(actual, JSON_INDENT(2));
            fprintf(stderr, "%s rows differ: %s\n", name, got);
            free(got);
            abort();
        }
        assert(json_array_size(actual) == json_array_size(expected));
        json_decref(actual);
    }
    json_decref(cases);
    puts("all ten shared stub cases match");
}

static json_t *question(void) {
    return json_pack("{s:s,s:s}", "type", "inline", "value", "Ready?");
}

static void *refuse_allocation(size_t size) {
    (void)size;
    return NULL;
}

static void boundaries(void) {
    json_t *settings = json_pack("{s:s}", "adapter", "stub");
    tt_client *owner = client(settings);
    json_decref(settings);
    json_t *q = question();
    json_t *options = json_pack("{s:s}", "threshold", "broken");
    tt_error *error = NULL;
    tt_call *call = tt_decide(owner, q, options, &error);
    assert(!call && error && equal_text(tt_error_code(error), "usage.threshold"));
    tt_client_free(owner);
    assert(tt_error_message(error).len > 0); /* Error owns its text. */
    tt_error_free(error);
    json_decref(options);

    settings = json_pack("{s:s}", "adapter", "stub");
    owner = client(settings);
    json_decref(settings);
    call = tt_decide(owner, q, NULL, &error);
    assert(call && !error);
    assert(!tt_send_text(call, (tt_text){"early", 5}, &error));
    assert(error && equal_text(tt_error_code(error), "usage.record"));
    tt_error_free(error);
    tt_row *row = NULL;
    assert(tt_next(call, &row, &error) == TT_INPUT && !row && !error);
    /* A structured identifier and quoted UTF-8 must cross without stringifying
       the record or treating its body as an instruction to the C wrapper. */
    json_t *record = json_pack("{s:{s:i},s:s}", "id", "slot", 9,
                               "body", "raw \"quoted\" 😀\n");
    assert(tt_send_json(call, record, &error) && !error);
    json_decref(record);
    tt_end_input(call);
    assert(tt_next(call, &row, &error) == TT_ROW && row && !error);
    json_t *id = json_pack("{s:i}", "slot", 9);
    json_t *actual_id = parse(tt_row_id(row));
    assert(json_equal(actual_id, id));
    json_decref(actual_id);
    assert(tt_row_status(row) == TT_ANSWERED && !tt_row_error(row));
    json_decref(id);
    tt_row_free(row);
    assert(tt_next(call, &row, &error) == TT_DONE && !row && !error);
    tt_call_free(call);

    /* Counted raw UTF-8 includes embedded NUL; an unsure answer remains an
       explicit JSON null, distinct from the absent answer on a failed row. */
    options = json_pack("{s:s}", "threshold", "0.01:1");
    call = tt_decide(owner, q, options, &error);
    json_decref(options);
    assert(call && !error);
    assert(tt_next(call, &row, &error) == TT_INPUT && !row && !error);
    const char raw[] = "quoted \"😀\"\n\0tail";
    assert(tt_send_text(call, (tt_text){raw, sizeof raw - 1}, &error) && !error);
    tt_end_input(call);
    assert(tt_next(call, &row, &error) == TT_ROW && row && !error);
    assert(tt_row_status(row) == TT_UNSURE && equal_text(tt_row_answer(row), "null"));
    double probability;
    assert(tt_row_probability(row, &probability));
    tt_row_free(row);
    assert(tt_next(call, &row, &error) == TT_DONE && !row && !error);
    tt_call_free(call);

    /* Jansson exposes a real host allocator. A bridge encoding failure keeps
       its local code and needs no further allocation to report the failure. */
    json_malloc_t allocate;
    json_free_t release;
    json_get_alloc_funcs(&allocate, &release);
    json_set_alloc_funcs(refuse_allocation, release);
    call = tt_decide(owner, q, NULL, &error);
    json_set_alloc_funcs(allocate, release);
    assert(!call && error && equal_text(tt_error_code(error), "local.io"));
    assert(tt_error_message(error).len > 0);
    tt_error_free(error);

    settings = json_pack("{s:s}", "adapter", "stub");
    json_set_alloc_funcs(refuse_allocation, release);
    tt_client *failed_client = tt_client_new(settings, &error);
    json_set_alloc_funcs(allocate, release);
    assert(!failed_client && error && equal_text(tt_error_code(error), "local.io"));
    tt_error_free(error);
    json_decref(settings);

    settings = json_pack("{s:s,s:i}", "adapter", "stub", "batch", 0);
    failed_client = tt_client_new(settings, &error);
    assert(!failed_client && error && equal_text(tt_error_code(error), "usage.batch"));
    tt_error_free(error);
    json_decref(settings);

    call = tt_decide(owner, q, NULL, &error);
    assert(call && !error);
    json_set_alloc_funcs(refuse_allocation, release);
    bool sent = tt_send_text(call, (tt_text){"early", 5}, &error);
    json_set_alloc_funcs(allocate, release);
    assert(!sent && error && equal_text(tt_error_code(error), "local.io"));
    tt_error_free(error);
    tt_call_free(call);

    call = tt_decide(owner, q, NULL, &error);
    assert(call && !error);
    assert(tt_next(call, &row, &error) == TT_INPUT && !row && !error);
    assert(tt_send_text(call, (tt_text){"evidence", 8}, &error) && !error);
    tt_end_input(call);
    json_set_alloc_funcs(refuse_allocation, release);
    tt_event event = tt_next(call, &row, &error);
    json_set_alloc_funcs(allocate, release);
    assert(event == TT_ERROR && !row && error
           && equal_text(tt_error_code(error), "local.io"));
    tt_error_free(error);
    tt_call_free(call);

    /* Empty input and NULL cleanup need no synthetic record. */
    call = tt_decide(owner, q, NULL, &error);
    assert(call && !error);
    tt_end_input(call);
    tt_end_input(call);
    assert(tt_next(call, &row, &error) == TT_DONE && !row && !error);
    tt_call_free(call);
    tt_client_free(owner);
    json_decref(q);
    tt_row_free(NULL);
    tt_error_free(NULL);
    tt_call_free(NULL);
    tt_client_free(NULL);
    puts("C ownership and translation boundaries pass");
}

/* Counted raw records retain unsigned integers and nested NUL keys without
 * decoding them through Jansson. The raw identifier survives all owners. */
static void lossless_identifier(void) {
    json_t *settings = json_pack("{s:s}", "adapter", "stub");
    tt_client *owner = client(settings);
    json_decref(settings);
    json_t *q = question();
    tt_error *error = NULL;
    tt_call *calls[2];
    for (size_t i = 0; i < 2; ++i) {
        calls[i] = tt_decide(owner, q, NULL, &error);
        assert(calls[i] && !error);
    }
    json_decref(q);
    tt_client_free(owner);
    for (size_t i = 0; i < 2; ++i) {
        tt_call *call = calls[i];
        tt_row *row = NULL;
        assert(tt_next(call, &row, &error) == TT_INPUT && !row && !error);
        if (i == 0) {
            json_t *nested = json_object();
            const char key[] = "a\0b";
            assert(json_object_setn_new(nested, key, sizeof key - 1, json_true()) == 0);
            json_t *record = json_pack("{s:{s:o},s:s}", "id", "nested", nested, "body", "evidence");
            assert(record && tt_send_json(call, record, &error) && !error);
            json_decref(record);
        } else {
            const char raw[] = "{\"id\":{\"nested\":{\"a\\u0000b\":18446744073709551615}},\"body\":\"evidence\"}";
            assert(tt_send_raw_json(call, (tt_text){raw, sizeof raw - 1}, &error) && !error);
        }
        tt_end_input(call);
        assert(tt_next(call, &row, &error) == TT_ROW && row && !error);
        tt_call_free(call);
        assert(tt_row_status(row) == TT_ANSWERED && !tt_row_error(row));
        assert(equal_text(tt_row_id(row), i == 0 ? "{\"nested\":{\"a\\u0000b\":true}}"
                                               : "{\"nested\":{\"a\\u0000b\":18446744073709551615}}"));
        assert(tt_row_answer(row).len && tt_row_detail(row).len);
        tt_row_free(row);
    }
    puts("Jansson and raw JSON identifiers survive their calls and client");
}

struct receiving { tt_call *call; tt_event event; tt_error *error; tt_row *row; };
static void *receive(void *argument) {
    struct receiving *state = argument;
    for (;;) {
        state->event = tt_next(state->call, &state->row, &state->error);
        if (state->event != TT_ROW) break;
        assert(tt_row_status(state->row) == TT_FAILED);
        tt_row_free(state->row);
        state->row = NULL;
    }
    return NULL;
}

static void backend(const char *url, bool cancel) {
    json_t *settings = json_pack("{s:s,s:i}", "url", url, "batch", 1);
    tt_client *owner = client(settings);
    json_decref(settings);
    json_t *q = question();
    tt_error *error = NULL;
    tt_call *call = tt_decide(owner, q, NULL, &error);
    assert(call && !error);
    json_decref(q);
    tt_row *row = NULL;
    assert(tt_next(call, &row, &error) == TT_INPUT);
    assert(tt_send_text(call, (tt_text){"evidence", 8}, &error));
    tt_end_input(call);
    if (cancel) {
        struct receiving state = {.call = call};
        pthread_t worker;
        assert(pthread_create(&worker, NULL, receive, &state) == 0);
        /* Parent sends this byte only after it has read the backend request. */
        assert(getchar() == 'c');
        tt_cancel(call);
        assert(pthread_join(worker, NULL) == 0);
        assert(state.event == TT_ERROR && !state.row && state.error);
        assert(equal_text(tt_error_code(state.error), "local.cancelled"));
        tt_error_free(state.error);
        puts("held request cancelled");
    } else {
        assert(tt_next(call, &row, &error) == TT_ROW && row && !error);
        assert(tt_row_status(row) == TT_FAILED && !tt_row_answer(row).len);
        assert(equal_text(tt_error_code(tt_row_error(row)), "backend.transport"));
        assert(tt_row_detail(row).len == 0);
        tt_row_free(row);
        assert(tt_next(call, &row, &error) == TT_ERROR && !row && error);
        assert(equal_text(tt_error_code(error), "backend.transport"));
        tt_error_free(error);
        puts("backend error preserved");
    }
    tt_call_free(call);
    tt_client_free(owner);
}

static void nul_question_name(const char *url) {
    json_t *settings = json_pack("{s:s}", "url", url);
    tt_client *owner = client(settings);
    json_decref(settings);
    json_t *questions = json_object();
    const char name[] = "bad\0name";
    json_t *definition = json_pack("{s:s}", "decide", "Ready?");
    assert(json_object_setn_new(questions, name, sizeof name - 1, definition) == 0);
    json_t *q = json_pack("{s:s,s:{s:o}}", "type", "inline", "value", "questions", questions);
    tt_error *error = NULL;
    tt_call *call = tt_annotate(owner, q, NULL, &error);
    assert(!call && error);
    assert(equal_text(tt_error_code(error), "usage.name"));
    puts("{\"error\":\"usage.name\"}");
    tt_error_free(error);
    json_decref(q);
    tt_client_free(owner);
}

int main(int argc, char **argv) {
    assert(argc >= 2);
    if (strcmp(argv[1], "nul-key") == 0) {
        assert(argc == 3);
        nul_question_name(argv[2]);
    } else if (strcmp(argv[1], "backend") == 0 || strcmp(argv[1], "cancel") == 0) {
        assert(argc == 3);
        backend(argv[2], strcmp(argv[1], "cancel") == 0);
    } else {
        construction(argv[1]);
        boundaries();
        lossless_identifier();
    }
    return EXIT_SUCCESS;
}
