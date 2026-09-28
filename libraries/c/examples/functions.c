/* The ten functions through the C door, one call each.
 *
 * `tests/door/` builds this program against the door, runs it against
 * the loopback backend's generic arm, and compares its output with
 * `functions.txt`. That arm gives the first option, level, label, or yes
 * 0.9 and shares the rest; a model would answer differently.
 *
 * Build it by hand beside an archive:
 *   cc -std=c11 -I include examples/functions.c -L lib -lthinkthen -o functions
 */

#include <stdio.h>
#include <string.h>

#include <thinkthen.h>

static const char *outcome_text(int outcome) {
    switch (outcome) {
    case THINKTHEN_YES:
        return "yes";
    case THINKTHEN_NO:
        return "no";
    default:
        return "unsure";
    }
}

/* Print the value member of the successful {value,facts} envelope. This
 * example's fixed output shows values; a host can also read the facts. */
static const char *value_end(const char *json) {
    int depth = 0;
    int quoted = 0;
    int escaped = 0;
    for (const char *at = json; *at; at++) {
        if (escaped) {
            escaped = 0;
        } else if (quoted && *at == '\\') {
            escaped = 1;
        } else if (*at == '"') {
            quoted = !quoted;
        } else if (!quoted && (*at == '[' || *at == '{')) {
            depth++;
        } else if (!quoted && (*at == ']' || *at == '}')) {
            depth--;
        } else if (!quoted && depth == 0 && *at == ',') {
            return at;
        }
    }
    return NULL;
}

/* One JSON-door call printed as the label and its answer value. */
static void door(const thinkthen_engine *tt, const char *label, const char *request) {
    char *reply = thinkthen_call(tt, request);
    if (reply == NULL) {
        printf("%s: %s\n", label, thinkthen_error_message(tt));
        return;
    }
    if (strcmp(request, "{\"usage\":true}") == 0) {
        printf("%s: %s\n", label, reply);
        thinkthen_free_string(reply);
        return;
    }
    const char *value = "{\"value\":";
    const char *start = strncmp(reply, value, strlen(value)) == 0 ? reply + strlen(value) : NULL;
    const char *end = start == NULL ? NULL : value_end(start);
    if (end == NULL) {
        printf("%s: invalid call envelope\n", label);
    } else {
        printf("%s: %.*s\n", label, (int)(end - start), start);
    }
    thinkthen_free_string(reply);
}

int main(void) {
    thinkthen_engine *tt = thinkthen_engine_new_with("{\"batch\":1}");
    if (tt == NULL) {
        fprintf(stderr, "the engine did not build\n");
        return 1;
    }

    const char *evidence = "I demand a refund today";
    thinkthen_answer answer = {THINKTHEN_UNSURE, 0.0};
    char *facts_json = NULL;
    size_t facts_len = 0;
    int rc = thinkthen_decide_with_facts(tt, "Is this a complaint?", evidence,
                                        strlen(evidence), &answer, &facts_json, &facts_len);
    printf("decide: %s %.2f\n", rc == 0 ? outcome_text(answer.outcome) : "error", answer.probability);
    thinkthen_free_string(facts_json);

    door(tt, "choose", "{\"choose\":\"Which team owns this?\",\"options\":[\"the refund desk\",\"the maybe desk\",\"anywhere else\"],\"evidence\":\"please route this ticket\"}");

    door(tt, "tag", "{\"tag\":\"Which words appear?\",\"labels\":[\"refund\",\"shipping\"],\"evidence\":\"the refund and the shipping\"}");

    door(tt, "score", "{\"score\":\"How urgent is this?\",\"levels\":[\"Routine.\",\"Soon.\",\"Immediate.\"],\"evidence\":\"maybe later\"}");

    door(tt, "filter", "{\"filter\":\"Is this a complaint?\",\"records\":[\"good morning\",\"I demand a refund today\"]}");

    door(tt, "rank", "{\"rank\":\"Is this a complaint?\",\"records\":[\"good morning\",\"I demand a refund today\"]}");

    door(tt, "find", "{\"find\":\"Which line asks for money back?\",\"units\":[\"good morning\",\"I want a refund\"]}");

    door(tt, "annotate", "{\"annotate\":{\"version\":1,\"questions\":{\"refund\":{\"decide\":\"Does the customer ask for a refund?\"},\"complaint\":{\"decide\":\"Is this a complaint?\",\"threshold\":0.5}}},\"records\":[\"maybe later\"]}");

    char *found = NULL;
    size_t found_len = 0;
    const char *sentence = "Maria Chen joined Northwind Freight in Chicago last spring.";
    facts_json = NULL;
    rc = thinkthen_recognize_with_facts(tt, "{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\",\"place\":\"A place name.\"}}}", sentence, strlen(sentence), &found, &found_len, &facts_json, &facts_len);
    if (rc != 0) {
        printf("recognize: %s\n", thinkthen_error_message(tt));
    } else {
        printf("recognize: %.*s\n", (int)found_len, found);
        thinkthen_free_string(found);
    }
    thinkthen_free_string(facts_json);

    const char *alerts[] = {
        "{\"name\":\"Checkout returns 500 at the payment step.\",\"kind\":\"alert\"}",
        "{\"name\":\"The payments database ran out of disk space.\",\"kind\":\"alert\"}",
    };
    size_t lengths[2] = {strlen(alerts[0]), strlen(alerts[1])};
    char *edges = NULL;
    size_t edges_len = 0;
    facts_json = NULL;
    rc = thinkthen_relate_with_facts(tt, "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}", alerts, lengths, 2, &edges, &edges_len, &facts_json, &facts_len);
    if (rc != 0) {
        printf("relate: %s\n", thinkthen_error_message(tt));
    } else {
        printf("relate: %.*s\n", (int)edges_len, edges);
        thinkthen_free_string(edges);
    }
    thinkthen_free_string(facts_json);

    door(tt, "usage", "{\"usage\":true}");

    thinkthen_engine_free(tt);
    return 0;
}
