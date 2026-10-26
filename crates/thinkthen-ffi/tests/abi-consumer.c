/* A loader refuses the installed ABI fixtures before any client or row access. */
#include "thinkthen.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>
#ifdef _WIN32
#include <windows.h>
static HMODULE library;
static void *symbol(const char *name) { return (void *)GetProcAddress(library, name); }
static void load(const char *filename) { library = LoadLibraryA(filename); assert(library); }
#else
#include <dlfcn.h>
static void *library;
static void *symbol(const char *name) { return dlsym(library, name); }
static void load(const char *filename) { library = dlopen(filename, RTLD_NOW | RTLD_LOCAL); assert(library); }
#endif

typedef uint32_t (*version_fn)(void);
typedef int32_t (*client_fn)(const uint8_t *, size_t, thinkthen_client **, thinkthen_error *);
typedef void (*client_free_fn)(thinkthen_client *);
typedef int32_t (*question_fn)(uint32_t, const uint8_t *, size_t, thinkthen_question **, thinkthen_error *);
typedef void (*question_free_fn)(thinkthen_question *);
typedef int32_t (*run_fn)(const thinkthen_client *, const thinkthen_question *, const uint8_t *, size_t, uint32_t, thinkthen_row *, thinkthen_error *);
typedef void (*row_free_fn)(thinkthen_row *);
typedef int32_t (*open_fn)(const thinkthen_client *, const thinkthen_question *, const thinkthen_text *, uint32_t, thinkthen_call **, thinkthen_error *);
typedef void (*call_free_fn)(thinkthen_call *);
_Static_assert(_Generic(&thinkthen_abi_version, version_fn: 1, default: 0), "version signature");
_Static_assert(_Generic(&thinkthen_client_open_json, client_fn: 1, default: 0), "client signature");
_Static_assert(_Generic(&thinkthen_run_one, run_fn: 1, default: 0), "run signature");
_Static_assert(_Generic(&thinkthen_call_open_question, open_fn: 1, default: 0), "typed open signature");
static void execute(void) {
    client_fn client_open = (client_fn)symbol("thinkthen_client_open_json");
    client_free_fn client_free = (client_free_fn)symbol("thinkthen_client_free");
    question_fn question_new = (question_fn)symbol("thinkthen_question_new");
    question_free_fn question_free = (question_free_fn)symbol("thinkthen_question_free");
    run_fn run_one = (run_fn)symbol("thinkthen_run_one");
    row_free_fn row_free = (row_free_fn)symbol("thinkthen_row_free");
    open_fn open_question = (open_fn)symbol("thinkthen_call_open_question");
    call_free_fn call_free = (call_free_fn)symbol("thinkthen_call_free");
    const char *settings = "{\"backend\":\"stub\"}";
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_client *client = NULL;
    assert(client_open((const uint8_t *)settings, strlen(settings), &client, &error) == THINKTHEN_OK);
    thinkthen_question *question = NULL;
    assert(question_new(THINKTHEN_FUNCTION_DECIDE, (const uint8_t *)"Ready?", 6, &question, &error) == THINKTHEN_OK);
    thinkthen_call *call = NULL;
    assert(open_question(client, question, NULL, THINKTHEN_OFFSET_CODE_POINTS, &call, &error) == THINKTHEN_OK);
    call_free(call);
    thinkthen_row row = {0}; row.struct_size = sizeof(row);
    assert(run_one(client, question, NULL, 0, THINKTHEN_RECORD_MISSING, &row, &error) == THINKTHEN_OK);
    assert(row.status == THINKTHEN_STATUS_FAILED && row.error_code == THINKTHEN_CODE_RECORD_MISSING);
    assert(row.position == 1);
    question_free(question); client_free(client); row_free(&row);
}
int main(int argc, char **argv) {
    assert(argc >= 2); load(argv[1]);
    version_fn version = (version_fn)symbol("thinkthen_abi_version");
    if (!version) { puts("usage.abi"); return 0; }
    uint32_t actual = version();
    if ((actual >> 16) != (THINKTHEN_ABI_VERSION >> 16) || (actual & 65535u) < (THINKTHEN_ABI_VERSION & 65535u)) { puts("usage.abi"); return 0; }
    for (int index = 2; index < argc; index++) if (!symbol(argv[index])) { puts("usage.abi"); return 0; }
    execute(); puts("accepted"); return 0;
}
