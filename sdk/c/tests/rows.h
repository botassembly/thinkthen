/* Public C row projection shared by installed consumers. */
typedef tt_call *(*function)(const tt_client *, const json_t *, const json_t *,
                              tt_error **);

static bool equal_text(tt_text actual, const char *expected) {
    return actual.len == strlen(expected)
        && memcmp(actual.data, expected, actual.len) == 0;
}

static json_t *string(tt_text value) { return json_stringn(value.data, value.len); }

static json_t *error_json(const tt_error *error) {
    return json_pack("{s:o,s:o}", "code", string(tt_error_code(error)),
                     "message", string(tt_error_message(error)));
}

/* Shared fixtures use Jansson-representable values. Lossless payload cases
 * compare raw spans directly in the installed consumer instead. */
static json_t *parse(tt_text value) {
    if (!value.len) return NULL;
    json_t *parsed = json_loadb(value.data, value.len,
                                JSON_DECODE_ANY | JSON_ALLOW_NUL, NULL);
    assert(parsed);
    return parsed;
}

static json_t *row_json(const tt_row *row) {
    const char *status = tt_row_status(row) == TT_ANSWERED ? "answered"
        : tt_row_status(row) == TT_UNSURE ? "unsure" : "failed";
    json_t *value = json_pack("{s:s}", "status", status);
    json_t *id = parse(tt_row_id(row));
    json_t *answer = parse(tt_row_answer(row));
    if (id) assert(json_object_set_new(value, "id", id) == 0);
    if (answer) assert(json_object_set_new(value, "answer", answer) == 0);
    const tt_error *error = tt_row_error(row);
    if (error) assert(json_object_set_new(value, "error", error_json(error)) == 0);
    double p = -1;
    if (tt_row_probability(row, &p))
        assert(json_object_set_new(value, "p", json_real(p)) == 0);
    else
        assert(p == -1);
    tt_text detail = tt_row_detail(row);
    if (detail.len) {
        json_t *parsed = json_loadb(detail.data, detail.len, JSON_DECODE_ANY, NULL);
        assert(parsed);
        assert(json_object_set_new(value, "detail", parsed) == 0);
    }
    return value;
}

static function named(const char *name) {
    static const struct { const char *name; function call; } functions[] = {
        {"decide", tt_decide}, {"choose", tt_choose}, {"tag", tt_tag},
        {"score", tt_score}, {"filter", tt_filter}, {"rank", tt_rank},
        {"find", tt_find}, {"annotate", tt_annotate},
        {"recognize", tt_recognize}, {"relate", tt_relate}
    };
    for (size_t i = 0; i < sizeof functions / sizeof *functions; ++i)
        if (strcmp(name, functions[i].name) == 0) return functions[i].call;
    abort();
}

