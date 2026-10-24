#include "internal.h"
#include <limits.h>
#include <string.h>

static int native_failure(Cursor *cursor, int code) {
    Table *table = (Table *)cursor->base.pVtab;
    return cursor_error(cursor, code, sqlite3_errmsg(table->db));
}

static int bind_parameters(Cursor *cursor) {
    const char *text = cursor->arguments[3] == NULL ? "[]" :
        (const char *)sqlite3_value_text(cursor->arguments[3]);
    size_t length = cursor->arguments[3] == NULL ? 2 :
        (size_t)sqlite3_value_bytes(cursor->arguments[3]);
    if (text == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    json_error_t failure = {0};
    json_t *parameters = json_loadb(text, length, JSON_DECODE_ANY | JSON_ALLOW_NUL, &failure);
    if (parameters == NULL || !json_is_array(parameters)) {
        json_decref(parameters);
        return cursor_error(cursor,
            json_error_code(&failure) == json_error_out_of_memory ? SQLITE_NOMEM : SQLITE_MISMATCH,
            "source parameters must be a JSON array of SQLite values");
    }
    int slots = sqlite3_bind_parameter_count(cursor->source);
    if (json_array_size(parameters) != (size_t)slots) {
        json_decref(parameters);
        return cursor_error(cursor, SQLITE_RANGE, "source parameter count does not match its bindings");
    }
    int result = SQLITE_OK;
    for (int index = 0; index < slots; ++index) {
        json_t *value = json_array_get(parameters, (size_t)index);
        int slot = index + 1;
        if (json_is_string(value))
            result = sqlite3_bind_text64(cursor->source, slot, json_string_value(value),
                json_string_length(value), SQLITE_TRANSIENT, SQLITE_UTF8);
        else if (json_is_integer(value))
            result = sqlite3_bind_int64(cursor->source, slot, json_integer_value(value));
        else if (json_is_real(value))
            result = sqlite3_bind_double(cursor->source, slot, json_real_value(value));
        else if (json_is_boolean(value))
            result = sqlite3_bind_int(cursor->source, slot, json_is_true(value));
        else if (json_is_null(value))
            result = sqlite3_bind_null(cursor->source, slot);
        else { result = SQLITE_MISMATCH; break; }
        if (result != SQLITE_OK) break;
    }
    json_decref(parameters);
    if (result == SQLITE_MISMATCH)
        return cursor_error(cursor, result, "source parameters cannot contain arrays or objects");
    return result == SQLITE_OK ? SQLITE_OK : native_failure(cursor, result);
}

int source_open(Cursor *cursor) {
    Table *table = (Table *)cursor->base.pVtab;
    const char *sql = (const char *)sqlite3_value_text(cursor->arguments[0]);
    int length = sqlite3_value_bytes(cursor->arguments[0]);
    if (sql == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    if (memchr(sql, '\0', (size_t)length) != NULL)
        return cursor_error(cursor, SQLITE_ERROR, "source SQL contains a zero byte");
    const char *tail = NULL;
    int result = sqlite3_prepare_v3(table->db, sql, length, 0, &cursor->source, &tail);
    if (result != SQLITE_OK) return native_failure(cursor, result);
    if (cursor->source == NULL)
        return cursor_error(cursor, SQLITE_ERROR, "source SQL must contain one row-producing statement");
    sqlite3_stmt *extra = NULL;
    int remaining = length - (int)(tail - sql);
    if (remaining != 0) {
        result = sqlite3_prepare_v3(table->db, tail, remaining, 0, &extra, NULL);
        if (result != SQLITE_OK) return native_failure(cursor, result);
        if (extra != NULL) {
            sqlite3_finalize(extra);
            return cursor_error(cursor, SQLITE_ERROR, "source SQL must contain only one statement");
        }
    }
    if (!sqlite3_stmt_readonly(cursor->source))
        return cursor_error(cursor, SQLITE_READONLY, "source SQL must be read-only");
    if (sqlite3_column_count(cursor->source) != 1)
        return cursor_error(cursor, SQLITE_MISMATCH, "source SQL must return exactly one column named record");
    const char *name = sqlite3_column_name(cursor->source, 0);
    if (name == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    if (sqlite3_stricmp(name, "record") != 0)
        return cursor_error(cursor, SQLITE_MISMATCH, "source SQL must return exactly one column named record");
    return bind_parameters(cursor);
}

static int sorted_record(sqlite3 *db, const unsigned char *text, int length, char **message) {
    int first = 0;
    while (first < length && (text[first] == ' ' || text[first] == '\t' ||
           text[first] == '\r' || text[first] == '\n')) ++first;
    if (first == length || (text[first] != '{' && text[first] != '[')) return SQLITE_OK;
    /* SQLite's JSON parser accepts the same numeric range as its constructors.
     * A host JSON parser could mistake a valid object with a large number for text. */
    sqlite3_stmt *check = NULL;
    int result = sqlite3_prepare_v2(db, "SELECT json_valid(?1)", -1, &check, NULL);
    if (result == SQLITE_OK)
        result = sqlite3_bind_text(check, 1, (const char *)text, length, SQLITE_STATIC);
    if (result == SQLITE_OK) result = sqlite3_step(check);
    if (result == SQLITE_ROW) {
        result = sqlite3_column_int(check, 0) ? SQLITE_MISMATCH : SQLITE_OK;
        if (result != SQLITE_OK)
            *message = sqlite3_mprintf("source sort lost a possible JSON subtype; stage ordered input "
                "in a table with an INTEGER PRIMARY KEY and apply json(document) in the final indexed projection");
    } else {
        *message = sqlite3_mprintf("%s", sqlite3_errmsg(db));
    }
    sqlite3_finalize(check);
    return result;
}

int source_chunk(Cursor *cursor) {
    enum { CHUNK_RECORDS = 128 };
    Table *table = (Table *)cursor->base.pVtab;
    thinkthen_sql_record records[CHUNK_RECORDS] = {{0}};
    sqlite3_value *values[CHUNK_RECORDS] = {0};
    size_t count = 0;
    int result = SQLITE_OK;
    int ended = 0;
    char *message = NULL;
    while (count < CHUNK_RECORDS) {
        if (sqlite3_is_interrupted(table->db)) { result = SQLITE_INTERRUPT; break; }
        result = sqlite3_step(cursor->source);
        if (result == SQLITE_DONE) { ended = 1; result = SQLITE_OK; break; }
        if (result != SQLITE_ROW) break;
        if (sqlite3_column_type(cursor->source, 0) != SQLITE_TEXT) {
            result = SQLITE_MISMATCH;
            break;
        }
        /* column_value is unprotected: duplicate before inspecting its subtype.
         * SQLite JSON text uses subtype 74 ('J'); see src/json.c JSON_SUBTYPE.
         * These are source-column values, not scalar-function arguments, so
         * SQLITE_SUBTYPE registration flags do not apply to this virtual table. */
        values[count] = sqlite3_value_dup(sqlite3_column_value(cursor->source, 0));
        if (values[count] == NULL) { result = SQLITE_NOMEM; break; }
        records[count].kind = sqlite3_value_subtype(values[count]) == 74
            ? THINKTHEN_SQL_JSON : THINKTHEN_SQL_TEXT;
        const unsigned char *text = sqlite3_value_text(values[count]);
        int length = sqlite3_value_bytes(values[count]);
        if (text == NULL) { result = SQLITE_NOMEM; break; }
        if (records[count].kind == THINKTHEN_SQL_TEXT &&
            sqlite3_stmt_status(cursor->source, SQLITE_STMTSTATUS_SORT, 0) != 0) {
            result = sorted_record(table->db, text, length, &message);
            if (result != SQLITE_OK) break;
        }
        records[count].data = text;
        records[count].len = (size_t)length;
        ++count;
        result = SQLITE_OK;
    }
    /* Preserve SQLite's diagnostic before freeing or finalizing anything. */
    if (result != SQLITE_OK && message == NULL)
        message = sqlite3_mprintf("%s",
            result == SQLITE_MISMATCH ? "source record must contain TEXT, not SQL NULL or another storage class" :
            result == SQLITE_NOMEM ? "out of memory" : sqlite3_errmsg(table->db));
    if (result == SQLITE_OK && count != 0) {
        thinkthen_buffer error = {0};
        if (thinkthen_sql_send_chunk(cursor->call, records, count, &error) != THINKTHEN_OK)
            result = engine_error(&error, &message);
    }
    for (size_t index = 0; index < CHUNK_RECORDS; ++index)
        sqlite3_value_free(values[index]);
    if (result != SQLITE_OK) {
        int code = cursor_error(cursor, message == NULL ? SQLITE_NOMEM : result,
                                message == NULL ? "out of memory" : message);
        sqlite3_free(message);
        return code;
    }
    if (ended) {
        result = sqlite3_finalize(cursor->source);
        cursor->source = NULL;
        if (result != SQLITE_OK) return native_failure(cursor, result);
        thinkthen_sql_end_input(cursor->call);
    }
    return SQLITE_OK;
}
