/* Canonical counted Request preview through the installed header. */
#include <thinkthen.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void check(int holds, const char *what) {
    if (!holds) { fprintf(stderr, "FAIL %s\n", what); exit(1); }
}
static const char p1[] = "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"asks for a refund\"},\"input\":{\"kind\":\"text\",\"text\":\"Refund me please.\"},\"options\":{\"max_requests_total\":0}}}";

static void refused(thinkthen_engine *engine, const char *input) {
    char *sentinel = (char *)(uintptr_t)7;
    size_t extent = 99;
    check(thinkthen_request_plan_json(engine, input, strlen(input), &sentinel, &extent) == THINKTHEN_EUSAGE,
          "canonical refusal kind");
    check(sentinel == (char *)(uintptr_t)7 && extent == 99, "refusal preserves outputs");
    const char *message = thinkthen_session_error_message();
    check(message && !strstr(message, "PRIVATE"), "refusal keeps safe diagnostic");
}
int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new();
    check(engine != NULL, "engine construction");
    char *out = (char *)(uintptr_t)7;
    size_t len = 99;
    check(thinkthen_request_plan_json(NULL, p1, sizeof(p1)-1, &out, &len) == THINKTHEN_EUSAGE,
          "null engine");
    check(out == (char *)(uintptr_t)7 && len == 99, "null engine keeps outputs");
    check(thinkthen_request_plan_json(engine, p1, sizeof(p1)-1, NULL, &len) == THINKTHEN_EUSAGE && len == 99,
          "null output");
    check(thinkthen_request_plan_json(engine, p1, sizeof(p1)-1, &out, NULL) == THINKTHEN_EUSAGE && out == (char *)(uintptr_t)7,
          "null length");
    union { char *out; size_t len; } alias;
    alias.out = (char *)(uintptr_t)7;
    check(thinkthen_request_plan_json(engine, p1, sizeof(p1)-1, &alias.out, &alias.len) == THINKTHEN_EUSAGE
          && alias.out == (char *)(uintptr_t)7, "aliased outputs");
    check(thinkthen_request_plan_json(engine, p1, SIZE_MAX, &out, &len) == THINKTHEN_EUSAGE,
          "unrepresentable input extent");
    check(thinkthen_request_plan_json(engine, NULL, 0, &out, &len) == THINKTHEN_EUSAGE,
          "empty request");
    refused(engine, "{\"schema\":\"thinkthen.request/1\",\"schema\":\"thinkthen.request/1\",\"PRIVATE\":true}");
    refused(engine, "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"PRIVATE\"},\"input\":{\"kind\":\"text\",\"text\":\"PRIVATE\"},\"options\":{\"batch\":null}}}");
    refused(engine, "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"filter\",\"question\":{\"kind\":\"text\",\"text\":\"PRIVATE\"},\"input\":{\"kind\":\"source\",\"source\":{\"paths\":[\"PRIVATE-MISSING\"]}}}}");
    /* Counted input deliberately omits the trailing NUL. */
    char *request = malloc(sizeof(p1)-1);
    check(request != NULL, "request storage");
    memcpy(request, p1, sizeof(p1)-1);
    check(thinkthen_request_plan_json(engine, request, sizeof(p1)-1, &out, &len) == THINKTHEN_OK,
          "canonical preview");
    memset(request, 0xff, sizeof(p1)-1);
    free(request);
    thinkthen_engine_free(engine);
    check(out && strlen(out) == len && out[len] == 0, "independent owned preview");
    puts(out);
    thinkthen_free_string(out);
    return 0;
}
