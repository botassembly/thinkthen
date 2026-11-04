/* A C user's program: one call per answer, kept-only filter, answer meanings and
 * reused output variables. The test that runs it chooses the backend through the
 * environment, so the program names none. */
#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>

static int failed(const char *step, const thinkthen_error *error) {
    fprintf(stderr, "%s: %d ", step, error->code);
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
    return 1;
}

static void print_text(const char *label, thinkthen_text text) {
    printf(" %s=", label);
    fwrite(text.data, 1, text.len, stdout);
}

static int decide(const thinkthen_client *client) {
    bool is_green = false;
    int rc = thinkthen_decide(client, "Is the build green?", "The build passed.",
                              THINKTHEN_NUL_TERMINATED, &is_green, NULL);
    printf("decide rc=%d answer=%d\n", rc, is_green);
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool untouched = true;
    rc = thinkthen_decide(client, "Is the build green?", "  \n", 3, &untouched, &error);
    printf("blank rc=%d code=%d answer=%d\n", rc, error.code, untouched);
    /* A question that is not UTF-8 is the same fault, with the same code, as in every C function. */
    rc = thinkthen_decide(client, "Ready \xff?", "The build passed.", THINKTHEN_NUL_TERMINATED,
                          &untouched, &error);
    printf("invalid rc=%d code=%d answer=%d\n", rc, error.code, untouched);
    thinkthen_error_free(&error);
    return 0;
}

static int choose(const thinkthen_client *client) {
    const char *const teams[] = {"billing", "shipping", "other"};
    size_t team = 99;
    int rc = thinkthen_choose(client, "Which team?", teams, 3, "first note",
                              THINKTHEN_NUL_TERMINATED, &team, NULL);
    printf("choose rc=%d team=%zu\n", rc, team);
    team = 99;
    rc = thinkthen_choose(client, "Which team?", teams, 3, "second note",
                          THINKTHEN_NUL_TERMINATED, &team, NULL);
    printf("unsure rc=%d team=%zu\n", rc, team);
    return 0;
}

static int tag(const thinkthen_client *client) {
    const char *const topics[] = {"billing", "refund", "shipping"};
    bool applies[3] = {true, true, true};
    int rc = thinkthen_tag(client, "What applies?", topics, 3, "a note",
                           THINKTHEN_NUL_TERMINATED, applies, NULL);
    printf("tag rc=%d applies=%d%d%d\n", rc, applies[0], applies[1], applies[2]);
    return 0;
}

static int score(const thinkthen_client *client) {
    const char *const levels[] = {"low", "high"};
    double urgency = -1;
    int rc = thinkthen_score(client, "How urgent?", levels, 2, "a note",
                             THINKTHEN_NUL_TERMINATED, &urgency, NULL);
    printf("score rc=%d urgency=%.2f\n", rc, urgency);
    return 0;
}

static int find(const thinkthen_client *client) {
    const char note[] = "\xF0\x9F\x98\x80\nAda\n";
    thinkthen_span line;
    memset(&line, 0, sizeof line);
    int rc = thinkthen_find(client, "Which line?", note, sizeof note - 1, &line, NULL);
    printf("find rc=%d present=%d start=%" PRIu64 " end=%" PRIu64 " inside=%d", rc, line.present,
           line.start, line.end, line.text.data == note + line.start);
    print_text("text", line.text);
    putchar('\n');
    return 0;
}

static int recognize(const thinkthen_client *client) {
    const char *const kinds[] = {"person"};
    thinkthen_value *people = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_recognize(client, kinds, 1, "Ada", THINKTHEN_NUL_TERMINATED, &people, &error);
    if (rc != THINKTHEN_OK) return failed("recognize", &error);
    size_t count = 0;
    thinkthen_name person;
    if (thinkthen_value_name_count(people, &count, &error) != THINKTHEN_OK
        || thinkthen_value_name_at(people, 0, &person, &error) != THINKTHEN_OK)
        return failed("names", &error);
    printf("recognize rc=%d count=%zu", rc, count);
    print_text("text", person.text);
    print_text("kind", person.kind);
    putchar('\n');
    thinkthen_value_free(people);
    return 0;
}

static int relate(const thinkthen_client *client) {
    const thinkthen_rule rules[] = {{.relation = {"works_for", 9}, .source = {"person", 6},
                                     .target = {"company", 7}, .either = 1}};
    thinkthen_value *links = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_relate(client, rules, 1, "Ada Acme Bea", THINKTHEN_NUL_TERMINATED, &links, &error);
    if (rc != THINKTHEN_OK) return failed("relate", &error);
    size_t count = 0;
    thinkthen_relation link;
    if (thinkthen_value_relation_count(links, &count, &error) != THINKTHEN_OK
        || thinkthen_value_relation_at(links, 0, &link, &error) != THINKTHEN_OK)
        return failed("relations", &error);
    printf("relate rc=%d count=%zu", rc, count);
    print_text("source", link.source.text);
    print_text("target", link.target.text);
    putchar('\n');
    thinkthen_value_free(links);
    return 0;
}

static int annotate(const thinkthen_client *client) {
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    thinkthen_question *ready = NULL, *topic = NULL;
    if (thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, "Ready?", THINKTHEN_NUL_TERMINATED, &ready, &error) != THINKTHEN_OK
        || thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE, "Which topic?", THINKTHEN_NUL_TERMINATED, &topic, &error) != THINKTHEN_OK
        || thinkthen_question_add_choice(topic, "billing", THINKTHEN_NUL_TERMINATED, &error) != THINKTHEN_OK
        || thinkthen_question_add_choice(topic, "shipping", THINKTHEN_NUL_TERMINATED, &error) != THINKTHEN_OK)
        return failed("questions", &error);
    const char *const names[] = {"ready", "topic"};
    const thinkthen_question *const questions[] = {ready, topic};
    thinkthen_value *note = NULL, *team = NULL, *missing = NULL;
    int rc = thinkthen_annotate(client, names, questions, 2, "alpha", THINKTHEN_NUL_TERMINATED, &note, &error);
    if (rc != THINKTHEN_OK) return failed("annotate", &error);
    thinkthen_text label;
    if (thinkthen_value_annotation_get(note, "topic", THINKTHEN_NUL_TERMINATED, &team, &error) != THINKTHEN_OK
        || thinkthen_value_choice(team, &label, &error) != THINKTHEN_OK)
        return failed("topic", &error);
    printf("annotate rc=%d", rc);
    print_text("topic", label);
    rc = thinkthen_value_annotation_get(note, "urgency", THINKTHEN_NUL_TERMINATED, &missing, &error);
    printf(" unknown=%d missing=%d\n", rc, missing == NULL);
    thinkthen_error_free(&error);
    thinkthen_value_free(team);
    thinkthen_value_free(note);
    thinkthen_question_free(ready);
    thinkthen_question_free(topic);
    return 0;
}

static int filter(const thinkthen_client *client) {
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    thinkthen_question *ready = NULL;
    thinkthen_call *call = NULL;
    if (thinkthen_question_new(THINKTHEN_FUNCTION_FILTER, "Ready?", THINKTHEN_NUL_TERMINATED, &ready, &error) != THINKTHEN_OK
        || thinkthen_question_set_band(ready, 0.2, 0.8, &error) != THINKTHEN_OK)
        return failed("question", &error);
    thinkthen_record_input records[5] = {
        {sizeof(thinkthen_record_input), THINKTHEN_RECORD_TEXT, "refund", 6},
        {sizeof(thinkthen_record_input), THINKTHEN_RECORD_TEXT, "parcel", 6},
        {sizeof(thinkthen_record_input), THINKTHEN_RECORD_TEXT, "uncertain", 9},
        {sizeof(thinkthen_record_input), THINKTHEN_RECORD_MISSING, NULL, 0},
        {sizeof(thinkthen_record_input), THINKTHEN_RECORD_TEXT, "   ", 3},
    };
    if (thinkthen_call_open_records_question(client, ready, NULL, records, 5, sizeof records[0],
                                             THINKTHEN_OFFSET_UTF8, &call, &error) != THINKTHEN_OK)
        return failed("open", &error);
    /* One row variable for every pull: passing it again frees what it held. */
    thinkthen_row complaint = THINKTHEN_ROW_INIT;
    int rc;
    while ((rc = thinkthen_call_next_kept(call, &complaint, &error)) != THINKTHEN_END) {
        if (rc < 0 && complaint.position == 0) return failed("next", &error);
        thinkthen_record_view original;
        if (thinkthen_row_original(&complaint, &original, NULL) != THINKTHEN_OK) return 1;
        printf("rc=%d position=%" PRIu64 " text=", rc, complaint.position);
        fwrite(original.data, 1, original.len, stdout);
        putchar('\n');
    }
    thinkthen_row_free(&complaint);
    thinkthen_error_free(&error);
    thinkthen_call_free(call);
    thinkthen_question_free(ready);
    return 0;
}

static int meanings(const thinkthen_client *client) {
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    thinkthen_question *ready = NULL, *team = NULL;
    if (thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, "Ready?", THINKTHEN_NUL_TERMINATED, &ready, &error) != THINKTHEN_OK
        || thinkthen_question_set_yes_means(ready, "ready to ship", THINKTHEN_NUL_TERMINATED, &error) != THINKTHEN_OK
        || thinkthen_question_set_no_means(ready, "still failing", THINKTHEN_NUL_TERMINATED, &error) != THINKTHEN_OK
        || thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE, "Which team?", THINKTHEN_NUL_TERMINATED, &team, &error) != THINKTHEN_OK
        || thinkthen_question_add_choice(team, "billing", THINKTHEN_NUL_TERMINATED, &error) != THINKTHEN_OK
        || thinkthen_question_add_choice(team, "shipping", THINKTHEN_NUL_TERMINATED, &error) != THINKTHEN_OK
        || thinkthen_question_set_description(team, "billing", THINKTHEN_NUL_TERMINATED,
                                              "charges and refunds", THINKTHEN_NUL_TERMINATED, &error) != THINKTHEN_OK)
        return failed("meanings", &error);
    int rc = thinkthen_question_set_description(team, "returns", THINKTHEN_NUL_TERMINATED,
                                                "a choice it does not have", THINKTHEN_NUL_TERMINATED, &error);
    printf("unknown rc=%d\n", rc);
    rc = thinkthen_question_set_yes_means(team, "yes", THINKTHEN_NUL_TERMINATED, &error);
    printf("choose yes rc=%d\n", rc);
    thinkthen_row row = THINKTHEN_ROW_INIT;
    rc = thinkthen_run_one(client, ready, "The build passed.", THINKTHEN_NUL_TERMINATED,
                           THINKTHEN_RECORD_TEXT, &row, &error);
    printf("ready rc=%d answer=%d\n", rc, row.answer_bool);
    rc = thinkthen_run_one(client, team, "A refund.", THINKTHEN_NUL_TERMINATED,
                           THINKTHEN_RECORD_TEXT, &row, &error);
    printf("team rc=%d answer=", rc);
    fwrite(row.answer.data, 1, row.answer.len, stdout);
    putchar('\n');
    thinkthen_row_free(&row);
    thinkthen_error_free(&error);
    thinkthen_question_free(ready);
    thinkthen_question_free(team);
    return 0;
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    if (thinkthen_client_open(NULL, &client, &error) != THINKTHEN_OK) return failed("client", &error);
    static const struct {
        const char *name;
        int (*run)(const thinkthen_client *);
    } modes[] = {{"decide", decide}, {"choose", choose}, {"tag", tag}, {"score", score},
                 {"find", find}, {"recognize", recognize}, {"relate", relate},
                 {"annotate", annotate}, {"filter", filter}, {"meanings", meanings}};
    int status = 2;
    for (size_t index = 0; index < sizeof modes / sizeof modes[0]; index++)
        if (strcmp(argv[1], modes[index].name) == 0) status = modes[index].run(client);
    thinkthen_client_free(client);
    return status;
}
