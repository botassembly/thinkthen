#include "loader.h"
#ifdef _WIN32
#include <windows.h>
#include <stdlib.h>
#else
#include <dlfcn.h>
#endif

#define SYMBOLS(X) \
    X(client_open) X(client_free) X(call_open) X(send_text) X(send_json) \
    X(end_input) X(next) X(cancel) X(free) X(buffer_free) X(row_free)
#define DECLARE(name) static __typeof__(&thinkthen_##name) p_##name;
SYMBOLS(DECLARE)

const char *tt_load(const char *path) {
#ifdef _WIN32
    int count = MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, path, -1, NULL, 0);
    if (!count) return "Invalid native library path";
    wchar_t *wide = malloc((size_t)count * sizeof(*wide));
    if (!wide) return "Cannot allocate native library path";
    MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, path, -1, wide, count);
    HMODULE handle = LoadLibraryW(wide);
    free(wide);
    if (!handle) return "Cannot load packaged ThinkThen native engine";
#define LOAD(name) p_##name = (__typeof__(p_##name))GetProcAddress(handle, "thinkthen_" #name); if (!p_##name) return "Missing ThinkThen native symbol: " #name;
#else
    void *handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
    if (!handle) return dlerror();
    /* The handle stays loaded, including after a symbol lookup failure. */
#define LOAD(name) p_##name = (__typeof__(p_##name))dlsym(handle, "thinkthen_" #name); if (!p_##name) return dlerror();
#endif
    SYMBOLS(LOAD)
    return NULL;
}
int32_t tt_client_open(const uint8_t *p, size_t n, thinkthen_client **out, thinkthen_buffer *e) { return p_client_open(p,n,out,e); }
void tt_client_free(thinkthen_client *p) { p_client_free(p); }
int32_t tt_call_open(const thinkthen_client *c, const uint8_t *p, size_t n, uint32_t u, thinkthen_call **out, thinkthen_buffer *e) { return p_call_open(c,p,n,u,out,e); }
int32_t tt_send_text(thinkthen_call *c, const uint8_t *p, size_t n, thinkthen_buffer *e) { return p_send_text(c,p,n,e); }
int32_t tt_send_json(thinkthen_call *c, const uint8_t *p, size_t n, thinkthen_buffer *e) { return p_send_json(c,p,n,e); }
void tt_end_input(thinkthen_call *c) { p_end_input(c); }
int32_t tt_next(thinkthen_call *c, thinkthen_row *r, thinkthen_buffer *e) { return p_next(c,r,e); }
void tt_cancel(thinkthen_call *c) { p_cancel(c); }
void tt_free(thinkthen_call *c) { p_free(c); }
void tt_buffer_free(thinkthen_buffer *b) { p_buffer_free(b); }
void tt_row_free(thinkthen_row *r) { p_row_free(r); }
