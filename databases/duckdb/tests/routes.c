/* Link only into explicit fixture extensions, never distributed packages. */
#include "thinkthen_fixture.h"
#include <stdio.h>
#include <string.h>

static thinkthen_fixture_routes *routes;

__attribute__((constructor)) static void install_routes(void) {
    const char *url = getenv("THINKTHEN_TEST_LOOPBACK_URL");
    if (url == NULL) return;
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version())) abort();
    thinkthen_error error = { .struct_size = sizeof(error) };
    if (thinkthen_fixture_routes_install((const uint8_t *)url, strlen(url),
            (const uint8_t *)url, strlen(url), &routes, &error) != THINKTHEN_OK) {
        const char *name = thinkthen_code_name(error.code);
        fprintf(stderr, "%s: %.*s\n", name != NULL ? name : "unknown",
                (int)error.message.len, (const char *)error.message.data);
        thinkthen_error_free(&error);
        abort();
    }
}

__attribute__((destructor)) static void release_routes(void) {
    thinkthen_fixture_routes_free(routes);
}
