/* An external consumer owns every result independently of its call. */
#include "thinkthen.h"
#include "thinkthen.h"
#include <assert.h>
#include <math.h>
#include <inttypes.h>
#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

static void ok(int32_t status, thinkthen_error *error) {
    assert(status == THINKTHEN_OK);
    assert(error->code == 0);
}


static void rejected(int32_t status, int32_t code, thinkthen_error *error) {
    assert(status == THINKTHEN_ERROR); assert(error->code == code); thinkthen_error_free(error);
}
static thinkthen_client *stub_client(thinkthen_error *error) {
    thinkthen_settings *settings = NULL; ok(thinkthen_settings_new(&settings, error), error);
    ok(thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, error), error);
    rejected(thinkthen_settings_set_backend(settings, UINT32_MAX, error), THINKTHEN_CODE_USAGE_ABI, error);
    ok(thinkthen_settings_set_batch(settings, 2, error), error);
    rejected(thinkthen_settings_set_batch(settings, 0, error), THINKTHEN_CODE_USAGE_BATCH, error);
    ok(thinkthen_settings_set_retry(settings, 0, 0, 1000, error), error);
    rejected(thinkthen_settings_set_retry(settings, 0, 0, 0, error), THINKTHEN_CODE_USAGE_SETTINGS, error);
    const char *key = "synthetic-c-key"; ok(thinkthen_settings_set_api_key(settings, (const uint8_t *)key, strlen(key), error), error);
    rejected(thinkthen_settings_set_api_key(settings, NULL, 0, error), THINKTHEN_CODE_USAGE_KEY_MISSING, error);
    const char *url = "http://127.0.0.1:1/v1"; ok(thinkthen_settings_set_url(settings, (const uint8_t *)url, strlen(url), error), error);
    const char *bad_url = "http://remote.invalid/v1"; rejected(thinkthen_settings_set_url(settings, (const uint8_t *)bad_url, strlen(bad_url), error), THINKTHEN_CODE_USAGE_URL, error);
    thinkthen_client *client = NULL; ok(thinkthen_client_open(settings, &client, error), error); thinkthen_settings_free(settings); return client;
}
static thinkthen_question *question(uint32_t function, thinkthen_error *error) {
    const char *wording = function == THINKTHEN_FUNCTION_RECOGNIZE || function == THINKTHEN_FUNCTION_RELATE || function == THINKTHEN_FUNCTION_ANNOTATE ? "" : "Ready?";
    thinkthen_question *value = NULL; ok(thinkthen_question_new(function, (const uint8_t *)wording, strlen(wording), &value, error), error);
    switch (function) {
    case THINKTHEN_FUNCTION_CHOOSE: ok(thinkthen_question_add_choice(value, (const uint8_t *)"one", 3, error), error); ok(thinkthen_question_add_choice(value, (const uint8_t *)"two", 3, error), error); break;
    case THINKTHEN_FUNCTION_TAG: ok(thinkthen_question_add_label(value, (const uint8_t *)"one", 3, error), error); break;
    case THINKTHEN_FUNCTION_SCORE: case THINKTHEN_FUNCTION_RANK: ok(thinkthen_question_add_level(value, (const uint8_t *)"low", 3, error), error); ok(thinkthen_question_add_level(value, (const uint8_t *)"high", 4, error), error); break;
    case THINKTHEN_FUNCTION_RECOGNIZE: ok(thinkthen_question_add_kind(value, (const uint8_t *)"person", 6, error), error); break;
    case THINKTHEN_FUNCTION_RELATE: {
        thinkthen_rule rule = {0}; rule.relation = (thinkthen_text){ (const uint8_t *)"works_for", 9 }; rule.source = (thinkthen_text){ (const uint8_t *)"person", 6 }; rule.target = (thinkthen_text){ (const uint8_t *)"organization", 12 }; rule.reads = (thinkthen_text){ (const uint8_t *)"works for", 9 }; rule.either = 1;
        ok(thinkthen_question_add_rule(value, &rule, error), error); rejected(thinkthen_question_add_rule(value, &rule, error), THINKTHEN_CODE_USAGE_QUESTION, error); break;
    }
    case THINKTHEN_FUNCTION_ANNOTATE: {
        thinkthen_question *child = question(THINKTHEN_FUNCTION_DECIDE, error); ok(thinkthen_question_add_entry(value, (const uint8_t *)"ready", 5, child, error), error); thinkthen_question_free(child); break;
    }
    default: break;
    }
    return value;
}
static void identity(thinkthen_client *client, thinkthen_question *question, uint8_t result[64], thinkthen_error *error) {
    thinkthen_row row = {0}; row.struct_size = sizeof(row); ok(thinkthen_run_one(client, question, NULL, 0, THINKTHEN_RECORD_MISSING, &row, error), error);
    assert(row.status == THINKTHEN_STATUS_FAILED && row.error_code == THINKTHEN_CODE_RECORD_MISSING);
    thinkthen_value *value = NULL; ok(thinkthen_row_values(&row, &value, error), error); thinkthen_row_free(&row);
    thinkthen_source source = {0}; ok(thinkthen_value_source(value, &source, error), error); assert(source.question_identity.len == 64); memcpy(result, source.question_identity.data, 64); thinkthen_value_free(value);
}
static void builders_and_prefixes(void) {
    thinkthen_error error = {0}; error.struct_size = sizeof(error); thinkthen_client *client = stub_client(&error);
    uint32_t functions[] = { THINKTHEN_FUNCTION_DECIDE, THINKTHEN_FUNCTION_FILTER, THINKTHEN_FUNCTION_CHOOSE, THINKTHEN_FUNCTION_TAG, THINKTHEN_FUNCTION_SCORE, THINKTHEN_FUNCTION_RANK, THINKTHEN_FUNCTION_FIND, THINKTHEN_FUNCTION_RECOGNIZE, THINKTHEN_FUNCTION_RELATE, THINKTHEN_FUNCTION_ANNOTATE };
    for (size_t i = 0; i < sizeof(functions) / sizeof(functions[0]); i++) {
        uint32_t function = functions[i]; thinkthen_question *value = question(function, &error);
        uint8_t before[64], after[64]; identity(client, value, before, &error);
        rejected(thinkthen_question_set_evidence(value, (const uint8_t *)"body", 4, &error), THINKTHEN_CODE_USAGE_POINTER, &error);
        identity(client, value, after, &error); assert(memcmp(before, after, 64) == 0);
        ok(thinkthen_question_set_evidence(value, (const uint8_t *)"/body", 5, &error), &error);
        if (function == THINKTHEN_FUNCTION_DECIDE || function == THINKTHEN_FUNCTION_FILTER || function == THINKTHEN_FUNCTION_CHOOSE) ok(thinkthen_question_set_band(value, 0.2, 0.8, &error), &error);
        else rejected(thinkthen_question_set_band(value, 0.2, 0.8, &error), THINKTHEN_CODE_USAGE_THRESHOLD, &error);
        if (function == THINKTHEN_FUNCTION_SCORE || function == THINKTHEN_FUNCTION_RANK || function == THINKTHEN_FUNCTION_ANNOTATE) rejected(thinkthen_question_set_cut(value, 0.7, &error), THINKTHEN_CODE_USAGE_THRESHOLD, &error);
        else ok(thinkthen_question_set_cut(value, 0.7, &error), &error);
        if (function == THINKTHEN_FUNCTION_RANK) {
            ok(thinkthen_question_set_top(value, INT32_MAX, &error), &error);
            rejected(thinkthen_question_set_top(value, (size_t)INT32_MAX + 1, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
            ok(thinkthen_question_set_top(value, 2, &error), &error);
            rejected(thinkthen_question_set_top(value, 0, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
        }
        thinkthen_question_free(value);
    }
    thinkthen_question *rank = question(THINKTHEN_FUNCTION_RANK, &error); uint8_t before[64], after[64]; identity(client, rank, before, &error);
    thinkthen_toolbox *toolbox = NULL; ok(thinkthen_toolbox_new(&toolbox, &error), &error); ok(thinkthen_toolbox_insert(toolbox, (const uint8_t *)"priority", 8, rank, &error), &error);
    rejected(thinkthen_toolbox_insert(toolbox, (const uint8_t *)"priority", 8, rank, &error), THINKTHEN_CODE_USAGE_NAME, &error);
    ok(thinkthen_question_add_level(rank, (const uint8_t *)"third", 5, &error), &error); thinkthen_question_free(rank);
    thinkthen_question *selected = NULL; rejected(thinkthen_toolbox_select(toolbox, (const uint8_t *)"absent", 6, &selected, &error), THINKTHEN_CODE_USAGE_NAME, &error); assert(selected == NULL);
    ok(thinkthen_toolbox_select(toolbox, (const uint8_t *)"priority", 8, &selected, &error), &error); thinkthen_toolbox_free(toolbox);
    identity(client, selected, after, &error); assert(memcmp(before, after, 64) == 0);
    thinkthen_question *annotation = question(THINKTHEN_FUNCTION_ANNOTATE, &error);
    rejected(thinkthen_question_add_entry(annotation, (const uint8_t *)"rank", 4, selected, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    rejected(thinkthen_question_add_entry(annotation, (const uint8_t *)"nested", 6, annotation, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    thinkthen_question_free(annotation); thinkthen_question_free(selected);
    thinkthen_question *decide = question(THINKTHEN_FUNCTION_DECIDE, &error);
    _Alignas(thinkthen_row) struct { uint32_t size; uint32_t guard; } short_row = { sizeof(uint32_t), UINT32_C(0x84ab1de6) };
    rejected(thinkthen_run_one(client, decide, NULL, 0, THINKTHEN_RECORD_MISSING, (thinkthen_row *)&short_row, &error), THINKTHEN_CODE_USAGE_ABI, &error); assert(short_row.guard == UINT32_C(0x84ab1de6));
    _Alignas(thinkthen_error) struct { uint32_t size; uint32_t guard; } short_error = { sizeof(uint32_t), UINT32_C(0xa4cf78e2) };
    thinkthen_settings *untouched = NULL; assert(thinkthen_settings_new(&untouched, (thinkthen_error *)&short_error) == THINKTHEN_ERROR); assert(untouched == NULL && short_error.guard == UINT32_C(0xa4cf78e2));
    thinkthen_row_free((thinkthen_row *)&short_row); thinkthen_error_free((thinkthen_error *)&short_error);
    assert(short_row.guard == UINT32_C(0x84ab1de6) && short_error.guard == UINT32_C(0xa4cf78e2));
    thinkthen_question *incomplete = NULL;
    ok(thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE, (const uint8_t *)"Ready?", 6, &incomplete, &error), &error);
    thinkthen_row row = {0}; row.struct_size = sizeof(row);
    rejected(thinkthen_run_one(client, incomplete, NULL, 0, THINKTHEN_RECORD_MISSING, &row, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    ok(thinkthen_question_add_choice(incomplete, (const uint8_t *)"one", 3, &error), &error);
    rejected(thinkthen_run_one(client, incomplete, NULL, 0, THINKTHEN_RECORD_MISSING, &row, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    ok(thinkthen_question_add_choice(incomplete, (const uint8_t *)"two", 3, &error), &error);
    ok(thinkthen_run_one(client, incomplete, NULL, 0, THINKTHEN_RECORD_MISSING, &row, &error), &error);
    assert(row.function == THINKTHEN_FUNCTION_CHOOSE); thinkthen_row_free(&row); thinkthen_question_free(incomplete);
    thinkthen_question_free(decide); thinkthen_client_free(client); thinkthen_error_free(&error);
}

static void finite_inputs(void) {
    const char *request = "{\"question\":{\"function\":\"decide\",\"question\":\"Ready?\"}}";
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_client *client = stub_client(&error);
    thinkthen_call *call = NULL;
    /* Admission errors win before descriptor access. */
    const char *bad_question = "{\"question\":{\"function\":\"choose\",\"question\":\"Which?\"}}";
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)bad_question, strlen(bad_question), NULL, 1, 0, THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    assert(call == NULL);
    const char *competing = "{\"question\":{\"function\":\"decide\",\"question\":\"Ready?\"},\"records\":[\"x\"]}";
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)competing, strlen(competing), NULL, 1, 0, THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), NULL, 1, sizeof(thinkthen_record_input), THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    /* A size-only prefix must be rejected without reading its absent fields. */
    uint32_t prefix = sizeof(prefix);
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), (const thinkthen_record_input *)&prefix, 1, sizeof(thinkthen_record_input), THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    thinkthen_record_input input = { sizeof(input), THINKTHEN_RECORD_TEXT, NULL, 0 };
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), &input, 1, sizeof(input) - 1, THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), &input, SIZE_MAX, sizeof(input), THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    input.struct_size++;
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), &input, 1, sizeof(input), THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    input.struct_size = sizeof(input); input.kind = UINT32_MAX;
    rejected(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), &input, 1, sizeof(input), THINKTHEN_OFFSET_CODE_POINTS, &call, &error), THINKTHEN_CODE_USAGE_ABI, &error);
    assert(call == NULL);
    ok(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), NULL, 0, 0, THINKTHEN_OFFSET_CODE_POINTS, &call, &error), &error);
    thinkthen_row row = {0}; row.struct_size = sizeof(row);
    assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END); thinkthen_call_free(call);
    /* Extended, unaligned slots and their bytes are copied before return. */
    enum { stride = sizeof(thinkthen_record_input) + 8, count = 4 };
    uint8_t slots[1 + stride * count]; memset(slots, 0, sizeof(slots));
    uint8_t json[] = "{\"id\":18446744073709551615,\"body\":\"x\"}";
    uint32_t kinds[] = { THINKTHEN_RECORD_TEXT, THINKTHEN_RECORD_JSON, THINKTHEN_RECORD_MISSING, THINKTHEN_RECORD_INVALID };
    for (size_t i = 0; i < count; i++) {
        input = (thinkthen_record_input){ stride, kinds[i], i == 1 ? json : NULL, i == 1 ? sizeof(json) - 1 : 0 };
        memcpy(slots + 1 + i * stride, &input, sizeof(input));
    }
    ok(thinkthen_call_open_records_json(client, (const uint8_t *)request, strlen(request), (const thinkthen_record_input *)(slots + 1), count, stride, THINKTHEN_OFFSET_CODE_POINTS, &call, &error), &error);
    memset(slots, 0, sizeof(slots)); memset(json, '!', sizeof(json)); thinkthen_client_free(client);
    for (size_t i = 0; i < count; i++) {
        ok(thinkthen_call_next(call, &row, &error), &error); assert(row.position == i + 1);
        if (i < 2) assert(row.status != THINKTHEN_STATUS_FAILED);
        else assert(row.status == THINKTHEN_STATUS_FAILED && row.error_code == (i == 2 ? THINKTHEN_CODE_RECORD_MISSING : THINKTHEN_CODE_RECORD_INVALID));
        if (i == 1) {
            thinkthen_value *value = NULL; thinkthen_id id = {0};
            ok(thinkthen_row_values(&row, &value, &error), &error); ok(thinkthen_value_id(value, &id, &error), &error);
            assert(id.kind == THINKTHEN_ID_UNSIGNED && id.unsigned_value == UINT64_MAX); thinkthen_value_free(value);
        }
        thinkthen_row_free(&row);
    }
    assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END); thinkthen_call_free(call);
    /* Conversion errors own their message and respect the supplied prefix. */
    struct { thinkthen_error error; uint64_t tail; } extended = {0};
    extended.error.struct_size = sizeof(extended); extended.tail = UINT64_C(0x7a3bd8e1);
    assert(thinkthen_settings_conversion_error(&extended.error) == THINKTHEN_ERROR);
    assert(extended.error.code == THINKTHEN_CODE_USAGE_SETTINGS && extended.error.message.len > 0);
    assert(extended.tail == UINT64_C(0x7a3bd8e1)); thinkthen_error_free(&extended.error); thinkthen_error_free(&extended.error);
    assert(extended.error.struct_size == sizeof(extended) && extended.error.message.data == NULL);
    rejected(thinkthen_question_conversion_error(&error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    _Alignas(thinkthen_error) struct { uint32_t size; uint32_t guard; } short_error = { sizeof(uint32_t), UINT32_C(0x8e5a12b7) };
    assert(thinkthen_settings_conversion_error((thinkthen_error *)&short_error) == THINKTHEN_ERROR);
    assert(thinkthen_question_conversion_error((thinkthen_error *)&short_error) == THINKTHEN_ERROR);
    assert(short_error.guard == UINT32_C(0x8e5a12b7));
    assert(thinkthen_settings_conversion_error(NULL) == THINKTHEN_ERROR);
    thinkthen_error_free(&error);
}

/* The Rust driver supplies canonical questions and reviewed expected rows. */
static void field(const char *name, thinkthen_buffer value) {
    if (value.len) { printf(",\"%s\":", name); fwrite(value.data, 1, value.len, stdout); }
}
static int named_fixture(int argc, char **argv) {
    assert(argc == 10);
    const char *mode = argv[1];
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_client *client = NULL;
    ok(thinkthen_client_open_json((const uint8_t *)argv[2], strlen(argv[2]), &client, &error), &error);
    uint32_t function = strcmp(argv[3], "choose") == 0 ? THINKTHEN_FUNCTION_CHOOSE : THINKTHEN_FUNCTION_RANK;
    thinkthen_question *question = NULL;
    ok(thinkthen_question_new(function, (const uint8_t *)argv[4], strlen(argv[4]), &question, &error), &error);
    for (int i = 5; i <= 6; i++) {
        if (function == THINKTHEN_FUNCTION_CHOOSE) ok(thinkthen_question_add_choice(question, (const uint8_t *)argv[i], strlen(argv[i]), &error), &error);
        else ok(thinkthen_question_add_level(question, (const uint8_t *)argv[i], strlen(argv[i]), &error), &error);
    }
    if (function == THINKTHEN_FUNCTION_RANK) ok(thinkthen_question_set_top(question, 1, &error), &error);
    if (strcmp(mode, "selected") == 0) {
        thinkthen_toolbox *toolbox = NULL; thinkthen_question *selected = NULL;
        ok(thinkthen_toolbox_new(&toolbox, &error), &error);
        ok(thinkthen_toolbox_insert(toolbox, (const uint8_t *)argv[7], strlen(argv[7]), question, &error), &error);
        ok(thinkthen_toolbox_select(toolbox, (const uint8_t *)argv[7], strlen(argv[7]), &selected, &error), &error);
        thinkthen_toolbox_free(toolbox); thinkthen_question_free(question); question = selected;
    }
    thinkthen_text name = { (const uint8_t *)argv[7], strlen(argv[7]) };
    const thinkthen_text *execution_name = strcmp(mode, "named") == 0 || strcmp(mode, "invalid") == 0 ? &name : NULL;
    thinkthen_call *call = NULL;
    int32_t status = thinkthen_call_open_question(client, question, execution_name, THINKTHEN_OFFSET_CODE_POINTS, &call, &error);
    thinkthen_question_free(question); thinkthen_client_free(client);
    if (strcmp(mode, "invalid") == 0) {
        assert(status == THINKTHEN_ERROR && error.code == THINKTHEN_CODE_USAGE_NAME && call == NULL);
        puts(thinkthen_code_name(error.code)); thinkthen_error_free(&error); return 0;
    }
    ok(status, &error);
    if (strcmp(mode, "named") == 0) memset(argv[7], 'x', strlen(argv[7]));
    int input = 8;
    for (;;) {
        thinkthen_row row = {0}; row.struct_size = sizeof(row);
        status = thinkthen_call_next(call, &row, &error);
        if (status == THINKTHEN_END) break;
        if (status == THINKTHEN_NEEDS_INPUT) {
            if (input < argc) { ok(thinkthen_call_send_text(call, (const uint8_t *)argv[input], strlen(argv[input]), &error), &error); input++; }
            else thinkthen_call_end_input(call);
        } else {
            ok(status, &error);
            thinkthen_value *value = NULL;
            ok(thinkthen_row_values(&row, &value, &error), &error);
            thinkthen_text projected_name = {0};
            ok(thinkthen_value_source_name(value, &projected_name, &error), &error);
            assert((projected_name.data != NULL) == (execution_name != NULL));
            if (execution_name) assert(projected_name.len == execution_name->len);
            thinkthen_value_free(value);
            printf("{\"function\":%u,\"status\":%u,\"position\":%" PRIu64, row.function, row.status, row.position);
            if (!isnan(row.probability)) printf(",\"probability\":%.17g", row.probability);
            field("answer", row.answer); field("detail", row.detail); field("source", row.source); field("error", row.error);
            puts("}"); thinkthen_row_free(&row);
        }
    }
    thinkthen_call_free(call); thinkthen_error_free(&error); return 0;
}

static void opaque_settings_snapshots(void) {
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    const char *json = "{\"backend\":\"openai\",\"retry\":{\"max_retries\":0}}";
    thinkthen_settings *settings = NULL, *key = NULL, *copy = NULL;
    ok(thinkthen_settings_from_json((const uint8_t *)json, strlen(json), &settings, &error), &error);
    ok(thinkthen_settings_new(&key, &error), &error);
    ok(thinkthen_settings_set_api_key(key, (const uint8_t *)"opaque-canary", 13, &error), &error);
    ok(thinkthen_settings_copy_api_key(settings, key, &error), &error);
    ok(thinkthen_settings_clone(settings, &copy, &error), &error);
    thinkthen_settings_free(settings); thinkthen_settings_free(key);
    thinkthen_client *client = NULL;
    ok(thinkthen_client_open(copy, &client, &error), &error);
    thinkthen_client_free(client);
    ok(thinkthen_settings_new(&key, &error), &error);
    ok(thinkthen_settings_copy_api_key(copy, key, &error), &error);
    rejected(thinkthen_client_open(copy, &client, &error), THINKTHEN_CODE_USAGE_KEY_MISSING, &error);
    thinkthen_settings_free(copy); thinkthen_settings_free(key); thinkthen_error_free(&error);
}

/* Input helper snapshots stay independent, and finite named inputs keep failed rows. */
static void input_helpers(void) {
    thinkthen_error error = {.struct_size = sizeof error};
    thinkthen_client *client = stub_client(&error);
    const char definition[] = "{\"function\":\"choose\",\"question\":\"Which?\",\"choices\":[\"red\",\"blue\"]}";
    thinkthen_question *original = NULL, *copy = NULL;
    ok(thinkthen_question_from_json((const uint8_t *)definition, sizeof definition - 1, &original, &error), &error);
    ok(thinkthen_question_clone(original, &copy, &error), &error);
    thinkthen_question_free(original);
    const char toolbox_json[] = "{\"color\":{\"function\":\"choose\",\"question\":\"Which?\",\"choices\":[\"red\",\"blue\"]},\"rank\":{\"function\":\"rank\",\"question\":\"How much?\",\"levels\":[\"low\",\"high\"]}}";
    thinkthen_toolbox *toolbox = NULL;
    ok(thinkthen_toolbox_from_definitions_json((const uint8_t *)toolbox_json, sizeof toolbox_json - 1, &toolbox, &error), &error);
    thinkthen_buffer saved = {0}, definitions = {0};
    ok(thinkthen_toolbox_to_json(toolbox, &saved, &error), &error);
    thinkthen_toolbox_free(toolbox); toolbox = NULL;
    ok(thinkthen_toolbox_from_json(saved.data, saved.len, &toolbox, &error), &error);
    thinkthen_buffer_free(&saved);
    ok(thinkthen_toolbox_definitions_json(toolbox, &definitions, &error), &error);
    thinkthen_toolbox *hydrated = NULL;
    ok(thinkthen_toolbox_from_definitions_json(definitions.data, definitions.len, &hydrated, &error), &error);
    thinkthen_buffer_free(&definitions);
    thinkthen_toolbox_free(toolbox); toolbox = hydrated;
    thinkthen_question *rank_snapshot = NULL;
    ok(thinkthen_toolbox_select_as(toolbox, (const uint8_t *)"rank", 4, THINKTHEN_FUNCTION_RANK, &rank_snapshot, &error), &error);
    thinkthen_question_free(rank_snapshot);
    thinkthen_question *selected = NULL;
    rejected(thinkthen_toolbox_select_as(toolbox, (const uint8_t *)"color", 5, THINKTHEN_FUNCTION_DECIDE, &selected, &error), THINKTHEN_CODE_USAGE_NAME, &error);
    assert(selected == NULL);
    ok(thinkthen_toolbox_select_as(toolbox, (const uint8_t *)"color", 5, THINKTHEN_FUNCTION_CHOOSE, &selected, &error), &error);
    thinkthen_toolbox_free(toolbox);
    uint32_t selected_function = 0;
    ok(thinkthen_question_function(selected, &selected_function, &error), &error);
    assert(selected_function == THINKTHEN_FUNCTION_CHOOSE);
    thinkthen_buffer question_json = {0};
    ok(thinkthen_question_to_json(selected, &question_json, &error), &error);
    thinkthen_question *reloaded = NULL;
    ok(thinkthen_question_from_json(question_json.data, question_json.len, &reloaded, &error), &error);
    thinkthen_buffer_free(&question_json);
    thinkthen_question_free(selected); selected = reloaded;
    uint8_t selected_identity[64], copied_identity[64];
    identity(client, selected, selected_identity, &error);
    identity(client, copy, copied_identity, &error);
    assert(memcmp(selected_identity, copied_identity, 64) == 0);
    thinkthen_record_input records[] = {
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_TEXT, .data = (const uint8_t *)"evidence", .len = 8},
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_MISSING},
        {.struct_size = sizeof(thinkthen_record_input), .kind = THINKTHEN_RECORD_JSON, .data = (const uint8_t *)"{\"id\":18446744073709551615,\"body\":\"last\"}", .len = 41},
    };
    records[2].len = strlen((const char *)records[2].data);
    thinkthen_text name = {(const uint8_t *)"execution", 9};
    thinkthen_call *call = NULL;
    ok(thinkthen_call_open_records_question(client, selected, &name, records, 3, sizeof records[0], THINKTHEN_OFFSET_UTF8, &call, &error), &error);
    thinkthen_question_free(selected);
    thinkthen_question_free(copy);
    thinkthen_client_free(client);
    for (size_t i = 0; i < 3; ++i) {
        thinkthen_row row = {.struct_size = sizeof row};
        ok(thinkthen_call_next(call, &row, &error), &error);
        assert(row.position == i + 1);
        assert(row.status == (i == 1 ? THINKTHEN_STATUS_FAILED : THINKTHEN_STATUS_ANSWERED));
        if (i == 1) assert(row.error_code == THINKTHEN_CODE_RECORD_MISSING);
        if (i == 2) assert(row.id.len == 20 && memcmp(row.id.data, "18446744073709551615", 20) == 0);
        thinkthen_value *value = NULL;
        ok(thinkthen_row_values(&row, &value, &error), &error);
        thinkthen_text actual_name = {0};
        ok(thinkthen_value_source_name(value, &actual_name, &error), &error);
        assert(actual_name.len == 9 && memcmp(actual_name.data, "execution", 9) == 0);
        thinkthen_value_free(value);
        thinkthen_row_free(&row);
    }
    thinkthen_row terminal = {.struct_size = sizeof terminal};
    assert(thinkthen_call_next(call, &terminal, &error) == THINKTHEN_END);
    thinkthen_call_free(call);
    thinkthen_row_free(&terminal);
    rejected(thinkthen_question_from_json((const uint8_t *)"{}", 2, &original, &error), THINKTHEN_CODE_USAGE_QUESTION, &error);
    rejected(thinkthen_cancelled_error(&error), THINKTHEN_CODE_LOCAL_CANCELLED, &error);
    assert(error.message.data == NULL);
}

int main(int argc, char **argv) {
    if (argc == 1) input_helpers();
    if (argc > 1) return named_fixture(argc, argv);
    opaque_settings_snapshots();
    finite_inputs();
    builders_and_prefixes();
    assert(thinkthen_abi_version() == THINKTHEN_ABI_VERSION);
    assert(thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, THINKTHEN_ABI_VERSION));
    assert(thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, THINKTHEN_ABI_VERSION + 1));
    assert(!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, THINKTHEN_ABI_VERSION - 1));
    assert(!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, THINKTHEN_ABI_VERSION + (1u << 16)));
    for (size_t index = 0; index < thinkthen_code_count(); index++) assert(thinkthen_code_name(thinkthen_code_at(index)) != NULL);
    assert(thinkthen_version()[0] != '\0');
    assert(strcmp(thinkthen_code_name(THINKTHEN_CODE_RECORD_MISSING), "record.missing") == 0);
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_settings *settings = NULL;
    ok(thinkthen_settings_new(&settings, &error), &error);
    ok(thinkthen_settings_set_backend(settings, THINKTHEN_BACKEND_STUB, &error), &error);
    thinkthen_client *client = NULL;
    ok(thinkthen_client_open(settings, &client, &error), &error);
    thinkthen_settings_free(settings);
    thinkthen_question *question = NULL;
    ok(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, (const uint8_t *)"Ready?", 6, &question, &error), &error);
    struct { thinkthen_row row; uint64_t tail; } result = {0};
    result.row.struct_size = sizeof(result);
    result.tail = UINT64_C(0x53cd8b02a17f9406);
    ok(thinkthen_run_one(client, question, NULL, 0, THINKTHEN_RECORD_MISSING, &result.row, &error), &error);
    assert(result.tail == UINT64_C(0x53cd8b02a17f9406));
    assert(result.row.function == THINKTHEN_FUNCTION_DECIDE);
    assert(result.row.status == THINKTHEN_STATUS_FAILED);
    assert(result.row.position == 1);
    assert(result.row.error_code == THINKTHEN_CODE_RECORD_MISSING);
    assert(isnan(result.row.probability));
    thinkthen_value *value = NULL;
    ok(thinkthen_row_values(&result.row, &value, &error), &error);
    thinkthen_source source = {0};
    ok(thinkthen_value_source(value, &source, &error), &error);
    assert(source.backend == THINKTHEN_BACKEND_STUB);
    assert(source.question_identity.len == 64);
    thinkthen_question_free(question);
    thinkthen_client_free(client);
    assert(result.row.error.len > 0);
    thinkthen_row_free(&result.row);
    thinkthen_row_free(&result.row);
    assert(result.row.struct_size == sizeof(result));
    assert(result.tail == UINT64_C(0x53cd8b02a17f9406));
    uint32_t status = 0;
    ok(thinkthen_value_status(value, &status, &error), &error);
    assert(status == THINKTHEN_STATUS_FAILED);
    thinkthen_error failure = {0}; failure.struct_size = sizeof(failure);
    ok(thinkthen_value_error(value, &failure, &error), &error);
    assert(failure.code == THINKTHEN_CODE_RECORD_MISSING);
    thinkthen_error_free(&failure);
    assert(source.question_identity.data[0] != 0);
    thinkthen_value_free(value);
    thinkthen_error_free(&error);
    thinkthen_error_free(&error);
    assert(error.struct_size == sizeof(error));
    return 0;
}
