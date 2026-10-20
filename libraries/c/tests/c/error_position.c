/* Installed owned errors retain native positions independently of other errors. */
#include <thinkthen.h>
#include "platform.h"
#include <string.h>

static void check(int condition, const char *message) {
    if (!condition) { fputs(message, stderr); fputc('\n', stderr); exit(1); }
}
static thinkthen_complete_utf8 text(const char *value) {
    return (thinkthen_complete_utf8){value, strlen(value)};
}
static void quoted(thinkthen_complete_utf8 value) {
    putchar('"');
    for (size_t i = 0; i < value.len; i++) {
        unsigned char c = (unsigned char)value.data[i];
        if (c == '"' || c == '\\') { putchar('\\'); putchar(c); }
        else if (c < 32) printf("\\u%04x", c);
        else putchar(c);
    }
    putchar('"');
}
static thinkthen_call_error *failed_call(thinkthen_engine *engine,
                                        const thinkthen_named_request *request) {
    thinkthen_call_result *result = NULL;
    thinkthen_call_error *error = NULL;
    int code = thinkthen_run_call(engine, request, NULL, NULL, NULL, &result, &error);
    check(code == THINKTHEN_EUSAGE && error && !result, "expected owned usage failure");
    check(thinkthen_call_error_code(error) == code, "owned error preserves kind");
    return error;
}
static THREAD_RESULT print_error(void *argument) {
    thinkthen_call_error *error = argument;
    const size_t *position = thinkthen_call_error_position(error);
    check(position != NULL, "native failure position is missing");
    printf("{\"position\":%zu,\"code\":%d,\"retryable\":%s,\"message\":",
           *position, thinkthen_call_error_code(error),
           thinkthen_call_error_retryable(error) ? "true" : "false");
    quoted(thinkthen_call_error_message(error));
    thinkthen_complete_utf8 facts = thinkthen_call_error_facts_json(error);
    fputs(",\"facts\":", stdout);
    if (facts.len) fwrite(facts.data, 1, facts.len, stdout); else fputs("null", stdout);
    puts("}");
    return THREAD_DONE;
}
struct produced { char **argv; thinkthen_call_error *error; const size_t *position; size_t value; };
static THREAD_RESULT create_errors(void *argument) {
    struct produced *produced = argument;
    char **argv = produced->argv;
    thinkthen_engine *engine = thinkthen_engine_new_with(argv[1]);
    check(engine != NULL, "engine construction failed");
    thinkthen_named_request request = {text(argv[2]), text(argv[3]), text(argv[4]), text(argv[5])};
    thinkthen_call_error *first = failed_call(engine, &request);
    const size_t *saved = thinkthen_call_error_position(first);
    check(saved != NULL, "native failure position is missing");
    size_t expected = *saved;
    thinkthen_call_error *second = failed_call(engine, &request);
    const size_t *other = thinkthen_call_error_position(second);
    check(other && other != saved && *other == expected, "errors own independent positions");
    request.function = text("unknown");
    thinkthen_call_error *absent = failed_call(engine, &request);
    check(thinkthen_call_error_position(absent) == NULL, "unlocated failure has no position");
    thinkthen_engine_free(engine);
    thinkthen_call_error_free(first);
    thinkthen_call_error_free(absent);
    check(*other == expected, "position survives engine and independent error destruction");
    produced->error = second;
    produced->position = other;
    produced->value = expected;
    return THREAD_DONE;
}
int main(int argc, char **argv) {
    if (argc < 6) return 2;
    check(thinkthen_call_error_position(NULL) == NULL, "null owner has no position");
    struct produced produced = {argv, NULL, NULL, 0};
    THREAD_TYPE producer;
    check(!fixture_start(&producer, create_errors, &produced), "producer thread failed");
    check(!fixture_join(producer), "producer join failed");
    check(produced.position && *produced.position == produced.value,
          "borrowed position survives the producing thread");
    THREAD_TYPE reader;
    check(!fixture_start(&reader, print_error, produced.error), "reader thread failed");
    check(!fixture_join(reader), "reader join failed");
    thinkthen_call_error_free(produced.error);
    thinkthen_call_error_free(NULL);
    return 0;
}
