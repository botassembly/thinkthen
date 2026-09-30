#include "ThinkThen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
/* "door REQUEST" prints one JSON-door reply. "door fields REQUEST" reads each
 * annotate row member through tt_field_read. "door plan VERB QUESTION SETTINGS
 * TEXT..." prints the plan object. "door limits" checks ticket 0291's zero
 * budgets and zero cap; "door helper" checks tt_field_read's edge table. */
static int failed(const TTFailure *failure) {
    static const char *kinds[] = {"none", "usage", "backend", "deadline", "local", "cancelled", "defect"};
    if (failure->kind < TTErrorUsage || failure->kind > TTErrorDefect) return 4;
    printf("{\"failed\":{\"kind\":\"%s\",\"code\":%d}}\n", kinds[failure->kind], failure->kind);
    return 0;
}
static void require(int good, const char *note) { if (!good) { fprintf(stderr, "FAIL: %s\n", note); exit(1); } }
static const char *state(const TTJSON *member) {
    static char text[128]; TTField field;
    require(tt_field_read(member, &field), "annotate member is not a field");
    static const char *kinds[] = {"none", "usage", "backend", "deadline", "local", "cancelled", "defect"};
    if (field.state == TTFieldUnresolved) return "unresolved";
    if (field.state == TTFieldValue) return "answered";
    snprintf(text, sizeof text, "failed %s %s", kinds[field.kind], field.cause);
    return text;
}
/* ADR 0112 section 4: null is unresolved, {"failed": ...} is a failure whose
 * unknown extra member reads without error, and any other value is answered. */
static void helper(void) {
    const char *cases[][2] = {{"null", "unresolved"}, {"true", "answered"}, {"\"billing\"", "answered"},
        {"[\"billing\",\"urgent\"]", "answered"}, {"1.2", "answered"},
        {"{\"failed\":{\"kind\":\"backend\",\"cause\":\"missing_probability\",\"later\":1}}", "failed backend missing_probability"}};
    for (size_t i = 0; i < sizeof cases / sizeof *cases; i++) {
        TTJSON *member = tt_json_parse(cases[i][0], strlen(cases[i][0]));
        require(member && !strcmp(state(member), cases[i][1]), cases[i][0]); tt_json_free(member);
    }
    const char *refused[] = {"{\"failed\":null}", "{\"team\":\"billing\"}", "{\"failed\":{\"kind\":\"later\",\"cause\":\"x\"}}"};
    for (size_t i = 0; i < 3; i++) {
        TTJSON *member = tt_json_parse(refused[i], strlen(refused[i])); TTField field = {TTFieldValue, TTErrorNone, NULL};
        require(member && !tt_field_read(member, &field) && field.state == TTFieldValue, refused[i]); tt_json_free(member);
    }
    puts("{\"helper\":\"pass\"}");
}
/* Ticket 0291: a zero cap and a zero budget each refuse before sending. Relate
 * gets two entities, since one entity has no pair to ask. */
static void limits(TTClient *client) {
    TTFailure f = {0}; TTDecision answer = {123, -1}; char *facts = NULL, *out = NULL; size_t n = 0;
    const char *capSettings = "{\"max_requests_total\":0,\"cache\":false}";
    TTClient *capped = [TTClient createWithSettings:capSettings length:strlen(capSettings) failure:&f];
    require(capped != nil, "capped engine");
    require([capped decide:"Is it?" text:"capped" length:6 deadline:-1 token:nil answer:&answer facts:&facts failure:&f] == TTErrorUsage &&
            f.kind == 1 && strstr(f.message, "process send budget") && !facts, "zero cap refuses as usage");
    [capped dealloc]; tt_failure_clear(&f);
    require(![client json:"{\"decide\":\"Is it?\",\"evidence\":\"zero-call\"}" deadline:0 token:nil failure:&f] && f.kind == TTErrorDeadline, "zero budget call");
    require([client recognize:"{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}" text:"zero-recognize" length:14
                       result:&out size:&n deadline:0 token:nil facts:&facts failure:&f] == TTErrorDeadline && f.kind == 3 && !out, "zero budget recognize");
    const char *records[] = {"{\"name\":\"A\",\"kind\":\"alert\"}", "{\"name\":\"B\",\"kind\":\"alert\"}"}; size_t lengths[] = {strlen(records[0]), strlen(records[1])};
    require([client relate:"{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}"
                     texts:records lengths:lengths count:2 result:&out size:&n deadline:0 token:nil facts:&facts failure:&f] == TTErrorDeadline && f.kind == 3 && !out, "zero budget relate");
    tt_failure_clear(&f);
    puts("{\"limits\":\"pass\"}");
}
int main(int argc, char **argv) {
    if (argc < 2) return 2;
    TTFailure failure = {0};
    const char *settings = getenv("TT_SETTINGS_JSON");
    TTClient *client = settings ? [TTClient createWithSettings:settings length:strlen(settings) failure:&failure] : [TTClient create];
    if (!client && settings) { int rc = failed(&failure); tt_failure_clear(&failure); return rc; }
    if (!client) return 3;
    int rc = 0;
    if (argc == 2 && !strcmp(argv[1], "helper")) helper();
    else if (argc == 2 && !strcmp(argv[1], "limits")) limits(client);
    else if (argc >= 5 && !strcmp(argv[1], "plan")) {
        size_t count = (size_t)argc - 5, lengths[16]; require(count <= 16, "plan texts");
        for (size_t i = 0; i < count; i++) lengths[i] = strlen(argv[5 + i]);
        char *plan = [client plan:argv[2] question:argv[3] texts:(const char *const *)argv + 5 lengths:lengths count:count settings:argv[4] failure:&failure];
        if (plan) { puts(plan); free(plan); } else rc = failed(&failure);
    } else if (argc == 3 && !strcmp(argv[1], "fields")) {
        char *reply = [client jsonBytes:argv[2] length:strlen(argv[2]) deadline:-1 token:nil failure:&failure];
        require(reply != NULL, "annotate call");
        TTJSON *tree = tt_json_parse(reply, strlen(reply)); const TTJSON *rows = tt_json_get(tree, "value");
        require(rows && rows->type == TTJSONArray, "annotate rows");
        putchar('[');
        for (size_t i = 0; i < rows->count; i++) {
            const TTJSON *row = rows->children[i];
            printf(i ? ",{" : "{");
            for (size_t j = 0; j < row->count; j++) printf("%s\"%s\":\"%s\"", j ? "," : "", row->keys[j], state(row->children[j]));
            putchar('}');
        }
        puts("]"); tt_json_free(tree); free(reply);
    } else if (argc == 2) {
        char *answer = [client jsonBytes:argv[1] length:strlen(argv[1]) deadline:-1 token:nil failure:&failure];
        if (answer) { puts(answer); free(answer); } else rc = failed(&failure);
    } else rc = 2;
    tt_failure_clear(&failure);
    [client dealloc];
    return rc;
}
