/* The JSON constructor keeps the old entry point, refuses hostile bytes,
 * and holds the process throttle across engines. */
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

static int failed;

static void check(int holds, const char *what) {
    if (!holds) {
        fprintf(stderr, "FAIL %s\n", what);
        failed = 1;
    }
}

int main(void) {
    thinkthen_engine *old = thinkthen_engine_new();
    thinkthen_engine *empty = thinkthen_engine_new_with(NULL);
    thinkthen_engine *object = thinkthen_engine_new_with("{}");
    check(old != NULL && empty != NULL && object != NULL, "empty settings match the old door");
    thinkthen_engine_free(old);
    thinkthen_engine_free(empty);
    thinkthen_engine_free(object);

    const char bad[] = "{\"\xff\":1}";
    check(thinkthen_engine_new_with(bad) == NULL, "invalid UTF-8 is refused");
    check(thinkthen_error_code(NULL) == THINKTHEN_EUSAGE, "invalid UTF-8 is usage");

    thinkthen_engine *first = thinkthen_engine_new_with("{\"throttle\":4}");
    check(first != NULL, "the first throttle is active");
    thinkthen_engine *second = thinkthen_engine_new_with("{\"throttle\":2}");
    check(second == NULL, "another throttle is refused");
    check(thinkthen_error_code(NULL) == THINKTHEN_EUSAGE, "throttle conflict is usage");
    check(strstr(thinkthen_error_message(NULL), "throttle") != NULL,
          "throttle conflict names the setting");
    thinkthen_engine_free(first);
    thinkthen_engine_free(second);
    return failed;
}
