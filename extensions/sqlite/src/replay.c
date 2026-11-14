#include "internal.h"
#include <string.h>

/* A cursor owns a disk-backed spool. It never shares paid results with another
 * cursor or retains them after its SQL statement closes. */
int replay_open(Cursor *cursor) {
    Table *table=(Table *)cursor->base.pVtab;
    sqlite3_stmt *version=NULL;
    int result=sqlite3_prepare_v2(table->db,"PRAGMA data_version",-1,&version,NULL);
    if(result==SQLITE_OK && sqlite3_step(version)==SQLITE_ROW)cursor->data_version=sqlite3_column_int64(version,0);
    else result=SQLITE_ERROR;
    sqlite3_finalize(version);
    if(result!=SQLITE_OK)return cursor_error(cursor,result,"cannot read source version");
    result=sqlite3_open("", &cursor->replay);
    if(result==SQLITE_OK) result=sqlite3_exec(cursor->replay,
        "PRAGMA cache_size=-32; PRAGMA journal_mode=OFF; CREATE TABLE rows(row BLOB,id,host_json BLOB,record);"
        "CREATE TABLE inputs(position INTEGER PRIMARY KEY,id,record)",NULL,NULL,NULL);
    if(result==SQLITE_OK) result=sqlite3_prepare_v2(cursor->replay,
        "INSERT INTO rows VALUES(?,?,?,?)",-1,&cursor->save,NULL);
    if(result==SQLITE_OK) result=sqlite3_prepare_v2(cursor->replay,"INSERT INTO inputs VALUES(?,?,?)",-1,&cursor->input_save,NULL);
    if(result==SQLITE_OK) result=sqlite3_prepare_v2(cursor->replay,"SELECT id,record FROM inputs WHERE position=?",-1,&cursor->input_read,NULL);
    if(result==SQLITE_OK)result=sqlite3_prepare_v2(cursor->replay,"SELECT json_remove(?1,'$.id'), ?1 -> '$.id'",-1,&cursor->projection,NULL);
    return result==SQLITE_OK ? result : cursor_error(cursor,result,"cannot open result spool");
}
/* Retain exact native values, including invalid input storage classes and rank order. */
int replay_save_input(Cursor *cursor,sqlite3_value *id,sqlite3_value *record) {
    int result=sqlite3_bind_int64(cursor->input_save,1,++cursor->input_count);
    if(result==SQLITE_OK)result=id?sqlite3_bind_value(cursor->input_save,2,id):sqlite3_bind_null(cursor->input_save,2);
    if(result==SQLITE_OK)result=sqlite3_bind_value(cursor->input_save,3,record);
    if(result==SQLITE_OK){result=sqlite3_step(cursor->input_save);if(result==SQLITE_DONE)result=SQLITE_OK;}
    sqlite3_reset(cursor->input_save);
    return result;
}
sqlite3_value *row_input(Cursor *cursor,int column) {
    sqlite3_reset(cursor->input_read);
    if(sqlite3_bind_int64(cursor->input_read,1,(sqlite3_int64)cursor->row.position)!=SQLITE_OK ||
       sqlite3_step(cursor->input_read)!=SQLITE_ROW)return NULL;
    return sqlite3_column_value(cursor->input_read,column);
}
sqlite3_value *row_id(Cursor *cursor) {
    return cursor->id_column>=0 ? row_input(cursor,0) : NULL;
}
/* SQLite's own JSON projection removes id before Jansson decodes the row. This
 * retains raw uint64 id digits that Jansson's signed integer type cannot hold. */
int row_projection(Cursor *cursor,const char *json,sqlite3_uint64 length) {
    sqlite3_reset(cursor->projection);
    int result=sqlite3_bind_text64(cursor->projection,1,json,length,SQLITE_TRANSIENT,SQLITE_UTF8);
    if(result==SQLITE_OK)result=sqlite3_step(cursor->projection);
    if(result!=SQLITE_ROW)return cursor_error(cursor,result,"cannot project engine row");
    cursor->fields=json_loadb((const char*)sqlite3_column_text(cursor->projection,0),
        (size_t)sqlite3_column_bytes(cursor->projection,0),JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL);
    if(!cursor->fields)return cursor_error(cursor,SQLITE_ERROR,"cannot decode engine row");
    cursor->projected_id.data=(uint8_t*)sqlite3_column_text(cursor->projection,1);
    cursor->projected_id.len=(size_t)sqlite3_column_bytes(cursor->projection,1);
    return SQLITE_OK;
}
int replay_save(Cursor *cursor) {
    sqlite3_value *id=row_id(cursor);
    thinkthen_buffer json={0};thinkthen_error error=THINKTHEN_ERROR_INIT;
    int result=thinkthen_row_host_json(&cursor->row,&json,&error);
    if(result!=THINKTHEN_OK){
        char *message=NULL;int code=engine_error(&error,&message);
        code=cursor_error(cursor,code,message);sqlite3_free(message);return code;
    }
    /* Only metadata enters the spool; an opaque result pointer is never persisted. */
    thinkthen_row metadata=cursor->row;metadata.private_data=NULL;
    result=sqlite3_bind_blob(cursor->save,1,&metadata,sizeof metadata,SQLITE_TRANSIENT);
    if(result==SQLITE_OK)result=id?sqlite3_bind_value(cursor->save,2,id):sqlite3_bind_null(cursor->save,2);
    if(result==SQLITE_OK)result=sqlite3_bind_blob64(cursor->save,3,json.data,json.len,SQLITE_TRANSIENT);
    if(result==SQLITE_OK)result=sqlite3_bind_value(cursor->save,4,row_input(cursor,1));
    if(result==SQLITE_OK){result=sqlite3_step(cursor->save);if(result==SQLITE_DONE)result=SQLITE_OK;}
    sqlite3_reset(cursor->save);
    if(result==SQLITE_OK)result=row_projection(cursor,(const char*)json.data,json.len);
    thinkthen_buffer_free(&json);thinkthen_error_free(&error);
    return result==SQLITE_OK?result:cursor_error(cursor,result,"cannot save result spool");
}
int replay_next(Cursor *cursor) {
    int result=sqlite3_step(cursor->read);
    cursor->eof=result==SQLITE_DONE;
    if(result==SQLITE_DONE)return SQLITE_OK;
    if(result!=SQLITE_ROW)return cursor_error(cursor,result,"cannot read result spool");
    memcpy(&cursor->row,sqlite3_column_blob(cursor->read,0),sizeof(cursor->row));
    result=row_projection(cursor,(const char*)sqlite3_column_blob(cursor->read,2),
        (sqlite3_uint64)sqlite3_column_bytes(cursor->read,2));
    if(result!=SQLITE_OK)return result;
    ++cursor->rowid;
    return SQLITE_OK;
}
int replay_same(Cursor *cursor,int mask,sqlite3_value **argv) {
    Table *table=(Table *)cursor->base.pVtab;
    if(!cursor->replay || cursor->mask!=mask || cursor->changes!=sqlite3_total_changes64(table->db)
        || cursor->generation!=table->registration->connection->generation)return 0;
    int index=0;
    for(int n=0;n<ARG_COUNT;n++)if(mask&(1<<n)) {
        sqlite3_value *old=cursor->arguments[n], *value=argv[index++];
        int type=sqlite3_value_type(old);
        if(type!=sqlite3_value_type(value))return 0;
        if(type==SQLITE_INTEGER) {if(sqlite3_value_int64(old)!=sqlite3_value_int64(value))return 0;}
        else if(type==SQLITE_FLOAT) {if(sqlite3_value_double(old)!=sqlite3_value_double(value))return 0;}
        else if(sqlite3_value_bytes(old)!=sqlite3_value_bytes(value) ||
            memcmp(sqlite3_value_blob(old),sqlite3_value_blob(value),(size_t)sqlite3_value_bytes(old)))return 0;
    }
    sqlite3_stmt *version=NULL;
    int same=sqlite3_prepare_v2(table->db,"PRAGMA data_version",-1,&version,NULL)==SQLITE_OK
        && sqlite3_step(version)==SQLITE_ROW && sqlite3_column_int64(version,0)==cursor->data_version;
    sqlite3_finalize(version);
    return same;
}
