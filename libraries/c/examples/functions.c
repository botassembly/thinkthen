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

/* One JSON-door call printed as the label and the answer. */
static void door(const thinkthen_engine *tt, const char *label, const char *request) {
    char *reply = thinkthen_call(tt, request);
    if (reply == NULL) {
        printf("%s: %s\n", label, thinkthen_error_message(tt));
        return;
    }
    printf("%s: %s\n", label, reply);
    thinkthen_free_string(reply);
}

int main(void) {
    thinkthen_engine *tt = thinkthen_engine_new();
    if (tt == NULL) {
        fprintf(stderr, "the engine did not build\n");
        return 1;
    }

    const char *evidence = "I demand a refund today";
    thinkthen_answer answer = {THINKTHEN_UNSURE, 0.0};
    int rc = thinkthen_decide(tt, "Is this a complaint?", evidence, strlen(evidence), &answer);
    printf("decide: %s %.2f\n", rc == 0 ? outcome_text(answer.outcome) : "error", answer.probability);

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
    rc = thinkthen_recognize(tt, "{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\",\"place\":\"A place name.\"}}}", sentence, strlen(sentence), &found, &found_len);
    if (rc != 0) {
        printf("recognize: %s\n", thinkthen_error_message(tt));
    } else {
        printf("recognize: %.*s\n", (int)found_len, found);
        thinkthen_free_string(found);
    }

    const char *alerts[] = {
        "{\"name\":\"Checkout returns 500 at the payment step.\",\"kind\":\"alert\"}",
        "{\"name\":\"The payments database ran out of disk space.\",\"kind\":\"alert\"}",
    };
    size_t lengths[2] = {strlen(alerts[0]), strlen(alerts[1])};
    char *edges = NULL;
    size_t edges_len = 0;
    rc = thinkthen_relate(tt, "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}", alerts, lengths, 2, &edges, &edges_len);
    if (rc != 0) {
        printf("relate: %s\n", thinkthen_error_message(tt));
    } else {
        printf("relate: %.*s\n", (int)edges_len, edges);
        thinkthen_free_string(edges);
    }

    door(tt, "usage", "{\"usage\":true}");

    thinkthen_engine_free(tt);
    return 0;
}
