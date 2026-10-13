"""Own canonical native value trees and expose counted COBOL accessors."""

DECLARATIONS = '''typedef struct tt_value tt_value;
void TT_VALUE_FREE(tt_value *);
int TT_VALUE_KIND(const tt_value *,uint32_t *);
int TT_VALUE_LENGTH(const tt_value *,uint64_t *);
int TT_VALUE_BOOLEAN(const tt_value *,uint32_t *);
int TT_VALUE_TEXT(const tt_value *,char *,uint64_t *);
int TT_VALUE_NAME(const tt_value *,uint64_t,char *,uint64_t *);
int TT_VALUE_ELEMENT(const tt_value *,uint64_t,tt_value **);
int TT_VALUE_FIELD(const tt_value *,const char *,uint64_t,tt_value **);
int TT_VALUE_HAS_FIELD(const tt_value *,const char *,uint64_t,uint32_t *);
int TT_VALUE_NULL(tt_value **);
int TT_VALUE_BOOL(uint32_t,tt_value **);
int TT_VALUE_STRING(const char *,uint64_t,tt_value **);
int TT_VALUE_NUMBER(const char *,uint64_t,tt_value **);
int TT_VALUE_ARRAY(const tt_value *const *,uint64_t,tt_value **);
int TT_VALUE_OBJECT(const thinkthen_cobol_text *,const tt_value *const *,uint64_t,tt_value **);
'''

SOURCE = r'''
#include <string.h>
struct tt_value {arena storage; thinkthen_complete_json_v1 *value;};
static thinkthen_complete_utf8_v1 copy_text(arena *storage,thinkthen_complete_utf8_v1 input) {
    if(input.len && !input.data) {storage->code=THINKTHEN_COBOL_REPRESENTATION; return (thinkthen_complete_utf8_v1){0};}
    char *bytes=allocate(storage,input.len,1);
    if(bytes && input.len) memcpy(bytes,input.data,input.len);
    return (thinkthen_complete_utf8_v1){bytes,input.len};
}
static thinkthen_complete_json_v1 *copy_value(arena *storage,const thinkthen_complete_json_v1 *input) {
    if(!input) {storage->code=THINKTHEN_COBOL_REPRESENTATION; return NULL;}
    thinkthen_complete_json_v1 *out=allocate(storage,1,sizeof(*out));
    if(!out) return NULL;
    out->kind=input->kind;
    switch(input->kind) {
    case THINKTHEN_COMPLETE_JSON_NULL_V1: break;
    case THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: out->data.boolean=input->data.boolean; break;
    case THINKTHEN_COMPLETE_JSON_STRING_V1: out->data.string=copy_text(storage,input->data.string); break;
    case THINKTHEN_COMPLETE_JSON_NUMBER_V1: out->data.number=copy_text(storage,input->data.number); break;
    case THINKTHEN_COMPLETE_JSON_ARRAY_V1: {
        size_t count=input->data.array.len;
        if(count && !input->data.array.data) {storage->code=THINKTHEN_COBOL_REPRESENTATION; break;}
        const thinkthen_complete_json_v1 **items=allocate(storage,count,sizeof(*items));
        out->data.array=(thinkthen_complete_json_array_v1){items,count};
        if(items) for(size_t i=0;i<count && !storage->code;i++) items[i]=copy_value(storage,input->data.array.data[i]);
        break;
    }
    case THINKTHEN_COMPLETE_JSON_OBJECT_V1: {
        size_t count=input->data.object.len;
        if(count && !input->data.object.data) {storage->code=THINKTHEN_COBOL_REPRESENTATION; break;}
        thinkthen_complete_json_entry_v1 *items=allocate(storage,count,sizeof(*items));
        out->data.object=(thinkthen_complete_json_object_v1){items,count};
        if(items) for(size_t i=0;i<count && !storage->code;i++) {
            items[i].name=copy_text(storage,input->data.object.data[i].name);
            items[i].value=copy_value(storage,input->data.object.data[i].value);
        }
        break;
    }
    default: storage->code=THINKTHEN_COBOL_REPRESENTATION; break;
    }
    return out;
}
void TT_VALUE_FREE(tt_value *value) {
    if(value) {release(&value->storage); free(value);}
}
static int own_value(const thinkthen_complete_json_v1 *input,tt_value **out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    tt_value *owner=calloc(1,sizeof(*owner));
    if(!owner) return THINKTHEN_ELOCAL;
    owner->value=copy_value(&owner->storage,input);
    if(owner->storage.code) {int code=owner->storage.code; TT_VALUE_FREE(owner); return code;}
    *out=owner; return 0;
}
int TT_VALUE_KIND(const tt_value *value,uint32_t *out) {
    if(!value || !out) return THINKTHEN_COBOL_REPRESENTATION;
    *out=value->value->kind; return 0;
}
int TT_VALUE_LENGTH(const tt_value *value,uint64_t *out) {
    if(!value || !out) return THINKTHEN_COBOL_REPRESENTATION;
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_ARRAY_V1) *out=value->value->data.array.len;
    else if(value->value->kind==THINKTHEN_COMPLETE_JSON_OBJECT_V1) *out=value->value->data.object.len;
    else return THINKTHEN_COBOL_REPRESENTATION;
    return 0;
}
int TT_VALUE_BOOLEAN(const tt_value *value,uint32_t *out) {
    if(!value || !out || value->value->kind!=THINKTHEN_COMPLETE_JSON_BOOLEAN_V1) return THINKTHEN_COBOL_REPRESENTATION;
    *out=value->value->data.boolean; return 0;
}
int TT_VALUE_TEXT(const tt_value *value,char *out,uint64_t *written) {
    if(!value) return THINKTHEN_COBOL_REPRESENTATION;
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_STRING_V1) return TT_SESSION_TEXT(&value->value->data.string,out,written);
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_NUMBER_V1) return TT_SESSION_TEXT(&value->value->data.number,out,written);
    return THINKTHEN_COBOL_REPRESENTATION;
}
int TT_VALUE_NAME(const tt_value *value,uint64_t position,char *out,uint64_t *written) {
    if(!value || value->value->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1 || !position || position>value->value->data.object.len) return THINKTHEN_COBOL_REPRESENTATION;
    return TT_SESSION_TEXT(&value->value->data.object.data[position-1].name,out,written);
}
int TT_VALUE_ELEMENT(const tt_value *value,uint64_t position,tt_value **out) {
    if(!value || !position) return THINKTHEN_COBOL_REPRESENTATION;
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_ARRAY_V1 && position<=value->value->data.array.len) return own_value(value->value->data.array.data[position-1],out);
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_OBJECT_V1 && position<=value->value->data.object.len) return own_value(value->value->data.object.data[position-1].value,out);
    return THINKTHEN_COBOL_REPRESENTATION;
}
static const thinkthen_complete_json_v1 *field_value(const tt_value *value,const char *name,uint64_t length) {
    if(!value || (length && !name) || value->value->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1) return NULL;
    for(size_t i=0;i<value->value->data.object.len;i++) {
        const thinkthen_complete_json_entry_v1 *field=&value->value->data.object.data[i];
        if(field->name.len==length && (!length || !memcmp(field->name.data,name,length))) return field->value;
    }
    return NULL;
}
int TT_VALUE_FIELD(const tt_value *value,const char *name,uint64_t length,tt_value **out) {
    const thinkthen_complete_json_v1 *field=field_value(value,name,length);
    return field ? own_value(field,out) : THINKTHEN_COBOL_REPRESENTATION;
}
int TT_VALUE_HAS_FIELD(const tt_value *value,const char *name,uint64_t length,uint32_t *out) {
    if(!value || !out || (length && !name) || value->value->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1) return THINKTHEN_COBOL_REPRESENTATION;
    *out=field_value(value,name,length)!=NULL; return 0;
}
int TT_VALUE_NULL(tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_NULL_V1},out);
}
int TT_VALUE_BOOL(uint32_t value,tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_BOOLEAN_V1,.data.boolean=value},out);
}
int TT_VALUE_STRING(const char *value,uint64_t length,tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_STRING_V1,.data.string={value,length}},out);
}
int TT_VALUE_NUMBER(const char *value,uint64_t length,tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_NUMBER_V1,.data.number={value,length}},out);
}
int TT_VALUE_ARRAY(const tt_value *const *values,uint64_t count,tt_value **out) {
    if(count && !values) return THINKTHEN_COBOL_REPRESENTATION;
    arena temporary={0};
    const thinkthen_complete_json_v1 **items=allocate(&temporary,count,sizeof(*items));
    if(items) for(uint64_t i=0;i<count;i++) {
        if(!values[i]) {temporary.code=THINKTHEN_COBOL_REPRESENTATION; break;}
        items[i]=values[i]->value;
    }
    int code=temporary.code ? temporary.code : own_value(&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_ARRAY_V1,.data.array={items,count}},out);
    release(&temporary); return code;
}
int TT_VALUE_OBJECT(const thinkthen_cobol_text *names,const tt_value *const *values,uint64_t count,tt_value **out) {
    if(count && (!values || !names)) return THINKTHEN_COBOL_REPRESENTATION;
    arena temporary={0};
    thinkthen_complete_json_entry_v1 *items=allocate(&temporary,count,sizeof(*items));
    if(items) for(uint64_t i=0;i<count;i++) {
        if(!values[i]) {temporary.code=THINKTHEN_COBOL_REPRESENTATION; break;}
        items[i]=(thinkthen_complete_json_entry_v1){{names[i].data,names[i].len},values[i]->value};
    }
    int code=temporary.code ? temporary.code : own_value(&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_OBJECT_V1,.data.object={items,count}},out);
    release(&temporary); return code;
}
'''
