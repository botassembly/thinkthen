#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

// Print the answer named name, "not sure", or "failed" with its code.
static void print_answer(const thinkthen_value *form, const char *name) {
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    thinkthen_value *answer = NULL;
    thinkthen_function function = THINKTHEN_FUNCTION_DECIDE;
    uint8_t is_yes = 0;
    thinkthen_text label = {.len = 0};
    double number = 0;
    int rc = thinkthen_value_annotation_get(form, name,
                                            THINKTHEN_NUL_TERMINATED, &answer,
                                            &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_value_function(answer, &function, &error);
    if (rc == THINKTHEN_OK && function == THINKTHEN_FUNCTION_DECIDE)
        rc = thinkthen_value_bool(answer, &is_yes, &error);
    else if (rc == THINKTHEN_OK && function == THINKTHEN_FUNCTION_CHOOSE)
        rc = thinkthen_value_choice(answer, &label, &error);
    else if (rc == THINKTHEN_OK)
        rc = thinkthen_value_number(answer, &number, &error);
    if (rc == THINKTHEN_UNSURE)
        fputs("not sure", stdout);
    else if (rc != THINKTHEN_OK)
        printf("failed (%s)", thinkthen_code_name(error.code));
    else if (function == THINKTHEN_FUNCTION_DECIDE)
        fputs(is_yes ? "true" : "false", stdout);
    else if (function == THINKTHEN_FUNCTION_CHOOSE)
        fwrite(label.data, 1, label.len, stdout);
    else
        printf("%g", number);
    thinkthen_value_free(answer);
    thinkthen_error_free(&error);
}

static const char *const messages[] = {
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
};

int main(void) {
    thinkthen_record_input records[3];
    for (size_t i = 0; i < 3; i++) {
        records[i].struct_size = sizeof records[i];
        records[i].kind = THINKTHEN_RECORD_TEXT;
        records[i].data = messages[i];
        records[i].len = THINKTHEN_NUL_TERMINATED;
    }
    const char *const names[] = {"refund", "topic", "urgency"};
    const char *const choices[] = {"billing", "shipping", "other"};
    const char *const levels[] = {"routine", "urgent"};
    thinkthen_client *client = NULL;
    thinkthen_question *refund = NULL, *topic = NULL, *urgency = NULL;
    thinkthen_question *triage = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row annotated = THINKTHEN_ROW_INIT;
    thinkthen_value *form = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    // The form: three named questions, each asked of every message.
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                                    "Does the customer request a refund?",
                                    THINKTHEN_NUL_TERMINATED, &refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE,
                                    "What is the message about?",
                                    THINKTHEN_NUL_TERMINATED, &topic, &error);
    for (size_t i = 0; rc == THINKTHEN_OK && i < 3; i++)
        rc = thinkthen_question_add_choice(topic, choices[i],
                                           THINKTHEN_NUL_TERMINATED, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_SCORE,
                                    "How urgent is the request?",
                                    THINKTHEN_NUL_TERMINATED, &urgency,
                                    &error);
    for (size_t i = 0; rc == THINKTHEN_OK && i < 2; i++)
        rc = thinkthen_question_add_level(urgency, levels[i],
                                          THINKTHEN_NUL_TERMINATED, &error);
    // annotate takes named questions, not question wording.
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_ANNOTATE, "", 0,
                                    &triage, &error);
    const thinkthen_question *const parts[] = {refund, topic, urgency};
    for (size_t n = 0; rc == THINKTHEN_OK && n < 3; n++)
        rc = thinkthen_question_add_entry(triage, names[n],
                                          THINKTHEN_NUL_TERMINATED, parts[n],
                                          &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open_records_question(
            client, triage, NULL, records, 3, sizeof records[0],
            THINKTHEN_OFFSET_UTF8, &call, &error);
    // One line per message: refund, topic and urgency. A failed message
    // is a row of its own, and the rest carry on.
    while (rc == THINKTHEN_OK
           && (rc = thinkthen_call_next(call, &annotated, &error))
                  == THINKTHEN_OK) {
        if (annotated.status == THINKTHEN_STATUS_FAILED) {
            printf("message %" PRIu64 " failed (%s)\n", annotated.position,
                   thinkthen_code_name(annotated.error_code));
            continue;
        }
        thinkthen_value_free(form);
        rc = thinkthen_row_values(&annotated, &form, &error);
        if (rc != THINKTHEN_OK)
            break;
        for (size_t n = 0; n < 3; n++) {
            if (n > 0)
                putchar(' ');
            print_answer(form, names[n]);
        }
        putchar('\n');
    }
    if (rc != THINKTHEN_END)
        report(&error);
    thinkthen_value_free(form);
    thinkthen_row_free(&annotated);
    thinkthen_call_free(call);
    thinkthen_question_free(triage);
    thinkthen_question_free(refund);
    thinkthen_question_free(topic);
    thinkthen_question_free(urgency);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_END ? EXIT_SUCCESS : EXIT_FAILURE;
}
