/* A source call carries one deadline through every native recognition. */
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
    if (argc != 2) return 1;
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}");
    if (!engine) return 2;
    /* The second response stays held while the deadline stops the call. */
    char *out = thinkthen_call_opts(engine, argv[1], 1000, NULL);
    int failed = out != NULL || thinkthen_error_code(engine) != THINKTHEN_EDEADLINE;
    /* A started deadline retains its configured budget and expiration. */
    failed |= strcmp(thinkthen_error_message(engine), "the deadline of 1 s passed before the call answered") != 0;
    if (failed) fprintf(stderr, "code %d: %s\n", thinkthen_error_code(engine), thinkthen_error_message(engine));
    const char *facts = thinkthen_error_facts_json(engine);
    if (facts) puts(facts);
    thinkthen_free_string(out);
    thinkthen_engine_free(engine);
    return failed;
}
