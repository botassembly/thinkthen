#include "thinkthen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#include <windows.h>
#else
#include <dlfcn.h>
#endif

int main(void) {
    uint32_t loaded = 0;
    const char *version;
    const char *path = getenv("THINKTHEN_TEST_LIBRARY");
    if (path) {
        typedef const char *(*bootstrap)(uint32_t *);
#ifdef _WIN32
        HMODULE library = LoadLibraryA(path);
        FARPROC symbol = library ? GetProcAddress(library, "thinkthen_version") : NULL;
        bootstrap check = NULL;
        if (symbol) memcpy(&check, &symbol, sizeof(check));
#else
        void *library = dlopen(path, RTLD_NOW | RTLD_LOCAL);
        bootstrap check = library ? (bootstrap)dlsym(library, "thinkthen_version") : NULL;
#endif
        if (!check) return 79;
        version = check(&loaded);
    } else {
        version = thinkthen_version(&loaded);
    }
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
