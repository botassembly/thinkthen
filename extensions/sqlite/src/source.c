#include "internal.h"
#include "source_limits.h"
#include <limits.h>
#include <string.h>

static int native_failure(Cursor *cursor, int code) {
    Table *table = (Table *)cursor->base.pVtab;
    return cursor_error(cursor, code, sqlite3_errmsg(table->db));
}

static int bind_parameters(Cursor *cursor) {
    const char *text = cursor->arguments[ARG_PARAMS] == NULL ? "[]" :
        (const char *)sqlite3_value_text(cursor->arguments[ARG_PARAMS]);
    size_t length = cursor->arguments[ARG_PARAMS] == NULL ? 2 :
        (size_t)sqlite3_value_bytes(cursor->arguments[ARG_PARAMS]);
    if (text == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    json_error_t failure = {0};
    json_t *parameters = json_loadb(text, length, JSON_DECODE_ANY | JSON_ALLOW_NUL, &failure);
    if (parameters == NULL || !json_is_array(parameters)) {
        json_decref(parameters);
        if (json_error_code(&failure) == json_error_out_of_memory)
            return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
        return usage_error(cursor, "params must be a JSON array of SQLite values");
    }
    int slots = sqlite3_bind_parameter_count(cursor->source);
    if (json_array_size(parameters) != (size_t)slots) {
        size_t count = json_array_size(parameters);
        json_decref(parameters);
        return usage_error(cursor, "params holds %llu values, but source_sql has %d parameters",
                           (unsigned long long)count, slots);
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
        return usage_error(cursor, "params cannot contain arrays or objects");
    return result == SQLITE_OK ? SQLITE_OK : native_failure(cursor, result);
}

int source_open(Cursor *cursor) {
    Table *table = (Table *)cursor->base.pVtab;
    static const char *const names[] = {"record_column", "id_column", "format"};
    for (int argument = ARG_RECORD; argument <= ARG_FORMAT; ++argument) {
        sqlite3_value *value = cursor->arguments[argument];
        if (value == NULL) continue;
        const unsigned char *text = sqlite3_value_text(value);
        if (text == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
        if (memchr(text, '\0', (size_t)sqlite3_value_bytes(value)) != NULL)
            return usage_error(cursor, "%s must not contain a zero byte", names[argument - ARG_RECORD]);
    }
    const char *sql = (const char *)sqlite3_value_text(cursor->arguments[0]);
    int length = sqlite3_value_bytes(cursor->arguments[0]);
    if (sql == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    if (memchr(sql, '\0', (size_t)length) != NULL)
        return usage_error(cursor, "source_sql must not contain a zero byte");
    const char *tail = NULL;
    int result = sqlite3_prepare_v3(table->db, sql, length, 0, &cursor->source, &tail);
    if (result != SQLITE_OK) return native_failure(cursor, result);
    if (cursor->source == NULL)
        return usage_error(cursor, "source_sql must contain one SELECT statement");
    sqlite3_stmt *extra = NULL;
    int remaining = length - (int)(tail - sql);
    if (remaining != 0) {
        result = sqlite3_prepare_v3(table->db, tail, remaining, 0, &extra, NULL);
        if (result != SQLITE_OK) return native_failure(cursor, result);
        if (extra != NULL) {
            sqlite3_finalize(extra);
            return usage_error(cursor, "source_sql must contain only one statement");
        }
    }
    if (!sqlite3_stmt_readonly(cursor->source))
        return usage_error(cursor, "source_sql must be read-only, such as a SELECT");
    const char *record=cursor->arguments[ARG_RECORD] ? (const char*)sqlite3_value_text(cursor->arguments[ARG_RECORD]) : "record";
    const char *id=cursor->arguments[ARG_ID] ? (const char*)sqlite3_value_text(cursor->arguments[ARG_ID]) : "id";
    cursor->record_column=-1;cursor->id_column=-1;
    for(int n=0;n<sqlite3_column_count(cursor->source);n++) {
        const char *name=sqlite3_column_name(cursor->source,n);
        if(sqlite3_stricmp(name,record)==0)cursor->record_column=n;
        if(sqlite3_stricmp(name,id)==0)cursor->id_column=n;
    }
    if(cursor->record_column<0)
        return usage_error(cursor,cursor->arguments[ARG_RECORD] ? "record_column '%s' names no column of source_sql"
            : "source_sql has no column named %s; name one with record_column",record);
    if(cursor->arguments[ARG_ID] && cursor->id_column<0)
        return usage_error(cursor,"id_column '%s' names no column of source_sql",id);
    cursor->json=cursor->arguments[ARG_FIELD]!=NULL;
    if(cursor->arguments[ARG_FORMAT]) {
        const char *format=(const char*)sqlite3_value_text(cursor->arguments[ARG_FORMAT]);
        if(!strcmp(format,"json"))cursor->json=1;
        else if(strcmp(format,"text"))
            return usage_error(cursor,"format must be 'text' or 'json', not '%s'",format);
        else if(cursor->json)
            return usage_error(cursor,"format 'text' cannot be combined with field, which reads JSON records");
    }
    return bind_parameters(cursor);
}

int source_chunk(Cursor *cursor) {
    Table *table = (Table *)cursor->base.pVtab;
    thinkthen_sql_input records[TT_SOURCE_CHUNK_RECORDS] = {{0}};
    sqlite3_value *values[TT_SOURCE_CHUNK_RECORDS] = {0};
    size_t count = 0;
    int result = SQLITE_OK;
    int ended = 0;
    char *message = NULL;
    while (count < TT_SOURCE_CHUNK_RECORDS) {
        if (sqlite3_is_interrupted(table->db)) { result = SQLITE_INTERRUPT; break; }
        result = sqlite3_step(cursor->source);
        if (result == SQLITE_DONE) { ended = 1; result = SQLITE_OK; break; }
        if (result != SQLITE_ROW) break;
        sqlite3_value *id = cursor->id_column<0 ? NULL : sqlite3_column_value(cursor->source,cursor->id_column);
        result = replay_save_input(cursor,id,sqlite3_column_value(cursor->source,cursor->record_column));
        if (result != SQLITE_OK) break;
        int type = sqlite3_column_type(cursor->source, cursor->record_column);
        if (type != SQLITE_TEXT) {
            records[count++].kind = type == SQLITE_NULL ? THINKTHEN_RECORD_MISSING : THINKTHEN_RECORD_INVALID;
            result = SQLITE_OK;
            continue;
        }
        values[count] = sqlite3_value_dup(sqlite3_column_value(cursor->source, cursor->record_column));
        if (values[count] == NULL) { result = SQLITE_NOMEM; break; }
        records[count].kind = cursor->json ? THINKTHEN_RECORD_JSON : THINKTHEN_RECORD_TEXT;
        const unsigned char *text = sqlite3_value_text(values[count]);
        int length = sqlite3_value_bytes(values[count]);
        if (text == NULL) { result = SQLITE_NOMEM; break; }
        records[count].data = (const char *)text;
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
        thinkthen_error error = { .struct_size = sizeof(error) };
        if (thinkthen_sql_call_send_chunk(cursor->call, records, count, &error) != THINKTHEN_OK)
            result = engine_error(&error, &message);
    }
    for (size_t index = 0; index < TT_SOURCE_CHUNK_RECORDS; ++index)
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
        thinkthen_sql_call_end_input(cursor->call);
    }
    return SQLITE_OK;
}
