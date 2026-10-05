/* Compare explicit file planning with its original evidence, with no asking. */
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
    if (argc != 4) return 1;
    thinkthen_engine *engine = thinkthen_engine_new();
    if (!engine) return 2;
    for (int i = 1; i <= 2; ++i) {
        char *out = NULL;
        size_t length = 0;
        if (thinkthen_plan_json(engine, argv[i], &out, &length) || !out || strlen(out) != length) return 3;
        puts(out);
        thinkthen_free_string(out);
    }
    char sentinel[] = "kept";
    char *out = sentinel;
    size_t length = 7;
    int code = thinkthen_plan_json(engine, argv[3], &out, &length);
    int failed = code != THINKTHEN_EUSAGE || out != sentinel || length != 7;
    thinkthen_engine_free(engine);
    return failed;
}
