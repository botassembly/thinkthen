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
        "PRAGMA cache_size=-32; PRAGMA journal_mode=OFF; CREATE TABLE rows(row BLOB,id,answer BLOB,error BLOB,detail BLOB,source BLOB,id_json BLOB)",NULL,NULL,NULL);
    if(result==SQLITE_OK) result=sqlite3_prepare_v2(cursor->replay,
        "INSERT INTO rows VALUES(?,?,?,?,?,?,?)",-1,&cursor->save,NULL);
    return result==SQLITE_OK ? result : cursor_error(cursor,result,"cannot open result spool");
}
int replay_save(Cursor *cursor) {
    size_t index=(size_t)(cursor->row.position-cursor->ids_offset)-1;
    thinkthen_buffer *buffers[]={&cursor->row.answer,&cursor->row.error,&cursor->row.detail,&cursor->row.source,&cursor->row.id};
    int result=sqlite3_bind_blob(cursor->save,1,&cursor->row,sizeof(cursor->row),SQLITE_TRANSIENT);
    if(result==SQLITE_OK) result=index<cursor->ids_count && cursor->ids[index]
        ? sqlite3_bind_value(cursor->save,2,cursor->ids[index])
        : sqlite3_bind_null(cursor->save,2);
    for(int n=0;n<5 && result==SQLITE_OK;n++) {
        thinkthen_buffer *value=buffers[n];
        result=value->data ? sqlite3_bind_blob64(cursor->save,n+3,value->data,value->len,SQLITE_TRANSIENT)
            : sqlite3_bind_null(cursor->save,n+3);
    }
    if(result==SQLITE_OK) {result=sqlite3_step(cursor->save);if(result==SQLITE_DONE)result=SQLITE_OK;}
    sqlite3_reset(cursor->save);
    return result==SQLITE_OK ? result : cursor_error(cursor,result,"cannot save result spool");
}
int replay_next(Cursor *cursor) {
    int result=sqlite3_step(cursor->read);
    cursor->eof=result==SQLITE_DONE;
    if(result==SQLITE_DONE)return SQLITE_OK;
    if(result!=SQLITE_ROW)return cursor_error(cursor,result,"cannot read result spool");
    memcpy(&cursor->row,sqlite3_column_blob(cursor->read,0),sizeof(cursor->row));
    thinkthen_buffer *buffers[]={&cursor->row.answer,&cursor->row.error,&cursor->row.detail,&cursor->row.source,&cursor->row.id};
    for(int n=0;n<5;n++) {
        buffers[n]->data=(uint8_t *)sqlite3_column_blob(cursor->read,n+2);
        buffers[n]->len=(size_t)sqlite3_column_bytes(cursor->read,n+2);
    }
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
