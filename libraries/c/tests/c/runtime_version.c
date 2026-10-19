#include "thinkthen.h"
#include <stdio.h>

int main(void) {
    uint32_t loaded = 0;
    const char *version = thinkthen_version(&loaded);
    if (loaded != THINKTHEN_INTERFACE_NUMBER) {
        fprintf(stderr, "ThinkThen C interface mismatch: header requires %u, loaded library %s provides %u\n",
                (unsigned)THINKTHEN_INTERFACE_NUMBER, version, (unsigned)loaded);
        return 78;
    }
    thinkthen_engine *engine = thinkthen_engine_new();
    if (!engine) return 1;
    thinkthen_engine_free(engine);
    printf("%s\n", version);
    return 0;
}
