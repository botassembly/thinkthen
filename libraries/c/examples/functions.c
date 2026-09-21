/* The ten functions, one example each, run by the examples runner.
 *
 * Every `c` snippet in examples.json appears in this file verbatim; the
 * runner checks that and compares this program's output to the file's
 * expected lines. Offline against the null backend, no stub, no key.
 *
 * Build and run (the runner does this):
 *   cc -std=c11 -Wall -Wextra -I../../contract/include examples/functions.c \
 *      -o build/functions -Ltarget/release -lthinkthen -Wl,-rpath,$PWD/target/release
 *   ENGINE_NULL=1 ./build/functions
 */

#include <stdio.h>
#include <stdlib.h>
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

/* One JSON-door call printed as the label and the reply. */
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

    door(tt, "filter", "{\"decide\":\"Is this a complaint?\",\"records\":[\"good morning\",\"I demand a refund today\"]}");

    door(tt, "rank", "{\"decide\":\"Is this a complaint?\",\"rank\":true,\"records\":[\"good morning\",\"I demand a refund today\"]}");

    door(tt, "find", "{\"find\":\"Which line asks for money back?\",\"units\":[\"good morning\",\"I want a refund\"]}");

    door(tt, "annotate", "{\"annotate\":{\"questions\":{\"refund\":{\"decide\":\"Does the customer ask for a refund?\"},\"complaint\":{\"decide\":\"Is this a complaint?\",\"threshold\":0.5}}},\"records\":[\"maybe later\"]}");

    char *found = NULL;
    unsigned long found_len = 0;
    const char *sentence = "Maria Chen joined Northwind Freight in Chicago last spring.";
    rc = thinkthen_recognize(tt, "{\"kinds\":[\"person\",\"organization\",\"place\"]}", sentence, strlen(sentence), &found, &found_len);
    if (rc != 0) {
        printf("recognize: %s\n", thinkthen_error_message(tt));
    } else {
        printf("recognize: %.*s\n", (int)found_len, found);
        thinkthen_free_string(found);
    }

    const char *alerts[] = {
        "Checkout returns 500 at the payment step.",
        "Card charges are failing for every customer.",
        "The nightly export ran two hours late.",
        "The payments database ran out of disk space.",
    };
    unsigned long lengths[4];
    for (int i = 0; i < 4; i++) {
        lengths[i] = strlen(alerts[i]);
    }
    char *edges = NULL;
    unsigned long edges_len = 0;
    rc = thinkthen_relate(tt, "{\"relations\":[{\"name\":\"caused_by\",\"source\":\"*\",\"target\":\"*\"}]}", alerts, lengths, 4, &edges, &edges_len);
    if (rc != 0) {
        printf("relate: %s\n", thinkthen_error_message(tt));
    } else {
        printf("relate: %.*s\n", (int)edges_len, edges);
        thinkthen_free_string(edges);
    }

    thinkthen_engine_free(tt);
    return 0;
}
