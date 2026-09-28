#define _DARWIN_C_SOURCE 1
#include <limits.h>
#include <mach-o/dyld.h>
#include <sqlite3.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int resident(const char *path) {
    char target[PATH_MAX];
    if (!realpath(path, target)) return -1;
    for (uint32_t i = 0; i < _dyld_image_count(); i++) {
        const char *name = _dyld_get_image_name(i);
        char current[PATH_MAX];
        if (name && realpath(name, current) && strcmp(current, target) == 0) return 1;
    }
    return 0;
}

static int scalar(sqlite3 *db, const char *sql, char *result, size_t size) {
    sqlite3_stmt *statement = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (rc != SQLITE_OK) return rc;
    rc = sqlite3_step(statement);
    if (rc == SQLITE_ROW) {
        const unsigned char *value = sqlite3_column_text(statement, 0);
        if (value) snprintf(result, size, "%s", value);
        rc = SQLITE_OK;
    }
    sqlite3_finalize(statement);
    return rc;
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    sqlite3 *db = NULL;
    if (sqlite3_open(":memory:", &db) != SQLITE_OK) return 3;
    sqlite3_enable_load_extension(db, 1);
    char *error = NULL;
    int load = sqlite3_load_extension(db, argv[1], NULL, &error);
    printf("version=%s\nload=%d\nerror=%s\n", sqlite3_libversion(), load, error ? error : "");
    if (error) sqlite3_free(error);
    if (load == SQLITE_OK) {
        char usage[512] = {0};
        int call = scalar(db, "SELECT thinkthen_usage()", usage, sizeof usage);
        printf("call=%d\nusage=%s\n", call, usage);
    }
    if (sqlite3_close(db) != SQLITE_OK) return 4;
    sqlite3 *later = NULL;
    if (sqlite3_open(":memory:", &later) != SQLITE_OK) return 5;
    char answer[32] = {0};
    int later_call = scalar(later, "SELECT 7", answer, sizeof answer);
    if (sqlite3_close(later) != SQLITE_OK) return 6;
    printf("later_call=%d\nlater=%s\nresident=%d\n", later_call, answer, resident(argv[1]));
    return later_call == SQLITE_OK ? 0 : 7;
}
