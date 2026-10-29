#define _POSIX_C_SOURCE 200809L
#include <sqlite3.h>
#include "../src/source_limits.h"
#include <jansson.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* An installed SQLite host, controlled by JSON lines from the outside tests. */
static sqlite3 *database;
static sqlite3_int64 ticks;
static sqlite3_stmt *cursors[2];

static void tick(sqlite3_context *context, int argc, sqlite3_value **values) {
    (void)argc;
    ++ticks;
    sqlite3_result_value(context, values[0]);
}

static void source_chunk_limit(sqlite3_context *context, int argc, sqlite3_value **values) {
    (void)argc; (void)values;
    sqlite3_result_int(context, TT_SOURCE_CHUNK_RECORDS);
}

static void native_memory(sqlite3_context *context, int argc, sqlite3_value **values) {
    (void)argc; (void)values;
    sqlite3_result_int64(context, sqlite3_memory_used());
}

static void tick_count(sqlite3_context *context, int argc, sqlite3_value **values) {
    (void)argc; (void)values;
    sqlite3_result_int64(context, ticks);
}

static void *interrupt_connection(void *context) {
    (void)context;
    if (getchar() == 'i') sqlite3_interrupt(database);
    return NULL;
}

static json_t *column(sqlite3_stmt *statement, int index) {
    switch (sqlite3_column_type(statement, index)) {
    case SQLITE_INTEGER: return json_integer(sqlite3_column_int64(statement, index));
    case SQLITE_FLOAT: return json_real(sqlite3_column_double(statement, index));
    case SQLITE_TEXT: return json_stringn((const char *)sqlite3_column_text(statement, index),
                                         (size_t)sqlite3_column_bytes(statement, index));
    default: return json_null();
    }
}

static json_t *run(json_t *command) {
    if (json_is_true(json_object_get(command, "interrupt_before"))) sqlite3_interrupt(database);
    json_t *rows = json_array();
    json_t *names = json_array();
    sqlite3_stmt *statement = NULL;
    json_t *slot = json_object_get(command, "slot");
    const char *operation = json_string_value(json_object_get(command, "operation"));
    int index = slot == NULL ? 0 : (int)json_integer_value(slot);
    if (index < 0 || index > 1) exit(3);
    int result = SQLITE_OK;
    if (operation != NULL && strcmp(operation, "close") == 0) {
        result = sqlite3_finalize(cursors[index]);
        cursors[index] = NULL;
    } else {
        if (operation != NULL && strcmp(operation, "step") == 0) statement = cursors[index];
        else {
            const char *sql = json_string_value(json_object_get(command, "sql"));
            result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
            json_t *parameters = json_object_get(command, "parameters");
            for (size_t n = 0; result == SQLITE_OK && n < json_array_size(parameters); ++n) {
                json_t *value = json_array_get(parameters, n);
                if (json_is_string(value)) result = sqlite3_bind_text(statement, (int)n + 1,
                    json_string_value(value), (int)json_string_length(value), SQLITE_TRANSIENT);
                else if (json_is_integer(value)) result = sqlite3_bind_int64(statement, (int)n + 1, json_integer_value(value));
                else if (json_is_real(value)) result = sqlite3_bind_double(statement, (int)n + 1, json_real_value(value));
                else if (json_is_object(value)) result = sqlite3_bind_blob(statement, (int)n + 1, "blob", 4, SQLITE_STATIC);
                else result = sqlite3_bind_null(statement, (int)n + 1);
            }
        }
        if (result == SQLITE_OK && statement != NULL) {
            for (int n = 0; n < sqlite3_column_count(statement); ++n)
                json_array_append_new(names, json_string(sqlite3_column_name(statement, n)));
            pthread_t thread;
            int interruptible = json_is_true(json_object_get(command, "interruptible"));
            if (interruptible && pthread_create(&thread, NULL, interrupt_connection, NULL) != 0) exit(3);
            while ((result = sqlite3_step(statement)) == SQLITE_ROW) {
                json_t *row = json_array();
                for (int n = 0; n < sqlite3_column_count(statement); ++n)
                    json_array_append_new(row, column(statement, n));
                json_array_append_new(rows, row);
                if (operation != NULL && (strcmp(operation, "open") == 0 || strcmp(operation, "step") == 0)) break;
            }
            if (interruptible) pthread_join(thread, NULL);
        }
        if (operation != NULL && strcmp(operation, "open") == 0) cursors[index] = statement;
        else if (operation == NULL || strcmp(operation, "step") != 0) {
            /* Save the diagnostic before finalize can replace it. */
            char *message = sqlite3_mprintf("%s", sqlite3_errmsg(database));
            sqlite3_finalize(statement);
            json_t *response = json_pack("{s:o,s:o,s:i,s:s}", "rows", rows, "columns", names,
                                         "code", result, "message", message);
            sqlite3_free(message);
            return response;
        }
    }
    return json_pack("{s:o,s:o,s:i,s:s}", "rows", rows, "columns", names,
                     "code", result, "message", sqlite3_errmsg(database));
}

int main(int argc, char **argv) {
    if ((argc != 2 && argc != 3) || sqlite3_open(":memory:", &database) != SQLITE_OK) return 2;
    sqlite3_enable_load_extension(database, 1);
    char *error = NULL;
    if (sqlite3_load_extension(database, argv[1], "sqlite3_thinkthensqlite_init", &error) != SQLITE_OK) {
        if (argc == 3) {
            sqlite3_free(error);
            sqlite3_stmt *statement = NULL;
            if (sqlite3_prepare_v2(database, "SELECT * FROM thinkthen_decide('SELECT 1 AS record','Ready?')", -1, &statement, NULL) == SQLITE_OK) return 3;
            sqlite3_finalize(statement);
            if (sqlite3_prepare_v2(database, "SELECT thinkthen_set('backend','stub')", -1, &statement, NULL) == SQLITE_OK) return 3;
            sqlite3_finalize(statement);
            if (sqlite3_prepare_v2(database, "SELECT count(*) FROM pragma_module_list WHERE name GLOB 'thinkthen_*'", -1, &statement, NULL) != SQLITE_OK ||
                sqlite3_step(statement) != SQLITE_ROW || sqlite3_column_int(statement, 0) != 0) return 3;
            sqlite3_finalize(statement);
            if (sqlite3_exec(database, "CREATE TABLE reused(value); INSERT INTO reused VALUES(1)", NULL, NULL, NULL) != SQLITE_OK) return 3;
            return sqlite3_close(database) == SQLITE_OK ? 0 : 3;
        }
        fprintf(stderr, "%s\n", error);
        sqlite3_free(error);
        return 2;
    }
    if (argc == 3) return 3;
    sqlite3_enable_load_extension(database, 0);
    sqlite3_create_function(database, "source_chunk_limit", 0, SQLITE_UTF8, NULL, source_chunk_limit, NULL, NULL);
    sqlite3_create_function(database, "tick", 1, SQLITE_UTF8, NULL, tick, NULL, NULL);
    sqlite3_create_function(database, "tick_count", 0, SQLITE_UTF8, NULL, tick_count, NULL, NULL);
    sqlite3_create_function(database, "native_memory", 0, SQLITE_UTF8, NULL, native_memory, NULL, NULL);
    char *line = NULL;
    size_t capacity = 0;
    while (getline(&line, &capacity, stdin) >= 0) {
        if (line[0] == '\n') continue;
        json_error_t failure;
        json_t *command = json_loads(line, JSON_ALLOW_NUL, &failure);
        if (command == NULL) return 3;
        json_t *response = run(command);
        char *text = json_dumps(response, JSON_COMPACT);
        if (text == NULL) return 3;
        puts(text);
        fflush(stdout);
        free(text);
        json_decref(response);
        json_decref(command);
    }
    free(line);
    for (int n = 0; n < 2; ++n) sqlite3_finalize(cursors[n]);
    if (sqlite3_close(database) != SQLITE_OK) return 3;
    return 0;
}
