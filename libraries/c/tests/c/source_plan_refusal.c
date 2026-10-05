/* Bounded source planning refuses while preserving caller-owned outputs. */
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
    if (argc != 4) return 1;
    thinkthen_engine *engine = thinkthen_engine_new_with(argv[1]);
    if (!engine) return 2;
    char sentinel[] = "kept";
    char *out = sentinel;
    size_t length = 7;
    int code = thinkthen_plan_json(engine, argv[2], &out, &length);
    const char *message = thinkthen_error_message(engine);
    int failed = code != THINKTHEN_EUSAGE || out != sentinel || length != 7 ||
                 !message || strcmp(message, argv[3]);
    if (failed) fprintf(stderr, "code %d: %s\n", code, message ? message : "none");
    if (out != sentinel) thinkthen_free_string(out);
    thinkthen_engine_free(engine);
    return failed;
}
