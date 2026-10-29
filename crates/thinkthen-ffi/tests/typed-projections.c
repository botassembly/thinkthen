/* This consumer has no JSON parser. It reads every answer and detail through typed views. */
#include "thinkthen.h"
#include <assert.h>
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

static void ok(int32_t result, thinkthen_error *error) { assert(result == THINKTHEN_OK); assert(error->code == 0); }
static void text(thinkthen_text value) {
    putchar('"');
    for (size_t i = 0; i < value.len; i++) {
        unsigned char c = value.data[i];
        if (c == '"' || c == '\\') printf("\\%c", c);
        else if (c < 32) printf("\\u%04x", c);
        else putchar(c);
    }
    putchar('"');
}
static void number(double value) {
    assert(isfinite(value)); char buffer[64];
    int size = snprintf(buffer, sizeof(buffer), "%.17g", value); assert(size > 0 && (size_t)size < sizeof(buffer));
    fputs(buffer, stdout); if (!strchr(buffer, '.') && !strchr(buffer, 'e') && !strchr(buffer, 'E')) fputs(".0", stdout);
}
static void label(thinkthen_probability value) { text(value.label); putchar(':'); number(value.probability); }
static void span(thinkthen_span value) {
    if (!value.present) { fputs("null", stdout); return; }
    fputs("{\"text\":", stdout); text(value.text); printf(",\"start\":%" PRIu64 ",\"end\":%" PRIu64 "}", value.start, value.end);
}
static void endpoint(thinkthen_endpoint value) {
    fputs("{\"text\":", stdout); text(value.text); printf(",\"start\":%" PRIu64 ",\"end\":%" PRIu64 ",\"kind\":", value.start, value.end); text(value.kind); putchar('}');
}
static void relation(thinkthen_relation value) {
    fputs("{\"source\":", stdout); endpoint(value.source); fputs(",\"target\":", stdout); endpoint(value.target); fputs(",\"relation\":", stdout); text(value.relation);
    fputs(",\"probability\":", stdout); number(value.probability); if (value.either) fputs(",\"either\":true", stdout); putchar('}');
}
static void names(thinkthen_value *value, thinkthen_error *error) {
    size_t count = 0;
    ok(thinkthen_value_name_count(value, &count, error), error); putchar('[');
    for (size_t i = 0; i < count; i++) {
        if (i) {
            putchar(',');
        }
        thinkthen_name item = {0}; ok(thinkthen_value_name_at(value, i, &item, error), error);
        fputs("{\"text\":", stdout); text(item.text); printf(",\"start\":%" PRIu64 ",\"end\":%" PRIu64 ",\"kind\":", item.start, item.end); text(item.kind); fputs(",\"probability\":", stdout); number(item.probability); putchar('}');
    } putchar(']');
}
static void body(thinkthen_value *, thinkthen_error *);
static void answer(thinkthen_value *value, uint32_t function, thinkthen_error *error) {
    size_t count = 0;
    switch (function) {
    case THINKTHEN_FUNCTION_DECIDE: case THINKTHEN_FUNCTION_FILTER: {
        uint8_t value_bool = 0; ok(thinkthen_value_bool(value, &value_bool, error), error); fputs(value_bool ? "true" : "false", stdout); break;
    }
    case THINKTHEN_FUNCTION_CHOOSE: {
        thinkthen_text choice = {0}; ok(thinkthen_value_choice(value, &choice, error), error); text(choice); break;
    }
    case THINKTHEN_FUNCTION_SCORE: {
        double result = 0; ok(thinkthen_value_number(value, &result, error), error); number(result); break;
    }
    case THINKTHEN_FUNCTION_RANK: {
        uint64_t rank = 0; ok(thinkthen_value_rank(value, &rank, error), error); printf("%" PRIu64, rank); break;
    }
    case THINKTHEN_FUNCTION_TAG:
        ok(thinkthen_value_tag_count(value, &count, error), error); putchar('[');
        for (size_t i = 0; i < count; i++) { if (i) putchar(','); thinkthen_text item = {0}; ok(thinkthen_value_tag_at(value, i, &item, error), error); text(item); } putchar(']'); break;
    case THINKTHEN_FUNCTION_FIND: {
        thinkthen_span found = {0}; ok(thinkthen_value_found(value, &found, error), error); span(found); break;
    }
    case THINKTHEN_FUNCTION_RECOGNIZE:
        names(value, error); break;
    case THINKTHEN_FUNCTION_RELATE:
        ok(thinkthen_value_relation_count(value, &count, error), error); putchar('[');
        for (size_t i = 0; i < count; i++) { if (i) putchar(','); thinkthen_relation item = {0}; ok(thinkthen_value_relation_at(value, i, &item, error), error); relation(item); } putchar(']'); break;
    case THINKTHEN_FUNCTION_ANNOTATE:
        ok(thinkthen_value_annotation_count(value, &count, error), error); putchar('{');
        for (size_t i = 0; i < count; i++) { if (i) putchar(','); thinkthen_text name = {0}; thinkthen_value *child = NULL; ok(thinkthen_value_annotation_at(value, i, &name, &child, error), error); text(name); fputs(":{", stdout); body(child, error); putchar('}'); thinkthen_value_free(child); } putchar('}'); break;
    default: assert(0);
    }
}
static void detail(thinkthen_value *value, uint32_t function, thinkthen_error *error) {
    size_t count = 0;
    switch (function) {
    case THINKTHEN_FUNCTION_CHOOSE: case THINKTHEN_FUNCTION_TAG:
        ok(thinkthen_value_label_count(value, &count, error), error); fputs("{\"labels\":{", stdout);
        for (size_t i = 0; i < count; i++) { if (i) putchar(','); thinkthen_probability item = {0}; ok(thinkthen_value_label_at(value, i, &item, error), error); label(item); } fputs("}}", stdout); break;
    case THINKTHEN_FUNCTION_SCORE: case THINKTHEN_FUNCTION_RANK:
        ok(thinkthen_value_level_count(value, &count, error), error); fputs("{\"levels\":[", stdout);
        for (size_t i = 0; i < count; i++) { if (i) putchar(','); double item = 0; ok(thinkthen_value_level_at(value, i, &item, error), error); number(item); } fputs("]}", stdout); break;
    case THINKTHEN_FUNCTION_RECOGNIZE:
        ok(thinkthen_value_piece_count(value, &count, error), error); fputs("{\"pieces\":[", stdout);
        for (size_t i = 0; i < count; i++) {
            if (i) {
                putchar(',');
            }
            thinkthen_span piece = {0}; ok(thinkthen_value_piece_at(value, i, &piece, error), error); fputs("{\"text\":", stdout); text(piece.text); printf(",\"start\":%" PRIu64 ",\"end\":%" PRIu64 ",\"tags\":{", piece.start, piece.end);
            size_t tags = 0; ok(thinkthen_value_piece_tag_count(value, i, &tags, error), error);
            for (size_t j = 0; j < tags; j++) { if (j) putchar(','); thinkthen_probability item = {0}; ok(thinkthen_value_piece_tag_at(value, i, j, &item, error), error); label(item); } fputs("}}", stdout);
        }
        fputs("],\"names\":[", stdout); ok(thinkthen_value_kind_count(value, &count, error), error);
        for (size_t i = 0; i < count; i++) {
            if (i) {
                putchar(',');
            }
            thinkthen_span name = {0}; ok(thinkthen_value_kind_span(value, i, &name, error), error); printf("{\"start\":%" PRIu64 ",\"end\":%" PRIu64 ",\"kinds\":{", name.start, name.end);
            size_t kinds = 0; ok(thinkthen_value_kind_probability_count(value, i, &kinds, error), error);
            for (size_t j = 0; j < kinds; j++) { if (j) putchar(','); thinkthen_probability item = {0}; ok(thinkthen_value_kind_probability_at(value, i, j, &item, error), error); label(item); } fputs("}}", stdout);
        } fputs("]}", stdout); break;
    case THINKTHEN_FUNCTION_RELATE:
        fputs("{\"names\":", stdout); names(value, error);
        ok(thinkthen_value_pair_count(value, &count, error), error); fputs(",\"pairs\":[", stdout);
        for (size_t i = 0; i < count; i++) { if (i) putchar(','); thinkthen_relation item = {0}; ok(thinkthen_value_pair_at(value, i, &item, error), error); relation(item); } fputs("]}", stdout); break;
    default: assert(0);
    }
}
static void body(thinkthen_value *value, thinkthen_error *error) {
    uint32_t function = 0, status = 0;
    ok(thinkthen_value_function(value, &function, error), error); ok(thinkthen_value_status(value, &status, error), error);
    printf("\"function\":%u,\"status\":%u", function, status);
    if (status == THINKTHEN_STATUS_FAILED) {
        thinkthen_error failure = {0}; failure.struct_size = sizeof(failure); ok(thinkthen_value_error(value, &failure, error), error);
        fputs(",\"error\":{\"code\":\"", stdout); fputs(thinkthen_code_name(failure.code), stdout); putchar('"'); if (failure.http_status) printf(",\"http_status\":%u", failure.http_status); putchar('}'); thinkthen_error_free(&failure); return;
    }
    if (status == THINKTHEN_STATUS_ANSWERED) { fputs(",\"answer\":", stdout); answer(value, function, error); }
    double probability = NAN; ok(thinkthen_value_probability(value, &probability, error), error); if (!isnan(probability)) { fputs(",\"probability\":", stdout); number(probability); }
    uint8_t has_detail = 0; ok(thinkthen_value_has_detail(value, &has_detail, error), error); if (has_detail) { fputs(",\"detail\":", stdout); detail(value, function, error); }
}
int main(int argc, char **argv) {
    assert(argc == 3 || (argc == 4 && strcmp(argv[3], "cancelled") == 0)); assert(thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()));
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_client *client = NULL;
    int32_t opened = thinkthen_client_open_json((const uint8_t *)argv[1], strlen(argv[1]), &client, &error);
    thinkthen_call *call = NULL;
    if (opened == THINKTHEN_OK) opened = thinkthen_call_open_json(client, (const uint8_t *)argv[2], strlen(argv[2]), THINKTHEN_OFFSET_CODE_POINTS, &call, &error);
    thinkthen_client_free(client);
    if (opened == THINKTHEN_ERROR) {
        assert(call == NULL);
        printf("{\"error\":\"%s\"}\n", thinkthen_code_name(error.code));
        thinkthen_error_free(&error);
        return 0;
    }
    ok(opened, &error);
    if (argc == 4) thinkthen_call_cancel(call);
    thinkthen_call_end_input(call);
    for (;;) {
        thinkthen_row row = {0}; row.struct_size = sizeof(row);
        int32_t result = thinkthen_call_next(call, &row, &error);
        if (result == THINKTHEN_END) break;
        if (result == THINKTHEN_ERROR) {
            assert(row.answer.data == NULL && row.detail.data == NULL);
            printf("{\"error\":\"%s\"}\n", thinkthen_code_name(error.code));
            assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END);
            break;
        }
        ok(result, &error);
        thinkthen_value *value = NULL; ok(thinkthen_row_values(&row, &value, &error), &error); uint64_t position = row.position; thinkthen_row_free(&row);
        putchar('{'); body(value, &error); printf(",\"position\":%" PRIu64, position);
        thinkthen_id id = {0}; ok(thinkthen_value_id(value, &id, &error), &error);
        if (id.kind != THINKTHEN_ID_ABSENT) { fputs(",\"id\":", stdout); if (id.kind == THINKTHEN_ID_STRING) text(id.text); else if (id.kind == THINKTHEN_ID_SIGNED) printf("%" PRId64, id.signed_value); else { assert(id.kind == THINKTHEN_ID_UNSIGNED); printf("%" PRIu64, id.unsigned_value); } }
        thinkthen_source source = {0}; ok(thinkthen_value_source(value, &source, &error), &error); printf(",\"source\":{\"backend\":%u,\"model\":", source.backend); text(source.model); fputs(",\"question_identity\":", stdout); text(source.question_identity);
        thinkthen_text name = {0}; ok(thinkthen_value_source_name(value, &name, &error), &error);
        if (name.data) { fputs(",\"name\":", stdout); text(name); }
        fputs("}}\n", stdout);
        thinkthen_value_free(value);
    }
    thinkthen_call_free(call); thinkthen_error_free(&error); return 0;
}
