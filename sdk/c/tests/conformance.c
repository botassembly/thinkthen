/* Raw canonical fixture transport through the installed C header. */
#include <thinkthen.h>
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void member(const char *name, thinkthen_buffer value) {
    if (!value.len) return;
    printf(",\"%s\":", name);
    assert(fwrite(value.data, 1, value.len, stdout) == value.len);
}

#ifndef _WIN32
#include "stress.h"
#endif

int main(int argc, char **argv) {
#ifndef _WIN32
    if (argc > 1 && strncmp(argv[1], "stress", 6) == 0) return stress_main(argc, argv);
#endif
    assert(argc == 3 || (argc == 4 && strcmp(argv[3], "cancelled") == 0));
    assert(thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()));
    thinkthen_client *client = NULL;
    thinkthen_call *call = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    int32_t status = thinkthen_client_open_json((const uint8_t *)argv[2], strlen(argv[2]), &client, &error);
    if (status == THINKTHEN_OK)
        status = thinkthen_call_open_json(client, (const uint8_t *)argv[1], strlen(argv[1]),
                                          THINKTHEN_OFFSET_CODE_POINTS, &call, &error);
    thinkthen_client_free(client);
    /* Initial records are copied by the engine; closing adds no extra record. */
    if (status == THINKTHEN_OK) {
        if (argc == 4) thinkthen_call_cancel(call);
        thinkthen_call_end_input(call);
    }
    printf("{\"rows\":[");
    size_t count = 0;
    size_t kept_count = 0;
    unsigned char *kept = NULL;
    while (status == THINKTHEN_OK) {
        thinkthen_row row = {.struct_size = sizeof row};
        status = thinkthen_call_next(call, &row, &error);
        if (status == THINKTHEN_OK) {
            if (row.function == THINKTHEN_FUNCTION_FILTER) {
                unsigned char *grown = realloc(kept, kept_count + 1);
                assert(grown != NULL);
                kept = grown;
                kept[kept_count++] = row.status == THINKTHEN_STATUS_ANSWERED && row.answer_bool != 0;
            }
            if (count++) putchar(',');
            const char *state = row.status == THINKTHEN_STATUS_ANSWERED ? "answered"
                : row.status == THINKTHEN_STATUS_UNSURE ? "unsure" : "failed";
            printf("{\"function\":%u,\"position\":%llu,\"status\":\"%s\"",
                   row.function, (unsigned long long)row.position, state);
            member("id", row.id);
            member("answer", row.answer);
            member("detail", row.detail);
            member("source", row.source);
            member("error", row.error);
            if (!isnan(row.probability)) printf(",\"probability\":%.17g", row.probability);
            putchar('}');
        }
        thinkthen_row_free(&row);
    }
    printf("],\"kept\":[");
    for (size_t i = 0; i < kept_count; ++i) printf("%s%s", i ? "," : "", kept[i] ? "true" : "false");
    printf("]");
    free(kept);
    if (status == THINKTHEN_ERROR)
        printf(",\"error\":\"%s\"", thinkthen_code_name(error.code));
    else assert(status == THINKTHEN_END);
    puts("}");
    thinkthen_call_free(call);
    thinkthen_error_free(&error);
    return 0;
}
