#include "internal.h"
#include <string.h>
#include <math.h>
#include <stdlib.h>

int engine_error(thinkthen_error *error, char **diagnostic) {
    int result = error->code == THINKTHEN_CODE_LOCAL_CANCELLED ? SQLITE_INTERRUPT : SQLITE_ERROR;
    const char *code = thinkthen_code_name(error->code);
    *diagnostic = code == NULL
        ? sqlite3_mprintf("%s: %s", THINKTHEN_ABI_ERROR_CODE, THINKTHEN_ABI_ERROR_MESSAGE)
        : sqlite3_mprintf("%s: %.*s", code, (int)error->message.len, (const char *)error->message.data);
    thinkthen_error_free(error);
    return *diagnostic == NULL ? SQLITE_NOMEM : result;
}

void connection_release(void *pointer) {
    Connection *connection = pointer;
    if (--connection->references == 0) {
        thinkthen_client_free(connection->client);
        sqlite3_free(connection->settings);
        sqlite3_free(connection);
    }
}

int connection_client(Connection *connection, const char *settings, size_t len,
                      char **diagnostic) {
    if (connection->client != NULL && connection->settings_len == len &&
        memcmp(connection->settings, settings, len) == 0) return SQLITE_OK;
    thinkthen_client *client = NULL;
    thinkthen_error error = { .struct_size = sizeof(error) };
    if (thinkthen_sql_client_open((const uint8_t *)settings, len, &client, &error) != THINKTHEN_OK)
        return engine_error(&error, diagnostic);
    char *copy = sqlite3_malloc64(len + 1);
    if (copy == NULL) {
        thinkthen_client_free(client);
        return SQLITE_NOMEM;
    }
    memcpy(copy, settings, len);
    copy[len] = '\0';
    thinkthen_client_free(connection->client);
    sqlite3_free(connection->settings);
    connection->client = client;
    ++connection->generation;
    connection->settings = copy;
    connection->settings_len = len;
    return SQLITE_OK;
}

void cursor_clear(Cursor *cursor) {
    thinkthen_sql_call_free(cursor->call);
    cursor->call = NULL;
    if (cursor->source != NULL) sqlite3_finalize(cursor->source);
    cursor->source = NULL;
    if(!cursor->replaying)thinkthen_row_free(&cursor->row);
    else {memset(&cursor->row,0,sizeof(cursor->row));cursor->row.struct_size=sizeof(cursor->row);}
    json_decref(cursor->fields);
    cursor->fields = NULL;
    for (int i = 0; i < ARG_COUNT; ++i) {
        sqlite3_value_free(cursor->arguments[i]);
        cursor->arguments[i] = NULL;
    }
    json_decref(cursor->answer);cursor->answer=NULL;
    json_decref(cursor->error);cursor->error=NULL;
    for(size_t n=0;n<cursor->ids_count;n++)sqlite3_value_free(cursor->ids[n]);
    sqlite3_free(cursor->ids);cursor->ids=NULL;cursor->ids_count=cursor->ids_capacity=cursor->ids_start=0;cursor->ids_offset=0;
    sqlite3_finalize(cursor->read);cursor->read=NULL;
    sqlite3_finalize(cursor->save);cursor->save=NULL;
    sqlite3_close(cursor->replay);cursor->replay=NULL;
    cursor->complete=cursor->replaying=0;
    cursor->eof = 1;
}

int cursor_error(Cursor *cursor, int code, const char *message) {
    char *copy = sqlite3_mprintf("%s", message);
    cursor_clear(cursor);
    sqlite3_free(cursor->base.pVtab->zErrMsg);
    cursor->base.pVtab->zErrMsg = copy;
    return copy == NULL ? SQLITE_NOMEM : code;
}

static uint8_t interrupted(void *context) {
    return sqlite3_is_interrupted((sqlite3 *)context) != 0;
}

int cursor_advance(Cursor *cursor) {
    Table *table = (Table *)cursor->base.pVtab;
    if(!cursor->replaying && cursor->row.position && cursor->row.function!=THINKTHEN_FUNCTION_RANK && cursor->row.position>cursor->ids_offset && cursor->row.position-cursor->ids_offset<=cursor->ids_count) {
        size_t previous=(size_t)(cursor->row.position-cursor->ids_offset)-1;
        sqlite3_value_free(cursor->ids[previous]);cursor->ids[previous]=NULL;cursor->ids_start=previous+1;
    }
    if(!cursor->replaying)thinkthen_row_free(&cursor->row);
    json_decref(cursor->fields);
    cursor->fields = NULL;
    json_decref(cursor->answer);cursor->answer=NULL;
    json_decref(cursor->error);cursor->error=NULL;
    if(cursor->replaying) {
        int result=replay_next(cursor);
        if(result!=SQLITE_OK || cursor->eof)return result;
        goto decoded;
    }
    for (;;) {
        if (sqlite3_is_interrupted(table->db))
            return cursor_error(cursor, SQLITE_INTERRUPT, "interrupted");
        thinkthen_error error = { .struct_size = sizeof(error) };
        int result = thinkthen_sql_call_poll(cursor->call, 10, interrupted, table->db,
                                       &cursor->row, &error);
        if (sqlite3_is_interrupted(table->db)) {
            thinkthen_error_free(&error);
            return cursor_error(cursor, SQLITE_INTERRUPT, "interrupted");
        }
        if (result == THINKTHEN_PENDING) continue;
        if (result == THINKTHEN_NEEDS_INPUT) {
            int code = source_chunk(cursor);
            if (code != SQLITE_OK) return code;
            continue;
        }
        if (result == THINKTHEN_ERROR) {
            char *message = NULL;
            int code = engine_error(&error, &message);
            if (code == SQLITE_NOMEM) return cursor_error(cursor, code, "out of memory");
            code = cursor_error(cursor, code, message);
            sqlite3_free(message);
            return code;
        }
        if (result == THINKTHEN_END) {
            cursor->complete = 1;
            cursor->eof = 1;
            thinkthen_sql_call_free(cursor->call);
            cursor->call = NULL;
            return SQLITE_OK;
        }
        int saved=replay_save(cursor);if(saved!=SQLITE_OK)return saved;
        ++cursor->rowid;
        decoded:
        if (cursor->row.error_code && thinkthen_code_name(cursor->row.error_code) == NULL)
            return cursor_error(cursor, SQLITE_ERROR, THINKTHEN_ABI_ERROR_CODE ": " THINKTHEN_ABI_ERROR_MESSAGE);
        if(cursor->row.answer.data)cursor->answer=json_loadb((const char*)cursor->row.answer.data,cursor->row.answer.len,JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL);
        if(cursor->row.error.data)cursor->error=json_loadb((const char*)cursor->row.error.data,cursor->row.error.len,JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL);
        cursor->eof = 0;
        return SQLITE_OK;
    }
}

static void result_buffer(sqlite3_context *context, const thinkthen_buffer *value) {
    if (value->data == NULL) { sqlite3_result_null(context); return; }
    sqlite3_result_text64(context, (const char *)value->data, value->len,
                          SQLITE_TRANSIENT, SQLITE_UTF8);
}

static void json_value(sqlite3_context *context,json_t *value) {
    if(!value || json_is_null(value))sqlite3_result_null(context);
    else if(json_is_string(value))sqlite3_result_text64(context,json_string_value(value),json_string_length(value),SQLITE_TRANSIENT,SQLITE_UTF8);
    else if(json_is_boolean(value))sqlite3_result_int(context,json_is_true(value));
    else if(json_is_integer(value))sqlite3_result_int64(context,json_integer_value(value));
    else if(json_is_real(value))sqlite3_result_double(context,json_real_value(value));
    else {
        char *encoded=json_dumps(value,JSON_COMPACT|JSON_ENCODE_ANY);
        if(!encoded)sqlite3_result_error_nomem(context);
        else {sqlite3_result_text(context,encoded,-1,SQLITE_TRANSIENT);free(encoded);}
    }
}
int cursor_column(sqlite3_vtab_cursor *base,sqlite3_context *context,int column) {
    Cursor *cursor=(Cursor*)base;Table *table=(Table*)base->pVtab;
    if(column>=table->column_count) {
        sqlite3_value *argument=cursor->arguments[table->arguments[column-table->column_count]];
        if(argument)sqlite3_result_value(context,argument);else sqlite3_result_null(context);
        return SQLITE_OK;
    }
    switch(table->columns[column]) {
    case COL_POSITION:sqlite3_result_int64(context,(sqlite3_int64)cursor->row.position);break;
    case COL_STATUS: {
        const char *status=cursor->row.status==THINKTHEN_STATUS_ANSWERED?"answered":cursor->row.status==THINKTHEN_STATUS_UNSURE?"unsure":cursor->row.status==THINKTHEN_STATUS_FAILED?"failed":NULL;
        if(!status)sqlite3_result_error(context,THINKTHEN_ABI_ERROR_CODE ": " THINKTHEN_ABI_ERROR_MESSAGE,-1);else sqlite3_result_text(context,status,-1,SQLITE_STATIC);break;
    }
    case COL_ANSWER:json_value(context,cursor->answer);break;
    case COL_PROBABILITY:if(isnan(cursor->row.probability))sqlite3_result_null(context);else sqlite3_result_double(context,cursor->row.probability);break;
    case COL_CODE:json_value(context,json_object_get(cursor->error,"code"));break;
    case COL_MESSAGE:json_value(context,json_object_get(cursor->error,"message"));break;
    case COL_HTTP:json_value(context,json_object_get(cursor->error,"http_status"));break;
    case COL_DETAIL:result_buffer(context,&cursor->row.detail);break;
    case COL_SOURCE:result_buffer(context,&cursor->row.source);break;
    case COL_KEPT:sqlite3_result_int(context,cursor->row.status==THINKTHEN_STATUS_ANSWERED && cursor->row.answer_bool);break;
    case COL_RANK:if(cursor->row.status==THINKTHEN_STATUS_ANSWERED)sqlite3_result_int64(context,(sqlite3_int64)cursor->row.rank);else sqlite3_result_null(context);break;
    case COL_TEXT:json_value(context,json_object_get(cursor->answer,"text"));break;
    case COL_START:json_value(context,json_object_get(cursor->answer,"start"));break;
    case COL_END:json_value(context,json_object_get(cursor->answer,"end"));break;
    case COL_ID: {
        if(cursor->replaying) {
            if(sqlite3_column_type(cursor->read,1)!=SQLITE_NULL)sqlite3_result_value(context,sqlite3_column_value(cursor->read,1));
            else result_buffer(context,&cursor->row.id);
            break;
        }
        size_t index=(size_t)(cursor->row.position-cursor->ids_offset)-1;
        if(index<cursor->ids_count && cursor->ids[index])sqlite3_result_value(context,cursor->ids[index]);
        else result_buffer(context,&cursor->row.id);
        break;
    }
    }
    return SQLITE_OK;
}
