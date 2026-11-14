#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s\n", thinkthen_error_string(error));
}

static const thinkthen_record_input messages[] = {
    THINKTHEN_TEXT_RECORD("Please refund my order."),
    THINKTHEN_TEXT_RECORD("Where is my parcel?"),
    THINKTHEN_TEXT_RECORD("I was charged twice for one invoice."),
};

int main(void) {
    const char *const topics[] = {"billing", "shipping", "other"};
    const char *const levels[] = {"routine", "urgent"};
    thinkthen_client *client = NULL;
    thinkthen_question *refund = NULL, *topic = NULL, *urgency = NULL, *questions = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row form = THINKTHEN_ROW_INIT;
    thinkthen_row refund_entry = THINKTHEN_ROW_INIT;
    thinkthen_row topic_entry = THINKTHEN_ROW_INIT;
    thinkthen_row urgency_entry = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool yes = false;
    int choice = 0;
    double score = 0;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                                    "Does the customer request a refund?",
                                    THINKTHEN_NUL_TERMINATED, &refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE, "What is the message about?",
                                    THINKTHEN_NUL_TERMINATED, &topic, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_choices(topic, topics, 3, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_SCORE, "How urgent is the request?",
                                    THINKTHEN_NUL_TERMINATED, &urgency, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_levels(urgency, levels, 2, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_ANNOTATE, NULL, 0, &questions, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_entry(questions, "refund", THINKTHEN_NUL_TERMINATED, refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_entry(questions, "topic", THINKTHEN_NUL_TERMINATED, topic, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_add_entry(questions, "urgency", THINKTHEN_NUL_TERMINATED, urgency, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open(client, questions, messages, 3, &call, &error);
    while (rc == THINKTHEN_OK) {
        rc = thinkthen_call_next(call, &form, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_err_if_failed(&form, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_answer_entry(&form, "refund", THINKTHEN_NUL_TERMINATED, &refund_entry, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_answer_entry(&form, "topic", THINKTHEN_NUL_TERMINATED, &topic_entry, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_answer_entry(&form, "urgency", THINKTHEN_NUL_TERMINATED, &urgency_entry, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_answer_bool(&refund_entry, &yes, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_answer_choice(&topic_entry, &choice, &error);
        if (rc == THINKTHEN_OK)
            rc = thinkthen_row_answer_score(&urgency_entry, &score, &error);
        if (rc == THINKTHEN_OK)
            printf("%s %s %g\n", yes ? "true" : "false", topics[choice], score);
    }
    if (rc != THINKTHEN_END)
        report(&error);
    thinkthen_row_free(&refund_entry);
    thinkthen_row_free(&topic_entry);
    thinkthen_row_free(&urgency_entry);
    thinkthen_row_free(&form);
    thinkthen_call_free(call);
    thinkthen_question_free(questions);
    thinkthen_question_free(refund);
    thinkthen_question_free(topic);
    thinkthen_question_free(urgency);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_END ? EXIT_SUCCESS : EXIT_FAILURE;
}
