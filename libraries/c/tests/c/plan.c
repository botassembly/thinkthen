/*
 * The plan door (ticket 0291): one preview of corpus P1 printed on
 * standard output, then every refusal row. Each refusal checks the return,
 * the recorded code, and that both out parameters kept their sentinels.
 * `tests/door/plan.rs` runs this with no key and counts the loopback
 * backend's requests after it, and expects none.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <io.h>
#include <fcntl.h>
#endif

#include <thinkthen.h>

static int failed;

static void refused(thinkthen_engine *tt, const char *name, const char *input) {
    char sentinel_text[] = "kept";
    char *out = sentinel_text;
    size_t out_len = 7;
    int rc = thinkthen_plan_json(tt, input, &out, &out_len);
    if (rc != THINKTHEN_EUSAGE || thinkthen_error_code(tt) != THINKTHEN_EUSAGE ||
        out != sentinel_text || out_len != 7) {
        fprintf(stderr, "FAIL %s: rc %d code %d message \"%s\"\n", name, rc,
                thinkthen_error_code(tt), thinkthen_error_message(tt));
        failed = 1;
    }
}

static void planned(thinkthen_engine *tt, const char *name, const char *input, int print) {
    char *out = NULL;
    size_t out_len = 0;
    int rc = thinkthen_plan_json(tt, input, &out, &out_len);
    if (rc != THINKTHEN_OK || out == NULL || strlen(out) != out_len) {
        fprintf(stderr, "FAIL %s: rc %d message \"%s\"\n", name, rc, thinkthen_error_message(tt));
        failed = 1;
        return;
    }
    if (print) {
        printf("%s\n", out);
    }
    thinkthen_free_string(out);
}

int main(void) {
#ifdef _WIN32
    if (_setmode(_fileno(stdin), _O_BINARY) == -1 ||
        _setmode(_fileno(stdout), _O_BINARY) == -1 ||
        _setmode(_fileno(stderr), _O_BINARY) == -1) return 1;
#endif
    char *out = NULL;
    size_t out_len = 0;
    const char *p1 = "{\"verb\":\"decide\",\"question\":\"asks for a refund\","
                     "\"input\":[\"Refund me please.\"],\"settings\":{}}";
    if (thinkthen_plan_json(NULL, p1, &out, &out_len) != THINKTHEN_EUSAGE || out != NULL) {
        fprintf(stderr, "FAIL a null engine\n");
        failed = 1;
    }
    thinkthen_engine *tt = thinkthen_engine_new();
    if (tt == NULL) {
        fprintf(stderr, "FAIL no engine came: %s\n", thinkthen_error_message(NULL));
        return 1;
    }
    planned(tt, "P1", p1, 1);
    planned(tt, "one text", "{\"verb\":\"decide\",\"question\":\"asks for a refund\","
                            "\"input\":\"Refund me please.\"}", 0);
    planned(tt, "a question object",
            "{\"verb\":\"choose\",\"question\":{\"choose\":\"Which team?\","
            "\"options\":[\"billing\",\"other\"]},\"input\":[\"a\",\"b\"],"
            "\"settings\":{\"batch\":1,\"deadline_ms\":5}}", 0);
    planned(tt, "settings members", "{\"verb\":\"tag\",\"question\":\"Which topics?\","
                                    "\"input\":[\"a\"],\"settings\":{\"labels\":[\"x\",\"y\"]}}", 0);

    refused(tt, "invalid settings", "{\"verb\":\"decide\",\"question\":\"q\",\"input\":[\"a\"],"
                                    "\"settings\":{\"batch\":0}}");
    refused(tt, "unknown settings key", "{\"verb\":\"decide\",\"question\":\"q\",\"input\":[\"a\"],"
                                        "\"settings\":{\"nope\":1}}");
    refused(tt, "a repeated member", "{\"verb\":\"decide\",\"verb\":\"decide\",\"question\":\"q\","
                                     "\"input\":[\"a\"]}");
    refused(tt, "a question field repeated in settings",
            "{\"verb\":\"decide\",\"question\":{\"decide\":\"q\",\"threshold\":0.6},"
            "\"input\":[\"a\"],\"settings\":{\"threshold\":0.7}}");
    refused(tt, "a question field in settings beside a question object",
            "{\"verb\":\"decide\",\"question\":{\"decide\":\"q\"},\"input\":[\"a\"],"
            "\"settings\":{\"model\":\"jev-1.13.0\"}}");
    refused(tt, "an unknown member", "{\"verb\":\"decide\",\"question\":\"q\",\"input\":[\"a\"],"
                                     "\"members\":[\"a\"]}");
    refused(tt, "a missing input", "{\"verb\":\"decide\",\"question\":\"q\"}");
    refused(tt, "a keyed input", "{\"verb\":\"decide\",\"question\":\"q\",\"input\":{\"7\":\"a\"}}");
    refused(tt, "a verb plan does not take", "{\"verb\":\"find\",\"question\":\"q\",\"input\":[\"a\"]}");
    refused(tt, "a question of another verb",
            "{\"verb\":\"decide\",\"question\":{\"choose\":\"Which?\",\"options\":[\"a\",\"b\"]},"
            "\"input\":[\"a\"]}");
    refused(tt, "a choose text without options", "{\"verb\":\"choose\",\"question\":\"q\",\"input\":[\"a\"]}");
    refused(tt, "not JSON", "decide q");
    refused(tt, "a null plan input", NULL);

    char sentinel_text[] = "kept";
    char *kept = sentinel_text;
    size_t kept_len = 7;
    if (thinkthen_plan_json(tt, p1, NULL, &kept_len) != THINKTHEN_EUSAGE || kept_len != 7 ||
        thinkthen_plan_json(tt, p1, &kept, NULL) != THINKTHEN_EUSAGE || kept != sentinel_text) {
        fprintf(stderr, "FAIL a null out pointer\n");
        failed = 1;
    }
    thinkthen_engine_free(tt);
    return failed;
}
